// use super::info::devices::{SmartSocket, SmartThermometer};
use super::info::DeviceInfoProvider;
use std::collections::HashMap;

const DEVICE_COUNT: usize = 5;
const EMPTY_ENTRY: &str = "";

pub struct SmartHouse<'a> {
    rooms: HashMap<&'a str, [&'a str; DEVICE_COUNT]>,
}

impl SmartHouse<'_> {
    pub fn new() -> Self {
        SmartHouse {
            rooms: HashMap::from([
                (
                    "bedroom",
                    ["socket1", "socket2", EMPTY_ENTRY, EMPTY_ENTRY, EMPTY_ENTRY],
                ),
                (
                    "kitchen",
                    ["socket1", "thermo", "thermo", EMPTY_ENTRY, EMPTY_ENTRY],
                ),
                (
                    "living_room",
                    ["thermo", "socket2", EMPTY_ENTRY, EMPTY_ENTRY, EMPTY_ENTRY],
                ),
            ]),
        }
    }

    #[allow(dead_code)]

    pub fn get_rooms(&self) -> impl Iterator<Item = &&str> {
        // Размер возвращаемого массива можно выбрать самостоятельно
        self.rooms.keys()
    }

    pub fn print_rooms_with_devices(&self) {
        // Размер возвращаемого массива можно выбрать самостоятельно
        println!("Rooms and Devices\n");
        for (key, value) in &self.rooms {
            print!("{}: ", key);
            for item in value {
                print!("{} ", item);
            }
        }
        println!("\n");
    }

    #[allow(dead_code)]
    pub fn devices_at_room(&self, room: &str) -> [&str; DEVICE_COUNT] {
        // Размер возвращаемого массива можно выбрать самостоятельно
        let mut result = [
            EMPTY_ENTRY,
            EMPTY_ENTRY,
            EMPTY_ENTRY,
            EMPTY_ENTRY,
            EMPTY_ENTRY,
        ];
        for (key, value) in &self.rooms {
            if *key == room {
                result = *value;
            }
        }
        result
    }

    pub fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        // перебор комнат и устройств в них для составления отчёта
        let mut report = EMPTY_ENTRY.to_owned();
        for (room, devicelist) in &self.rooms {
            // println!("\n {}", *room);
            for device in *devicelist {
                // print!("{}, ", device)
                report.push_str(&provider.info(room, device));
            }
        }
        report
    }
}
