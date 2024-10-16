use super::{SmartDevice, DeviceInfoProvider::DeviceInfoProvider};

/// Источник данных для построения отчёта(заимствующий)
pub struct BorrowingDeviceInfoProvider<'a> {
    /// Устройства, по которым строится отчёт
    pub devices: Vec<&'a dyn SmartDevice>,
}

impl<'a> Default for BorrowingDeviceInfoProvider<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> BorrowingDeviceInfoProvider<'a> {
    /// Инициализация источника данных
    pub fn new() -> BorrowingDeviceInfoProvider<'a> {
        BorrowingDeviceInfoProvider {
            devices: Vec::new(),
        }
    }
}

impl<'a> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a> {
    /// Получить список устройств в источнике
    fn get_devices(&self) -> Vec<&'a dyn SmartDevice> {
        self.devices.clone()
    }
}
