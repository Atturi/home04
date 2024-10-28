use crate::Thermometer::Thermometer;

#[cfg(test)]
pub mod test_thermometer
{
    use super::Thermometer;
    #[test]
    pub fn get_correct_temperature()
    {
        let thermometer = Thermometer::new("thermometer".to_string(), 19.3, "room".to_string());

        assert_eq!(19.3, thermometer.get_temperature());
    }
}
