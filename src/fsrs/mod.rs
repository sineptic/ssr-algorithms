use s_text_input_f::ParagraphItem;
use serde::{Deserialize, Serialize};
use ssr_core::task::level::TaskLevel;
use std::time::SystemTime;

mod level;
use level::{Level, Quality, RepetitionContext};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    level: Option<Level>,
    input_blocks: s_text_input_f::Blocks,
    correct_answer: s_text_input_f::Response,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Shared {
    weights: [f32; 19],
}
impl Default for Shared {
    fn default() -> Self {
        Self {
            weights: fsrs::DEFAULT_PARAMETERS,
        }
    }
}
impl ssr_core::task::SharedState<'_> for Shared {}

impl Task {
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
    fn gen_feedback_form(
        &mut self,
        user_answer: Vec<Vec<String>>,
        directive: String,
        qualities_strings: Vec<String>,
    ) -> Vec<s_text_input_f::Block> {
        let mut feedback = s_text_input_f::to_answered(
            self.input_blocks.clone(),
            user_answer,
            self.correct_answer.clone(),
        );
        feedback.push(s_text_input_f::Block::Paragraph(vec![]));
        feedback.push(s_text_input_f::Block::Paragraph(vec![ParagraphItem::Text(
            directive,
        )]));
        feedback.push(s_text_input_f::Block::OneOf(qualities_strings));
        feedback
    }

    fn get_feedback(
        &mut self,
        user_answer: Vec<Vec<String>>,
        directive: String,
        qualities_strings: Vec<String>,
        interaction: &mut impl FnMut(
            Vec<s_text_input_f::Block>,
        ) -> Result<Vec<Vec<String>>, std::io::Error>,
        qualities: Vec<Quality>,
    ) -> Result<Quality, std::io::Error> {
        let feedback = self.gen_feedback_form(user_answer, directive, qualities_strings);
        let user_feedback = interaction(feedback)?;
        let i = s_text_input_f::response_as_one_of(user_feedback.last().unwrap().to_owned())
            .unwrap()
            .unwrap();
        let quality = qualities[i];
        Ok(quality)
    }

    fn complete_inner(
        &mut self,
        user_answer: Vec<Vec<String>>,
        shared_state: &Shared,
        retrievability_goal: f64,
        interaction: &mut impl FnMut(s_text_input_f::Blocks) -> std::io::Result<Vec<Vec<String>>>,
    ) -> std::io::Result<Quality> {
        let next_states = self.next_states(shared_state, retrievability_goal);
        Ok(
            match s_text_input_f::eq_response(&user_answer, &self.correct_answer, true, false) {
                false => self.feedback_wrong(user_answer, next_states, interaction)?,
                true => self.feedback_correct(user_answer, next_states, interaction)?,
            },
        )
    }
    fn next_states(&self, shared: &Shared, retrievability_goal: f64) -> fsrs::NextStates {
        let fsrs = level::fsrs(shared);
        let now = chrono::Local::now();
        fsrs.next_states(
            self.level.as_ref().map(|l| l.memory_state(&fsrs)),
            retrievability_goal as f32,
            level::sleeps_between(self.level.as_ref().map_or(now, |l| l.last_review), now)
                .try_into()
                .unwrap(),
        )
        .unwrap()
    }

    fn feedback_correct(
        &mut self,
        user_answer: Vec<Vec<String>>,
        next_states: fsrs::NextStates,
        interaction: &mut impl FnMut(s_text_input_f::Blocks) -> std::io::Result<Vec<Vec<String>>>,
    ) -> std::io::Result<Quality> {
        let qualities = vec![Quality::Hard, Quality::Good, Quality::Easy];
        let qualities_strings = vec![
            format!("Hard {}d", next_states.hard.interval),
            format!("Good {}d", next_states.good.interval),
            format!("Easy {}d", next_states.easy.interval),
        ];
        let directive = "All answers correct! Choose difficulty:".to_string();
        self.get_feedback(
            user_answer,
            directive,
            qualities_strings,
            interaction,
            qualities,
        )
    }

    fn feedback_wrong(
        &mut self,
        user_answer: Vec<Vec<String>>,
        next_states: fsrs::NextStates,
        interaction: &mut impl FnMut(s_text_input_f::Blocks) -> std::io::Result<Vec<Vec<String>>>,
    ) -> std::io::Result<Quality> {
        self.get_feedback(
            user_answer,
            "Your answer is wrong.".into(),
            vec![format!("OK {}h", next_states.again.interval * 24.)],
            interaction,
            vec![Quality::Again],
        )
    }
}
impl ssr_core::task::Task<'_> for Task {
    type SharedState = Shared;

    fn next_repetition(
        &self,
        shared_state: &Self::SharedState,
        retrievability_goal: f64,
    ) -> SystemTime {
        if let Some(ref level) = self.level {
            level.next_repetition(shared_state, retrievability_goal)
        } else {
            SystemTime::UNIX_EPOCH
        }
    }

    fn complete(
        &mut self,
        shared_state: &mut Self::SharedState,
        desired_retention: f64,
        interaction: &mut impl FnMut(
            s_text_input_f::Blocks,
        ) -> std::io::Result<s_text_input_f::Response>,
    ) -> std::io::Result<()> {
        let review_time = chrono::Local::now();
        let user_answer = interaction(self.input_blocks.clone())?;
        let quality =
            self.complete_inner(user_answer, shared_state, desired_retention, interaction)?;
        if let Some(ref mut level) = self.level {
            level.update(
                shared_state,
                RepetitionContext {
                    quality,
                    review_time,
                },
            );
        } else {
            self.level = Some(Level::new(quality, review_time));
        }
        Ok(())
    }
}
