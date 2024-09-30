use level::{Level, Quality};
use s_text_input_f::ParagraphItem;
use serde::{Deserialize, Serialize};
use ssr_core::task::{level::TaskLevel, Task};
use std::time::SystemTime;

mod level;

#[derive(Serialize, Deserialize)]
pub struct WriteAnswer {
    level: Level,
    input_blocks: s_text_input_f::Blocks,
    correct_answer: s_text_input_f::Response,
}

impl WriteAnswer {
    pub fn new(
        input_blocks: s_text_input_f::Blocks,
        correct_answer: s_text_input_f::Response,
    ) -> Self {
        Self {
            level: Default::default(),
            input_blocks,
            correct_answer,
        }
    }
}

impl<'a> Task<'a> for WriteAnswer {
    type SharedState = ();

    fn next_repetition(&self, retrievability_goal: f64) -> SystemTime {
        self.level.next_repetition(retrievability_goal)
    }

    fn complete(
        &mut self,
        _: &mut (),
        interaction: &mut impl FnMut(
            s_text_input_f::Blocks,
        ) -> std::io::Result<s_text_input_f::Response>,
    ) -> std::io::Result<()> {
        let user_answer = interaction(self.input_blocks.clone())?;
        match s_text_input_f::eq_response(&user_answer, &self.correct_answer, true, false) {
            false => {
                let qualities = [
                    Quality::CompleteBlackout,
                    Quality::IncorrectResponseButCorrectRemembered,
                    Quality::IncorrectResponseAndSeemedEasyToRecall,
                ];
                let qualities_strings = vec![
                    "complete blackout".to_string(),
                    "incorrect response, but correct remembered".to_string(),
                    "incorrect response, but seemed easy to recall".to_string(),
                ];

                let mut feedback = s_text_input_f::to_asnwered(
                    self.input_blocks.clone(),
                    user_answer,
                    self.correct_answer.clone(),
                );
                feedback.push(s_text_input_f::Block::Paragraph(vec![]));
                feedback.push(s_text_input_f::Block::Paragraph(vec![ParagraphItem::Text(
                    "Choose difficulty:".to_string(),
                )]));
                feedback.push(s_text_input_f::Block::OneOf(qualities_strings));

                let user_feedback = interaction(feedback)?;
                let i =
                    s_text_input_f::response_as_one_of(user_feedback.last().unwrap().to_owned())
                        .unwrap()
                        .unwrap();
                let quality = qualities[i];

                self.level.update(&mut (), (SystemTime::now(), quality));
            }
            true => {
                let qualities = [
                    Quality::CorrectResponseRecalledWithSeriousDifficulty,
                    Quality::CorrectResponseAfterHesitation,
                    Quality::PerfectResponse,
                ];
                let qualities_string = vec![
                    "recalled with serious difficulty".to_string(),
                    "correct, but after hesitation".to_string(),
                    "perfect response".to_string(),
                ];
                let feedback = vec![
                    s_text_input_f::Block::Paragraph(vec![s_text_input_f::ParagraphItem::Text(
                        "All answers correct!".to_string(),
                    )]),
                    s_text_input_f::Block::OneOf(qualities_string),
                ];
                let user_feedback = interaction(feedback)?;
                let i =
                    s_text_input_f::response_as_one_of(user_feedback.last().unwrap().to_owned())
                        .unwrap()
                        .unwrap();
                let quality = qualities[i];

                self.level.update(&mut (), (SystemTime::now(), quality));
            }
        }
        Ok(())
    }
}
