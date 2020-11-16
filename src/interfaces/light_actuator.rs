// src/interfaces/light_actuator.rs
pub trait LightActuator {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
}
