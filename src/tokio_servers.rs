use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::str::from_utf8;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::UdpSocket;

#[tokio::main]
pub async fn atcp_serve(device_info: &str) -> Result<(), Box<dyn std::error::Error>> {
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
                // if let Err(e) = socket.write_all(&buf[0..n]).await {
                //     eprintln!("failed to write to socket; err = {:?}", e);
                //     return;
                // }

                let received = from_utf8(buf.get(0..n).unwrap()).unwrap();
                println!("Request: {}", received);

                match received {
                    "info" => {
                        let response = device_info.as_bytes();
                        socket.write_all(response).await.expect("Command failed");
                    }
                    "on" => {
                        let response = "Socket  ON";
                        socket
                            .write_all(response.as_bytes())
                            .await
                            .expect("Command failed");
                    }
                    "off" => {
                        let response = "Socket  OFF";
                        socket
                            .write_all(response.as_bytes())
                            .await
                            .expect("Command failed");
                    }
                    _ => {
                        let response = "Unknown cmd";
                        socket
                            .write_all(response.as_bytes())
                            .await
                            .expect("Connection error");
                    }
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
    async fn run(self, dev_info: &str) -> Result<(), io::Error> {
        let Server {
            socket,
            mut buf,
            mut to_send,
        } = self;
        let input = String::from(dev_info);
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
pub async fn audp_serve(addr: &str, dev_info: &str) -> Result<(), Box<dyn Error>> {
    // let addr = "127.0.0.1:8888".to_string();

    let socket = UdpSocket::bind(addr).await?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket,
        buf: vec![0; 1024],
        to_send: None,
    };

    // This starts the server task.
    server.run(dev_info).await?;

    Ok(())
}
