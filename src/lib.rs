#![allow(non_snake_case)]
pub mod OwningDeviceInfoProvider;
pub mod BorrowingDeviceInfoProvider;
pub mod DeviceInfoProvider;
pub mod House;
pub mod Room;
pub mod Thermometer;

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
/*
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
}*/
