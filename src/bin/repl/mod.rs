use lib_lisp::Runtime;
use reedline::{Reedline, Signal};

mod prompt;

use prompt::ReplPrompt;

pub struct Repl {
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
                Signal::Success(buffer) => {
                    if &buffer == "exit" {
                        break;
                    };

                    self.handle_buffer(&buffer)
                }
                Signal::CtrlD | Signal::CtrlC => break,
            }
        }

        println!("exiting...");
        Ok(())
    }

    fn handle_buffer(&self, buffer: &str) {
        match self.evaluate_buffer(buffer) {
            Ok(result) => println!("{result}"),
            Err(err) => println!("{err}"),
        }
    }

    fn evaluate_buffer(&self, buffer: &str) -> lib_lisp::Result<String> {
        let expression = self.runtime.parse(buffer)?;
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
