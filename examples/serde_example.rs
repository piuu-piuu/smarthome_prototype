use serde::{Deserialize, Serialize};
use smarthome_4::devices::{SmartSocket, SmartThermometer};
use smarthome_4::models::{self, SmartHouse};
// use smarthome_4::paths::*;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use mongodb::{Client, Database};

pub async fn save_db(house: SmartHouse, mongo: &Database) -> Result<(), String> {
    mongo
        .collection::<SmartHouse>("main_house")
        .insert_one(&house, None)
        .await
        .map_err(|err| format!("DB_ERROR: {}", err))?;
    Ok(())
}

fn build_house() -> SmartHouse {
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

    house
}

fn main() {}
