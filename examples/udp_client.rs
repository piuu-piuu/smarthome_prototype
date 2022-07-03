// use smarthome_4::devices::*;
// use smarthome_4::remote::*;
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
    let mut buf = [0; 10];
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
    println!("{:?}", buf)
}
