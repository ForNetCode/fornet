use std::sync::Arc;
use std::time::Duration;

use webrtc::api::media_engine::MediaEngine;
use webrtc::api::APIBuilder;
use webrtc::api::interceptor_registry::{configure_nack, configure_rtcp_reports};
use webrtc::ice_transport::ice_credential_type::RTCIceCredentialType;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::policy::ice_transport_policy::RTCIceTransportPolicy;
use webrtc::peer_connection::RTCPeerConnection;

const TURN_SERVER: &str = "turn:113.31.103.71:13478";
const SELF_TURN_SERVER: &str = "turn:192.168.31.37:3478";


pub async fn simple() -> anyhow::Result<()> {
    let answer_connection = answer().await?;
    let offer_connection = offer().await?;
    let offer = offer_connection.create_offer(None).await?;
    offer_connection.set_local_description(offer.clone()).await?;
    let mut gather_complete = offer_connection.gathering_complete_promise().await;
    let _ = gather_complete.recv().await;


    let offer = offer_connection.local_description().await.unwrap();
    tracing::info!("Offer: {offer:?}");

    answer_connection.set_remote_description(offer).await?;


    let answer = answer_connection.create_answer(None).await?;
    answer_connection.set_local_description(answer.clone()).await?;
    let mut gather_complete = answer_connection.gathering_complete_promise().await;

    let _ = gather_complete.recv().await;

    tracing::info!("answer collect Session Description finish");
    let answer_local_description = answer_connection.local_description().await.unwrap();
    //tracing::info!("answer description: {answer_local_description:?}");


    offer_connection.set_remote_description(answer_local_description).await?;
    Ok(())
}

async fn answer() -> anyhow::Result<Arc<RTCPeerConnection>> {
    // let config = RTCConfiguration {
    //     ice_servers: vec![RTCIceServer {
    //         urls: vec!["stun:stun.l.google.com:19302".to_owned()],
    //         ..Default::default()
    //     }],
    //     ..Default::default()
    // };
    //let config = RTCConfiguration::default();

    let config = RTCConfiguration {
        ice_servers: vec![
            RTCIceServer {
                urls: vec![TURN_SERVER.to_owned()],
                username: "answer".to_owned(),
                credential: "test".to_owned(),
                credential_type: RTCIceCredentialType::Password,
            },
            RTCIceServer {
                urls: vec![SELF_TURN_SERVER.to_owned()],
                username: "answer".to_owned(),
                credential: "test".to_owned(),
                credential_type: RTCIceCredentialType::Password,
            },
        ],
        ice_transport_policy: RTCIceTransportPolicy::Relay,
        //peer_identity: "offer".to_owned(),
        ..Default::default()
    };
    let mut m = MediaEngine::default();
    let mut registry = Registry::new();
    //register_default_interceptors(registry, &mut m);
    registry = configure_nack(registry, &mut m);
    registry = configure_rtcp_reports(registry);
    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
        .build();

    let peer_connection = Arc::new(api.new_peer_connection(config).await?);
    let w_pc = Arc::downgrade(&peer_connection);
    peer_connection.on_ice_candidate(Box::new(move |c| {
        let w_pc2 = w_pc.clone();
        Box::pin(async move {
            if let Some(c) = c {
                if let Some(w_pc) = w_pc2.upgrade() {
                    let remote_description = w_pc.remote_description().await;
                    tracing::info!("answer on ice connection: {remote_description:?}")
                }
            }
        })
    }));


    peer_connection.on_peer_connection_state_change(Box::new(move |s| {
        tracing::info!("answer peer connection State has changed: {s}");

        Box::pin(async {})
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

async fn offer() -> anyhow::Result<Arc<RTCPeerConnection>> {

    // let config = RTCConfiguration {
    //     ice_servers: vec![RTCIceServer {
    //         urls: vec!["stun:stun.l.google.com:19302".to_owned()],
    //         ..Default::default()
    //     }],
    //     ..Default::default()
    // };
    let config = RTCConfiguration {
        ice_servers: vec![
            RTCIceServer {
                urls: vec![TURN_SERVER.to_owned()],
                username: "offer".to_owned(),
                credential: "test".to_owned(),
                credential_type: RTCIceCredentialType::Password,
            },
            RTCIceServer {
                urls: vec![SELF_TURN_SERVER.to_owned()],
                username: "offer".to_owned(),
                credential: "test".to_owned(),
                credential_type: RTCIceCredentialType::Password,
            }
        ],
        ice_transport_policy: RTCIceTransportPolicy::Relay,

        //peer_identity: "answer".to_owned(),
        ..Default::default()
    };

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

    let peer_connection = Arc::new(api.new_peer_connection(config).await?);
    let w_pc = Arc::downgrade(&peer_connection);
    peer_connection.on_ice_candidate(Box::new(move |c| {
        let w_pc2 = w_pc.clone();
        Box::pin(async move {
            if let Some(c) = c {
                if let Some(w_pc) = w_pc2.upgrade() {
                    let remote_description = w_pc.remote_description().await;
                    tracing::info!(" on ice connection: {remote_description:?}")
                }
            }
        })
    }));


    peer_connection.on_peer_connection_state_change(Box::new(move |s| {
        println!("Peer Connection State has changed: {s}");

        Box::pin(async {})
    }));

    let data_channel = peer_connection.create_data_channel("data", None).await?;

    let dc = Arc::downgrade(&data_channel);
    data_channel.on_open(Box::new(move || {
        //let data_channel = data_channel.clone();
        tracing::info!(" DataChannel Open");
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
