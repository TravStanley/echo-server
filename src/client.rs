use std::error::Error;
use std::io;
use tokio::io::Interest;
use tokio::net::TcpStream;
use tokio::time::{Duration, interval};

pub async fn run_client() -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let mut buf = vec![0; 1024];

    let mut ticker = interval(Duration::from_secs(5));

    loop {
        tokio::select! {
            ready = stream.ready(Interest::READABLE) => {
                let ready = ready?;
                if ready.is_readable() {
                    match stream.try_read(&mut buf) {
                        Ok(0) => {
                            println!("connection closed by server");
                            break;
                        }
                        Ok(n) => {
                            println!("read {} bytes: {:?}", n, &buf[..n]);
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                        Err(e) => return Err(e.into()),
                    }
                }
            }

            // 5 second timer to stop spam
            _ = ticker.tick() => {
                let ready = stream.ready(Interest::WRITABLE).await?;
                if ready.is_writable() {
                    match stream.try_write(b"hello world\n") {
                        Ok(n) => println!("wrote {} bytes", n),
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                        Err(e) => return Err(e.into()),
                    }
                }
            }
        }
    }

    Ok(())
}
