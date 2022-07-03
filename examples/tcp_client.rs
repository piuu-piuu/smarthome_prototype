use smarthome_4::devices::*;
use smarthome_4::remote::*;

#[allow(unused)]
fn main() {
    reach_tcp(SMARTSOCKET, "info");
    reach_tcp(SMARTSOCKET, "on");
    reach_tcp(SMARTSOCKET, "off");
    reach_tcp(SMARTSOCKET, "hello world");
}
