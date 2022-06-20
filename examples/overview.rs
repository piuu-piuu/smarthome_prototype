// ***** Пример использования библиотеки умный дом

use smarthome_4::devices::*;
use smarthome_4::info::*;
use smarthome_4::smarthouse::*;

fn main() {
    // Инициализация устройств
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

    // Инициализация дома
    let house = SmartHouse::new();

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };

    let report2 = house.create_report(&info_provider_2);

    house.print_rooms_with_devices();
    // Выводим отчёты на экран:
    println!("Report #1:\n{report1}");
    println!("Report #2:\n{report2}");
}
