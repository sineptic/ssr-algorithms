use level::{Level, Quality};
use serde::{Deserialize, Serialize};
use ssr_core::task::{level::TaskLevel, Feedback, Task};
use std::{collections::HashSet, time::SystemTime};

mod level;

#[derive(Serialize, Deserialize)]
pub struct WriteAnswer {
    level: Level,
    description: String,
    correct_answers: HashSet<String>,
    explanation: Option<String>,
}

impl WriteAnswer {
    pub fn new(
        description: String,
        correct_answers: impl IntoIterator<Item = String>,
        explanation: Option<String>,
    ) -> Self {
        Self {
            level: Default::default(),
            description,
            correct_answers: correct_answers.into_iter().collect(),
            explanation,
        }
    }
}

impl<'a> Task<'a> for WriteAnswer {
    type SharedState = ();
    fn get_desctiption(&self) -> &str {
        &self.description
    }

    fn next_repetition(&self, retrievability_goal: f64) -> SystemTime {
        self.level.next_repetition(retrievability_goal)
    }

    fn complete(
        mut self,
        _: &mut (),
        mut interaction: impl ssr_core::task::UserInteraction,
    ) -> (Self, Feedback) {
        let user_answer = interaction.get_string(None::<String>, &self.description);
        match self.correct_answers.contains(&user_answer) {
            false => {
                let items = [
                    Quality::CompleteBlackout,
                    Quality::IncorrectResponseButCorrectRemembered,
                    Quality::IncorrectResponseAndSeemedEasyToRecall,
                ];
                let quality = items[interaction.select_item(Some("choose difficulty"), &items)];
                self.level.update(&mut (), (SystemTime::now(), quality));
                let correct_answers = self.correct_answers.clone().into_iter().collect();
                let explanation = self.explanation.clone();
                (
                    self,
                    Feedback::WrongAnswer {
                        correct_answers,
                        explanation,
                    },
                )
            }
            true => {
                let items = [
                    Quality::CorrectResponseRecalledWithSeriousDifficulty,
                    Quality::CorrectResponseAfterHesitation,
                    Quality::PerfectResponse,
                ];
                let quality = items[interaction.select_item(Some("choose difficulty"), &items)];
                self.level.update(&mut (), (SystemTime::now(), quality));
                (self, Feedback::CorrectAnswer)
            }
        }
    }
}
