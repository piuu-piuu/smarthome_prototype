use smarthome_4::devices::*;
// use smarthome_4::info::*;
// use smarthome_4::smarthouse::*;

fn main() {
    let socket1 = SmartSocket {
        name: "socket1",
        info: "Smart Socket 220V 50VA",
    };
    socket1.tcpconnect(SMARTSOCKET);
}
