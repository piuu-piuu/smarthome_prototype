// use mongodb::Database;

use super::errors::SmartHouseError;
use super::info::DeviceInfoProvider;
use super::models::*;
use std::collections::{BTreeMap, HashSet};

const EMPTY_ENTRY: &str = "";

// pub struct SmartHouse<'a> {
//     pub name: String,
//     pub rooms: BTreeMap<String, HashSet<String>>,
// }

impl SmartHouse {
    pub fn new() -> Self {
        SmartHouse {
            name: String::from("User House"),
            rooms: BTreeMap::from([
                (
                    "bedroom".to_string(),
                    HashSet::from_iter(vec!["socket1".to_string()].into_iter()),
                ),
                (
                    "kitchen".to_string(),
                    HashSet::from_iter(
                        vec!["socket2".to_string(), "thermo1".to_string()].into_iter(),
                    ),
                ),
                (
                    "bathroom".to_string(),
                    HashSet::from_iter(
                        vec!["socket1".to_string(), "thermo1".to_string()].into_iter(),
                    ),
                ),
            ]),
        }
    }

    #[warn(unused_variables)]
    pub fn insert_room(&mut self, room_name: String) {
        let device_list: HashSet<String> = HashSet::from_iter(vec![].into_iter());
        // if !self.rooms.contains_key(&room_name) {
        //     self.rooms.insert(room_name, device_list);
        // }
        self.rooms.entry(room_name).or_insert(device_list);
    }

    pub fn delete_room(&mut self, room_name: String) {
        if self.rooms.contains_key(&room_name) {
            self.rooms.remove(&room_name);
        }
    }

    pub fn insert_device(&mut self, new_device: String, room_name: String) {
        let device_list_option: Option<&HashSet<String>> = self.rooms.get(&room_name);
        let mut device_list: HashSet<String>;
        match device_list_option {
            Some(device_list_option) => {
                device_list = device_list_option.clone();
                device_list.insert(new_device);
                self.rooms.insert(room_name, device_list);
            }
            None => {
                println!("No devices' placeholder for insertion")
            }
        }
    }

    pub fn delete_device(&mut self, device_name: String, room_name: String) {
        let device_list_option: Option<&HashSet<String>> = self.rooms.get(&room_name);
        let mut device_list: HashSet<String>;
        match device_list_option {
            Some(device_list_option) => {
                device_list = device_list_option.clone();
                if device_list.contains(&device_name) {
                    device_list.remove(&device_name);
                }
            }
            None => {
                println!("No devices' placeholder for deletion")
            }
        }
    }

    // pub fn get_rooms(&self) -> impl Iterator<Item = &&str> {
    //     self.rooms.keys()
    // }

    pub fn print_rooms_with_devices(&self) {
        println!("Rooms and Devices: ");
        for (key, value) in &self.rooms {
            print!("{}: ", key);
            for item in value {
                print!("{} ", item);
            }
        }
        println!();

        impl Default for SmartHouse {
            fn default() -> Self {
                Self::new()
            }
        }
    }

    pub fn devices_at_room(&self, room: &str) -> Vec<String> {
        // Размер возвращаемого массива можно выбрать самостоятельно
        let mut result = HashSet::new();
        for (key, value) in &self.rooms {
            if *key == room {
                result = value.clone();
            }
        }
        Vec::from_iter(result)
    }

    pub fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        // перебор комнат и устройств в них для составления отчёта
        let mut report = EMPTY_ENTRY.to_owned();
        for (room, devicelist) in &self.rooms {
            for device in devicelist {
                match provider.info(room, device) {
                    Ok(output) => report.push_str(&output),
                    Err(SmartHouseError::NoDeviceFound) => {
                        println!("{}: {} - {}", room, device, SmartHouseError::NoDeviceFound)
                    }
                    Err(SmartHouseError::NoDeviceName) => {
                        println!("{}", SmartHouseError::NoDeviceName)
                    }
                }
            }
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn test_rooms() {
    //     let house = SmartHouse::new();
    //     house.get_rooms().next();
    // }
    #[test]
    fn test_device_list() {
        let house = SmartHouse::new();
        house.print_rooms_with_devices();
    }

    #[test]
    fn test_devices() {
        let house = SmartHouse::new();
        house.devices_at_room("kitchen");
        house.devices_at_room("bathroom");
        house.devices_at_room("bedroom");
    }

    #[test]
    fn test_house_name() {
        let house = SmartHouse::new();
        println!("{}", house.name);
    }

    #[test]
    fn test_new_room() {
        let mut house = SmartHouse::new();
        house.insert_room("living_room".to_string());
    }

    #[test]
    fn delete_room() {
        let mut house = SmartHouse::new();
        house.delete_room("living_room".to_string());
    }

    #[test]
    fn test_new_device() {
        let mut house = SmartHouse::new();
        house.insert_device("socket2".to_string(), "bedroom".to_string())
    }

    #[test]
    fn test_delete_device() {
        let mut house = SmartHouse::new();
        house.delete_device("socket2".to_string(), "bedroom".to_string())
    }

    #[test]
    fn test_devices_again() {
        let house = SmartHouse::new();
        house.devices_at_room("kitchen");
        house.devices_at_room("bathroom");
        house.devices_at_room("bedroom");
        house.devices_at_room("living_room");
    }
}
