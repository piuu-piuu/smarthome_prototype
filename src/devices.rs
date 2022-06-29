// Пользовательские устройства
//
// Приложение, имитирующее работу умной розетки, управляемой по TCP.

pub const SMARTSOCKET: &'static str = "127.0.0.1:7878";

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
            handle_connection(stream);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let received = from_utf8(&buffer).unwrap();
    println!("Request: {}", received);

    match received {
        "Test" => {
            let response = "Socket ready".as_bytes();
            stream.write(response).unwrap();
            // stream.flush().unwrap();
        }
        "On" => {
            let response = "Socket is ON";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        "Off_" => {
            let response = "Socket is OFF";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        _ => {}
    }
}
