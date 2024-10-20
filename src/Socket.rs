use super::SmartDevice;

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
