use smarthome_4::devices::*;
use smarthome_4::remote::*;

#[allow(unused)]
fn main() {
    reach(SMARTSOCKET, "info");
    reach(SMARTSOCKET, "on");
    reach(SMARTSOCKET, "off");
    reach(SMARTSOCKET, "hello world");
}
