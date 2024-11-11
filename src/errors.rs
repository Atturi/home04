use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ErrorDeviceAlreadyExists{}

impl Error for ErrorDeviceAlreadyExists{}

impl fmt::Display for ErrorDeviceAlreadyExists
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Device already exists in this room")
    }
}

#[derive(Debug)]
pub struct ErrorRoomAlreadyExists{}

impl Error for ErrorRoomAlreadyExists{}

impl fmt::Display for ErrorRoomAlreadyExists
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Room already exists in this house")
    }
}
