use std::error::Error;
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn run_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("Client {} disconnected", addr);
                        return;
                    }
                    Ok(n) => {
                        let data = &buf[..n];
                        println!(
                            "Received from {}: {:?}",
                            addr,
                            String::from_utf8_lossy(data)
                        );

                        if let Err(e) = socket.write_all(data).await {
                            eprintln!("Failed to write to {}: {}", addr, e);
                            return;
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // Not ready yet, just keep looping
                        continue;
                    }
                    Err(e) => {
                        eprintln!("Error on {}: {}", addr, e);
                        return;
                    }
                }
            }
        });
    }
}
