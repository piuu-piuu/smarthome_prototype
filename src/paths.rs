use actix_web::{
    web::{self},
    HttpResponse,
};

use crate::models::SmartHouse;

pub fn add_room(house: &mut SmartHouse, new_room: web::Path<String>) -> HttpResponse {
    SmartHouse::insert_room(house, new_room.into_inner());
    HttpResponse::Ok().json("New room added.")
}
