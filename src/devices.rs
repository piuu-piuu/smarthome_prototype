// Пользовательские устройства
//
// Приложение, имитирующее работу умной розетки, управляемой по TCP.

pub const SMARTSOCKET: &str = "127.0.0.1:7878";

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str::from_utf8;

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
