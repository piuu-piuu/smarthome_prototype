use std::error::Error;
use std::io;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::UdpSocket;

#[tokio::main]
pub async fn atcp_serve() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        async {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        }
        .await;
    }
}

// An UDP echo server that just sends back everything that it receives.

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}

impl Server {
    async fn run(self) -> Result<(), io::Error> {
        let Server {
            socket,
            mut buf,
            mut to_send,
        } = self;
        let input = String::from("20`C \n");
        loop {
            // are there incoming connection?
            if let Some((_size, peer)) = to_send {
                println!("Client {} connected", peer);
                socket.send_to(input.as_bytes(), peer).await?;
            }
            to_send = Some(socket.recv_from(&mut buf).await?);
        }
    }
}

#[tokio::main]
pub async fn audp_serve() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8888".to_string();

    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket,
        buf: vec![0; 1024],
        to_send: None,
    };

    // This starts the server task.
    server.run().await?;

    Ok(())
}
