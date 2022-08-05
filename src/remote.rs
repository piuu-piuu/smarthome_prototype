// TCP-клиент умной розетки, позволяющий:&
// включать и выключать розетку,
// запрашивать информацию о текущем состоянии и потребляемой мощности розетки.

use std::io::Write;
use std::net::{TcpStream, UdpSocket};
use std::str::from_utf8;

pub fn reach_tcp(device_host: &str, message: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(device_host)?;

    let msg = message.as_bytes();
    stream.write_all(msg)?;
    println!("Sent message '{}', awaiting reply...", message);

    let buffer = [0_u8; 1024];
    let received = from_utf8(&buffer)
        .map_err(|err| ("TCP error {}", err))
        .unwrap();

    println!(">>> {} ", received);
    println!("<<<");
    Ok(received.to_string())
}

pub fn read_udp<'a>(addr: &'a str, command: &'a str) -> String {
    let socket = UdpSocket::bind(addr).expect("Could not bind client socket");
    socket
        .connect("127.0.0.1:8888")
        .expect("Could not connect to server");
    let input = String::from(command);
    let mut buffer = [0u8; 1500];

    socket
        .send(input.as_bytes())
        .expect("Failed to write to server");

    socket
        .recv_from(&mut buffer)
        .expect("Could not read into buffer");
    print!(
        "{}",
        std::str::from_utf8(&buffer).expect("Could not write buffer as string")
    );
    let res: Vec<&str> = std::str::from_utf8(&buffer).unwrap().split(' ').collect();
    res[0].to_string()
}
