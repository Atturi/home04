#[allow(unused_imports)]
use crate::Thermometer::Thermometer;
#[allow(unused_imports)]
use crate::SmartDevice::SmartDevice;
#[allow(unused_imports)]
use crate::Socket::Socket;

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

#[cfg(test)]
pub mod test_socket
{
    use super::{Socket, SmartDevice};

    #[test]
    pub fn get_init_description()
    {
        let socket = Socket::new("socket".to_string(), 3, true, "room".to_string());

        assert_eq!(3, socket.get_power_consumption());
        assert_eq!(true, socket.is_active());
        assert_eq!("socket".to_string(), socket.info());
        assert_eq!("room".to_string(), socket.get_room_name());
    }

    #[test]
    pub fn switch_on_off()
    {
        let mut socket = Socket::new("socket".to_string(), 3, false, "room".to_string());

        socket.switch_on();
        assert_eq!(true, socket.is_active());

        socket.switch_off();
        assert_eq!(false, socket.is_active());
    }

    #[test]
    pub fn set_room_name()
    {
        let mut socket = Socket::new("socket".to_string(), 3, false, "room1".to_string());

        socket.set_room_name("room2".to_string());

        assert_eq!("room2".to_string(), socket.get_room_name());
    }
}
