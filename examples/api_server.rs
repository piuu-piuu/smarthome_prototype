use mongodb::Client;
use smarthome_4::{
    devices::{SmartSocket, SmartThermometer},
    info::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider},
    models::SmartHouse,
};
use std::sync::Mutex;

// use super::models::SmartHouse;
use actix_web::{
    middleware::Logger,
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

    let database = Data::new(client.database("house"));

    let res = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::clone(&house))
            .app_data(Data::clone(&database))
            .service(web::resource("/add_room/{new_room}").route(web::get().to(paths::add_room)))
            .service(web::resource("/del_room/{room}").route(web::get().to(paths::del_room)))
            .service(web::resource("/add/{device}/{room}").route(web::get().to(paths::add_device)))
            .service(web::resource("/del/{device}/{room}").route(web::get().to(paths::add_device)))
            .service(web::resource("/list_all_devices").route(web::get().to(paths::all_devices)))
            .service(web::resource("/at/{room}").route(web::get().to(paths::devices_at_room)))
            .service(web::resource("/commit").route(web::get().to(paths::db_commit)))
            .service(web::resource("/house_report").route(web::get().to(paths::house_report)))
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await;

    println!("Yep");
    res
}
