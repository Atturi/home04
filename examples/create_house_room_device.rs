use std::collections::HashSet;
use SmartHouse::House::House;
use SmartHouse::Room::Room;
use SmartHouse::Socket::Socket;

fn main() {
    let mut house = House::new("house".to_string());
    let mut socket = Socket::new("socket".to_string(), 1, true, "".to_string());
    let mut room = Room {
        name: "room".to_string(),
        devices: HashSet::new(),
    };

    room.add_device(Box::new(socket.clone()), &mut socket);

    house.add_room(room);
}
