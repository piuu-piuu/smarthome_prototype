// TCP-клиент умной розетки, позволяющий:&
// включать и выключать розетку,
// запрашивать информацию о текущем состоянии и потребляемой мощности розетки.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::{from_utf8, Bytes};

pub fn reach(device_host: &str, message: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(device_host)?;

    let msg = message.as_bytes();
    stream.write(msg)?;
    println!("Sent message '{}', awaiting reply...", message);

    let mut buffer = [0 as u8; 12];
    stream.read(&mut buffer)?;
    let received = from_utf8(&buffer).unwrap();

    println!("... OK {} ", received);

    println!("Terminated.");
    Ok(())
}
