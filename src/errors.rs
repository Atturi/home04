use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ErrorDeviceAlreadyExists {}

impl Error for ErrorDeviceAlreadyExists {}

impl fmt::Display for ErrorDeviceAlreadyExists {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Устройство с таким именем уже существует")
    }
}

#[derive(Debug)]
pub struct ErrorRoomAlreadyExists {}

impl Error for ErrorRoomAlreadyExists {}

impl fmt::Display for ErrorRoomAlreadyExists {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Помещение с таким именем уже существует")
    }
}

#[derive(Debug)]
pub struct ErrorDeviceNotExists {}

impl Error for ErrorDeviceNotExists {}

impl fmt::Display for ErrorDeviceNotExists {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Устройство с таким именем не найдено")
    }
}

#[derive(Debug)]
pub struct ErrorRoomNotExists {}

impl Error for ErrorRoomNotExists {}

impl fmt::Display for ErrorRoomNotExists {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Помещение с таким именем не найдено")
    }
}
