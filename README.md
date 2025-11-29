# Roadrunner

A tree-walking interpreter for the Monkey programming language, implemented in Rust.

## About

This project is a Rust implementation based on Thorsten Ball's excellent book [**Writing An Interpreter In Go**](https://interpreterbook.com/). While the original book uses Go, this implementation translates those concepts into idiomatic Rust, taking advantage of Rust's strong type system, pattern matching, and functional programming features.

## Features

- **Lexer**: Tokenizes Monkey source code into a stream of tokens
- **Parser**: Recursive descent parser (Pratt parsing) that builds an Abstract Syntax Tree (AST)
- **Evaluator**: Tree-walking interpreter that executes the AST
- **REPL**: Interactive Read-Eval-Print Loop for experimenting with the language
- **Environment**: Lexical scoping with support for closures
- **Functional Programming**: Implemented using functional patterns with free functions and iterators

## The Monkey Language

Monkey is a simple programming language with:

- **Data Types**: Integers, Booleans, Null
- **Arithmetic**: `+`, `-`, `*`, `/`
- **Comparison**: `==`, `!=`, `<`, `>`
- **Logical**: `!` (bang operator)
- **Variables**: `let x = 5;`
- **Functions**: First-class functions with closures
  ```monkey
  let add = fn(x, y) { x + y; };
  add(5, 3); // => 8
  ```
- **Conditionals**: `if`/`else` expressions
  ```monkey
  if (x > 10) { return true; } else { return false; }
  ```
- **Return Statements**: Early returns from functions
- **Closures**: Functions capture their environment
  ```monkey
  let newAdder = fn(x) {
    fn(y) { x + y };
  };
  let addTwo = newAdder(2);
  addTwo(3); // => 5
  ```

## Project Structure

```
src/
├── bin/
│   └── repl.rs       # Interactive REPL
├── lexer.rs          # Tokenization
├── token.rs          # Token types
├── parser.rs         # Parsing and AST construction
├── ast.rs            # Abstract Syntax Tree definitions
├── evaluator.rs      # Tree-walking interpreter
├── object.rs         # Runtime object types
├── environment.rs    # Variable scoping and storage
└── lib.rs            # Library entry point
```

## Building and Running

### Prerequisites

- Rust 1.70+ (with Cargo)

### Build

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

All 142 tests should pass, including deep recursion tests (100 levels).

### Run the REPL

```bash
cargo run --bin repl
```

Example REPL session:

```
>> let x = 5;
5
>> let y = 10;
10
>> let add = fn(a, b) { a + b; };
fn(a, b) { (a + b) }
>> add(x, y);
15
>> let factorial = fn(n) { if (n == 0) { 1 } else { n * factorial(n - 1); } };
fn(n) { if (n == 0) { 1 } else { (n * factorial((n - 1))) } }
>> factorial(5);
120
```

### Lint and Format

```bash
cargo clippy    # Lint
cargo fmt       # Format
```

## Implementation Highlights

### Functional Programming Style

This implementation emphasizes functional programming patterns:

- **Free Functions**: The evaluator uses pure functions instead of methods on an empty struct
- **Iterator Trait**: The `Lexer` implements Rust's `Iterator` trait for functional composition
- **Immutability**: Prefers immutable data structures where possible
- **Functional Combinators**: Uses `.map()`, `.try_fold()`, and `.unwrap_or()` instead of imperative loops

### Environment Design

The environment uses `Rc<RefCell<Environment>>` to enable:
- **Shared ownership** of parent scopes (multiple closures can share an environment)
- **Interior mutability** for variable binding
- **Proper closure semantics** where functions capture their defining environment

### Error Handling

Errors propagate through the `Object::Error` variant, allowing the interpreter to continue after errors (useful for the REPL).

## Testing

The project includes comprehensive tests covering:

- Lexer tokenization
- Parser correctness and operator precedence
- Expression evaluation (arithmetic, boolean, conditionals)
- Function calls and closures
- Error handling
- Deep recursion (verified up to 100 levels)

Run tests with coverage:

```bash
cargo test -- --nocapture
```

## Development

### Code Style

- Follow Rust naming conventions (snake_case for functions, PascalCase for types)
- Use `rustfmt` with settings in `rustfmt.toml`
- Maximum line width: 100 characters
- Document all public APIs

### Making Changes

1. Create a feature branch
2. Write tests first (TDD)
3. Implement the feature
4. Run `cargo test` and `cargo clippy`
5. Format with `cargo fmt`
6. Submit a pull request

See `AGENTS.md` for detailed development guidelines.

## Known Limitations

This interpreter is intentionally simple and educational. It does not include:

- Arrays or hash maps (future extension)
- String type (future extension)
- Built-in functions beyond basic operators
- Module system or imports
- Garbage collection optimization (relies on Rust's `Rc` reference counting)

## References

- [**Writing An Interpreter In Go**](https://interpreterbook.com/) by Thorsten Ball
- [**Writing A Compiler In Go**](https://compilerbook.com/) by Thorsten Ball (potential future work)

## License

This is an educational project for learning interpreter implementation in Rust.

## Acknowledgments

- **Thorsten Ball** for the excellent "Writing An Interpreter In Go" book
- The Rust community for comprehensive documentation and tooling
