use super::errors::SmartHouseError;
use super::info::DeviceInfoProvider;
use std::collections::{BTreeMap, HashSet};

const EMPTY_ENTRY: &str = "";

pub struct SmartHouse<'a> {
    pub name: &'a str,
    pub rooms: BTreeMap<&'a str, HashSet<&'a str>>,
}

impl<'a> SmartHouse<'a> {
    pub fn new() -> Self {
        SmartHouse {
            name: "User House",
            rooms: BTreeMap::from([
                ("bedroom", HashSet::from_iter(vec!["socket1"].into_iter())),
                (
                    "kitchen",
                    HashSet::from_iter(vec!["socket2", "thermo1"].into_iter()),
                ),
                (
                    "bathroom",
                    HashSet::from_iter(vec!["socket1", "thermo1"].into_iter()),
                ),
            ]),
        }
    }

    #[warn(unused_variables)]
    pub fn insert_room(&mut self, room_name: &'a str) {
        let device_list: HashSet<&str> = HashSet::from_iter(vec![].into_iter());
        self.rooms.insert(room_name, device_list);
    }

    pub fn insert_device(&mut self, new_device: &'a str, room_name: &'a str) {
        let mut device_list: HashSet<&str> = self.rooms.get(room_name).unwrap().clone();
        device_list.insert(new_device);
        self.rooms.insert(room_name, device_list);
    }

    pub fn get_rooms(&self) -> impl Iterator<Item = &&str> {
        // Размер возвращаемого массива можно выбрать самостоятельно
        self.rooms.keys()
    }

    pub fn print_rooms_with_devices(&self) {
        println!("Rooms and Devices: ");
        for (key, value) in &self.rooms {
            print!("{}: ", key);
            for item in value {
                print!("{} ", item);
            }
        }
        println!();

        impl<'a> Default for SmartHouse<'a> {
            fn default() -> Self {
                Self::new()
            }
        }
    }

    pub fn devices_at_room(&self, room: &str) -> Vec<&str> {
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
                        println!("no device found for pair {}, {}", room, device)
                    }
                    Err(SmartHouseError::NoDeviceName) => {
                        println!("no device name given!")
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
    #[test]
    fn test_rooms() {
        let house = SmartHouse::new();
        house.get_rooms().next();
    }
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
        house.insert_room("living_room");
    }

    #[test]
    fn test_new_device() {
        let mut house = SmartHouse::new();
        house.insert_device("socket2", "bedroom")
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
