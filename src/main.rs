use interpreter::lexer::Lexer;
use interpreter::token::TokenType;
fn main() {
    let input = "let five = 5 + 10;";
    let mut lexer = Lexer::new(input);

    loop {
        let tok = lexer.next_token();
        println!("{:?}", tok);
        if tok.token_type == TokenType::EOF {
            break;
        }
    }
}
