pub mod leitner_system;
pub mod super_memory_2;

// mod free_spaced_repetition_sheduled_4 {
//     use std::collections::HashSet;
//
//     use level::Level;
//     use serde::{Deserialize, Serialize};
//
//     type Weights = [f64; 17];
//     const _DEFAULT_PARAMETRS: Weights = [
//         0.4, 0.6, 2.4, 5.8, 4.93, 0.94, 0.86, 0.01, 1.49, 0.14, 0.94, 2.18, 0.05, 0.34, 1.26, 0.29,
//         2.61,
//     ];
//
//     mod level {
//         use serde::{Deserialize, Serialize};
//         use ssr_core::task::level::TaskLevel;
//         use std::time::{Duration, SystemTime};
//
//         use super::Weights;
//
//         const _DECAY: f64 = -0.5;
//         const _FACTOR: f64 = 19. / 81.;
//
//         #[derive(Default, Serialize, Deserialize)]
//         pub struct Level {
//             last_repetition: Option<SystemTime>,
//             stability: f64,
//             difficulty: f64,
//         }
//
//         #[derive(Clone, Copy)]
//         #[repr(u8)]
//         pub enum Grade {
//             Again = 1,
//             Hard = 2,
//             Good = 3,
//             Easy = 4,
//         }
//         impl TaskLevel for Level {
//             type Context = (Weights, SystemTime, Grade);
//
//             fn update(&mut self, (weights, now, grade): Self::Context) {
//                 const _DAY: Duration = Duration::new(60 * 60 * 24, 0);
//                 if self.last_repetition.is_none() {
//                     self.stability = weights[grade as usize - 1];
//                     self.difficulty = weights[4] - (grade as i64 - 3) as f64 * weights[5];
//                 } else {
//                     self.difficulty = weights[7] * weights[4]
//                         + (1. - weights[7])
//                             * (self.difficulty - weights[6] * (grade as i64 - 3) as f64);
//                     self.stability = match grade {
//                         Grade::Again => {
//                             weights[11]
//                                 * self.difficulty.powf(-weights[12])
//                                 * ((self.stability + 1.).powf(weights[13]))
//                                 * f64::exp(weights[14] * (1 - todo!()))
//                         }
//                         Grade::Hard => todo!(),
//                         Grade::Good => todo!(),
//                         Grade::Easy => todo!(),
//                     }
//                 }
//                 self.last_repetition = Some(now);
//             }
//
//             fn next_repetition(&self, retrievability_goal: f64) -> SystemTime {
//                 if let Some(last_repetition) = self.last_repetition {
//                     const DAY: Duration = Duration::new(60 * 60 * 24, 0);
//                     last_repetition
//                         + DAY.mul_f64(9. * self.stability * (1. / retrievability_goal - 1.))
//                 } else {
//                     SystemTime::now()
//                 }
//             }
//         }
//         impl Level {
//             pub fn retrievability(&self, duration: Duration) -> f64 {
//                 assert!(self.last_repetition.is_some());
//                 const DAY: Duration = Duration::new(60 * 60 * 24, 0);
//                 let t = duration.as_secs_f64() / DAY.as_secs_f64();
//                 (1. - t / (9. * self.stability)).powi(-1)
//             }
//         }
//     }
//
//     #[derive(Serialize, Deserialize)]
//     pub struct WriteAnswer {
//         level: Level,
//         description: String,
//         correct_answers: HashSet<String>,
//         explanation: Option<String>,
//     }
//
//     impl WriteAnswer {
//         pub fn new(
//             description: String,
//             correct_answers: impl IntoIterator<Item = String>,
//             explanation: Option<String>,
//         ) -> Self {
//             Self {
//                 level: Default::default(),
//                 description,
//                 correct_answers: correct_answers.into_iter().collect(),
//                 explanation,
//             }
//         }
//     }
// }
