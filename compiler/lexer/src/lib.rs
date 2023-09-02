#![feature(let_chains)]

mod cursor;
mod lexer;
mod token;

pub use cursor::*;
pub use lexer::*;
pub use token::*;

#[cfg(test)]
mod LexerTests {
    use super::*;

    #[test]
    fn integer_token() {
        let mut lexer = Lexer::new("10");

        assert_eq!(
            lexer.next(),
            Token::new(TokenKind::Literal(LiteralKind::Int(10)))
        )
    }
    #[test]
    fn string_token() {
        let mut lexer = Lexer::new("\"some string\"");

        assert_eq!(
            lexer.next(),
            Token::new(TokenKind::Literal(LiteralKind::String(String::from("some string"))))
        )
    }

    #[test]
    fn multiple_tokens(){
        let mut lexer = Lexer::new("4095 \"another string\"");

        assert_eq!(
            lexer.next(),
            Token::new(TokenKind::Literal(LiteralKind::Int(4095)))
        );

        assert_eq!(
            lexer.next(),
            Token::new(TokenKind::Literal(LiteralKind::String(String::from("another string"))))
        );
    }

    
}
