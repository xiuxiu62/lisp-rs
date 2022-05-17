use tracing::Level;

mod repl;

use repl::Repl;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    Repl::default().run()
}
