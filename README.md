# roz

An interpreter written in Rust.

## Current Progress

Basic arithmetic expressions containing add, sub, mul and div can be evaluated.

```Rust
let input_expr = "(5 - 2) * 5 / 3".to_string();

let mut lexer = Lexer::new(&input_expr);
lexer.scan_tokens();

let mut parser = Parser::new(lexer.tokens);

let expr = parser.expression();
let mut interpreter = Interpreter;
```
