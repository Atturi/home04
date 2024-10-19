use std::collections::HashSet;
use super::{DeviceInfoProvider, Room::*};

/// Умный дом
pub struct House {
    /// Название
    pub name: String,
    /// Помещения
    rooms: HashSet<Room>,
}

impl House {
    /// Инициализация дома
    pub fn new(name: String) -> Self {
        House {
            name,
            rooms: HashSet::new(),
        }
    }
    /// Получить список названий помещений
    pub fn get_rooms(&self) -> Vec<String> {
        let result: Vec<String> = self.rooms.iter().map(|room| room.name.clone()).collect();

        result
    }
    /// Добавить помещение
    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room);
    }
    /// Построение отчёта по источнику информации
    pub fn create_report(&self, info_provider: &dyn DeviceInfoProvider::DeviceInfoProvider) -> String {
        let mut report = String::new();

        for device in info_provider.get_devices().iter() {
            let device_result: String = match self.rooms.get(&Room {
                name: device.get_room_name(),
                devices: HashSet::new(),
            }) {
                Some(room) => {
                    let dvs: HashSet<String> =
                        HashSet::from_iter(room.devices.iter().map(|d| d.info()));
                    match dvs.get(&(*device).info()) {
                        //nm.devices.get(&device){
                        Some(dv) => format!("{} : {}\n", room.name, dv),
                        None => format!(
                            "Ошибка! Устройство {} не найдено в помещении {}\n",
                            (*device).info(),
                            (*device).get_room_name()
                        ),
                    }
                }
                None => format!("Ошибка! Помещение {} не найдено\n", device.get_room_name()),
            };

            report.push_str(&device_result);
        }

        report
    }
}
