use std::collections::VecDeque;
use crate::lexer::{Token};
use crate::node::{DefFuncNode, DefStructNode, DefTypeAliasNode, ImportNode, Node, TypeNode};

pub struct Parser {
    tokens: VecDeque<Token>
}

pub fn expect_error(expected: &str, actual: &str) -> String {
    format!("{} expected, got {}", expected, actual)
}

impl Parser {
    fn new(&self, tokens: VecDeque<Token>) -> Parser {
        Parser { tokens }
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.front()
    }

    fn advance_token(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    fn expect_token(&mut self) -> Result<Token, String> {
        let tok = self.advance_token();
        match tok {
            Some(tok) => Ok(tok),
            None => Err("expected a token".to_string())
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<Node>, String> {
        let mut nodes = vec![];
        loop {
            let tok = self.advance_token();
            let node = match tok {
                Some(Token::Import) => self.parse_import()?,
                Some(Token::Fn) => self.parse_def_func()?,
                Some(Token::Type) => self.parse_def_type()?,
                Some(Token::Struct) => self.parse_def_struct()?,
                Some(tok) => return Err(expect_error("import, fn, or type", tok.to_text())),
                None => return Ok(nodes)
            };
            nodes.push(node)
        }
    }

    fn parse_import(&mut self) -> Result<Node, String> {
        let tok = self.expect_token()?;
        match tok {
            Token::Iden(iden) => {
                let node = ImportNode { iden };
                Ok(Node::Import(node))
            }
            tok =>
                Err(expect_error("<iden>", tok.to_text())),
        }
    }

    fn parse_def_func(&mut self) -> Result<Node, String> {
        let tok = self.expect_token()?;
        let iden = match tok {
            Token::Iden(iden) => iden,
            tok =>
                return Err(expect_error("<iden>", tok.to_text()))
        };

        let tok = self.expect_token()?;
        let args = match tok {
            Token::LParen => self.parse_type_pairs(Token::RParen)?,
            tok =>
                return Err(expect_error("(", tok.to_text()))
        };

        let tok = self.peek_token();
        let ret = match tok {
            Some(Token::Colon) => {
                self.advance_token();
                let type_node = self.parse_type()?;
                Some(type_node)
            }
            _ => None
        };

        let body = vec![];
        let node = DefFuncNode { iden, args, ret, body };
        Ok(Node::DefFunc(node))
    }

    fn parse_type_pairs(&mut self, term: Token) -> Result<Vec<(String, TypeNode)>, String> {
        let mut args = vec![];
        loop {
            let tok = self.expect_token()?;
            let iden_arg = match tok {
                Token::Iden(iden_arg) => iden_arg,
                tok =>
                    return Err(expect_error("<iden>", tok.to_text()))
            };

            let type_node = self.parse_type()?;
            args.push((iden_arg, type_node));

            let tok = self.expect_token()?;
            match tok {
                Token::Comma => (),
                tok => {
                    if tok == term {
                        break
                    } else {
                        let m = format!("{} or ,", term.to_text());
                        return Err(expect_error(&m, tok.to_text()))
                    }
                }
            }
        }
        Ok(args)
    }

    fn parse_type(&mut self) -> Result<TypeNode, String> {
        let tok = self.expect_token()?;
        match tok {
            Token::Iden(iden) => Ok(TypeNode::Iden(iden)),
            Token::Fn => {
                let tok = self.expect_token()?;
                match tok {
                    Token::LParen => {
                        let type_node = self.parse_fn_type()?;
                        Ok(type_node)
                    }
                    token => Err(expect_error("<fn>", token.to_text()))
                }
            }
            Token::LBracket => {
                let tok = self.expect_token()?;
                match tok {
                    Token::RBracket => {
                        let type_node = Box::new(self.parse_type()?);
                        Ok(TypeNode::Array(type_node))
                    }
                    token => Err(expect_error("]", token.to_text()))
                }
            }
            token => Err(expect_error("<iden> or <array>", token.to_text()))
        }
    }

    fn parse_fn_type(&mut self) -> Result<TypeNode, String> {
        let mut args = vec![];
        loop {
            let type_node = self.parse_type()?;
            args.push(type_node);

            let tok = self.expect_token()?;
            match tok {
                Token::Comma => (),
                Token::RParen => break,
                tok =>
                    return Err(expect_error(", or )", tok.to_text()))
            }
        }

        let tok = self.peek_token();
        let ret = match tok {
            Some(Token::Colon) => {
                self.advance_token();
                let type_node = self.parse_type()?;
                Some(Box::new(type_node))
            }
            _ => None
        };

        Ok(TypeNode::Fn(args, ret))
    }

    fn parse_def_type(&mut self) -> Result<Node, String> {
        let tok = self.expect_token()?;
        let iden = match tok {
            Token::Iden(iden) => iden,
            tok =>
                return Err(expect_error("<iden>", tok.to_text()))
        };

        let type_node = self.parse_type()?;
        let node = DefTypeAliasNode { iden, type_node };
        Ok(Node::DefTypeAlias(node))
    }

    fn parse_def_struct(&mut self) -> Result<Node, String> {
        let tok = self.expect_token()?;
        let iden = match tok {
            Token::Iden(iden) => iden,
            tok =>
                return Err(expect_error("<iden>", tok.to_text()))
        };

        let tok = self.expect_token()?;
        match tok {
            Token::LBrace => {
                let fields = self.parse_type_pairs(Token::RBrace)?;
                let node = DefStructNode{ iden, fields };
                Ok(Node::DefStruct(node))
            }
            tok =>
                Err(expect_error("{", tok.to_text()))
        }
    }
}

mod test {
    use crate::node::{BinopNode, WhileNode, Bop, DefFuncNode, GuardNode, FuncNode, TypeNode};
    use crate::node::Bop::{Plus, Leq, Multiply, Minus};
    use crate::node::Const::Int;
    use crate::node::Node::{Assign, Binop, Constant, DefFunc, Func, Guard, Return, Variable, While};

    #[test]
    fn test_parse_loop() {
        let expect_node =
            While(WhileNode{
                cond: Box::new(Binop(BinopNode {
                    op: Bop::Lt,
                    lhs: Box::new(Variable("i".to_string())),
                    rhs: Box::new(Variable("n".to_string())),
                })),
                body: vec![
                    Assign(
                        "acc".to_string(),
                        Box::new(Binop(BinopNode {
                            op: Multiply,
                            lhs: Box::new(Variable("acc".to_string())),
                            rhs: Box::new(Variable("x".to_string())),
                        }))
                    ),
                    Assign(
                        "i".to_string(),
                        Box::new(Binop(BinopNode {
                            op: Plus,
                            lhs: Box::new(Variable("i".to_string())),
                            rhs: Box::new(Variable("1".to_string())),
                        }))
                    ),
                ]
            });
    }

    #[test]
    fn test_parse_func() {
        let expect_node =
            DefFunc(DefFuncNode{
                iden: "sum".to_string(),
                args: vec![("n".to_string(), TypeNode::Iden("int".to_string()))],
                ret: Some(TypeNode::Iden("int".to_string())),
                body: vec![
                    Guard(GuardNode{
                        cond: Box::new(Binop(BinopNode{
                            op: Leq,
                            lhs: Box::new(Variable("n".to_string())),
                            rhs: Box::new(Constant(Int(0))),
                        })),
                        this: Box::new(Constant(Int(0))),
                    }),
                    Return(Box::new(Binop(BinopNode{
                        op: Plus,
                        lhs: Box::new(Func(FuncNode{
                            iden: "fib".to_string(),
                            args: vec![
                                Binop(BinopNode{
                                    op: Minus,
                                    lhs: Box::new(Variable("n".to_string())),
                                    rhs: Box::new(Constant(Int(1))),
                                })
                            ],
                        })),
                        rhs: Box::new(Constant(Int(1))),
                    })))
                ],
            });
    }
}