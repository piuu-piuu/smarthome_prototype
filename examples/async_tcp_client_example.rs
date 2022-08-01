use smarthome_4::tokio_clients::*;

const SMARTSOCKET: &str = "127.0.0.1:7878";

fn main() {
    areach_tcp(SMARTSOCKET, "info").expect("Connection failed.");
    areach_tcp(SMARTSOCKET, "on").expect("Connection failed.");
    areach_tcp(SMARTSOCKET, "off").expect("Connection failed.");
}
