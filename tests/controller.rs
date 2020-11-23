// embedded-presence-lighting-rust\tests\controller.rs

use embedded_presence_lighting::controller::controller::Controller;
use embedded_presence_lighting::controller::state_machine::PresenceLightingStateMachine;
use embedded_presence_lighting::interfaces::stub_devices::{
    StubAmbientLightSensor, StubLightActuator, StubMotionSensor,
};

#[test]
fn controller_turns_light_on_when_motion_and_low_light() {
    let mut motion = StubMotionSensor::new();
    motion.set_motion(true);

    let light = StubAmbientLightSensor::new(true);
    let actuator = StubLightActuator::new();

    let fsm = PresenceLightingStateMachine::default();
    let mut controller = Controller::new(fsm, motion, light, actuator);

    controller.poll();

    assert!(controller.actuator().is_on());
}

#[test]
fn controller_turns_light_off_after_timeout() {
    let mut motion = StubMotionSensor::new();
    motion.set_motion(true);

    let light = StubAmbientLightSensor::new(true);
    let actuator = StubLightActuator::new();

    // Use zero durations to avoid sleeps in integration tests.
    let fsm = PresenceLightingStateMachine::new(0, 0);
    let mut controller = Controller::new(fsm, motion, light, actuator);

    controller.poll();
    assert!(controller.actuator().is_on());

    // First tick should trigger turn off immediately.
    controller.tick();
    assert!(!controller.actuator().is_on());

    // Second tick clears cooldown -> idle, light stays off.
    controller.tick();
    assert!(!controller.actuator().is_on());
}
