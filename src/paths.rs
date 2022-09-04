use std::sync::Mutex;

use actix_web::{
    web::{self, Data},
    HttpResponse,
};
use mongodb::Database;

use crate::models::SmartHouse;

pub async fn add_room(
    house: web::Data<Mutex<SmartHouse>>,
    new_room: web::Path<String>,
    db: Data<Database>,
) -> HttpResponse {
    let mut da_haus = house.lock().unwrap();
    da_haus.insert_room(new_room.into_inner());
    // getting result
    let result = da_haus.clone();
    HttpResponse::Ok().json(result)
}
