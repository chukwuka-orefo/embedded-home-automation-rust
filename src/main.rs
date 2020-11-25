// embedded-presence-lighting-rust\src\main.rs

use std::thread;
use std::time::Duration;

use controller::controller::Controller;
use controller::state_machine::PresenceLightingStateMachine;
use interfaces::stub_devices::*;

mod controller;
mod interfaces;

fn main() {
    println!("[main] starting embedded-presence-lighting (stub mode)");

    let motion = StubMotionSensor::new();
    let light = StubAmbientLightSensor::new(true);
    let actuator = StubLightActuator::new();

    let fsm = PresenceLightingStateMachine::default();
    let mut controller = Controller::new(fsm, motion, light, actuator);

    loop {
        // Poll sensors and handle any resulting events
        controller.poll();

        // Allow FSM time-based transitions
        controller.tick();

        // Prevent busy looping
        thread::sleep(Duration::from_millis(200));
    }
}
