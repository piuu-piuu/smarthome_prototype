use smarthome_4::models::SmartHouse;

use mongodb::Database;

pub async fn save_db(house: SmartHouse, mongo: &Database) -> Result<(), String> {
    mongo
        .collection::<SmartHouse>("main_house")
        .insert_one(&house, None)
        .await
        .map_err(|err| format!("DB_ERROR: {}", err))?;
    Ok(())
}

fn main() {
    // Инициализация дома
    let house = SmartHouse::new();

    let serialized = serde_json::to_string(&house).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: SmartHouse = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
