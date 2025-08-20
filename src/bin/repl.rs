use roadrunner::lexer::Lexer;
use roadrunner::parser::Parser;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

fn main() -> Result<(), anyhow::Error> {
    println!("Hello! This is the Roadrunner programming language!");
    tracing::debug!("Debug Test: Application started");

    println!("Feel free to type in commands");

    let mut rl = DefaultEditor::new()?;

    if !tracing::dispatcher::has_been_set() {
        let file_appender = tracing_appender::rolling::daily("./logs", "trace.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        std::mem::forget(guard); // Prevent early drop of the logging guard
        let _ = std::fs::write("./logs/debug_test.log", "Testing log output: Successful\n");
        tracing_subscriber::fmt()
            .with_writer(non_blocking)
            .with_ansi(false) // Disable ANSI escape codes for clean logs
            .with_max_level(tracing::Level::DEBUG)
            .try_init()
            .expect("Tracing initialization failed");
        tracing::info!("Tracing information initialized");
    }

    loop {
        tracing::debug!("Awaiting user input...");
        let readline = rl.readline("⚡: ");
        match readline {
            Ok(line) => {
                // Tokenize the input here, for example:
                let mut lexer = Lexer::new(&line);
                let mut parser = Parser::new(&mut lexer);
                tracing::debug!("Parsing program with input: {:?}", line);
                let program = parser.parse_program();

                if !parser.errors.is_empty() {
                    tracing::error!("Parser errors encountered: {:?}", parser.errors);
                    for err in parser.errors.iter() {
                        println!("\t{}", err.message);
                    }
                }

                println!("{}", program.string());
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C pressed. Exiting.");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D pressed. Exiting.");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
