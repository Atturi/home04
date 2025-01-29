#[allow(unused_imports)]
use crate::house::House;
#[allow(unused_imports)]
use crate::room::Room;
#[allow(unused_imports)]
use crate::smart_device::SmartDevice;
#[allow(unused_imports)]
use crate::socket::Socket;
#[allow(unused_imports)]
use crate::thermometer::Thermometer;
#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(test)]
pub mod test_thermometer {
    use super::{SmartDevice, Thermometer};

    #[test]
    pub fn get_init_description() {
        let thermometer =
            Thermometer::new("thermometer".to_string(), 19.3, Some("room".to_string()));

        assert_eq!(19.3, thermometer.get_temperature());
        assert_eq!("thermometer".to_string(), thermometer.info());
        assert_eq!(Some("room".to_string()), thermometer.get_room_name());
    }

    #[test]
    pub fn set_room_name() {
        let mut thermometer =
            Thermometer::new("thermometer".to_string(), 19.3, Some("room1".to_string()));

        thermometer.set_room_name("room2".to_string());

        assert_eq!(Some("room2".to_string()), thermometer.get_room_name());
    }
}

#[cfg(test)]
pub mod test_socket {
    use super::{SmartDevice, Socket};

    #[test]
    pub fn get_init_description() {
        let socket = Socket::new("socket".to_string(), 3, true, Some("room".to_string()));

        assert_eq!(3, socket.get_power_consumption());
        assert_eq!(true, socket.is_active());
        assert_eq!("socket".to_string(), socket.info());
        assert_eq!(Some("room".to_string()), socket.get_room_name());
    }

    #[test]
    pub fn switch_on_off() {
        let mut socket = Socket::new("socket".to_string(), 3, false, Some("room".to_string()));

        socket.switch_on();
        assert_eq!(true, socket.is_active());

        socket.switch_off();
        assert_eq!(false, socket.is_active());
    }

    #[test]
    pub fn set_room_name() {
        let mut socket = Socket::new("socket".to_string(), 3, false, Some("room1".to_string()));

        socket.set_room_name("room2".to_string());

        assert_eq!(Some("room2".to_string()), socket.get_room_name());
    }
}

#[cfg(test)]
pub mod test_room {
    use super::{HashMap, Room, SmartDevice, Socket};

    #[test]
    pub fn eq() {
        let room1 = Room {
            name: "room_name".to_string(),
            devices: HashMap::new(),
        };
        let room2 = Room {
            name: "room_name".to_string(),
            devices: HashMap::new(),
        };
        let room3 = Room {
            name: "second_room_name".to_string(),
            devices: HashMap::new(),
        };

        assert_eq!(true, room1.eq(&room2));
        assert_eq!(false, room1.eq(&room3));
    }

    #[test]
    pub fn add_device_get_devices() {
        let mut socket1 = Socket::new("socket1".to_string(), 0, false, None);
        let mut socket2 = Socket::new("socket2".to_string(), 1, true, None);

        let mut room_socket = Room {
            name: "room_socket".to_string(),
            devices: HashMap::new(),
        };

        let _ = room_socket.add_device(Box::new(socket1.clone()), &mut socket1);
        let _ = room_socket.add_device(Box::new(socket1.clone()), &mut socket1);
        let _ = room_socket.add_device(Box::new(socket2.clone()), &mut socket2);

        assert_eq!(2, room_socket.get_devices().len());
    }

    #[test]
    pub fn remove_device() {
        let mut socket1 = Socket::new("socket1".to_string(), 0, false, None);
        let mut socket2 = Socket::new("socket2".to_string(), 2, true, None);

        let mut room = Room {
            name: "room".to_string(),
            devices: HashMap::new(),
        };

        let _ = room.add_device(Box::new(socket1.clone()), &mut socket1);
        let _ = room.add_device(Box::new(socket2.clone()), &mut socket2);

        let _ = room.remove_device(&socket1.info());

        assert_eq!(1, room.devices.len());

        let name = "qwerty".to_string();

        let _ = room.remove_device(&name);

        assert_eq!(1, room.devices.len());
    }
}

#[cfg(test)]
pub mod test_house {
    use super::{HashMap, House, Room};

    #[test]
    pub fn get_init_description() {
        let house = House::new("house1".to_string());

        assert_eq!("house1".to_string(), house.name);
    }

    #[test]
    pub fn add_room_get_rooms() {
        let mut house = House::new("house".to_string());
        let room1 = Room {
            name: "room1".to_string(),
            devices: HashMap::new(),
        };
        let room2 = Room {
            name: "room2".to_string(),
            devices: HashMap::new(),
        };
        let room3 = Room {
            name: "room2".to_string(),
            devices: HashMap::new(),
        };

        assert_eq!(0, house.get_rooms().len());

        let _ = house.add_room(room1);
        let _ = house.add_room(room2);
        let _ = house.add_room(room3);

        assert_eq!(2, house.get_rooms().len());
    }

    #[test]
    pub fn remove_room() {
        let mut house = House::new("house".to_string());

        let room1 = Room {
            name: "room1".to_string(),
            devices: HashMap::new(),
        };
        let room2 = Room {
            name: "room2".to_string(),
            devices: HashMap::new(),
        };

        let _ = house.add_room(room1);
        let _ = house.add_room(room2);

        let room_name = "room1".to_string();

        let _ = house.remove_room(&room_name);

        assert_eq!(1, house.get_rooms().len());
    }
}
