use std::sync::Arc;
use anyhow::anyhow;
use tracing_subscriber;

use webrtc::api::media_engine::MediaEngine;
use webrtc::api::APIBuilder;
use webrtc::api::interceptor_registry::{configure_nack, configure_rtcp_reports};
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::RTCPeerConnection;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    tracing_subscriber::fmt::init();
    //let answer_connection = answer().await?;

    let offer_connection = offer().await?;
    let offer = offer_connection.create_offer(None).await?;
    tracing::info!("Offer: {offer:?}");
    offer_connection.set_local_description(offer.clone()).await?;

    //answer_connection.set_remote_description(offer).await?;
    //let answer = answer_connection.create_answer(None).await?;
    //answer_connection.set_local_description(answer.clone()).await?;
    //offer_connection.set_remote_description(answer).await?;

    tokio::signal::ctrl_c().await?;
    Ok(())
}

async fn answer() -> anyhow::Result<Arc<RTCPeerConnection>> {
    let config = RTCConfiguration {
        ice_servers: vec![RTCIceServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            ..Default::default()
        }],
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
    peer_connection.on_ice_candidate(Box::new(move |c|{
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


    peer_connection.on_peer_connection_state_change(Box::new(move |s|{
        tracing::info!("Answer Peer Connection State has changed: {s}");
        Box::pin(async {  })
    }));

    peer_connection.on_data_channel(Box::new(move |d| {
        let d_label = d.label().to_owned();
        let d_id = d.id();
        tracing::info!("New DataChannel {d_label} {d_id}");
        Box::pin(async move {
            let d2 =  Arc::clone(&d);
            let d_label2 = d_label.clone();
            let d_id2 = d_id;
            d.on_open(Box::new(move|| {
                tracing::info!("Data channel '{d_label2}'-'{d_id2}' open. Random messages will now be sent to any connected DataChannels every 5 seconds");
                Box::pin(async {  })
            }));
            d.on_message(Box::new(move |msg|{
                tracing::info!("anwser receive data label: {d_label}:");
                Box::pin(async {  })
            }));
        })
    }));



    Ok(peer_connection)
}
async fn offer() -> anyhow::Result<Arc<RTCPeerConnection>>{

    let config = RTCConfiguration {
        ice_servers: vec![RTCIceServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            ..Default::default()
        }],
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
    peer_connection.on_ice_candidate(Box::new(move |c|{
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


    peer_connection.on_peer_connection_state_change(Box::new(move |s|{
        println!("Peer Connection State has changed: {s}");

        Box::pin(async {  })
    }));

    let data_channel = peer_connection.create_data_channel("data",None).await?;

    //let dc = Arc::downgrade(&data_channel);
    data_channel.on_open(Box::new(move || {
        tracing::info!(" DataChannel Open");
        Box::pin(async {
            //data_channel.send_text("".to_owned()).await?;
        })
    }));
    Ok(peer_connection)
}
