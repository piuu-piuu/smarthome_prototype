use std::{error::Error, fmt};

use super::devices::{SmartSocket, SmartThermometer};

const EMPTY_ENTRY: &str = "";

pub trait DeviceInfoProvider {
    // метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn info(&self, room_name: &str, device_name: &str) -> Result<String, NoDeviceFoundError>;
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

#[derive(Debug)]
pub struct NoDeviceFoundError;

impl fmt::Display for NoDeviceFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No such device in this room!")
    }
}

impl Error for NoDeviceFoundError {}

#[allow(unused_assignments)]
impl DeviceInfoProvider for OwningDeviceInfoProvider<'_> {
    fn info(&self, room_name: &str, device_name: &str) -> Result<String, NoDeviceFoundError> {
        let mut output = EMPTY_ENTRY.to_owned();
        if !device_name.is_empty() {
            if device_name == self.socket.name {
                output = format!("{}: {} {}. ", room_name, device_name, self.socket.info);
                Ok(output)
            } else {
                // output = format!("Error {} at {}. ", device_name, room_name);
                Err(NoDeviceFoundError)
            }
        } else {
            Err(NoDeviceFoundError)
        }
    }
}

#[allow(unused_assignments)]
impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn info(&self, room_name: &str, device_name: &str) -> Result<String, NoDeviceFoundError> {
        let mut output = EMPTY_ENTRY.to_owned();
        if !device_name.is_empty() {
            if device_name == self.socket.name {
                output = format!("{}: {} {}. ", room_name, device_name, self.socket.info);
                Ok(output)
            } else if device_name == self.thermo.name {
                output = format!("{}: {} {}. ", room_name, device_name, self.thermo.info);
                Ok(output)
            } else {
                // output = format!("Error {} at {}. ", device_name, room_name);
                Err(NoDeviceFoundError)
            }
        } else {
            Err(NoDeviceFoundError)
        }
    }
}
