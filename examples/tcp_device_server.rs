use smarthome_4::devices::*;
// use smarthome_4::info::*;
// use smarthome_4::smarthouse::*;

#[allow(dead_code)]
fn main() {
    let socket1 = SmartSocket {
        name: "socket1",
        info: "SmartSocket",
    };
    socket1.tcpconnect(SMARTSOCKET);
}
