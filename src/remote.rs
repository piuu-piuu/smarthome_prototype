// TCP-клиент умной розетки, позволяющий:
// включать и выключать розетку,
// запрашивать информацию о текущем состоянии и потребляемой мощности розетки.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

pub fn reach(device_host: &str, message: &str) {
    match TcpStream::connect(device_host) {
        Ok(mut stream) => {
            println!("Successfully connected to device. ");
            let msg = message.as_bytes();
            stream.write(msg).unwrap();
            println!("Sent message, awaiting reply...");

            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            let received = from_utf8(&buffer).unwrap();

            println!("Responce: {}. ", received);
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
