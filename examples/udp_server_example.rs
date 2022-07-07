use smarthome_4::devices::{DeviceDataUDP, SmartThermometer};
use std::net::UdpSocket;
use std::thread;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind socket");

    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("Failed to clone socket");
        // println!("socket cloned");
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    println!("Handling connection from {}", src);
                    let input = String::from("20`C \n");
                    sock.send_to(input.as_bytes(), src)
                        .expect("Failed to write to server");
                    // sock.send_to(&buf, &src).expect("Failed to send a response");
                });
            }
            Err(e) => {
                eprintln!("couldn't recieve a datagram: {}", e);
            }
        }
    }
}
