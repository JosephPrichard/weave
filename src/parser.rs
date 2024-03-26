use std::collections::VecDeque;
use crate::lexer::{TokenContext, Token};
use crate::node::{DefFuncNode, DefStructNode, DefTypeAliasNode, ImportNode, Node, TypeNode};

pub struct Parser {
    tokens: VecDeque<TokenContext>
}

impl Parser {
    fn new(tokens: VecDeque<TokenContext>) -> Parser {
        Parser { tokens }
    }

    fn peek_token(&self) -> Option<&TokenContext> {
        self.tokens.front()
    }

    fn consume_token(&mut self) {
        self.tokens.pop_front();
    }

    fn next_token(&mut self) -> Option<TokenContext> {
        self.tokens.pop_front()
    }

    fn advance_token(&mut self) -> Result<TokenContext, String> {
        let opt_tok = self.next_token();
        match opt_tok {
            Some(tok) => Ok(tok),
            None => Err("expected token, but reached end of the stream".to_string())
        }
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), String> {
        let opt_tok = self.advance_token()?;
        match opt_tok {
            tok if tok.kind == expected => Ok(()),
            tok => Err(format!("{} token expected, got {}", expected.to_text(), &tok))
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<Node>, String> {
        let mut nodes = vec![];
        while let Some(tok) = self.next_token() {
            let node = match tok.kind {
                Token::Import => self.parse_import()?,
                Token::Fn => self.parse_def_func()?,
                Token::Type => self.parse_def_type()?,
                Token::Struct => self.parse_def_struct()?,
                _ => return Err(format!("import, fn, or type expected, got {}", &tok)),
            };
            nodes.push(node)
        }
        Ok(nodes)
    }

    fn parse_import(&mut self) -> Result<Node, String> {
        let tok = self.advance_token()?;
        match tok.kind {
            Token::Iden(iden) => {
                let node = ImportNode { iden };
                Ok(Node::Import(node))
            }
            _ => return Err(format!("expected <iden> in import, got {}", &tok)),
        }
    }

    fn parse_def_func(&mut self) -> Result<Node, String> {
        let tok = self.advance_token()?;
        let iden = match tok.kind {
            Token::Iden(iden) => iden,
            _ => return Err(format!("expected <iden> in function definition, got {}", &tok)),
        };

        self.expect_token(Token::LParen)?;

        let args =  self.parse_type_pairs(Token::RParen)?;
        let ret = self.parse_ret_type()?;
        let body = vec![];
        let node = DefFuncNode { iden, args, ret, body };

        Ok(Node::DefFunc(node))
    }

    fn parse_type_pairs(&mut self, term: Token) -> Result<Vec<(String, TypeNode)>, String> {
        let mut args = vec![];
        loop {
            let tok = self.advance_token()?;
            let iden_arg = match tok.kind {
                Token::Iden(iden_arg) => iden_arg,
                typ if typ == term => break,
                _ => {
                    return Err(format!("expected {} or <iden> in function definition, got {}", term.to_text(), &tok))
                }
            };

            let type_node = self.parse_type()?;
            args.push((iden_arg, type_node));

            let tok = self.advance_token()?;
            match tok.kind {
                Token::Comma => continue,
                typ if typ == term => break,
                _ => {
                    return Err(format!("expected {} or ',' in function definition, got {}", term.to_text(), &tok))
                }
            }
        }
        Ok(args)
    }

    fn parse_ret_type(&mut self) -> Result<Option<TypeNode>, String> {
        let opt_tok = self.peek_token();
        match opt_tok {
            Some(tok) => match tok.kind {
                Token::Arrow => {
                    self.consume_token();
                    let type_node = self.parse_type()?;
                    Ok(Some(type_node))
                }
                _ => Ok(None)
            }
            _ => Ok(None)
        }
    }

    fn parse_type(&mut self) -> Result<TypeNode, String> {
        let tok = self.advance_token()?;
        match tok.kind {
            Token::Iden(iden) => Ok(TypeNode::Iden(iden)),
            Token::Fn => {
                let tok = self.advance_token()?;
                match tok.kind {
                    Token::LParen => {
                        let type_node = self.parse_fn_type()?;
                        Ok(type_node)
                    }
                    _ => return Err(format!("expected ')' after <fn>, got {}", &tok))
                }
            }
            Token::LBracket => {
                let tok = self.advance_token()?;
                match tok.kind {
                    Token::RBracket => {
                        let type_node = Box::new(self.parse_type()?);
                        Ok(TypeNode::Array(type_node))
                    }
                    _ => return Err(format!("expected '[]' before an array type, got {}", &tok))
                }
            }
            _ => return Err(format!("expected <iden>, <fn>, or <array> as type definition, got {}", &tok))
        }
    }

    fn parse_fn_type(&mut self) -> Result<TypeNode, String> {
        let mut args = vec![];
        loop {
            let type_node = self.parse_type()?;
            args.push(type_node);

            let tok = self.advance_token()?;
            match tok.kind {
                Token::Comma => continue,
                Token::RParen => break,
                _ => return Err(format!("expected ',' or ')' after argument type in fn type, got {}", &tok))
            }
        }

        let ret = self.parse_ret_type()?.map(|t| Box::new(t));
        Ok(TypeNode::Fn(args, ret))
    }

    fn parse_def_type(&mut self) -> Result<Node, String> {
        let tok = self.advance_token()?;
        let iden = match tok.kind {
            Token::Iden(iden) => iden,
            _ => return Err(format!("expected ',' or ')' after argument type in fn type, got {}", &tok))
        };

        let type_node = self.parse_type()?;
        let node = DefTypeAliasNode { iden, type_node };
        Ok(Node::DefTypeAlias(node))
    }

    fn parse_def_struct(&mut self) -> Result<Node, String> {
        let tok = self.advance_token()?;
        let iden = match tok.kind {
            Token::Iden(iden) => iden,
            _ => return Err(format!("expected <iden> after a struct definition, got {}", &tok))
        };

        self.expect_token(Token::LBrace)?;

        let fields = self.parse_type_pairs(Token::RBrace)?;
        let node = DefStructNode{ iden, fields };

        Ok(Node::DefStruct(node))
    }
}

mod test {
    use std::io::{BufReader, Cursor};
    use crate::lexer::Lexer;
    use crate::node::{BinopNode, WhileNode, Bop, DefFuncNode, GuardNode, FuncNode, TypeNode, DefStructNode};
    use crate::node::Bop::{Plus, Leq, Multiply, Minus};
    use crate::node::Const::Int;
    use crate::node::Node::{Assign, Binop, Constant, DefFunc, DefStruct, Func, Guard, Return, Variable, While};
    use crate::parser::Parser;

    #[test]
    fn test_parse_def() {
        let program = "
            struct Point {
                x int,
                y int,
            }
            fn concat_points(p1 Point, p2 Point) -> []Point
        ";
        let reader = BufReader::new(Cursor::new(program));
        let tokens = Lexer::new(reader).read_tokens().unwrap();
        println!("tokens {:?}", tokens);

        let actual_nodes = Parser::new(tokens).parse_program().unwrap();
        let expect_nodes = vec![
            DefStruct(DefStructNode{
                iden: "Point".to_string(),
                fields: vec![
                    ("x".to_string(), TypeNode::Iden("int".to_string())),
                    ("y".to_string(), TypeNode::Iden("int".to_string()))
                ],
            }),
            DefFunc(DefFuncNode {
                iden: "concat_points".to_string(),
                args: vec![
                    ("p1".to_string(), TypeNode::Iden("Point".to_string())),
                    ("p2".to_string(), TypeNode::Iden("Point".to_string()))
                ],
                ret: Some(TypeNode::Array(
                    Box::new(TypeNode::Iden("Point".to_string()))
                )),
                body: vec![],
            })
        ];
        assert_eq!(actual_nodes, expect_nodes)
    }

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