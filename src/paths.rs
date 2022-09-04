use std::{ops::Deref, sync::Mutex};

use actix_web::{
    web::{self, Data},
    HttpResponse,
};
use mongodb::{bson::doc, Database};

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

pub async fn db_commit(house: web::Data<Mutex<SmartHouse>>, db: Data<Database>) -> HttpResponse {
    let da_haus = house.lock().unwrap().clone();

    db.collection::<SmartHouse>("house")
        .delete_many(
            doc! {
               "name": "User House"
            },
            None,
        )
        .await;
    db.collection::<SmartHouse>("house")
        .insert_one(da_haus, None)
        .await
        .map_err(|err| format!("DB_ERROR: {}", err));
    HttpResponse::Ok().json("Commited.")
}
