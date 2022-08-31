use serde::{Deserialize, Serialize};
use smarthome_4::devices;
use smarthome_4::devices::{SmartSocket, SmartThermometer};
use smarthome_4::models::{self, SmartHouse};

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use mongodb::{Client, Database};

pub async fn save_db(house: SmartHouse<'_>, mongo: &Database) -> Result<(), String> {
    mongo
        .collection::<SmartHouse>("main_house")
        .insert_one(&house, None)
        .await
        .map_err(|err| format!("DB_ERROR: {}", err))?;
    Ok(())
}

fn build_house() -> SmartHouse<'static> {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("failed to connect");

    let coll = client.database("house_1").collection::<SmartHouse>("SH");
    let house = build_house();
    coll.insert_one(house, None).await;

    HttpServer::new(move || {
        App::new().app_data(Data::new(client.database("house_1")))
        // .service(web::resource("/users").route(web::post().to(webc::save_new)))
        // .service(web::resource("/users/{username}").route(web::get().to(webc::find)))
    })
    // .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
