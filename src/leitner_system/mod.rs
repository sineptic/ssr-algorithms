use level::LeitnerSystemLevel;
use serde::{Deserialize, Serialize};
use ssr_core::task::{level::TaskLevel, Feedback, Task, UserInteraction};
use std::{collections::HashSet, time::SystemTime};

mod level;

#[derive(Serialize, Deserialize)]
pub struct LeitnerSystem {
    level: LeitnerSystemLevel,
    description: String,
    correct_answers: HashSet<String>,
    explanation: Option<String>,
}

impl LeitnerSystem {
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

impl<'a> Task<'a> for LeitnerSystem {
    fn get_desctiption(&self) -> &str {
        &self.description
    }

    fn until_next_repetition(&self) -> std::time::Duration {
        self.level.until_next_repetition()
    }

    fn complete(
        &mut self,
        mut interaction: impl UserInteraction,
    ) -> Feedback<impl Iterator<Item = &String>> {
        let user_answer = interaction.get_string(None::<String>, &self.description);
        match self.correct_answers.contains(&user_answer) {
            false => {
                self.level.failure(SystemTime::now());
                Feedback::WrongAnswer {
                    correct_answers: self.correct_answers.iter(),
                    explanation: &self.explanation,
                }
            }
            true => {
                self.level.success(SystemTime::now());
                Feedback::CorrectAnswer
            }
        }
    }
}
