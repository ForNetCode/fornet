use std::sync::Arc;
use std::time::Duration;

use webrtc_ice::agent::Agent;
use webrtc_ice::agent::agent_config::AgentConfig;
use webrtc_ice::candidate::Candidate;
use webrtc_ice::candidate::candidate_base::unmarshal_candidate;
use webrtc_ice::network_type::NetworkType;
use webrtc_ice::udp_network::UDPNetwork;
use webrtc_ice::url::Url;
use webrtc_util::Conn;

//const TURN_SERVER: &str = "turn:113.31.103.71:13478";
const SELF_TURN_SERVER: &str = "turn:192.168.31.37:3478";


pub async fn simple_ice_api() -> anyhow::Result<()> {
    let answer = create_agent().await?;
    let offer = create_agent().await?;
    //offer.gather_candidates()?;
    //answer.gather_candidates()?;


    let _offer = offer.clone();
    answer.on_candidate(Box::new(move |c|{
        let offer = _offer.clone();
        Box::pin(async move{
            if let Some(c) = c {
                let c = c.marshal();
                tracing::info!("answer candidate: {c}");
                let c: Arc<dyn Candidate + Send + Sync> = Arc::new(unmarshal_candidate(&c).unwrap());
                offer.add_remote_candidate(&c);
            }
        })
    }));

    let _answer = answer.clone();
    offer.on_candidate(Box::new(move |c|{
        let answer = _answer.clone();
        Box::pin(async move{
            if let Some(c) = c {
                let c = c.marshal();
                tracing::info!("offer candidate: {c}");
                let c: Arc<dyn Candidate + Send + Sync> = Arc::new(unmarshal_candidate(&c).unwrap());
                answer.add_remote_candidate(&c);
            }
        })
    }));



    let offer_credentials = offer.get_local_user_credentials().await;
    let answer_credentials = answer.get_local_user_credentials().await;




    offer.gather_candidates()?;
    answer.gather_candidates()?;



    tokio::spawn(async move {
        let (_cancel_tx1, cancel_rx1) = tokio::sync::mpsc::channel::<()>(1);
        match answer.accept(cancel_rx1, offer_credentials.0, offer_credentials.1).await {
             Ok(answer_conn) => {
                 let mut buf = vec![0u8; 1600];
                 while let Ok(n) = answer_conn.recv(&mut buf).await {
                     tracing::info!("Received: '{}'", std::str::from_utf8(&buf[..n]).unwrap());
                 }
            }
            Err(e) => {
                tracing::warn!("answer error:{e}");
            }
        }

    });

    tokio::spawn(async move{
        let (_cancel_tx2, cancel_rx2) = tokio::sync::mpsc::channel::<()>(1);
        tokio::time::sleep(Duration::from_secs(2)).await;
        match offer.dial(cancel_rx2, answer_credentials.0, answer_credentials.1).await {
           Ok(offer_conn) => {
                loop {
                    let _ = offer_conn.send(b"123").await;
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
            Err(e) =>  tracing::warn!("offer error:{e}")
        };


    });

    Ok(())
}

async fn create_agent() -> anyhow::Result<Arc<Agent>>{
    let config = AgentConfig {
        urls: vec![Url::parse_url(SELF_TURN_SERVER).unwrap()],
        udp_network: UDPNetwork::Ephemeral(Default::default()),
        network_types: vec![NetworkType::Udp4],
        ..Default::default()
    };
    let ice_agent = Arc::new(Agent::new(config).await?);
    ice_agent.on_connection_state_change(Box::new(move |c| {
        println!("ICE Connection State has changed: {c}");
        Box::pin(async move {})
    }));

    return Ok(ice_agent);
}
