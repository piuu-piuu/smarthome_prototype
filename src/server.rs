use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use mongodb::Client;
use models::SmartHouse;
use smarthome_4::paths;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("failed to connect");

        HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.database("house_2")))
            .service(web::resource("/add_room").route(web::post().to(paths::add_room)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
