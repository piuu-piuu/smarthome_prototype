use smarthome_4::tokio_servers::*;

fn main() {
    audp_serve().expect("Server failure.");
}
