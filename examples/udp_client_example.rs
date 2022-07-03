use std::net::UdpSocket;
use std::{io, str};

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8000").expect("Could not bind client socket");
    socket
        .connect("127.0.0.1:8888")
        .expect("Could not connect to server");
    // loop {
    let mut input = String::new();
    input = String::from("read");
    let mut buffer = [0u8; 1500];

    socket
        .send(input.as_bytes())
        .expect("Failed to write to server");

    socket
        .recv_from(&mut buffer)
        .expect("Could not read into buffer");
    print!(
        "{}",
        str::from_utf8(&buffer).expect("Could not write buffer as string")
    );
    // }
}
