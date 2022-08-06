// TCP-клиент умной розетки, позволяющий:&
// включать и выключать розетку,
// запрашивать информацию о текущем состоянии и потребляемой мощности розетки.

use std::io::Write;
use std::net::{TcpStream, UdpSocket};
use std::str::from_utf8;

use crate::errors::NetworkError;

pub fn reach_tcp(device_host: &str, message: &str) -> Result<String, NetworkError> {
    let msg = message.as_bytes();
    let stream = TcpStream::connect(device_host);
    match stream {
        Ok(_) => {
            println!("TCP connection established.")
        }
        Err(_e) => return Err(NetworkError::TcpConnectionError),
    }
    let rx_message = stream.unwrap().write_all(msg);
    match rx_message {
        Ok(_) => {}
        Err(_e) => return Err(NetworkError::TcpWriteError),
    }
    println!("Sent message '{}', awaiting reply...", message);

    let buffer = [0_u8; 1024];
    let received = from_utf8(&buffer);
    let received = match received {
        Ok(content) => {
            println!(">>> {} ", content);
            println!("<<<");
            Ok(content.to_string())
        }
        Err(_) => Err(NetworkError::TcpConnectionError),
    };
    return received;
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
