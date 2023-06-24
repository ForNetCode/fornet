use std::str::FromStr;
use clap::{Arg, Command};
use tracing_subscriber::EnvFilter;
use fornet_lib::api::ApiResponse;
use fornet_lib::wr_manager::DeviceInfoResp;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
pub async fn main() -> anyhow::Result<()> {
    let mut command = Command::new("fornet-cli")
        .version(env!("CARGO_PKG_VERSION"))
        .author("ForNetCode <zsy.evan@gmail.com>")
        .subcommand(
            Command::new("join").arg(Arg::new("invite_token").help("base64 token").required(true)),
        ).subcommand(
        Command::new("list").about("list connected network status, it now only support connect one network"),
    );

    if cfg!(target_os = "macos") {
        command = command.subcommand(
            Command::new("autoLaunch")
                .about("auto launch when os up")
                .subcommands([
                    Command::new("enable").about("enable auto launch"),
                    Command::new("disable").about("disable auto launch"),
                    Command::new("status").about("if auto launch is enabled"),
                ])
        );
    }


    let matches = command.get_matches();


    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::from_str("info").unwrap()))
        .init();

    match matches.subcommand() {
        Some(("join", sub_matches)) => {
            let invite_code = sub_matches
                .get_one::<String>("invite_token")
                .expect("required invite_token");
            match fornet_lib::api::command_api::join_network(invite_code).await {
                Ok(mut stream) => {
                    while let Ok(Some(message)) = stream.next_line().await {
                        let message = serde_json::from_str::<ApiResponse<String>>(&message).unwrap();
                        if message.ok {
                            println!("{}", message.data);
                        } else {
                            eprintln!("{}", message.data);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("join network error: {:?}", e);
                }
            }
        }
        Some(("list", _)) => {
            match fornet_lib::api::command_api::list_network().await {
                Ok(networks) => {
                    let networks: ApiResponse<Vec<DeviceInfoResp>> = serde_json::from_str(&networks).unwrap();
                    let networks: Vec<String> = networks.data.into_iter().map(|x| x.name).collect();
                    println!("{}", networks.join(","));
                }
                Err(e) => {
                    eprintln!("list network error: {:?}", e);
                }
            }
        }
        Some(("autoLaunch", sub_match)) => {
            let sub_command = sub_match.subcommand().map(|(v, _)| v).unwrap_or("");
            match fornet_lib::api::command_api::auto_launch(sub_command).await {
                Ok(data) => {
                    let response: ApiResponse<String> = serde_json::from_str(&data).unwrap();
                    println!("{}", response.data);
                }
                Err(e) => { eprintln!("{}", e); }
            }
        }
        _ => {
            println!("please type: fornet-cli --help for more information.")
        }
    }
    Ok(())
}





