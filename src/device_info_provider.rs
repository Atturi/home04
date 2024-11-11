use super::smart_device::SmartDevice;

/// Источник данных для построения отчёта
pub trait DeviceInfoProvider {
    fn get_devices(&self) -> Vec<&dyn SmartDevice>;
}
