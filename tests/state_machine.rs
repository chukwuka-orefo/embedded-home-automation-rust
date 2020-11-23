// embedded-presence-lighting-rust\tests\state_machine.rs

use embedded_presence_lighting::controller::state_machine::{
    PresenceLightingStateMachine, State,
};

#[test]
fn idle_motion_low_light_turns_on() {
    let mut fsm = PresenceLightingStateMachine::new(300, 10);

    let turned_on = fsm.on_motion_detected(true);

    assert!(turned_on);
    assert_eq!(fsm.get_state(), State::LightOn);
}

#[test]
fn light_turns_off_after_timeout_and_cooldown_expires() {
    // Use zero durations to avoid sleeps in integration tests.
    let mut fsm = PresenceLightingStateMachine::new(0, 0);

    assert!(fsm.on_motion_detected(true));
    assert_eq!(fsm.get_state(), State::LightOn);

    // First tick transitions LightOn -> Cooldown and returns true (turn off).
    assert!(fsm.tick());
    assert_eq!(fsm.get_state(), State::Cooldown);

    // Second tick transitions Cooldown -> Idle.
    assert!(!fsm.tick());
    assert_eq!(fsm.get_state(), State::Idle);
}
