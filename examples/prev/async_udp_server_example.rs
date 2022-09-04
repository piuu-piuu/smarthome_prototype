use smarthome_4::devices::{AsyncUdpServer, SmartThermometer};

fn main() {
    // audp_serve().expect("Server failure.");
    let therm1 = SmartThermometer::new("therm");
    let therminfo = "21`C";
    therm1.a_udp_send("127.0.0.1:8888", therminfo);
}
