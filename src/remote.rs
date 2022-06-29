// TCP-клиент умной розетки, позволяющий:&
// включать и выключать розетку,
// запрашивать информацию о текущем состоянии и потребляемой мощности розетки.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

pub fn reach(device_host: &str, message: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(device_host)?;

    let msg = message.as_bytes();
    stream.write_all(msg)?;
    println!("Sent message '{}', awaiting reply...", message);

    let mut buffer = [0_u8; 1024];
    let read_len = stream.read(&mut buffer)?;
    let received = from_utf8(buffer.get(0..read_len).unwrap()).unwrap();

    println!(">>> {} ", received);

    println!("<<<");
    Ok(())
}
