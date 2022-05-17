use reedline::{Prompt, PromptEditMode, PromptHistorySearch};
use std::borrow::Cow;

#[derive(Default)]
pub struct ReplPrompt;

impl Prompt for ReplPrompt {
    fn render_prompt_left(&self) -> Cow<str> {
        "〉".into()
    }

    fn render_prompt_right(&self) -> std::borrow::Cow<str> {
        "".into()
    }

    fn render_prompt_indicator(&self, _edit_mode: PromptEditMode) -> Cow<str> {
        "".into()
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        "".into()
    }

    fn render_prompt_history_search_indicator(
        &self,
        _history_search: PromptHistorySearch,
    ) -> Cow<str> {
        "".into()
    }
}
