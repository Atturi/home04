use crate::Thermometer::Thermometer;
use crate::SmartDevice::SmartDevice;

#[cfg(test)]
pub mod test_thermometer
{
    use super::{Thermometer, SmartDevice};
    #[test]
    pub fn get_init_description()
    {
        let thermometer = Thermometer::new("thermometer".to_string(), 19.3, "room".to_string());

        assert_eq!(19.3, thermometer.get_temperature());
        assert_eq!("thermometer".to_string(), thermometer.info());
        assert_eq!("room".to_string(), thermometer.get_room_name());
    }

    #[test]
    pub fn set_room_name()
    {
        let mut thermometer = Thermometer::new("thermometer".to_string(), 19.3, "room1".to_string());

        thermometer.set_room_name("room2".to_string());

        assert_eq!("room2".to_string(), thermometer.get_room_name());
    }
}
