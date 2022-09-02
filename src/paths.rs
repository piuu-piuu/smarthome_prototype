use actix_web::{
    web::{self},
    HttpResponse,
};

use crate::models::SmartHouse;

pub fn insert_room<'a>(house: &'a mut SmartHouse, new_room: web::Path<String>) -> HttpResponse {
    SmartHouse::insert_room(house, &new_room);
    HttpResponse::Ok().json("New room inserted")
}
