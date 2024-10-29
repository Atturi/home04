extern crate SmartHouse;
use std::collections::HashSet;
use SmartHouse::BorrowingDeviceInfoProvider::BorrowingDeviceInfoProvider;
use SmartHouse::House::House;
use SmartHouse::OwningDeviceInfoProvider::OwningDeviceInfoProvider;
use SmartHouse::Room::Room;
use SmartHouse::Socket::Socket;
use SmartHouse::Thermometer::Thermometer;

#[test]
fn test_test() {
    let mut socket1 = Socket::new(String::from("socket1"), 0, false, "".to_string());
    let mut socket2 = Socket::new(String::from("socket2"), 1, true, "".to_string());

    let mut room_socket = Room {
        name: "room_socket".to_string(),
        devices: HashSet::new(),
    };

    room_socket.add_device(Box::new(socket1.clone()), &mut socket1);
    room_socket.add_device(Box::new(socket1.clone()), &mut socket1);
    room_socket.add_device(Box::new(socket2.clone()), &mut socket2);

    let mut thermometer1 =
        Thermometer::new("thermometer1".to_string(), -11.3_f32, "garden".to_string());

    let mut room_thermometer = Room {
        name: "room_thermometer".to_string(),
        devices: HashSet::new(),
    };

    room_thermometer.add_device(Box::new(thermometer1.clone()), &mut thermometer1);

    let mut house = House::new("house1".to_string());

    house.add_room(room_socket);
    house.add_room(room_thermometer);

    let mut bdip = BorrowingDeviceInfoProvider::new();
    bdip.devices.push(&socket1);
    bdip.devices.push(&socket2);

    let report_bdip = house.create_report(&bdip);

    assert!(report_bdip.contains("socket1"));
    assert!(report_bdip.contains("socket2"));

    let mut odip = OwningDeviceInfoProvider::new();

    let socket3 = Socket::new(
        "socket3".to_string(),
        4,
        true,
        "not_existing_room".to_string(),
    );

    let socket4 = Socket::new("socket4".to_string(), 5, true, "room_socket".to_string());

    odip.devices.push(Box::new(socket1));
    odip.devices.push(Box::new(socket3));
    odip.devices.push(Box::new(thermometer1));
    odip.devices.push(Box::new(socket4));

    let report_odip = house.create_report(&odip);

    assert!(report_odip.contains("socket1"));
    assert!(report_odip.contains("Ошибка! Помещение not_existing_room не найдено"));
    assert!(report_odip.contains("thermometer1"));
    assert!(report_odip.contains("Ошибка! Устройство socket4 не найдено в помещении room_socket"));
}
