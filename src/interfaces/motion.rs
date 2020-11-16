// src/interfaces/motion.rs
pub trait MotionSensor {
    fn motion_detected(&self) -> bool;
}
