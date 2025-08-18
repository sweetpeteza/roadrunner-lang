use roadrunner::ast::program::Program;
use roadrunner::ast::traits::Node;
use roadrunner::lexer::Lexer;
use roadrunner::parser::Parser;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() -> Result<(), anyhow::Error> {
    println!("Hello! This is the Roadrunner programming language!");
    println!("Feel free to type in commands");

    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("⚡: ");
        match readline {
            Ok(line) => {
                // Tokenize the input here, for example:
                let mut lexer = Lexer::new(&line);
                let mut parser = Parser::new(&mut lexer);
                let program = parser.parse_program();

                if parser.errors.len() > 0 {
                    for err in parser.errors.iter() {
                        println!("\t{}", err.message);
                    }
                }

                println!("{:?}", program.string());
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
