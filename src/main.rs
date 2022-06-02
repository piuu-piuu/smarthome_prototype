// Метка todo - реализовать самостоятельно

// ***** Пример библиотеки "Умный дом" со статическим содержимым

// Библиотека предоставляет структуру дома в комнатах которого расположены устройства.

// Дом имеет название и содержит несколько помещений.
// Библиотека позволяет запросить список помещений в доме.
// Помещение имеет уникальное название и содержит названия нескольких устройств.
// Устройство имеет уникальное в рамках помещения имя.
// Библиотека позволяет получать список устройств в помещении.
// Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома.
// Эта функция принимает в качестве аргумента обобщённый тип,
// позволяющий получить текстовую информацию о состоянии устройства, для включения в отчёт.
// Эта информация должна предоставляться для каждого устройства на основе данных о положении устройства в доме: имени комнаты и имени устройства.
// Если устройство не найдено в источнике информации, то вместо текста о состоянии вернуть сообщение об ошибке.
// Привести пример типа, предоставляющего текстовую информацию об устройствах в доме для составления отчёта.

// "Принято", если:
// Утилита cargo clippy не выдаёт предупреждений.
// Команда cargo fmt --check не выдаёт предупреждений.
// В функции main инициализируется "Умный дом" и источник информации об устройствах.
// На экран выводится список комнат и устройств в них. На экран выводится отчёт о состоянии дома.

use std::collections::HashMap;

const DEVICE_COUNT: usize = 5;

const ROOMS_COUNT: usize = 3;

struct SmartHouse<'a> {
    rooms: HashMap<&'a str, [&'a str; DEVICE_COUNT]>,
}

impl SmartHouse<'_> {
    fn new() -> Self {
        SmartHouse {
            rooms: HashMap::from([
                ("bedroom", ["socket", "socket1", "", "", ""]),
                ("kitchen", ["socket1", "socket2", "thermo", "", ""]),
                ("living_room", ["thermo", "socket3", "thermo1", "", ""]),
            ]),
        }
    }

    fn get_rooms(&self) -> [&str; ROOMS_COUNT] {
        // Размер возвращаемого массива можно выбрать самостоятельно
        let i = 0;
        let mut result = ["", "", ""];
        for (key, _value) in &self.rooms {
            result[i] = key;
        }
        result
    }

    fn devices(&self, room: &str) -> [&str; DEVICE_COUNT] {
        // Размер возвращаемого массива можно выбрать самостоятельно
        let mut result = ["", "", "", "", ""];
        for (key, value) in &self.rooms {
            if *key == room {
                result = *value;
            }
        }
        result
    }

    fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        // todo!("перебор комнат и устройств в них для составления отчёта")
        let mut report = "".to_owned();
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

trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn info(&self, room_name: &str, device_name: &str) -> String;
}

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:
struct SmartSocket<'a> {
    name: &'a str,
    info: &'a str,
}
struct SmartThermometer<'a> {
    name: &'a str,
    info: &'a str,
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
struct OwningDeviceInfoProvider<'a> {
    socket: SmartSocket<'a>,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket<'a>,
    thermo: &'b SmartThermometer<'b>,
}

// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации

impl DeviceInfoProvider for OwningDeviceInfoProvider<'_> {
    fn info(&self, room_name: &str, device_name: &str) -> String {
        let mut output = "".to_owned();
        if device_name == self.socket.name {
            output.push_str(room_name);
            output.push_str(" - ");
            output.push_str(device_name);
            output.push_str(" - ");
            output.push_str(self.socket.info);
            output.push_str("\n");
        }
        output
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn info(&self, room_name: &str, device_name: &str) -> String {
        let mut output = "".to_owned();
        if device_name == self.socket.name {
            output.push_str(room_name);
            output.push_str(" - ");
            output.push_str(device_name);
            output.push_str(" - ");
            output.push_str(self.socket.info);
            output.push_str("\n");
        } else if device_name == self.thermo.name {
            if device_name == self.thermo.name {
                output.push_str(room_name);
                output.push_str(" - ");
                output.push_str(device_name);
                output.push_str(" - ");
                output.push_str(self.thermo.info);
                output.push_str("\n");
            }
        }
        output
    }
}

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
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1:\n{report1}");
    println!("Report #2:\n{report2}");

    println!("{:?}", house.get_rooms());
    println!("{:?}", house.devices("bedroom"));
}
