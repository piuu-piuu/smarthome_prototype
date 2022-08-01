use smarthome_4::{devices::SmartSocket, tokio_servers::*};

fn main() {
    let socket1 = SmartSocket {
        name: "socket1",
        info: "Smart Socket 220V 50VA",
    };
    atcp_serve(socket1.info).expect("Server failure.");
}
