use serde::{Deserialize, Serialize};
use smarthome_4::devices;
use smarthome_4::devices::{SmartSocket, SmartThermometer};
use smarthome_4::models::{self, SmartHouse};

fn main() {
    let socket1 = SmartSocket {
        name: "socket1",
        info: "SmartSocket",
    };

    let socket2 = SmartSocket {
        name: "socket2",
        info: "SmartSocket",
    };

    let thermo = SmartThermometer {
        // name error
        name: "hermo1",
        info: "SmartThermometer",
    };

    // Инициализация дома
    let house = SmartHouse::new();

    let serialized = serde_json::to_string(&house).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: SmartHouse = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
