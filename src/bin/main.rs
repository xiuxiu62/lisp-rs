use lib_lisp::{list, num, sym, Expression, Result, Runtime};
use tracing::{debug, info, warn, Level};

fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    run()
}

fn run() -> Result<()> {
    info!("initializing runtime");
    let runtime = Runtime::default();

    let data = "(+ 2 2)";
    let expression = runtime.parse(data)?;
    let result = runtime.evaluate(&expression.0)?;

    println!("{result}");

    // println!("{:?}", list!(num!(2), num!(2), sym!("+")));

    Ok(())
}
