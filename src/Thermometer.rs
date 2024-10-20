use super::SmartDevice::SmartDevice;

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
