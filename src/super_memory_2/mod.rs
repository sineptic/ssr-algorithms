use level::{Level, Quality};
use serde::{Deserialize, Serialize};
use ssr_core::task::{level::TaskLevel, Feedback, InterationItem, Task, UserInteraction};
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

    fn complete(mut self, _: &mut (), interaction: &mut impl UserInteraction) -> (Self, Feedback) {
        let user_answer = {
            let mut user_answer = interaction.interact(vec![
                InterationItem::Text(self.description.clone()),
                InterationItem::BlankField,
            ]);
            assert!(user_answer.len() == 1);
            user_answer.swap_remove(0)
        };
        match self.correct_answers.contains(&user_answer) {
            false => {
                let items = [
                    Quality::CompleteBlackout,
                    Quality::IncorrectResponseButCorrectRemembered,
                    Quality::IncorrectResponseAndSeemedEasyToRecall,
                ];
                let quality = items[interaction.interact(vec![
                    InterationItem::Text("choose difficulty".to_string()),
                    InterationItem::OneOf(vec![
                        (0, "complete blackout".to_string()),
                        (1, "incorrect response, but correct remembered".to_string()),
                        (
                            2,
                            "incorrect response, but seemed easy to recall".to_string(),
                        ),
                    ]),
                ])[0]
                    .parse::<usize>()
                    .unwrap()];
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
                let quality = items[interaction.interact(vec![
                    InterationItem::Text("choose difficulty".to_string()),
                    InterationItem::OneOf(vec![
                        (0, "recalled with serious difficulty".to_string()),
                        (1, "correct, but after hesitation".to_string()),
                        (2, "perfect response".to_string()),
                    ]),
                ])[0]
                    .parse::<usize>()
                    .unwrap()];
                self.level.update(&mut (), (SystemTime::now(), quality));
                (self, Feedback::CorrectAnswer)
            }
        }
    }
}
