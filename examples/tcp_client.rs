use smarthome_4::devices::*;
use smarthome_4::remote::*;

#[allow(unused)]
fn main() {
    reach(SMARTSOCKET, "Test");
    reach(SMARTSOCKET, "On");
    reach(SMARTSOCKET, "Off");
}
