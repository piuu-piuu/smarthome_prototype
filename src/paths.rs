use actix_web::{
    web::{self, Data},
    HttpResponse,
};
use mongodb::{bson::doc, Database};
use std::sync::Mutex;

#[allow(unused_imports)]
use crate::devices::{SmartSocket, SmartThermometer};
use crate::{
    info::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider},
    models::SmartHouse,
};

#[allow(unused_variables)]
pub async fn db_commit(house: web::Data<Mutex<SmartHouse>>, db: Data<Database>) -> HttpResponse {
    let da_haus = house.lock().unwrap();

    db.collection::<SmartHouse>("house")
        .delete_many(
            doc! {
               "name": "User House"
            },
            None,
        )
        .await
        .expect("DB drop error.");
    let commit_house = da_haus.clone();
    db.collection::<SmartHouse>("house")
        .insert_one(commit_house, None)
        .await
        .map_err(|err| format!("DB_ERROR: {}", err))
        .expect("Commit error.");
    // HttpResponse::Ok().json("Commited.")
    let result = da_haus.clone();
    HttpResponse::Ok().json(result)
}

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

pub async fn del_room(
    house: web::Data<Mutex<SmartHouse>>,
    room: web::Path<String>,
) -> HttpResponse {
    let mut da_haus = house.lock().unwrap();
    da_haus.delete_room(room.into_inner());
    // getting result
    let result = da_haus.clone();
    HttpResponse::Ok().json(result)
}

pub async fn add_device(
    house: web::Data<Mutex<SmartHouse>>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let mut da_haus = house.lock().unwrap();
    let (device, room) = path.into_inner();
    da_haus.insert_device(device, room);
    // getting result
    let result = da_haus.clone();
    HttpResponse::Ok().json(result)
}

pub async fn del_device(
    house: web::Data<Mutex<SmartHouse>>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let mut da_haus = house.lock().unwrap();
    let (device, room) = path.into_inner();
    da_haus.delete_device(device, room);
    // getting result
    let result = da_haus.clone();
    HttpResponse::Ok().json(result)
}

pub async fn all_devices(house: web::Data<Mutex<SmartHouse>>) -> HttpResponse {
    let da_haus = house.lock().unwrap();
    let result = da_haus.clone();
    HttpResponse::Ok().json(result)
}

pub async fn devices_at_room(
    house: web::Data<Mutex<SmartHouse>>,
    room: web::Path<String>,
) -> HttpResponse {
    let da_haus = house.lock().unwrap();
    let result = da_haus.devices_at_room(&room.into_inner());
    // getting result
    // let result = da_haus.clone();
    HttpResponse::Ok().json(result)
}

pub async fn house_report(house: web::Data<Mutex<SmartHouse>>) -> HttpResponse {
    let house = house.lock().unwrap();
    let socket1 = SmartSocket {
        name: "socket1",
        info: "Smart Socket 220V 50VA",
    };
    let socket2 = SmartSocket {
        name: "socket2",
        info: "Smart Socket 220V 50VA",
    };
    let thermo1 = SmartThermometer {
        name: "thermo1",
        info: "SmartThermometer 20'C",
    };
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo1,
    };
    let report1 = house.create_report(&info_provider_1);
    let report2 = house.create_report(&info_provider_2);

    // HttpResponse::Ok().json(format!("{}; {}", report1, report2))
    HttpResponse::Ok().json(report2)
}
