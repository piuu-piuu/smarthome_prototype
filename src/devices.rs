// Пользовательские устройства
//

pub const SMARTSOCKET: &str = "127.0.0.1:7878";

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::str::from_utf8;
use std::thread;

use crate::tokio_servers::audp_serve;

pub struct SmartSocket<'a> {
    pub name: &'a str,
    pub info: &'a str,
}
pub struct SmartThermometer<'a> {
    pub name: &'a str,
    pub info: &'a str,
}

pub trait TcpServer {
    fn tcpconnect(&self, host: &str);
}

pub trait UdpServer {
    fn udp_send(&self, addr: &str);
}

pub trait AsyncUdpServer {
    fn a_udp_send(&self, addr: &str, device_data: &str);
}

#[allow(clippy::new_without_default)]
impl SmartThermometer<'_> {
    pub fn new() -> Self {
        Self {
            name: "therm",
            info: "smart therm",
        }
    }
}

#[allow(unused_variables)]
impl TcpServer for SmartSocket<'_> {
    fn tcpconnect(&self, host: &str) {
        let listener = TcpListener::bind(host).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established!");
            println!("{}", &self.info);
            handle_tcp_connection(stream, self.info);
        }
    }
}

fn handle_tcp_connection(mut stream: TcpStream, device_info: &str) {
    loop {
        let mut buffer = [0; 1024];

        let read_len = stream.read(&mut buffer).unwrap();
        if read_len == 0 {
            println!("Client {} disconnected", stream.peer_addr().unwrap());
            break;
        }

        let received = from_utf8(buffer.get(0..read_len).unwrap()).unwrap();
        println!("Request: {}", received);

        match received {
            "info" => {
                let response = device_info.as_bytes();
                stream.write_all(response).unwrap();
                stream.flush().unwrap();
            }
            "on" => {
                let response = "Socket  ON";
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            "off" => {
                let response = "Socket  OFF";
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            _ => {
                let response = "Unknown cmd";
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
    }
}

impl UdpServer for SmartThermometer<'_> {
    fn udp_send(&self, addr: &str) {
        let socket = UdpSocket::bind(addr).expect("Could not bind socket");

        loop {
            let mut buf = [0u8; 1500];
            let sock = socket.try_clone().expect("Failed to clone socket");
            match socket.recv_from(&mut buf) {
                Ok((_, src)) => {
                    thread::spawn(move || {
                        println!("Client {} connected", src);
                        let input = String::from("20`C \n");
                        sock.send_to(input.as_bytes(), src)
                            .expect("Failed to send data");
                    });
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
    }
}

impl AsyncUdpServer for SmartThermometer<'_> {
    fn a_udp_send(&self, addr: &str, device_data: &str) {
        audp_serve(addr, device_data).expect("No answer from device");
    }
}
