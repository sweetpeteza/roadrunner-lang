use roadrunner::lexer::lexer::Lexer;
use roadrunner::token::token::Token;
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

                loop {
                    let token = lexer.next_token();
                    if token == Token::Eof {
                        break;
                    }
                    println!("{:?}", token);
                }
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
