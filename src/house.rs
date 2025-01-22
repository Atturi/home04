use colored::Colorize;
use super::{device_info_provider::DeviceInfoProvider, errors::*, room::*};
use std::collections::HashSet;

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
    pub fn add_room(&mut self, room: Room) -> Result<(), ErrorRoomAlreadyExists> {
        if !self.rooms.contains(&room) {
            self.rooms.insert(room);
            return Ok(());
        }

        Err(ErrorRoomAlreadyExists {})
    }
    /// Построение отчёта по источнику информации
    pub fn create_report(&self, info_provider: &dyn DeviceInfoProvider) -> String {
        let mut report = String::new();

        for device in info_provider.get_devices().iter() {
            let device_result: String = match self.rooms.get(&Room {
                name: match device.get_room_name() {
                    Some(n) => n,
                    None => "".to_string(),
                },
                devices: HashSet::new(),
            }) {
                Some(room) => {
                    let dvs: HashSet<String> =
                        HashSet::from_iter(room.devices.iter().map(|d| d.info()));
                    match dvs.get(&(*device).info()) {
                        //nm.devices.get(&device){
                        Some(dv) => format!("{} : {}\n", room.name, dv),
                        None => format!(
                            "{} Устройство {} не найдено в помещении {}\n",
                            "Ошибка!".red(),
                            (*device).info(),
                            match (*device).get_room_name() {
                                Some(r) => r,
                                None => "NULL".to_string(),
                            }
                        ),
                    }
                }
                None => format!(
                    "{} Помещение {} не найдено\n",
                    "Ошибка!".red(),
                    match device.get_room_name() {
                        Some(r) => r,
                        None => "NULL".to_string(),
                    }
                ),
            };

            report.push_str(&device_result);
        }

        report
    }
}
