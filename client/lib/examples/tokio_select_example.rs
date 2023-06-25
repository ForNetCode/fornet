use std::time::Duration;

async fn interval() {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        println!("interval...")
    }
}
#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let task = tokio::spawn(async move {
       loop {
           tokio::select! {
               _ = interval() => {
                   println!("interval finish");
               }
           }
       }
    });
    tokio::time::sleep(Duration::from_secs(5)).await;
    task.abort();
    println!("task is finish: {}", task.is_finished());
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("task is finish: {}", task.is_finished());
    tokio::signal::ctrl_c().await.unwrap();
    Ok(())
}