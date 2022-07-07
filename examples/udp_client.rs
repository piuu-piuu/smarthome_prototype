use smarthome_4::remote::*;
use std::net::UdpSocket;
use std::{io, str};

fn main() {
    // "127.0.0.1:8000"
    read_udp("127.0.0.1:8000", "read")
}
