use super::{errors::*, smart_device::SmartDevice};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Помещение
pub struct Room {
    /// Название помещения
    pub name: String,
    /// Множество устройств в помещении
    pub devices: HashMap<String, Box<dyn SmartDevice>>,
}

impl Room {
    /// Получить список устройств в помещении
    pub fn get_devices(&self) -> Vec<String> {
        let result: Vec<String> = self.devices.keys().map(|x| x.clone()).collect();

        result
    }

    /// Добавить устройство в помещение
    pub fn add_device(
        &mut self,
        mut device: Box<dyn SmartDevice>,
        device_origin: &mut dyn SmartDevice,
    ) -> Result<(), ErrorDeviceAlreadyExists> {
        if !self.devices.contains_key(&device.info()) {
            device.set_room_name(self.name.clone());
            self.devices.insert(device.info().clone(), device);
            device_origin.set_room_name(self.name.clone());
            return Ok(());
        }

        Err(ErrorDeviceAlreadyExists {})
    }

    /// Удалить устройство из помещения
    pub fn remove_device(&mut self, device_name: &String) -> Result<(), ErrorDeviceNotExists> {
        match self.devices.remove(device_name) {
            Some(_) => Ok(()),
            None => Err(ErrorDeviceNotExists {}),
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
