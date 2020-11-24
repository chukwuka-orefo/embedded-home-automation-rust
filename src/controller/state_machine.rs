// embedded-presence-lighting-rust\src\controller\state_machine.rs
use std::fmt;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    MotionDetected,
    LightOn,
    Cooldown,
}

impl State {
    pub fn as_str(&self) -> &'static str {
        match self {
            State::Idle => "idle",
            State::MotionDetected => "motion_detected",
            State::LightOn => "light_on",
            State::Cooldown => "cooldown",
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Deterministic finite state machine for presence based lighting control.
///
/// This struct contains no hardware logic. It only models behaviour and time.
/// Sensor readings and actuators are injected via method calls.
pub struct PresenceLightingStateMachine {
    state: State,

    light_on_timeout: Duration,
    cooldown: Duration,

    last_motion: Option<Instant>,
    state_entered: Instant,
}

impl PresenceLightingStateMachine {
    pub fn new(light_on_timeout_seconds: u64, cooldown_seconds: u64) -> Self {
        PresenceLightingStateMachine {
            state: State::Idle,
            light_on_timeout: Duration::from_secs(light_on_timeout_seconds),
            cooldown: Duration::from_secs(cooldown_seconds),
            last_motion: None,
            state_entered: Instant::now(),
        }
    }

    fn enter_state(&mut self, new_state: State, now: Instant) {
        let old_state = self.state;

        self.state = new_state;
        self.state_entered = now;

        if old_state != new_state
            && old_state != State::MotionDetected
            && new_state != State::MotionDetected
        {
            println!("[fsm] state: {} -> {}", old_state, new_state);
        }
    }

    pub fn on_motion_detected(&mut self, ambient_light_is_low: bool) -> bool {
        let now = Instant::now();
        self.on_motion_detected_at(ambient_light_is_low, now)
    }

    fn on_motion_detected_at(&mut self, ambient_light_is_low: bool, now: Instant) -> bool {
        match self.state {
            State::Idle => {
                self.enter_state(State::MotionDetected, now);

                if ambient_light_is_low {
                    self.last_motion = Some(now);
                    self.enter_state(State::LightOn, now);
                    return true;
                }

                self.enter_state(State::Idle, now);
                false
            }
            State::LightOn => {
                self.last_motion = Some(now);
                false
            }
            State::MotionDetected | State::Cooldown => {
                // Motion during cooldown and transient detection state is ignored
                false
            }
        }
    }

    pub fn tick(&mut self) -> bool {
        let now = Instant::now();
        self.tick_at(now)
    }

    fn tick_at(&mut self, now: Instant) -> bool {
        if self.state == State::LightOn {
            if let Some(last_motion) = self.last_motion {
                if now.duration_since(last_motion) >= self.light_on_timeout {
                    self.enter_state(State::Cooldown, now);
                    return true;
                }
            }
        }

        if self.state == State::Cooldown {
            if now.duration_since(self.state_entered) >= self.cooldown {
                self.enter_state(State::Idle, now);
            }
        }

        false
    }

    pub fn get_state(&self) -> State {
        self.state
    }
}

impl Default for PresenceLightingStateMachine {
    fn default() -> Self {
        PresenceLightingStateMachine::new(300, 10)
    }
}

#[cfg(test)]
impl PresenceLightingStateMachine {
    pub fn test_on_motion_detected_at(
        &mut self,
        ambient_light_is_low: bool,
        now: Instant,
    ) -> bool {
        self.on_motion_detected_at(ambient_light_is_low, now)
    }

    pub fn test_tick_at(&mut self, now: Instant) -> bool {
        self.tick_at(now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idle_motion_with_ambient_light_high_stays_idle() {
        let t0 = Instant::now();
        let mut m = PresenceLightingStateMachine::new(300, 10);

        let turned_on = m.on_motion_detected_at(false, t0);

        assert!(!turned_on);
        assert_eq!(m.state, State::Idle);
    }

    #[test]
    fn idle_motion_with_ambient_light_low_turns_on_and_enters_light_on() {
        let t0 = Instant::now();
        let mut m = PresenceLightingStateMachine::new(300, 10);

        let turned_on = m.on_motion_detected_at(true, t0);

        assert!(turned_on);
        assert_eq!(m.state, State::LightOn);
        assert_eq!(m.last_motion, Some(t0));
        assert_eq!(m.state_entered, t0);
    }

    #[test]
    fn light_on_motion_refreshes_timer() {
        let t0 = Instant::now();
        let mut m = PresenceLightingStateMachine::new(300, 10);

        assert!(m.on_motion_detected_at(true, t0));
        assert_eq!(m.last_motion, Some(t0));

        let t1 = t0 + Duration::from_secs(5);
        let turned_on = m.on_motion_detected_at(true, t1);

        assert!(!turned_on);
        assert_eq!(m.state, State::LightOn);
        assert_eq!(m.last_motion, Some(t1));
    }

    #[test]
    fn light_on_times_out_then_cooldown_then_idle() {
        let t0 = Instant::now();
        let mut m = PresenceLightingStateMachine::new(300, 10);

        assert!(m.on_motion_detected_at(true, t0));
        assert_eq!(m.state, State::LightOn);

        let t_before = t0 + Duration::from_secs(299);
        assert!(!m.tick_at(t_before));
        assert_eq!(m.state, State::LightOn);

        let t_timeout = t0 + Duration::from_secs(300);
        assert!(m.tick_at(t_timeout));
        assert_eq!(m.state, State::Cooldown);
        assert_eq!(m.state_entered, t_timeout);

        let t_motion_during_cooldown = t_timeout + Duration::from_secs(1);
        assert!(!m.on_motion_detected_at(true, t_motion_during_cooldown));
        assert_eq!(m.state, State::Cooldown);

        let t_cooldown_not_done = t_timeout + Duration::from_secs(9);
        assert!(!m.tick_at(t_cooldown_not_done));
        assert_eq!(m.state, State::Cooldown);

        let t_cooldown_done = t_timeout + Duration::from_secs(10);
        assert!(!m.tick_at(t_cooldown_done));
        assert_eq!(m.state, State::Idle);
    }
}
