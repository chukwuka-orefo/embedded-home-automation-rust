// embedded-presence-lighting-rust\src\controller\controller.rs

use crate::controller::state_machine::PresenceLightingStateMachine;
use crate::interfaces::motion::MotionSensor;
use crate::interfaces::light_sensor::AmbientLightSensor;
use crate::interfaces::light_actuator::LightActuator;

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
        }
    }

    pub fn handle_motion_event(&mut self) {
        let ambient_light_is_low = self.light_sensor.is_low();

        let should_turn_on =
            self.fsm.on_motion_detected(ambient_light_is_low);

        if should_turn_on {
            self.light_actuator.turn_on();
        }
    }

    pub fn tick(&mut self) {
        let should_turn_off = self.fsm.tick();

        if should_turn_off {
            self.light_actuator.turn_off();
        }
    }
}
