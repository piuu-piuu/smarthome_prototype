use std::sync::Mutex;

use actix_web::{
    web::{self},
    HttpResponse,
};

use crate::models::SmartHouse;

// #[post("/add_room")]
// pub async fn add_room(house: &mut SmartHouse, new_room: web::Path<String>) -> HttpResponse {
//     SmartHouse::insert_room(house, new_room.into_inner());
//     HttpResponse::Ok().json("New room added.")
// }

pub async fn add_room(
    house: web::Data<Mutex<SmartHouse>>,
    new_room: web::Path<String>,
) -> HttpResponse {
    let mut da_haus = house.lock().unwrap();
    da_haus.insert_room(new_room.into_inner());
    // getting result
    let result = da_haus.clone();
    HttpResponse::Ok().json(result)
}
