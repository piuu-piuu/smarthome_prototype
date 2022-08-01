use smarthome_4::tokio_clients::*;

fn main() {
    audp_client().expect("Server failure.");
}
