use std::hash::{Hash, Hasher};

/// Шаблон для умных устройств
pub trait SmartDevice {
    fn info(&self) -> String;
    fn get_room_name(&self) -> Option<String>;
    fn set_room_name(&mut self, name: String);
}

impl Eq for dyn SmartDevice {}
impl PartialEq for dyn SmartDevice {
    fn eq(&self, other: &dyn SmartDevice) -> bool {
        self.info() == other.info() && self.get_room_name() == other.get_room_name()
    }
}

impl Hash for dyn SmartDevice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.info().hash(state)
    }
}
