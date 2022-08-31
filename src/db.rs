// use mongodb::Database;

// impl SmartHouse {
//     pub async fn save(self, mongo: &Database) -> Result<(), String> {
//         mongo
//             .collection::<NewUser>("users")
//             .insert_one(&self, None)
//             .await
//             .map_err(|err| format!("DB_ERROR: {}", err))?;
//         Ok(())
//     }
// }
