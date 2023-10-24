use tracing_subscriber;
mod simple;
mod reconnect;
mod simple_stun;




#[tokio::main]
async fn main() -> anyhow::Result<()>{
    tracing_subscriber::fmt::init();
    //simple::simple().await.unwrap();
    simple_stun::simple_stun().await.unwrap();
    //reconnect::reconnect().await.unwrap();
    tokio::signal::ctrl_c().await?;
    Ok(())
}