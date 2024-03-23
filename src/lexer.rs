use std::io::{BufRead, BufReader, Read};
use crate::node::{Bop, Uop};

#[derive(Debug, PartialEq)]
pub enum Token {
    Int(i32),
    Float(f64),
    Char(char),
    String(String),
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Dot,
    Comma,
    Declare,
    Assign,
    Binop(Bop),
    Unop(Uop),
    AssignOp(Bop),
    Iden(String),
    True,
    False,
    Fn,
    Struct,
    Type,
    Return,
    Break,
    Continue,
    While,
    For,
    In,
    Sep
}

pub struct Lexer<T: BufRead> {
    reader: BufReader<T>
}

impl<T: BufRead> Lexer<T> {
    pub fn new(reader: BufReader<T>) -> Lexer<T> {
        Lexer { reader }
    }

    fn read(&mut self) -> Option<char> {
        let mut buffer = [0; 1];
        let count = self.reader.read(&mut buffer).unwrap();
        if count > 0 {
            Some(buffer[0] as char)
        } else {
            None
        }
    }

    fn peek(&mut self) -> Option<char> {
        let buffer = self.reader.fill_buf().unwrap();
        if !buffer.is_empty() {
            Some(buffer[0] as char)
        } else {
            None
        }
    }

    fn consume(&mut self) {
        self.reader.consume(1)
    }

    fn skip_spaces(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' {
                self.consume()
            } else {
                return
            }
        }
    }

    fn scan_char(&mut self) -> Token {
        let mut str = String::new();
        while let Some(c) = self.peek() {
            if c == '\'' {
                break
            }
            str.push(c);
            self.consume()
        }
        if str.len() != 1 {
            panic!("Invalid char: '{}' a char literal must be 1 character", str)
        } else {
            Token::Char(str.chars().nth(0).unwrap())
        }
    }

    fn scan_string(&mut self) -> Token {
        let mut str = String::new();
        while let Some(c) = self.peek() {
            if c == '\"' {
                break
            }
            str.push(c);
            self.consume()
        }
        Token::String(str)
    }

    fn scan_number(&mut self, c: char) -> Token {
        let mut is_int = true;
        let mut tok = String::from(c);
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
               break
            }
            if c == '.' {
                is_int = false;
            }
            tok.push(c);
            self.consume()
        }
        if is_int {
            Token::Int(tok.parse::<i32>().unwrap())
        } else {
            Token::Float(tok.parse::<f64>().unwrap())
        }
    }

    fn scan_keyword(&mut self, c: char) -> Token {
        let mut tok = String::from(c);
        while let Some(c) = self.peek() {
            if !c.is_alphanumeric() {
                break;
            }
            tok.push(c);
            self.consume()
        }
        match tok.as_str() {
            "fn" => Token::Fn,
            "struct" => Token::Struct,
            "type" => Token::Type,
            "return" => Token::Return,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "while" => Token::While,
            "for" => Token::For,
            "in" => Token::In,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Iden(tok),
        }
    }

    fn scan_operator(&mut self, c: char) -> Token  {
        let mut tok = String::from(c);
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                break
            }
            tok.push(c);
            self.consume()
        }
        match tok.as_str() {
            "*" => Token::Binop(Bop::Multiply),
            "**" => Token::Binop(Bop::Exp),
            "*=" => Token::AssignOp(Bop::Multiply),
            "**=" => Token::AssignOp(Bop::Exp),
            "-" => Token::Binop(Bop::Subtract),
            "-=" => Token::AssignOp(Bop::Subtract),
            "+" => Token::Binop(Bop::Add),
            "+=" => Token::AssignOp(Bop::Add),
            "/" => Token::Binop(Bop::Divide),
            "/=" => Token::AssignOp(Bop::Divide),
            ":=" => Token::Declare,
            "=" => Token::Assign,
            "==" => Token::Binop(Bop::Eq),
            "!=" => Token::Binop(Bop::Neq),
            "<=" => Token::Binop(Bop::Leq),
            "<" => Token::Binop(Bop::Lt),
            ">=" => Token::Binop(Bop::Geq),
            ">" => Token::Binop(Bop::Gt),
            "&&" => Token::Binop(Bop::And),
            "||" => Token::Binop(Bop::Or),
            _ => panic!("Invalid token: '{}' while scanning operator", tok),
        }
    }

    pub fn read_token(&mut self) -> Option<Token> {
        self.skip_spaces();
        match self.read() {
            Some(c) => Some(
                match c {
                    '\n' => Token::Sep,
                    '[' => Token::LBracket,
                    ']' => Token::RBracket,
                    '(' => Token::LParen,
                    ')' => Token::RParen,
                    '{' => Token::LBrace,
                    '}' => Token::RBrace,
                    '\'' => self.scan_char(),
                    '\"' => self.scan_string(),
                    ' ' | '\t' => panic!("Invalid token: should not contain whitespace, this is a bug"),
                    _ => {
                        if c.is_digit(10) || c == '-' {
                            self.scan_number(c)
                        } else if c.is_alphanumeric() {
                            self.scan_keyword(c)
                        } else {
                            self.scan_operator(c)
                        }
                    }
                }
            ),
            None => None
        }
    }

    pub fn read_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while let Some(tok) = self.read_token() {
            println!("{:?}", tok);
            tokens.push(tok)
        }
        tokens
    }
}

mod test {
    use std::io::{BufReader, Cursor};
    use crate::lexer::Lexer;
    use crate::lexer::Token::{Assign, Binop, Declare, Iden, Int, LBrace, RBrace, Sep, While};
    use crate::node::Bop;

    #[test]
    fn test_peek_consume() {
        let text = String::from("abcdefg");
        let mut actual_text = String::new();

        let reader = BufReader::new(Cursor::new(text.clone()));
        let mut lexer = Lexer::new(reader);
        while let Some(c) = lexer.peek() {
            actual_text.push(c);
            lexer.consume();
        }

        assert_eq!(text, actual_text)
    }

    #[test]
    fn test_lex_loop() {
        let program = "
            x := 0
            while i < n {
                x = x + 2
            }
        ";
        println!("Lexing:\n{}", program);

        let reader = BufReader::new(Cursor::new(program));
        let actual_tokens = Lexer::new(reader).read_tokens();
        let expect_tokens = vec![
            Sep,
            Iden("x".to_string()),
            Declare,
            Int(0),
            Sep,
            While,
            Iden("i".to_string()),
            Binop(Bop::Lt),
            Iden("n".to_string()),
            LBrace,
            Sep,
            Iden("x".to_string()),
            Assign,
            Iden("x".to_string()),
            Binop(Bop::Add),
            Int(2),
            Sep,
            RBrace,
            Sep
        ];
        assert_eq!(actual_tokens, expect_tokens)
    }
}