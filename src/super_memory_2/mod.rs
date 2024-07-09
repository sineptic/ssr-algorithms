use level::{Quality, SuperMemoryLevel};
use serde::{Deserialize, Serialize};
use ssr_core::task::{level::TaskLevel, Feedback, Task};
use std::{
    collections::HashSet,
    time::{Duration, SystemTime},
};

mod level;

#[derive(Serialize, Deserialize)]
pub struct SuperMemory {
    level: SuperMemoryLevel,
    description: String,
    correct_answers: HashSet<String>,
    explanation: Option<String>,
}

impl<'a> Task<'a> for SuperMemory {
    fn new(
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

    fn get_desctiption(&self) -> &str {
        &self.description
    }

    fn until_next_repetition(&self) -> Duration {
        self.level.until_next_repetition()
    }

    fn complete(
        &mut self,
        mut interaction: impl ssr_core::task::UserInteraction,
    ) -> ssr_core::task::Feedback<impl Iterator<Item = &String>> {
        let user_answer = interaction.get_string(None::<String>, &self.description);
        match self.correct_answers.contains(&user_answer) {
            false => {
                let items = [
                    Quality::CompleteBlackout,
                    Quality::IncorrectResponseButCorrectRemembered,
                    Quality::IncorrectResponseAndSeemedEasyToRecall,
                ];
                let quality = items[interaction.select_item(Some("choose difficulty"), &items)];
                self.level.failure((SystemTime::now(), quality));
                Feedback::WrongAnswer {
                    correct_answers: self.correct_answers.iter(),
                    explanation: &self.explanation,
                }
            }
            true => {
                let items = [
                    Quality::CorrectResponseRecalledWithSeriousDifficulty,
                    Quality::CorrectResponseAfterHesitation,
                    Quality::PerfectResponse,
                ];
                let quality = items[interaction.select_item(Some("choose difficulty"), &items)];
                self.level.success((SystemTime::now(), quality));
                Feedback::CorrectAnswer
            }
        }
    }
}
