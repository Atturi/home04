use std::collections::HashMap;
use SmartHouse::house::House;
use SmartHouse::room::Room;
use SmartHouse::socket::Socket;

fn main() {
    let mut house = House::new("house".to_string());
    let mut socket = Socket::new("socket".to_string(), 1, true, None);
    let mut room = Room {
        name: "room".to_string(),
        devices: HashMap::new(),
    };

    let _ = room.add_device(Box::new(socket.clone()), &mut socket);

    let _ = house.add_room(room);
}
