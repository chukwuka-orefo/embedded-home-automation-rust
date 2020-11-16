// src/interfaces/stub_devices.rs

use super::motion::MotionSensor;
use super::light_sensor::AmbientLightSensor;
use super::light_actuator::LightActuator;

pub struct StubMotionSensor {
    motion: bool,
}

impl StubMotionSensor {
    pub fn new() -> Self {
        Self { motion: false }
    }

    pub fn set_motion(&mut self, motion: bool) {
        self.motion = motion;
    }
}

impl MotionSensor for StubMotionSensor {
    fn motion_detected(&self) -> bool {
        self.motion
    }
}

pub struct StubAmbientLightSensor {
    low: bool,
}

impl StubAmbientLightSensor {
    pub fn new(low: bool) -> Self {
        Self { low }
    }
}

impl AmbientLightSensor for StubAmbientLightSensor {
    fn is_low(&self) -> bool {
        self.low
    }
}

pub struct StubLightActuator {
    on: bool,
}

impl StubLightActuator {
    pub fn new() -> Self {
        Self { on: false }
    }

    pub fn is_on(&self) -> bool {
        self.on
    }
}

impl LightActuator for StubLightActuator {
    fn turn_on(&mut self) {
        self.on = true;
    }

    fn turn_off(&mut self) {
        self.on = false;
    }
}
