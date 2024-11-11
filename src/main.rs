use std::collections::HashSet;
use SmartHouse::{
    borrowing_device_info_provider::BorrowingDeviceInfoProvider, house::*,
    owning_device_info_provider::OwningDeviceInfoProvider, room::*, socket::*, thermometer::*,
};

fn main() {
    let mut socket1 = Socket::new(String::from("socket1"), 0, false, None);
    let mut socket2 = Socket::new(String::from("socket2"), 1, true, None);

    let mut room_socket = Room {
        name: "room_socket".to_string(),
        devices: HashSet::new(),
    };

    match room_socket.add_device(Box::new(socket1.clone()), &mut socket1) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
    match room_socket.add_device(Box::new(socket1.clone()), &mut socket1) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
    match room_socket.add_device(Box::new(socket2.clone()), &mut socket2) {
        Ok(()) => (),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\nУстройства помещения {}:", room_socket.name);
    for name in room_socket.get_devices().iter() {
        println!("{name}");
    }

    let mut thermometer1 = Thermometer::new(
        "thermometer1".to_string(),
        -11.3_f32,
        Some("garden".to_string()),
    );

    let mut room_thermometer = Room {
        name: "room_thermometer".to_string(),
        devices: HashSet::new(),
    };

    match room_thermometer.add_device(Box::new(thermometer1.clone()), &mut thermometer1) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\nУстройства помещения {}:", room_thermometer.name);
    for name in room_thermometer.get_devices().iter() {
        println!("{name}");
    }

    let mut house = House::new("house1".to_string());

    match house.add_room(room_socket) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }

    match house.add_room(room_thermometer) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\nПомещения дома {}:", house.name);
    for room in house.get_rooms().iter() {
        println!("{}", room);
    }

    println!("\n\x1b[93mСоздание отчётов:\x1b[0m\n");

    let mut bdip = BorrowingDeviceInfoProvider::new();
    bdip.devices.push(&socket1);
    bdip.devices.push(&socket2);

    println!("Отчёт 1(borrowing):");
    println!("{}\n", house.create_report(&bdip));

    let mut odip = OwningDeviceInfoProvider::new();

    let socket3 = Socket::new(
        "socket3".to_string(),
        4,
        true,
        Some("not_existsting_room".to_string()),
    );
    let socket4 = Socket::new(
        "socket4".to_string(),
        5,
        true,
        Some("room_socket".to_string()),
    );

    odip.devices.push(Box::new(socket1));
    odip.devices.push(Box::new(socket3));
    odip.devices.push(Box::new(thermometer1));
    odip.devices.push(Box::new(socket4));

    println!("Отчёт 2(owning):");
    println!("{}\n", house.create_report(&odip));
}
