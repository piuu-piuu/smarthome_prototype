// use smarthome_4::devices::{SmartSocket, SmartThermometer, TcpServer, UdpServer, SMARTSOCKET};
use smarthome_4::devices::{SmartSocket, SmartThermometer};
use smarthome_4::info::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};
use smarthome_4::models::SmartHouse;
// use smarthome_4::remote::{reach_tcp, read_udp};
// use std::thread;

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
    let thermo1 = SmartThermometer {
        name: "thermo1",
        info: "SmartThermometer",
    };
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let _report1 = house.create_report(&info_provider_1);

    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo1,
    };
    let report1 = house.create_report(&info_provider_1);
    let report2 = house.create_report(&info_provider_2);

    assert_ne!(report1, "");
    assert_ne!(report2, "")
}

// #[allow(unused_variables)]
// #[test]
// fn tcp_test() {
//     let socket1 = SmartSocket {
//         name: "socket1",
//         info: "Smart Socket 220V 50VA",
//     };
//     thread::spawn(move || socket1.tcpconnect(SMARTSOCKET));
//     match reach_tcp(SMARTSOCKET, "info") {
//         Ok(response) => {
//             assert_eq!(response, "Smart Socket 220V 50VA")
//         }
//         Err(e) => {}
//     }
// }

// #[test]
// fn udp_test() {
//     let therm1 = SmartThermometer::new();
//     thread::spawn(move || therm1.udp_send("0.0.0.0:8888"));
//     let response = read_udp("127.0.0.1:8000", "read");
//     assert_eq!(response, "20`C")
// }
