

mod test {
    use crate::node::{BinopNode, WhileNode, Bop, DefFuncNode, GuardNode, FuncNode};
    use crate::node::Bop::{Add, Leq, Multiply, Subtract};
    use crate::node::Const::Int;
    use crate::node::Node::{Assign, Binop, Constant, DefFunc, Func, Guard, Return, Variable, While};
    use crate::node::TypeNode::IntType;

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
                            op: Add,
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
                args: vec![("n".to_string(), IntType)],
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
                        op: Add,
                        lhs: Box::new(Func(FuncNode{
                            iden: "fib".to_string(),
                            args: vec![
                                Binop(BinopNode{
                                    op: Subtract,
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