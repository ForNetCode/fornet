use shell_candy::{ShellTask, ShellTaskBehavior, ShellTaskLog, ShellTaskOutput};
use crate::protobuf::config::Interface;

#[derive(Default,Debug)]
pub struct Scripts {
    pub pre_up: Option<String>,
    pub post_up: Option<String>,
    pub pre_down: Option<String>,
    pub post_down: Option<String>,
}

impl Scripts {
    pub fn load_from_interface(interface: &Interface) -> Self {
        Self {
            pre_up: interface.pre_up.clone(),
            post_up: interface.post_up.clone(),
            pre_down: interface.pre_down.clone(),
            post_down: interface.post_down.clone(),
        }
    }
}
//TODO: add log and handle if this would block.
pub fn run_opt_script(script:&Option<String>) -> shell_candy::Result<Option<()>> {
    if let Some(ref script) = script {
        Ok(Some(run_script(script)?))
    } else {
        Ok(None)
    }
}

pub fn run_script(script: &str) -> shell_candy::Result<()> {
    for script in script.split(";") {
        let task = ShellTask::new(script.trim())?;
        match task.run(|line| {
            match line {
                ShellTaskLog::Stdout(data)=> {
                    tracing::debug!("shell output: {data}");
                },
                ShellTaskLog::Stderr(error) => {
                    tracing::warn!("shell output: {error}");
                }
            }
            ShellTaskBehavior::<()>::Passthrough
        }) {
            Ok(data) => {
                tracing::info!("script success:{script}");
                //Ok(data)
            },
            Err(e) => {
                tracing::info!("script failure:{e}");
                return Err(e)
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use shell_candy::{ShellTaskLog, ShellTaskBehavior, ShellTask};

    #[test]
    fn test_ping() {
        let task = ShellTask::new("rustc --version").unwrap();
        task.run(|line| {
            match line {
                ShellTaskLog::Stdout(message) | ShellTaskLog::Stderr(message) => eprintln!("info: {}", &message),
            }
            ShellTaskBehavior::<()>::Passthrough
        }).unwrap();
    }


    #[test]
    fn script_split(){
        let scripts = "iptables -A FORWARD -i for0 -j ACCEPT; iptables -A FORWARD -o for0 -j ACCEPT ; iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE";
        for script in scripts.split(";"){

            println!("script: {script}, {}", script.trim());
        }
    }
}
