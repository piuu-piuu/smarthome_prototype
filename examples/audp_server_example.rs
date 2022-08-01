use smarthome_4::{
    devices::{AsyncUdpServer, SmartThermometer},
    tokio_servers::*,
};

fn main() {
    // audp_serve().expect("Server failure.");
    let therm1 = SmartThermometer::new();

    therm1.a_udp_send("0.0.0.0:8888");
}
