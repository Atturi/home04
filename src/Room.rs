use super::SmartDevice::SmartDevice;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

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
