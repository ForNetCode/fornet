use tracing_subscriber;
mod simple;
mod reconnect;
mod simple_stun;

mod simple_ice_api;

mod simple_tcp;




#[tokio::main]
async fn main() -> anyhow::Result<()>{
    tracing_subscriber::fmt::init();
    //simple::simple().await.unwrap();
    //simple_stun::simple_stun().await.unwrap();
    //reconnect::reconnect().await.unwrap();

    //simple_ice_api::simple_ice_api().await.unwrap();
    simple_tcp::simple_tcp().await.unwrap();
    tokio::signal::ctrl_c().await?;
    Ok(())
}