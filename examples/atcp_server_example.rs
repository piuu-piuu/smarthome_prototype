use smarthome_4::tokio_servers::*;

fn main() {
    atcp_serve().expect("Server failure.");
}
