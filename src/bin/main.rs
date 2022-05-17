use lib_lisp::Runtime;
use reedline::{Prompt, PromptEditMode, PromptHistorySearch, Reedline, Signal};
use std::borrow::Cow;
use tracing::{debug, Level};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    Repl::default().run()
}

struct Repl {
    line_editor: Reedline,
    prompt: ReplPrompt,
    runtime: Runtime,
}

impl Repl {
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("press C-c or C-d to exit...");

        loop {
            let sig = self.line_editor.read_line(&self.prompt)?;
            match sig {
                Signal::Success(buffer) => self.handle_buffer(&buffer),
                Signal::CtrlD | Signal::CtrlC => {
                    println!("exiting...");
                    break Ok(());
                }
            }
        }
    }

    fn handle_buffer(&self, buffer: &str) {
        match self.evaluate_buffer(buffer) {
            Ok(result) => println!("{result}"),
            Err(err) => println!("{err}"),
        }
    }

    fn evaluate_buffer(&self, buffer: &str) -> lib_lisp::Result<String> {
        let expression = self.runtime.parse(&buffer)?;
        let result = self.runtime.evaluate(&expression.0)?;

        Ok(format!("{result}"))
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self {
            line_editor: Reedline::create(),
            prompt: ReplPrompt::default(),
            runtime: Runtime::default(),
        }
    }
}

#[derive(Default)]
struct ReplPrompt;

impl Prompt for ReplPrompt {
    fn render_prompt_left(&self) -> std::borrow::Cow<str> {
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
