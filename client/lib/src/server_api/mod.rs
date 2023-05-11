use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(unix)] {
        mod unix;
        pub use self::unix::*;
    } else if #[cfg(windows)] {
        mod windows;
        pub use self::windows::*;
    }
}

#[cfg(test)]
mod test {
    use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
    use tokio_stream::StreamExt;
    use crate::server_api::{init_api_server, send_command};

    #[tokio::test]
    async fn test_api() {
        let (sender, mut receiver) = tokio::sync::mpsc::channel::<super::APISocket>(10);
        init_api_server(sender).unwrap();
        tokio::spawn(async move {
            while let Some(mut d) = receiver.recv().await {
                //let (read, mut write) = d.split();
                const BUFFER_SIZE: usize = 1024;
                let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE];
                let size = d.read(&mut buffer).await.unwrap();
                let command = String::from_utf8_lossy(&buffer[..size]);
                match command.as_ref() {
                    "hello" => {
                        d.write("world".as_bytes()).await.unwrap();
                    }
                    "ping" => {
                        d.write(b"pong").await.unwrap();
                    }
                    _ => {}
                }
            }
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        let result = send_command("hello").await.unwrap();
        assert_eq!(result, "world".to_string());
        let result2 = send_command("ping").await.unwrap();
        assert_eq!(result2, "pong".to_string());
    }
}