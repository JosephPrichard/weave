use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
pub struct TokenContext {
    pub kind: Token,
    pub lpos: Position,
    pub rpos: Position,
}

impl Display for TokenContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} between {} and {}", self.kind, self.lpos, self.rpos)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: u32,
    pub col: u32,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "line: {}, col: {}", self.line, self.col)
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    IntLit(i32),
    FloatLit(f64),
    CharLit(char),
    StrLit(String),
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
    AssignOp(Aop),
    Operator(Op),
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
    Import,
    SemiColon,
    Arrow,
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Plus,
    Exp,
    Minus,
    Multiply,
    Divide,
    Eq,
    Neq,
    Leq,
    Geq,
    Lt,
    Gt,
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum Aop {
    Plus,
    Exp,
    Minus,
    Multiply,
    Divide,
}

impl Token {
    pub fn to_text(&self) -> &'static str {
        match self {
            Token::IntLit(_) => "<int>",
            Token::FloatLit(_) => "<float>",
            Token::CharLit(_) => "<char>",
            Token::StrLit(_) => "<string>",
            Token::LParen => "':'",
            Token::RParen => "')'",
            Token::LBracket => "'['",
            Token::RBracket => "']'",
            Token::LBrace => "'{'",
            Token::RBrace => "'}'",
            Token::Dot => "'.'",
            Token::Comma => "','",
            Token::Declare => "':='",
            Token::Assign => "'='",
            Token::AssignOp(_) => "<assignop>",
            Token::Operator(_) => "<operator>",
            Token::Iden(_) => "<iden>",
            Token::True => "true",
            Token::False => "false",
            Token::Fn => "fn",
            Token::Struct => "struct",
            Token::Type => "type",
            Token::Return => "return",
            Token::Break => "break",
            Token::Continue => "continue",
            Token::While => "while",
            Token::For => "for",
            Token::In => "in",
            Token::Import => "import",
            Token::SemiColon => "';'",
            Token::Arrow => "'->'"
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_text())?;
        Ok(())
    }
}

pub struct Lexer<T: BufRead> {
    reader: BufReader<T>,
    pos: Position,
}

impl<T: BufRead> Lexer<T> {
    pub fn new(reader: BufReader<T>) -> Lexer<T> {
        Lexer { reader, pos: Position { line: 0, col: 0 } }
    }

    fn read(&mut self) -> Result<Option<char>, String> {
        let mut buffer = [0; 1];
        match self.reader.read(&mut buffer) {
            Ok(count) => Ok(
                if count > 0 {
                    let c = buffer[0] as char;
                    if c == '\n' {
                        self.pos.line += 1
                    } else {
                        self.pos.col += 1
                    }
                    Some(c)
                } else {
                    None
                }
            ),
            Err(err) => Err(err.to_string())
        }
    }

    fn peek(&mut self) -> Result<Option<char>, String> {
        match self.reader.fill_buf() {
            Ok(buffer) => Ok(
                if !buffer.is_empty() {
                    Some(buffer[0] as char)
                } else {
                    None
                }
            ),
            Err(err) => Err(err.to_string())
        }
    }

    fn consume(&mut self) {
        self.reader.consume(1)
    }

    fn skip_spaces(&mut self) -> Result<(), String> {
        while let Some(c) = self.peek()? {
            if c.is_whitespace() {
                self.consume()
            } else {
                break;
            }
        }
        Ok(())
    }

    fn match_escseq(c: char, term: char) -> Result<char, String> {
        match c {
            '\\' => Ok('\\'),
            'n' => Ok('\n'),
            't' => Ok('\t'),
            'r' => Ok('\r'),
            '0' => Ok('\0'),
            _ if c == term => Ok(term),
            _ => Err(format!("Invalid esc seq: '\\{}'", c)),
        }
    }

    fn match_control(&mut self, c: char, lpos: Position) -> Option<TokenContext> {
        let tok = match c {
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ',' => Token::Comma,
            '.' => Token::Dot,
            ';' => Token::SemiColon,
            _ => return None
        };
        Some(TokenContext { kind: tok, lpos, rpos: self.pos })
    }

    fn is_control(c: char) -> bool {
        "[](){},.;".contains(c)
    }

    fn scan_text(&mut self, term: char) -> Result<(String, Position, Position), String> {
        let lpos = self.pos;
        let mut isesc = false;
        let mut str = String::new();
        while let Some(c) = self.read()? {
            if isesc {
                isesc = false;
                let c = Self::match_escseq(c, term)?;
                str.push(c)
            } else {
                if c == '\\' {
                    isesc = true
                } else if c == term {
                    break;
                } else {
                    str.push(c)
                }
            }
        };

        Ok((str, lpos, self.pos))
    }

    fn scan_char(&mut self) -> Result<TokenContext, String> {
        let (str, lpos, rpos) = self.scan_text('\'')?;
        let first_char = str.chars().nth(0);

        match (first_char, str.len()) {
            (Some(c), 1) => {
                let token = TokenContext { kind: Token::CharLit(c), lpos, rpos };
                Ok(token)
            }
            _ => Err(format!("Invalid char: '{}' a char literal length 1 between {} and {}", str, lpos, rpos))
        }
    }

    fn scan_string(&mut self) -> Result<TokenContext, String> {
        let (str, lpos, rpos) = self.scan_text('\"')?;
        let tok = TokenContext { kind: Token::StrLit(str), lpos, rpos };
        Ok(tok)
    }

    fn scan_number(&mut self, c: char) -> Result<TokenContext, String> {
        let lpos = self.pos;
        let mut is_int = true;
        let mut tokstr = String::from(c);
        while let Some(c) = self.peek()? {
            if !c.is_alphanumeric() {
                break;
            }
            if c == '.' {
                is_int = false;
            }
            tokstr.push(c);
            self.consume()
        }

        let rpos = self.pos;
        if is_int {
            match tokstr.parse::<i32>() {
                Ok(int) => Ok(TokenContext { kind: Token::IntLit(int), lpos, rpos }),
                Err(_) => Err(format!("Invalid int: cannot lex {} between {} and {}", tokstr, lpos, rpos))
            }
        } else {
            match tokstr.parse::<f64>() {
                Ok(float) => Ok(TokenContext { kind: Token::FloatLit(float), lpos, rpos }),
                Err(_) => Err(format!("Invalid float: cannot lex {} between {} and {}", tokstr, lpos, rpos))
            }
        }
    }

    fn scan_keyword(&mut self, c: char) -> Result<TokenContext, String> {
        let lpos = self.pos;
        let mut tokstr = String::from(c);
        while let Some(c) = self.peek()? {
            if '_' != c && !c.is_alphanumeric() {
                break;
            }
            tokstr.push(c);
            self.consume()
        }

        let tok = match tokstr.as_str() {
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
            "import" => Token::Import,
            _ => Token::Iden(tokstr),
        };

        Ok(TokenContext { kind: tok, lpos, rpos: self.pos })
    }

    fn scan_special(&mut self, c: char) -> Result<TokenContext, String> {
        let lpos = self.pos;
        let mut tok = String::from(c);
        while let Some(c) = self.peek()? {
            if c.is_whitespace() || c.is_alphanumeric() || Self::is_control(c) {
                break;
            }
            tok.push(c);
            self.consume()
        }

        let tok = match tok.as_str() {
            "*" => Token::Operator(Op::Multiply),
            "**" => Token::Operator(Op::Exp),
            "*=" => Token::AssignOp(Aop::Multiply),
            "**=" => Token::AssignOp(Aop::Exp),
            "-" => Token::Operator(Op::Minus),
            "-=" => Token::AssignOp(Aop::Minus),
            "+" => Token::Operator(Op::Plus),
            "+=" => Token::AssignOp(Aop::Plus),
            "/" => Token::Operator(Op::Divide),
            "/=" => Token::AssignOp(Aop::Divide),
            ":=" => Token::Declare,
            "=" => Token::Assign,
            "==" => Token::Operator(Op::Eq),
            "!=" => Token::Operator(Op::Neq),
            "<=" => Token::Operator(Op::Leq),
            "<" => Token::Operator(Op::Lt),
            ">=" => Token::Operator(Op::Geq),
            ">" => Token::Operator(Op::Gt),
            "&&" => Token::Operator(Op::And),
            "||" => Token::Operator(Op::Or),
            "->" => Token::Arrow,
            _ => return Err(format!("Invalid token: '{}' while scanning", tok))
        };

        Ok(TokenContext { kind: tok, lpos, rpos: self.pos })
    }

    pub fn read_token(&mut self) -> Result<Option<TokenContext>, String> {
        self.skip_spaces()?;

        if let Some(c) = self.read()? {
            let token = match self.match_control(c, self.pos) {
                Some(token) => token,
                None => match c {
                    '\'' => self.scan_char()?,
                    '\"' => self.scan_string()?,
                    _ if c.is_digit(10) => self.scan_number(c)?,
                    _ if c.is_alphanumeric() => self.scan_keyword(c)?,
                    _ => self.scan_special(c)?
                }
            };
            Ok(Some(token))
        } else {
            Ok(None)
        }
    }

    pub fn read_tokens(&mut self) -> Result<VecDeque<TokenContext>, String> {
        let mut tokens = VecDeque::new();
        while let Some(tok) = self.read_token()? {
            tokens.push_back(tok)
        }
        Ok(tokens)
    }
}

mod test {
    use std::collections::VecDeque;
    use std::io::{BufRead, BufReader, Cursor};
    use crate::lexer::{Lexer, Op, Token};
    use crate::lexer::Token::{Arrow, Assign, CharLit, Comma, Declare, Dot, Fn, Iden, IntLit, LBrace, LBracket, LParen, Operator, RBrace, RBracket, Return, RParen, SemiColon, StrLit, Struct, While};

    fn lex_tokens<T: BufRead>(reader: BufReader<T>) -> VecDeque<Token> {
        Lexer::new(reader)
            .read_tokens()
            .unwrap()
            .into_iter()
            .map(|tok| { tok.kind })
            .collect()
    }

    #[test]
    fn test_peek_consume() {
        let text = String::from("abcdefg");
        let mut actual_text = String::new();

        let reader = BufReader::new(Cursor::new(text.clone()));
        let mut lexer = Lexer::new(reader);
        while let Some(c) = lexer.peek().unwrap() {
            actual_text.push(c);
            lexer.consume();
        }

        assert_eq!(text, actual_text)
    }

    #[test]
    fn test_lex_loop() {
        let program = "
            x := 0;
            while i < n {
                x = x + 2;
            }
        ";
        println!("Lexing:\n{}", program);

        let reader = BufReader::new(Cursor::new(program));
        let actual_tokens = lex_tokens(reader);
        let expect_tokens = vec![
            Iden("x".to_string()),
            Declare,
            IntLit(0),
            SemiColon,
            While,
            Iden("i".to_string()),
            Operator(Op::Lt),
            Iden("n".to_string()),
            LBrace,
            Iden("x".to_string()),
            Assign,
            Iden("x".to_string()),
            Operator(Op::Plus),
            IntLit(2),
            SemiColon,
            RBrace,
        ];
        assert_eq!(actual_tokens, expect_tokens)
    }

    #[test]
    fn test_lex_func() {
        let program = "
            fn concat_persons(x1 Person, x2 Person) {
                x := \"Names:\";
                x = x1.name + x2.name;
                return x;
            }
        ";
        println!("Lexing:\n{}", program);

        let reader = BufReader::new(Cursor::new(program));
        let actual_tokens = lex_tokens(reader);
        let expect_tokens = vec![
            Fn,
            Iden("concat_persons".to_string()),
            LParen,
            Iden("x1".to_string()),
            Iden("Person".to_string()),
            Comma,
            Iden("x2".to_string()),
            Iden("Person".to_string()),
            RParen,
            LBrace,
            Iden("x".to_string()),
            Declare,
            StrLit("Names:".to_string()),
            SemiColon,
            Iden("x".to_string()),
            Assign,
            Iden("x1".to_string()),
            Dot,
            Iden("name".to_string()),
            Operator(Op::Plus),
            Iden("x2".to_string()),
            Dot,
            Iden("name".to_string()),
            SemiColon,
            Return,
            Iden("x".to_string()),
            SemiColon,
            RBrace,
        ];
        assert_eq!(actual_tokens, expect_tokens)
    }

    #[test]
    fn test_lex_literal() {
        let program = "
            x := \" \\n \\t \\\\ \";
            y := \'\\n\';
        ";
        println!("Lexing:\n{}", program);

        let reader = BufReader::new(Cursor::new(program));
        let actual_tokens = lex_tokens(reader);
        let expect_tokens = vec![
            Iden("x".to_string()),
            Declare,
            StrLit(" \n \t \\ ".to_string()),
            SemiColon,
            Iden("y".to_string()),
            Declare,
            CharLit('\n'),
            SemiColon,
        ];
        assert_eq!(actual_tokens, expect_tokens)
    }

    #[test]
    fn test_lex_def() {
        let program = "
            struct Point {
                x int,
                y int,
            }
            fn concat_points(p1 Point, p2 Point) -> []Point
        ";
        let reader = BufReader::new(Cursor::new(program));
        let actual_tokens = lex_tokens(reader);
        let expect_tokens = vec![
            Struct,
            Iden("Point".to_string()),
            LBrace,
            Iden("x".to_string()),
            Iden("int".to_string()),
            Comma,
            Iden("y".to_string()),
            Iden("int".to_string()),
            Comma,
            RBrace,
            Fn,
            Iden("concat_points".to_string()),
            LParen,
            Iden("p1".to_string()),
            Iden("Point".to_string()),
            Comma,
            Iden("p2".to_string()),
            Iden("Point".to_string()),
            RParen,
            Arrow,
            LBracket,
            RBracket,
            Iden("Point".to_string()),
        ];
        assert_eq!(actual_tokens, expect_tokens)
    }
}