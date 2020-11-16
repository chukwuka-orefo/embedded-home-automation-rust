// embedded-presence-lighting-rust\src\main.rs

use controller::controller::Controller;
use controller::state_machine::PresenceLightingStateMachine;
use interfaces::stub_devices::*;

mod controller;
mod interfaces;

fn main() {
    let motion = StubMotionSensor::new();
    let light = StubAmbientLightSensor::new(true);
    let actuator = StubLightActuator::new();

    let fsm = PresenceLightingStateMachine::default();
    let mut controller = Controller::new(fsm, motion, light, actuator);

    controller.handle_motion_event();
    controller.tick();
}
