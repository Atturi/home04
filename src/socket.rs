use super::smart_device::SmartDevice;

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
    room: Option<String>,
}

impl Socket {
    /// Создание розетки
    pub fn new(
        info: String,
        power_consumption: u32,
        is_active: bool,
        room: Option<String>,
    ) -> Socket {
        Socket {
            info,
            power_consumption,
            is_active,
            room,
        }
    }
    /// Получить текущую потребляемую мощность
    pub fn get_power_consumption(&self) -> u32 {
        self.power_consumption
    }
    /// Включить розетку
    pub fn switch_on(&mut self) {
        self.is_active = true;
    }
    /// Выключить розетку
    pub fn switch_off(&mut self) {
        self.is_active = false;
    }
    /// Получить текущее состояние розетки
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    /// Изменить помещение
    fn _set_room(&mut self, r: String) {
        self.room = Some(r);
    }
}

impl SmartDevice for Socket {
    fn info(&self) -> String {
        self.info.clone()
    }

    fn get_room_name(&self) -> Option<String> {
        self.room.clone()
    }

    fn set_room_name(&mut self, name: String) {
        self.room = Some(name);
    }
}
