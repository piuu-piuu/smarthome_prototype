use std::error::Error;
use std::net::SocketAddr;
use std::str::from_utf8;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::net::UdpSocket;

#[tokio::main]
pub async fn areach_tcp(device_host: &str, message: &str) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(device_host).await?;

    let msg = message.as_bytes();
    stream.write_all(msg).await?;
    println!("Sent message '{}', awaiting reply...", message);

    let mut buffer = [0_u8; 1024];
    let read_len = stream.read(&mut buffer).await?;
    let received = from_utf8(buffer.get(0..read_len).unwrap()).unwrap();
    println!(">>> {} \n<<<", received);
    Ok(())
}

#[tokio::main]
pub async fn audp_client() -> Result<(), Box<dyn Error>> {
    let remote_addr = SocketAddr::from(([127, 0, 0, 1], 8888));
    // use port 0 to let the operating system allocate an available port for us.
    let local_addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 0));

    let socket = UdpSocket::bind(local_addr).await?;
    socket.connect(&remote_addr).await?;
    // command has no actual meaning here, as device is multicasting
    let command = "read".as_bytes();
    socket.send(&command).await?;

    const MAX_DATAGRAM_SIZE: usize = 65_507;
    let mut data = vec![0u8; MAX_DATAGRAM_SIZE];
    let len = socket.recv(&mut data).await?;
    println!("{}", String::from_utf8_lossy(&data[..len]));
    Ok(())
}
