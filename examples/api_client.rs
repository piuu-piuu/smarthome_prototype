use reqwest::Error;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Deserialize, Debug)]
struct SmartHouseResponce {
    name: String,
    rooms: BTreeMap<String, HashSet<String>>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let commands = Vec::from([
        "add_room/new-kitchen",
        "del_room/new-kitchen",
        // POSTMAN OK
        // "/at/{room}",
        // "/commit",
        // "/all_devices",
        // "/add/socket2/bathroom",
        // !:
        // "/del/socket1/bathroom",
    ]);

    for command in commands {
        let request_url = format!("http://127.0.0.1:8888/{}", command);
        println!("Requested {}, got: ", request_url);
        let response = reqwest::get(&request_url).await?;
        let house_response: SmartHouseResponce = response.json().await?;
        println!("{:?}", house_response);
    }
    Ok(())
}
