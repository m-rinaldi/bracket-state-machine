use std::time::Duration;
use super::DeltaTime;

pub struct Stopwatch {
    current: Duration,
    threshold: Duration,
}

impl Stopwatch {
    pub fn new(threshold: Duration) -> Self {
        Stopwatch {
            current: Duration::new(0, 0),
            threshold,
        }
    }

    pub fn reset(&mut self) {
        self.current = Duration::new(0, 0);
    }

    pub fn tick(&mut self, dt: DeltaTime) {
        if self.is_over() {
            return;
        }

        self.current = self.current.saturating_add(*dt);
    }

    pub fn is_over(&mut self) -> bool {
        return self.current >= self.threshold;
    }
}