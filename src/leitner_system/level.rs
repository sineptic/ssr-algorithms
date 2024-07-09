use serde::{Deserialize, Serialize};
use ssr_core::task::level::TaskLevel;
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize)]
pub struct LeitnerSystemLevel {
    pub(crate) group: u32,
    pub(crate) last_repetition_time: SystemTime,
}

impl Default for LeitnerSystemLevel {
    fn default() -> Self {
        Self {
            group: 1,
            last_repetition_time: SystemTime::UNIX_EPOCH,
        }
    }
}

impl TaskLevel for LeitnerSystemLevel {
    type Context = SystemTime;
    fn success(&mut self, current_time: SystemTime) {
        self.last_repetition_time = current_time;
        self.group = (self.group + 1).clamp(1, 4);
    }

    fn failure(&mut self, current_time: SystemTime) {
        self.last_repetition_time = current_time;
        self.group = (self.group - 1).clamp(1, 4);
    }

    fn until_next_repetition(&self) -> Duration {
        const DAY: Duration = Duration::new(60 * 60 * 24, 0);
        (self.last_repetition_time + DAY * self.group)
            .duration_since(SystemTime::now())
            .unwrap_or_default()
    }
}
