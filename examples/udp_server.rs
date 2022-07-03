use smarthome_4::devices::{SmartThermometer, *};
// use smarthome_4::info::*;
// use smarthome_4::smarthouse::*;

fn main() {
    let thermo1 = SmartThermometer {
        name: "socket1",
        info: "use UDP connection at 127.0.0.1:34254 for readings",
    };
    let t_socket = SmartThermometer::<'_>::init_host("127.0.0.1:34254");
    loop {
        thermo1.udp_data(&t_socket, "", "20".as_bytes().to_vec().as_ref())
    }
}
