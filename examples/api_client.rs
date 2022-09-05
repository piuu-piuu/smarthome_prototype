use reqwest::Error;
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};

#[allow(dead_code)]
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
        "add/socket2/bathroom",
        "del/socket2/bathroom",
        "commit",
        "list_all_devices",
    ]);

    for command in commands {
        let request_url = format!("http://127.0.0.1:8888/{}", command);
        println!("Requested {}, got: ", request_url);
        let response = reqwest::get(&request_url).await?;
        let house_response: SmartHouseResponce = response.json().await?;
        println!("{:?}", house_response);
    }

    let request_url = format!("http://127.0.0.1:8888/{command}", command = "house_report");
    println!("Requested {}, got: ", request_url);
    let response = reqwest::get(&request_url).await?;
    let report_response: String = response.json().await?;
    println!("{:?}", report_response);

    Ok(())
}
