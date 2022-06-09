// Домашнее задание : делим и тестируем прототип "умного дома"

// Разделить логически целостные элементы библиотеки ""умный дом"" на отдельные файлы.
// Покрыть тестами требования к библиотеке.
// Создать example использования библиотеки.
// ------------------------------------------------------------------------------------------------------------
// Библиотека предоставляет структуру дома в комнатах которого расположены устройства.
// Дом имеет название и содержит несколько помещений.
// Библиотека позволяет запросить список помещений в доме.
// Помещение имеет уникальное название и содержит названия нескольких устройств.
// Устройство имеет уникальное в рамках помещения имя.
// Библиотека позволяет получать список устройств в помещении.
// Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома.
// Эта функция принимает в качестве аргумента обобщённый тип, позволяющий получить
// текстовую информацию о состоянии устройства, для включения в отчёт.
// Эта информация должна предоставляться для каждого устройства на основе данных
// о положении устройства в доме: имени комнаты и имени устройства.
// Если устройство не найдено в источнике информации, то вместо текста о состоянии вернуть сообщение об ошибке.
// Шаблон для описания сущностей библиотеки: https://gist.github.com/76dff7177f19ff7e802b1e121865afe4
// -------------------------------------------------------------------------------------------------------------
// Статус "Принято" ставится, если:
//     Код логически верно разбит на модули.
//     Утилита cargo clippy не выдаёт предупреждений.
//     Команда cargo fmt --check --all не выдаёт предупреждений.
//     Написаны тесты для функционала библиотеки.
//      В примере инициализируется "Умный дом" и источник информации об устройствах.
//      На экран выводится список комнат и устройств в них.
//      На экран выводится отчёт о состоянии дома.

use std::collections::HashMap;

const DEVICE_COUNT: usize = 5;

const EMPTY_ENTRY: &str = "";

struct SmartHouse<'a> {
    rooms: HashMap<&'a str, [&'a str; DEVICE_COUNT]>,
}

impl SmartHouse<'_> {
    fn new() -> Self {
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

    fn get_rooms(&self) -> impl Iterator<Item = &&str> {
        // Размер возвращаемого массива можно выбрать самостоятельно
        self.rooms.keys()
    }

    fn print_rooms_with_devices(&self) {
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
    fn devices_at_room(&self, room: &str) -> [&str; DEVICE_COUNT] {
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

    fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
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

trait DeviceInfoProvider {
    // метод, возвращающий состояние устройства по имени комнаты и имени устройства
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
// Могут как хранить устройства, так и заимствовать.
struct OwningDeviceInfoProvider<'a> {
    socket: SmartSocket<'a>,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket<'a>,
    thermo: &'b SmartThermometer<'b>,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider<'_> {
    fn info(&self, room_name: &str, device_name: &str) -> String {
        let mut output = EMPTY_ENTRY.to_owned();
        if !device_name.is_empty() {
            if device_name == self.socket.name {
                output = format!("{} - {} - {}\n", room_name, device_name, self.socket.info);
            } else {
                output = format!("Error {} at {}\n", device_name, room_name);
            }
        }
        output
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn info(&self, room_name: &str, device_name: &str) -> String {
        let mut output = EMPTY_ENTRY.to_owned();
        if !device_name.is_empty() {
            if device_name == self.socket.name {
                output = format!("{} - {} - {}\n", room_name, device_name, self.socket.info);
            } else if device_name == self.thermo.name {
                if device_name == self.thermo.name {
                    output = format!("{} - {} - {}\n", room_name, device_name, self.thermo.info);
                }
            } else {
                output = format!("Error {} at {}\n", device_name, room_name);
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
