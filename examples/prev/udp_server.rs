use smarthome_4::devices::{SmartThermometer, UdpServer};
// use smarthome_4::info::*;
// use smarthome_4::smarthouse::*;

fn main() {
    let therm1 = SmartThermometer::new();
    therm1.udp_send("0.0.0.0:8888");
}
