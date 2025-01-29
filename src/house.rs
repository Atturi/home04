use super::{device_info_provider::DeviceInfoProvider, errors::*, room::*};
use colored::Colorize;
use std::collections::{HashMap, HashSet};

/// Умный дом
pub struct House {
    /// Название
    pub name: String,
    /// Помещения
    rooms: HashMap<String, Room>,
}

impl House {
    /// Инициализация дома
    pub fn new(name: String) -> Self {
        House {
            name,
            rooms: HashMap::new(),
        }
    }
    /// Получить список названий помещений
    pub fn get_rooms(&self) -> Vec<String> {
        let result: Vec<String> = self.rooms.keys().map(|name| name.clone()).collect();

        result
    }
    /// Добавить помещение
    pub fn add_room(&mut self, room: Room) -> Result<(), ErrorRoomAlreadyExists> {
        if !self.rooms.contains_key(&room.name) {
            self.rooms.insert(room.name.clone(), room);
            return Ok(());
        }

        Err(ErrorRoomAlreadyExists {})
    }

    /// Удалить помещение
    pub fn remove_room(&mut self, room_name: &String) -> Result<(), ErrorRoomNotExists> {
        match self.rooms.remove(room_name) {
            Some(_) => Ok(()),
            None => Err(ErrorRoomNotExists {}),
        }
    }

    /// Построение отчёта по источнику информации
    pub fn create_report(&self, info_provider: &dyn DeviceInfoProvider) -> String {
        let mut report = String::new();
        let mut room_name: String;

        for device in info_provider.get_devices().iter() {
            let device_result: String = match self.rooms.get(match device.get_room_name() {
                Some(n) => {
                    room_name = n;
                    &room_name
                }
                None => {
                    room_name = "".to_string();
                    &room_name
                }
            }) {
                Some(room) => {
                    let dvs: HashSet<String> = HashSet::from_iter(
                        room.devices.iter().map(|device_name| device_name.0.clone()),
                    );
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
