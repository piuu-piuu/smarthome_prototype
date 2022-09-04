use mongodb::Client;
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
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("failed to connect");

    let house = Data::new(Mutex::new(SmartHouse::new()));
    let database = Data::new(client.database("house_2"));

    let res = HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&house))
            .app_data(Data::clone(&database))
            .service(web::resource("/add_room/{new_room}").route(web::post().to(paths::add_room)))
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await;

    // write to db here?
    println!("Yep");
    res
}
