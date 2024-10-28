use level::Level;
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

impl Task<'_> for WriteAnswer {
    type SharedState = ();

    fn next_repetition(&self, shared: &(), _: f64) -> SystemTime {
        self.level.next_repetition(shared, 0.)
    }

    fn complete(
        &mut self,
        _: &mut (),
        _desired_retention: f64,
        interaction: &mut impl FnMut(
            s_text_input_f::Blocks,
        ) -> std::io::Result<s_text_input_f::Response>,
    ) -> std::io::Result<()> {
        let user_answer = interaction(self.input_blocks.clone())?;
        match s_text_input_f::eq_response(&user_answer, &self.correct_answer, true, false) {
            false => {
                let mut feedback = s_text_input_f::to_answered(
                    self.input_blocks.clone(),
                    user_answer,
                    self.correct_answer.clone(),
                );
                feedback.push(s_text_input_f::Block::Paragraph(vec![]));
                feedback.push(s_text_input_f::Block::OneOf(vec!["OK".to_string()]));
                interaction(feedback)?;
                self.level.update(&mut (), (SystemTime::now(), false));
            }
            true => {
                let feedback = vec![
                    s_text_input_f::Block::Paragraph(vec![s_text_input_f::ParagraphItem::Text(
                        "All answers correct!".to_string(),
                    )]),
                    s_text_input_f::Block::OneOf(vec!["OK".to_string()]),
                ];
                interaction(feedback)?;
                self.level.update(&mut (), (SystemTime::now(), true));
            }
        }
        Ok(())
    }
}
