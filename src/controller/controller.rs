// embedded-presence-lighting-rust\src\controller\controller.rs

use crate::controller::state_machine::PresenceLightingStateMachine;
use crate::interfaces::motion::MotionSensor;
use crate::interfaces::light_sensor::AmbientLightSensor;
use crate::interfaces::light_actuator::LightActuator;

/// Controller wiring layer.
///
/// Owns the finite state machine and the hardware abstraction interfaces.
/// Translates sensor readings into FSM events and FSM decisions into actuator calls.
pub struct Controller<M, L, A>
where
    M: MotionSensor,
    L: AmbientLightSensor,
    A: LightActuator,
{
    fsm: PresenceLightingStateMachine,
    motion_sensor: M,
    light_sensor: L,
    light_actuator: A,
    last_motion_signal: bool,
}

impl<M, L, A> Controller<M, L, A>
where
    M: MotionSensor,
    L: AmbientLightSensor,
    A: LightActuator,
{
    pub fn new(
        fsm: PresenceLightingStateMachine,
        motion_sensor: M,
        light_sensor: L,
        light_actuator: A,
    ) -> Self {
        Controller {
            fsm,
            motion_sensor,
            light_sensor,
            light_actuator,
            last_motion_signal: false,
        }
    }

    /// Poll sensors and handle any resulting events.
    ///
    /// This method should be called by the runtime loop.
    pub fn poll(&mut self) {
        let motion_detected = self.motion_sensor.motion_detected();

        if motion_detected && !self.last_motion_signal {
            println!("[controller] motion detected");
        }

        self.last_motion_signal = motion_detected;

        if motion_detected {
            self.handle_motion_event();
        }
    }

    fn handle_motion_event(&mut self) {
        let ambient_light_is_low = self.light_sensor.is_low();

        let should_turn_on =
            self.fsm.on_motion_detected(ambient_light_is_low);

        if should_turn_on {
            println!("[controller] light turned on");
            self.light_actuator.turn_on();
        }
    }

    /// Advance time-based state transitions.
    ///
    /// Should be called regularly by the runtime loop.
    pub fn tick(&mut self) {
        let should_turn_off = self.fsm.tick();

        if should_turn_off {
            println!("[controller] light turned off");
            self.light_actuator.turn_off();
        }
    }

    /// Expose actuator state for testing and inspection.
    pub fn actuator(&self) -> &A {
        &self.light_actuator
    }
}
