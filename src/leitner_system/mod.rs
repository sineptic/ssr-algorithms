use level::Level;
use serde::{Deserialize, Serialize};
use ssr_core::task::{level::TaskLevel, Feedback, Task, UserInteraction};
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

    fn next_repetition(&self, _: f64) -> SystemTime {
        self.level.next_repetition(0.)
    }

    fn complete(mut self, _: &mut (), mut interaction: impl UserInteraction) -> (Self, Feedback) {
        let user_answer = interaction.get_string(None::<String>, &self.description);
        match self.correct_answers.contains(&user_answer) {
            false => {
                self.level.update(&mut (), (SystemTime::now(), false));
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
                self.level.update(&mut (), (SystemTime::now(), true));
                (self, Feedback::CorrectAnswer)
            }
        }
    }
}
