// src/interfaces/light_sensor.rs
pub trait AmbientLightSensor {
    fn is_low(&self) -> bool;
}
