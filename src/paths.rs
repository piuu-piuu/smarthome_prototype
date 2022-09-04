use std::sync::Mutex;

use actix_web::{
    web::{self, Data},
    HttpResponse,
};
use mongodb::{bson::doc, Database};

#[allow(unused_imports)]
use crate::devices::{SmartSocket, SmartThermometer};
use crate::models::SmartHouse;

#[allow(unused_variables)]
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

#[allow(unused_variables)]
pub async fn db_commit(house: web::Data<Mutex<SmartHouse>>, db: Data<Database>) -> HttpResponse {
    let da_haus = house.lock().unwrap().clone();

    db.collection::<SmartHouse>("house")
        .delete_many(
            doc! {
               "name": "User House"
            },
            None,
        )
        .await
        .expect("DB drop error.");
    db.collection::<SmartHouse>("house")
        .insert_one(da_haus, None)
        .await
        .map_err(|err| format!("DB_ERROR: {}", err))
        .expect("Commit error.");
    HttpResponse::Ok().json("Commited.")
}
