use rstest::rstest;

use crate::ast::traits::Expression;


#[rstest]
fn test_identifier_expression() {
    use crate::ast::identifier::Identifier;
    use crate::ast::program::Program;
    use crate::lexer::lexer::Lexer;
    use crate::parser::parser::Parser;

    let input = "foobar;";

    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);
    let program: Program<Identifier> = parser.parse_program();

    assert_eq!(program.statements.len(), 1);

    //
    // let statemants = program.unwrap().statements.into_iter();;
    //
    // let first_statement = statemants.next().unwrap();
    //
    // assert_eq!(
    //     first_statement.token_literal(),
    //     "foobar"
    // );
}
