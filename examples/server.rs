use smarthome_4::models::SmartHouse;
use std::sync::Mutex;

// use super::models::SmartHouse;
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use smarthome_4::paths;
// use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let client = Client::with_uri_str("mongodb://localhost:27017")
    //     .await
    //     .expect("failed to connect");

    let house = Data::new(Mutex::new(SmartHouse::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&house))
            .service(web::resource("/add_room").route(web::post().to(paths::add_room)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
