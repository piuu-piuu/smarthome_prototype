use super::devices::{SmartSocket, SmartThermometer};

const EMPTY_ENTRY: &str = "";

pub trait DeviceInfoProvider {
    // метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn info(&self, room_name: &str, device_name: &str) -> Option<String>;
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствовать.
pub struct OwningDeviceInfoProvider<'a> {
    pub socket: SmartSocket<'a>,
}
pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket<'a>,
    pub thermo: &'b SmartThermometer<'b>,
}

#[allow(unused_assignments)]
impl DeviceInfoProvider for OwningDeviceInfoProvider<'_> {
    fn info(&self, room_name: &str, device_name: &str) -> Option<String> {
        let mut output = EMPTY_ENTRY.to_owned();
        if !device_name.is_empty() {
            if device_name == self.socket.name {
                output = format!("{} - {} - {}. ", room_name, device_name, self.socket.info);
                Some(output)
            } else {
                output = format!("Error {} at {}. ", device_name, room_name);
                None
            }
        } else {
            None
        }
    }
}

#[allow(unused_assignments)]
impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn info(&self, room_name: &str, device_name: &str) -> Option<String> {
        let mut output = EMPTY_ENTRY.to_owned();
        if !device_name.is_empty() {
            if device_name == self.socket.name {
                output = format!("{} - {} - {}. ", room_name, device_name, self.socket.info);
                Some(output)
            } else {
                if device_name == self.thermo.name {
                    output = format!("{} - {} - {}. ", room_name, device_name, self.thermo.info);
                    Some(output)
                } else {
                    output = format!("Error {} at {}. ", device_name, room_name);
                    None
                }
            }
        } else {
            None
        }
    }
}
