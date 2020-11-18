// embedded-presence-lighting-rust\tests\controller.rs

use embedded_presence_lighting::controller::controller::Controller;
use embedded_presence_lighting::controller::state_machine::PresenceLightingStateMachine;
use embedded_presence_lighting::interfaces::stub_devices::{
    StubAmbientLightSensor, StubLightActuator, StubMotionSensor,
};

#[test]
fn controller_turns_light_on_when_motion_and_low_light() {
    let motion = StubMotionSensor::new();
    let light = StubAmbientLightSensor::new(true);
    let actuator = StubLightActuator::new();

    let fsm = PresenceLightingStateMachine::default();
    let mut controller = Controller::new(fsm, motion, light, actuator);

    controller.handle_motion_event();

    assert!(controller.light_actuator.is_on());
}

#[test]
fn controller_turns_light_off_after_timeout() {
    let motion = StubMotionSensor::new();
    let light = StubAmbientLightSensor::new(true);
    let actuator = StubLightActuator::new();

    let fsm = PresenceLightingStateMachine::new(1, 1);
    let mut controller = Controller::new(fsm, motion, light, actuator);

    controller.handle_motion_event();
    assert!(controller.light_actuator.is_on());

    std::thread::sleep(std::time::Duration::from_secs(2));

    controller.tick();
    assert!(!controller.light_actuator.is_on());
}

