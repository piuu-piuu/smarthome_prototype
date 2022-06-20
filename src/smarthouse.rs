use super::info::DeviceInfoProvider;
use std::collections::BTreeMap;

const DEVICE_COUNT: usize = 5;
const EMPTY_ENTRY: &str = "";

#[allow(dead_code)]
pub struct SmartHouse<'a, 'b> {
    pub name: &'a str,
    pub rooms: BTreeMap<&'b str, [&'b str; DEVICE_COUNT]>,
}

impl SmartHouse<'_, '_> {
    pub fn new() -> Self {
        SmartHouse {
            name: "User House",
            rooms: BTreeMap::from([
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
        print!("\n");
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
            for device in *devicelist {
                match provider.info(room, device) {
                    Some(output) => report.push_str(&output),
                    None => (),
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
    }
    #[test]
    fn test_house_name() {
        let house = SmartHouse::new();
        house.name;
    }
}
