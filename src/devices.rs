// Пользовательские устройства
//
// Приложение, имитирующее работу умной розетки, управляемой по TCP.

pub const SMARTSOCKET: &str = "127.0.0.1:7878";

use std::io::prelude::*;
use std::net;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::str::from_utf8;
use std::thread;

pub struct SmartSocket<'a> {
    pub name: &'a str,
    pub info: &'a str,
}
pub struct SmartThermometer<'a> {
    pub name: &'a str,
    pub info: &'a str,
}

pub trait TcpConnect {
    fn tcpconnect(&self, host: &str);
}

pub trait DeviceDataUDP {
    fn udp_send(&self, addr: &str);
}

impl SmartThermometer<'_> {
    pub fn new() -> Self {
        Self {
            name: "therm",
            info: "smart therm",
        }
    }
}

#[allow(unused_variables)]
impl TcpConnect for SmartSocket<'_> {
    fn tcpconnect(&self, host: &str) {
        let listener = TcpListener::bind(host).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            // println!("HTTP/1.1 200 OK\r\n\r\n");
            println!("Connection established!");
            println!("{}", &self.info);
            handle_connection(stream, self.info);
        }
    }
}

fn handle_connection(mut stream: TcpStream, device_info: &str) {
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

impl DeviceDataUDP for SmartThermometer<'_> {
    fn udp_send(&self, addr: &str) {
        let socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind socket");

        loop {
            let mut buf = [0u8; 1500];
            let sock = socket.try_clone().expect("Failed to clone socket");
            // println!("socket cloned");
            match socket.recv_from(&mut buf) {
                Ok((_, src)) => {
                    thread::spawn(move || {
                        println!("Client {} connected", src);
                        let input = String::from("20`C \n");
                        sock.send_to(input.as_bytes(), src)
                            .expect("Failed to send data");
                        // sock.send_to(&buf, &src).expect("Failed to send a response");
                    });
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
    }
}
