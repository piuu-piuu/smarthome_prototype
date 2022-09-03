use actix_web::{
    web::{self},
    HttpResponse,
};

use crate::models::SmartHouse;

pub fn insert_room(house: &mut SmartHouse, new_room: web::Path<String>) -> HttpResponse {
    SmartHouse::insert_room(house, (&new_room).to_string());
    HttpResponse::Ok().json("New room inserted")
}
