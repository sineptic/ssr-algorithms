use serde::{Deserialize, Serialize};
use ssr_core::task::level::TaskLevel;
use std::{
    cmp::max_by,
    fmt::Display,
    time::{Duration, SystemTime},
};

#[derive(Serialize, Deserialize)]
pub struct SuperMemoryLevel {
    e_factor: f64,
    repetition_number: u32,
    interval: Duration,
    last_repetition: SystemTime,
    repetition_required: bool,
}
impl Default for SuperMemoryLevel {
    fn default() -> Self {
        Self {
            e_factor: 2.5,
            repetition_number: 1,
            interval: Default::default(),
            last_repetition: SystemTime::now(),
            repetition_required: false,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Quality {
    CompleteBlackout = 0,
    IncorrectResponseButCorrectRemembered = 1,
    IncorrectResponseAndSeemedEasyToRecall = 2,
    CorrectResponseRecalledWithSeriousDifficulty = 3,
    CorrectResponseAfterHesitation = 4,
    PerfectResponse = 5,
}
// TODO:
impl Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            match self {
                Quality::CompleteBlackout => "complete blackout",
                Quality::IncorrectResponseButCorrectRemembered => "correct remembered",
                Quality::IncorrectResponseAndSeemedEasyToRecall => "seem easy to recall",
                Quality::CorrectResponseRecalledWithSeriousDifficulty => "serious difficulty",
                Quality::CorrectResponseAfterHesitation => "after hesitation",
                Quality::PerfectResponse => "perfect",
            }
        )
    }
}

impl SuperMemoryLevel {
    fn on_repetition(&mut self, (now, quality): (SystemTime, Quality)) {
        let q: f64 = (quality as u8).into();
        self.e_factor = max_by(
            self.e_factor + 0.1 - (5. - q) * (0.08 + (5. - q) * 0.02),
            1.3,
            |a, b| a.partial_cmp(b).unwrap(),
        );
        self.repetition_number += 1;
        if (quality as u8) < 3 {
            self.repetition_number = 1;
            self.repetition_required = true;
        } else {
            self.repetition_required = false;
        }
        self.last_repetition = now;
    }
}

impl TaskLevel for SuperMemoryLevel {
    type Context = (SystemTime, Quality);
    fn success(&mut self, ctx: Self::Context) {
        self.on_repetition(ctx);
    }

    fn failure(&mut self, ctx: Self::Context) {
        self.on_repetition(ctx);
    }

    fn until_next_repetition(&self) -> Duration {
        if self.repetition_required {
            Duration::default()
        } else {
            (self.last_repetition + self.interval)
                .elapsed()
                .unwrap_or_default()
        }
    }
}
