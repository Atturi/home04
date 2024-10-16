pub mod OwningDeviceInfoProvider;
pub mod BorrowingDeviceInfoProvider;
pub mod DeviceInfoProvider;
pub mod House;

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

/// Шаблон для умных устройств
pub trait SmartDevice {
    fn info(&self) -> String;
    fn get_room_name(&self) -> String;
    fn set_room_name(&mut self, name: String);
}

impl Eq for dyn SmartDevice {}
impl PartialEq for dyn SmartDevice {
    fn eq(&self, other: &dyn SmartDevice) -> bool {
        self.info() == other.info() && self.get_room_name() == other.get_room_name()
    }
}

impl Hash for dyn SmartDevice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.info().hash(state)
    }
}

/// Умная розетка
#[derive(Clone)]
#[allow(dead_code)]
pub struct Socket {
    /// Описание
    info: String,
    /// Потребляемая мощность
    power_consumption: u32,
    /// Состояние(true - включена)
    is_active: bool,
    /// Помещение, в котором находится устройство
    room: String,
}

impl Socket {
    /// Создание розетки
    pub fn new(info: String, power_consumption: u32, is_active: bool, room: String) -> Socket {
        Socket {
            info,
            power_consumption,
            is_active,
            room,
        }
    }
    /// Получить текущую потребляемую мощность
    fn _get_power_consumption(&self) -> u32 {
        todo!();
    }
    /// Включить розетку
    fn _switch_on(&mut self) {
        todo!();
    }
    /// Выключить розетку
    fn _switch_off(&mut self) {
        todo!();
    }
    /// Изменить помещение
    fn _set_room(&mut self, r: String) {
        self.room = r;
    }
}

impl SmartDevice for Socket {
    fn info(&self) -> String {
        self.info.clone()
    }

    fn get_room_name(&self) -> String {
        self.room.clone()
    }

    fn set_room_name(&mut self, name: String) {
        self.room = name;
    }
}

/// Умный термометр
#[derive(Clone)]
#[allow(dead_code)]
pub struct Thermometer {
    /// Описание термометра
    info: String,
    /// Температура
    temperature: f32,
    /// Помещение, в котором находится термометр
    room: String,
}

impl Thermometer {
    /// Создание термометра
    pub fn new(info: String, temperature: f32, room: String) -> Thermometer {
        Thermometer {
            info,
            temperature,
            room,
        }
    }
    /// Получить текущую температуру
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

impl SmartDevice for Thermometer {
    fn info(&self) -> String {
        self.info.clone()
    }

    fn get_room_name(&self) -> String {
        self.room.clone()
    }

    fn set_room_name(&mut self, name: String) {
        self.room = name;
    }
}

/// Помещение
pub struct Room {
    /// Название помещения
    pub name: String,
    /// Множество устройств в помещении
    pub devices: HashSet<Box<dyn SmartDevice>>,
}

impl Room {
    /// Получить список устройств в помещении
    pub fn get_devices(&self) -> Vec<String> {
        let result: Vec<String> = self.devices.iter().map(|x| x.info().clone()).collect();

        result
    }

    pub fn add_device(
        &mut self,
        mut device: Box<dyn SmartDevice>,
        device_origin: &mut dyn SmartDevice,
    ) {
        if !self.devices.contains(&device) {
            device.set_room_name(self.name.clone());
            self.devices.insert(device);
            device_origin.set_room_name(self.name.clone());
        }
    }
}

impl Eq for Room {}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Room {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
/*
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
}*/
