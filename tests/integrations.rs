use smarthome_4::devices::{SmartSocket, SmartThermometer};
use smarthome_4::info::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};
use smarthome_4::smarthouse::SmartHouse;

#[test]
fn house_has_name() {
    let house = SmartHouse::new();
    assert!(!house.name.is_empty());
}
#[test]
fn house_has_rooms() {
    let house = SmartHouse::new();
    assert_ne!(house.rooms.keys().len(), 0);
}

#[test]
fn house_rooms_has_devices() {
    let house = SmartHouse::new();
    let socket1 = SmartSocket {
        name: "socket1",
        info: "SmartSocket",
    };
    let socket2 = SmartSocket {
        name: "socket2",
        info: "SmartSocket",
    };
    let thermo = SmartThermometer {
        name: "thermo",
        info: "SmartThermometer",
    };
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let _report1 = house.create_report(&info_provider_1);
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let report2 = house.create_report(&info_provider_2);

    assert_eq!(report2.to_string(), "bedroom - socket2 - SmartSocket. kitchen - thermo - SmartThermometer. kitchen - thermo - SmartThermometer. living_room - thermo - SmartThermometer. living_room - socket2 - SmartSocket. ")
}
