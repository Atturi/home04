#[allow(unused_imports)]
use crate::House::House;
#[allow(unused_imports)]
use crate::Room::Room;
#[allow(unused_imports)]
use crate::SmartDevice::SmartDevice;
#[allow(unused_imports)]
use crate::Socket::Socket;
#[allow(unused_imports)]
use crate::Thermometer::Thermometer;
#[allow(unused_imports)]
use std::collections::HashSet;

#[cfg(test)]
pub mod test_thermometer {
    use super::{SmartDevice, Thermometer};

    #[test]
    pub fn get_init_description() {
        let thermometer = Thermometer::new("thermometer".to_string(), 19.3, Some("room".to_string()));

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
    use super::{HashSet, Room, Socket};

    #[test]
    pub fn eq() {
        let room1 = Room {
            name: "room_name".to_string(),
            devices: HashSet::new(),
        };
        let room2 = Room {
            name: "room_name".to_string(),
            devices: HashSet::new(),
        };
        let room3 = Room {
            name: "second_room_name".to_string(),
            devices: HashSet::new(),
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
            devices: HashSet::new(),
        };

        room_socket.add_device(Box::new(socket1.clone()), &mut socket1);
        room_socket.add_device(Box::new(socket1.clone()), &mut socket1);
        room_socket.add_device(Box::new(socket2.clone()), &mut socket2);

        assert_eq!(2, room_socket.get_devices().len());
    }
}

#[cfg(test)]
pub mod test_house {
    use super::{HashSet, House, Room};

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
            devices: HashSet::new(),
        };
        let room2 = Room {
            name: "room2".to_string(),
            devices: HashSet::new(),
        };
        let room3 = Room {
            name: "room2".to_string(),
            devices: HashSet::new(),
        };

        assert_eq!(0, house.get_rooms().len());

        house.add_room(room1);
        house.add_room(room2);
        house.add_room(room3);

        assert_eq!(2, house.get_rooms().len());
    }
}
