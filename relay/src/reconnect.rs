use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Sender;

use webrtc::api::media_engine::MediaEngine;
use webrtc::api::APIBuilder;
use webrtc::api::interceptor_registry::{configure_nack, configure_rtcp_reports};
use webrtc::ice_transport::ice_candidate::{RTCIceCandidate, RTCIceCandidateInit};
use webrtc::ice_transport::ice_credential_type::RTCIceCredentialType;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::offer_answer_options::RTCOfferOptions;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::policy::ice_transport_policy::RTCIceTransportPolicy;
use webrtc::peer_connection::RTCPeerConnection;

const TURN_SERVER: &str = "turn:113.31.103.71:13478";
const SELF_TURN_SERVER: &str = "turn:192.168.31.37:3478";


fn get_relay_config(user_name:String) -> RTCConfiguration {
    RTCConfiguration {
        ice_servers: vec![RTCIceServer {
            urls: vec![TURN_SERVER.to_owned()],
            username: user_name.clone(),
            credential: "test".to_owned(),
            credential_type: RTCIceCredentialType::Password,
        }, RTCIceServer {
            urls: vec![SELF_TURN_SERVER.to_owned()],
            username: user_name.clone(),
            credential: "test".to_owned(),

            credential_type: RTCIceCredentialType::Password,
        }],
        ice_transport_policy: RTCIceTransportPolicy::Relay,
//peer_identity: "offer".to_owned(),
        ..Default::default()
    }
}

fn get_default_config() -> RTCConfiguration {
    return RTCConfiguration {
        ..Default::default()
    };
}


pub async fn reconnect() -> anyhow::Result<()> {
    let (offer_sender, mut offer_receiver) = tokio::sync::mpsc::channel::<RTCIceCandidate>(10);
    let (answer_sender, mut answer_receiver) = tokio::sync::mpsc::channel::<RTCIceCandidate>(10);
    let (offer_close_sender, mut offer_close_receiver) = tokio::sync::mpsc::channel::<()>(5);
    let (answer_close_sender, mut answer_close_receiver) = tokio::sync::mpsc::channel::<()>(5);


    let answer_connection = answer(answer_sender, answer_close_sender).await?;
    let offer_connection = offer(offer_sender, offer_close_sender).await?;

    let a_c = answer_connection.clone();
    let o_c = offer_connection.clone();

    tokio::spawn(async move {
        while let Some(candidate) = offer_receiver.recv().await {
            tracing::info!("offer candidate: {candidate:?}");
            let _ = a_c.add_ice_candidate(candidate.to_json().unwrap()).await;
        }
        tracing::info!("offer listen candidate finish");
    });

    tokio::spawn(async move {
        while let Some(candidate) = answer_receiver.recv().await {
            tracing::info!("answer candidate: {candidate:?}");
            let _ = o_c.add_ice_candidate(candidate.to_json().unwrap()).await;
        }
        tracing::info!("answer listen candidate finish");
    });

    let offer = offer_connection.create_offer(None).await?;
    offer_connection.set_local_description(offer.clone()).await?;

    // let mut gather_complete = offer_connection.gathering_complete_promise().await;
    // let _ = gather_complete.recv().await;
    // let offer = offer_connection.local_description().await.unwrap();

    tracing::info!("Offer: {offer:?}");

    answer_connection.set_remote_description(offer).await?;


    let answer = answer_connection.create_answer(None).await?;
    answer_connection.set_local_description(answer.clone()).await?;

    // let mut gather_complete= answer_connection.gathering_complete_promise().await;
    // let _ = gather_complete.recv().await;
    // let answer = answer_connection.local_description().await.unwrap();
    // tracing::info!("answer collect Session Description finish");

    offer_connection.set_remote_description(answer).await?;

    tokio::spawn(async move {
        while let Some(_) = offer_close_receiver.recv().await {
            tokio::time::sleep(Duration::from_secs(2)).await;
            let offer = offer_connection.create_offer(Some(RTCOfferOptions{ice_restart: true, ..Default::default()})).await.unwrap();
            offer_connection.set_local_description(offer.clone()).await.unwrap();
            let _ = answer_connection.set_remote_description(offer).await;

            let answer = answer_connection.create_answer(None).await.unwrap();
            answer_connection.set_local_description(answer.clone()).await.unwrap();
            let _ = offer_connection.set_remote_description(answer).await;
        }
    });

    Ok(())
}

async fn answer(sender: Sender<RTCIceCandidate>, close_sender: Sender<()>) -> anyhow::Result<Arc<RTCPeerConnection>> {
    // let config = RTCConfiguration {
    //     ice_servers: vec![RTCIceServer {
    //         urls: vec!["stun:stun.l.google.com:19302".to_owned()],
    //         ..Default::default()
    //     }],
    //     ..Default::default()
    // };
    //let config = RTCConfiguration::default();


    let mut m = MediaEngine::default();
    let mut registry = Registry::new();
    //register_default_interceptors(registry, &mut m);
    registry = configure_nack(registry, &mut m);
    registry = configure_rtcp_reports(registry);
    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
        .build();

    let peer_connection = Arc::new(api.new_peer_connection(get_relay_config("answer".to_owned())).await?);
    let w_pc = Arc::downgrade(&peer_connection);
    let sender = Arc::new(sender);
    peer_connection.on_ice_candidate(Box::new(move |c| {
        let w_pc2 = w_pc.clone();

        let sender = sender.clone();
        Box::pin(async move {
            if let Some(c) = c {
                let _ = sender.send(c).await;
            }
        })
    }));


    peer_connection.on_peer_connection_state_change(Box::new(move |s| {
        tracing::info!("answer peer connection state has changed: {s}");
        Box::pin(async {
            //let _ = close_sender.send(()).await;
        })
    }));

    peer_connection.on_data_channel(Box::new(move |d| {
        let d_label = d.label().to_owned();
        let d_id = d.id();
        tracing::info!("New DataChannel {d_label} {d_id}");
        Box::pin(async move {
            let d2 = Arc::clone(&d);
            let d_label2 = d_label.clone();
            let d_id2 = d_id;
            d.on_open(Box::new(move || {
                tracing::info!("Data channel '{d_label2}'-'{d_id2}' open. Random messages will now be sent to any connected DataChannels every 5 seconds");
                Box::pin(async {})
            }));
            d.on_message(Box::new(move |msg| {
                let msg_str = String::from_utf8(msg.data.to_vec()).unwrap();
                tracing::info!("answer receive data label: {d_label}, data: {msg_str}");
                Box::pin(async {})
            }));
        })
    }));


    Ok(peer_connection)
}

async fn offer(sender: Sender<RTCIceCandidate>, close_sender: Sender<()>) -> anyhow::Result<Arc<(RTCPeerConnection)>> {
    //let (sender, receiver) = tokio::sync::mpsc::channel::<RTCSessionDescription>(10);
    // let config = RTCConfiguration {
    //     ice_servers: vec![RTCIceServer {
    //         urls: vec!["stun:stun.l.google.com:19302".to_owned()],
    //         ..Default::default()
    //     }],
    //     ..Default::default()
    // };


    //let config = RTCConfiguration::default();
    let mut m = MediaEngine::default();
    let mut registry = Registry::new();
    //register_default_interceptors(registry, &mut m);
    registry = configure_nack(registry, &mut m);
    registry = configure_rtcp_reports(registry);
    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
        .build();

    let peer_connection = Arc::new(api.new_peer_connection(get_relay_config("offer".to_owned())).await?);
    let w_pc = Arc::downgrade(&peer_connection);
    let sender = Arc::new(sender);
    peer_connection.on_ice_candidate(Box::new(move |c| {
        let w_pc2 = w_pc.clone();
        let sender = sender.clone();
        Box::pin(async move {
            if let Some(c) = c {
                let _ = sender.send(c).await;
            }
        })
    }));


    peer_connection.on_peer_connection_state_change(Box::new(move |s| {
        tracing::info!("offer peer connection state has changed: {s}");
        let close_sender = close_sender.clone();
        Box::pin(async move {
            if s == RTCPeerConnectionState::Failed {
                let _ = close_sender.send(()).await;
            }
        })
    }));

    let data_channel = peer_connection.create_data_channel("data", None).await?;

    let dc = Arc::downgrade(&data_channel);
    data_channel.on_open(Box::new(move || {
        //let data_channel = data_channel.clone();
        tracing::info!("DataChannel Open");
        Box::pin(async move {
            loop {
                if let Some(data_channel) = dc.upgrade() {
                    data_channel.send_text("1".to_owned()).await;
                    tokio::time::sleep(Duration::from_secs(5)).await;
                } else {
                    break;
                }
            }
            //
        })
    }));
    Ok(peer_connection)
}
