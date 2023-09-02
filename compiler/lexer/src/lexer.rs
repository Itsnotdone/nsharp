use crate::{Cursor, LiteralKind, Token, TokenKind, KeywordKind, SeparatorKind};

pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            cursor: Cursor::new(input),
        }
    }

    pub fn next(&mut self) -> Token {
        let mut word = String::new();
        while let Some(ch) = self.cursor.bump() {
            match ch {
                ch if ch.is_whitespace() => {
                    if let Some(token) = self.check(&mut word){
                        return token
                    }
                    continue;
                }

                ch if ch.is_numeric() => {
                    word.clear();
                    word.push(ch);
                    return self.tokenize_number(&mut word);
                }

                '"' => {
                    word.clear();
                    return self.tokenize_string(&mut word);
                }

                ch if ch.is_alphabetic() => {
                    word.push(ch);

                    if !self.cursor.first().is_alphabetic() && !self.cursor.first().is_numeric(){
                        if let Some(token) = self.check(&mut word){
                            return token;
                        }
                    }
                }

                _ => {
                    if let Some(token) = self.check_separators(ch){
                        return token;
                    }

                    if let Some(token) = self.check_operators(ch){
                        return token;
                    }
                }
            };
        }
        Token::new(TokenKind::EOF)
    }

    fn check(&mut self, word: &mut String) -> Option<Token>{
        if let Some(token) = self.check_keywords(word){
            return Some(token);
        }

        if word.len() > 0{
            return Some(Token::new(
                TokenKind::Identifier(word.clone())
            ));
        }
        None
    }

    fn check_operators(&mut self, ch: char) -> Option<Token>{
        None
    }

    fn check_separators(&mut self, ch: char) -> Option<Token>{
        let separator_kind = match ch{
            '(' => SeparatorKind::OpenParent,
            ')' => SeparatorKind::CloseParent,
            '{' => SeparatorKind::OpenBracet,
            '}' => SeparatorKind::CloseBracket,
            ';' => SeparatorKind::Semicolon,
            _ => return None
        };

        return Some(Token::new(TokenKind::Separator(separator_kind)))
    }

    fn check_keywords(&mut self, word: &mut String) -> Option<Token>{
        let keyword_kind = match word.as_str(){
            "public" => KeywordKind::Public,
            "private" => KeywordKind::Private,
            "void" => KeywordKind::Void,

            _ => return None
        };

        Some(Token::new(TokenKind::Keyword(keyword_kind)))
    }


    fn tokenize_number(&mut self, word: &mut String) -> Token {
        while let Some(ch) = self.cursor.bump() && ch.is_numeric(){
            word.push(ch);

            if !self.cursor.first().is_numeric(){
                return Token::new(
                    TokenKind::Literal(
                            LiteralKind::Int(
                                if let Ok(number) = word.parse::<i32>(){
                                    number
                                } else {
                                    panic!("Expected integer");
                                }
                            )
                        )
                    );
            } 
        }

        panic!("Cannot create literal token");
    }

    fn tokenize_string(&mut self, word: &mut String) -> Token {

        while let Some(ch) = self.cursor.bump() && ch != '"'{
            word.push(ch);
        }

        if !word.is_empty(){
            return Token::new(
                TokenKind::Literal(
                        LiteralKind::String(word.clone())
                    )
                )
        }
        panic!("Cannot create literal token");
    }
}
