// embedded-presence-lighting-rust\tests\state_machine.rs

use std::time::{Duration, Instant};

use embedded_presence_lighting::controller::state_machine::{
    PresenceLightingStateMachine, State,
};

#[test]
fn idle_motion_low_light_turns_on() {
    let t0 = Instant::now();
    let mut fsm = PresenceLightingStateMachine::new(300, 10);

    let turned_on = fsm.on_motion_detected_at(true, t0);

    assert!(turned_on);
    assert_eq!(fsm.get_state(), State::LightOn);
}

#[test]
fn light_turns_off_after_timeout_and_cooldown_expires() {
    let t0 = Instant::now();
    let mut fsm = PresenceLightingStateMachine::new(5, 2);

    assert!(fsm.on_motion_detected_at(true, t0));
    assert_eq!(fsm.get_state(), State::LightOn);

    let t_timeout = t0 + Duration::from_secs(5);
    assert!(fsm.tick_at(t_timeout));
    assert_eq!(fsm.get_state(), State::Cooldown);

    let t_done = t_timeout + Duration::from_secs(2);
    fsm.tick_at(t_done);

    assert_eq!(fsm.get_state(), State::Idle);
}
