use lexer::*;

fn main() {
    let mut lexer = Lexer::new(include_str!("test.ns"));

    let mut token = None;

    while token != Some(Token::new(TokenKind::EOF)){
        token = Some(lexer.next());
        println!("{:?}", token);
    }
}
