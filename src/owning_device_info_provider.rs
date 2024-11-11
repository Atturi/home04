use super::{device_info_provider::DeviceInfoProvider, smart_device::SmartDevice};

pub struct OwningDeviceInfoProvider {
    pub devices: Vec<Box<dyn SmartDevice>>,
}

impl OwningDeviceInfoProvider {
    pub fn new() -> OwningDeviceInfoProvider {
        OwningDeviceInfoProvider {
            devices: Vec::new(),
        }
    }
}

impl Default for OwningDeviceInfoProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_devices(&self) -> Vec<&dyn SmartDevice> {
        let mut result_vec: Vec<&dyn SmartDevice> = Vec::new();

        for device in self.devices.iter() {
            result_vec.push(&(**device));
        }

        result_vec
    }
}
