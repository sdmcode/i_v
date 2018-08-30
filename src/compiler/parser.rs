use std::iter::Peekable;

use compiler::token::Token;

use std::clone::Clone;

#[derive(Debug)]
pub struct Expression {
    id: u32,
    expression_type: ExpressionType,
    pub return_type: ReturnType
}

impl Expression {
    pub fn new(ident: u32, t: ExpressionType, rt: ReturnType) -> Self {
        Expression {
            id: ident,
            expression_type: t,
            return_type: rt
        }
    }

    pub fn to_string(&self) -> String {
        let mut ret = "Expr: ".to_string();
        ret.push_str(&self.id.to_string());

        return ret.to_string()

    }
}

#[derive(Debug)]
pub enum ExpressionType {

    Literal(Token),

    LiteralExpression(Box<Expression>),

    VarExpression(Box<Expression>),
    ConstExpression(Box<Expression>),

    UnaryExpression(Box<Expression>),
    BinaryExpression(Box<Expression>, Box<Expression>),

    ConditionalExpression(Box<Expression>, Box<Expression>),

    LoopExpression(Box<Expression>),

    FunctionExpression(Box<Function>),

    FunctionHeaderExpression(FunctionHeader)
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReturnType {
    ReturnVoid,
    ReturnBool,
    ReturnString,
    ReturnFloat,
    ReturnInteger,
    ReturnCollection,
    ReturnStruct,
    ReturnInvalid,
    ReturnArguments,
    ReturnContinue,
    ReturnFunction,
    ReturnFunctionHeader,
    ReturnEOF
}

impl From<Token> for ReturnType {
    fn from(tok: Token) -> Self {
        match tok {
            Token::VoidDecl => ReturnType::ReturnVoid,
            Token::BooleanDecl | Token::BooleanLiteral(_) => ReturnType::ReturnBool,
            Token::IntegerDecl | Token::IntegerLiteral(_) => ReturnType::ReturnInteger,
            Token::StringDecl | Token::StringLiteral(_) => ReturnType::ReturnString,
            Token::FloatDecl | Token::FloatLiteral(_) => ReturnType::ReturnFloat,
            Token::CollectionDecl => ReturnType::ReturnCollection,
            Token::StructDecl => ReturnType::ReturnStruct,
            Token::RightParenthesis => ReturnType::ReturnArguments,
            Token::Comma => ReturnType::ReturnContinue,
            Token::EOF => ReturnType::ReturnEOF,
            _ => ReturnType::ReturnInvalid
        }
    }
}

#[derive(Debug)]
pub struct Argument {
    return_type: ReturnType,
    ident: String
}

impl Argument {
    pub fn new(r_t: ReturnType, id: String) -> Argument {
        Argument {
            return_type: r_t,
            ident: id
        }
    }
}

#[derive(Debug)]
pub struct FunctionHeader {
    pub name: String,
    pub return_type: ReturnType,
    pub args: Vec<Argument>
}

impl FunctionHeader {
    pub fn new(n: String, rt: ReturnType, a: Vec<Argument>) -> FunctionHeader {
        FunctionHeader {
            name: n,
            return_type: rt,
            args: a
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub header: FunctionHeader,
    pub body: Box<Expression>
}

#[derive(Debug)]
pub enum ParseResult {
    Success(Expression),
    Failed(String)
}

pub struct Parser {
    tokens: Vec<Token>,
    node_count: u32
}

impl Parser {
    pub fn new(mut toks: Vec<Token>) -> Parser {
        Parser {
            tokens: toks,
            node_count: 0
        }
    }

    fn parse_function_header(&mut self) -> ParseResult {
        let mut popped = self.tokens.pop();
        let mut ident = String::new();
        match popped {
            Some(Token::Identifier(name)) => {
                ident = name;
                popped = self.tokens.pop();
                match popped {
                    Some(Token::Colon) => {
                        popped = self.tokens.pop();
                        let ret_type = ReturnType::from(popped.unwrap());
                        match ret_type {
                            ReturnType::ReturnInvalid => return ParseResult::Failed("Expected return type after function definition".to_string()),
                            _ => {
                                let popped = self.tokens.pop().unwrap();
                                match popped {
                                    Token::LeftParenthesis => {
                                        let mut args = Vec::new();
                                        loop {
                                            let rt = ReturnType::from(self.tokens.pop().unwrap());
                                            match rt {
                                                ReturnType::ReturnVoid => {
                                                    if args.len() > 0 {
                                                        return ParseResult::Failed("Unexpected void return type".to_string())
                                                    }
                                                    let f = FunctionHeader::new(ident, ret_type, args);
                                                    let popped = self.tokens.pop().unwrap();
                                                    match popped {
                                                        Token::RightParenthesis => {
                                                            self.node_count += 1;
                                                            let e = ExpressionType::FunctionHeaderExpression(f);

                                                            return ParseResult::Success(Expression::new(self.node_count, e, ReturnType::ReturnFunctionHeader))
                                                        },
                                                        _ => return ParseResult::Failed("Expected ')' after arguments".to_string())
                                                    }
                                                },

                                                ReturnType::ReturnInteger | ReturnType::ReturnString |
                                                ReturnType::ReturnBool | ReturnType::ReturnFloat |
                                                ReturnType::ReturnStruct | ReturnType::ReturnCollection => {
                                                    let popped = self.tokens.pop().unwrap();
                                                    match popped {
                                                        Token::Colon => {
                                                            let popped = self.tokens.pop().unwrap();
                                                            match popped {
                                                                Token::Identifier(arg_name) => {
                                                                    args.push(Argument::new(rt, arg_name));
                                                                },
                                                                _ => return ParseResult::Failed("Expected argument name after ':'".to_string())
                                                            }
                                                        },
                                                        _ => return ParseResult::Failed("Expected ')' after arguments".to_string())
                                                    }
                                                },

                                                ReturnType::ReturnContinue => (),

                                                ReturnType::ReturnEOF => return ParseResult::Failed("Unexpected end of file".to_string()),

                                                ReturnType::ReturnArguments => {
                                                    if args.len() > 0 {
                                                        let f = FunctionHeader::new(ident, rt, args);
                                                        self.node_count += 1;
                                                        let e = ExpressionType::FunctionHeaderExpression(f);

                                                        return ParseResult::Success(Expression::new(self.node_count, e, ReturnType::ReturnFunctionHeader))
                                                    } else {
                                                        return ParseResult::Failed("Expected argument list".to_string())
                                                    }
                                                },
                                                _ => return ParseResult::Failed("Unexpected argument".to_string())
                                            }
                                        }
                                    },
                                    _ => return ParseResult::Failed("Expected '(' after return type".to_string())
                                }
                            }
                        }
                    },
                    _ => return ParseResult::Failed("Expected ':' after identifier".to_string())
                }
            },
            _ => return ParseResult::Failed("Expected function identifier".to_string())
        }
    }

    fn parse_expression(&mut self, expected: ReturnType) -> ParseResult {
        let next = self.tokens.pop().unwrap();

        let uber_next = self.tokens.pop().unwrap();

        if uber_next == Token::Semicolon {

            if ReturnType::from(next.clone()) != expected {
                return ParseResult::Failed("Invalid return type".to_string())
            }

            match next {
                Token::BooleanLiteral(_) => {
                    self.node_count += 1;
                    return ParseResult::Success(Expression::new(self.node_count,
                    ExpressionType::Literal(next), ReturnType::ReturnBool))
                },

                Token::IntegerLiteral(_) => {
                    self.node_count += 1;
                    return ParseResult::Success(Expression::new(self.node_count,
                    ExpressionType::Literal(next), ReturnType::ReturnInteger))
                },

                Token::FloatLiteral(_) => {
                    self.node_count += 1;
                    return ParseResult::Success(Expression::new(self.node_count,
                    ExpressionType::Literal(next), ReturnType::ReturnFloat))
                },

                Token::StringLiteral(_) => {
                    self.node_count += 1;
                    return ParseResult::Success(Expression::new(self.node_count,
                    ExpressionType::Literal(next), ReturnType::ReturnString))
                },
                _ => return ParseResult::Failed("Expected literal value".to_string())
            }
        }
        return ParseResult::Failed("Expected ';' or expression".to_string())
    }

    fn parse_var_decl(&mut self) -> ParseResult {
        let mut name = String::new();

        match self.tokens.pop().unwrap() {
            Token::Identifier(ident) => {
                name = ident;

                match self.tokens.pop().unwrap() {
                    Token::Colon => {
                        let rt = ReturnType::from(self.tokens.pop().unwrap());
                        let expctd = rt.clone();
                        match rt {
                            ReturnType::ReturnInteger | ReturnType::ReturnString |
                            ReturnType::ReturnBool | ReturnType::ReturnFloat |
                            ReturnType::ReturnStruct | ReturnType::ReturnCollection => {
                                match self.tokens.pop().unwrap() {
                                    Token::Assign => {
                                        let res = self.parse_expression(expctd.clone());
                                        match res {
                                            ParseResult::Success(expr) => {
                                                if expr.return_type != expctd {
                                                    return ParseResult::Failed("Invalid return type".to_string());
                                                }
                                                self.node_count += 1;
                                                return ParseResult::Success(Expression::new(self.node_count, ExpressionType::VarExpression(Box::new(expr)), rt))
                                            }
                                            _ => return res
                                        }
                                    },
                                    _ => return ParseResult::Failed("Expected '=' after return type".to_string())
                                }
                            },
                            _ => return ParseResult::Failed("Expected return type after ':'".to_string())
                        }
                    },
                    _ => return ParseResult::Failed("Expected ':' after identifier".to_string())
                }
            },
            _ => return ParseResult::Failed("Expected identifier".to_string())
        }
    }

    pub fn parse(&mut self) -> ParseResult {

        loop {

            let cur_token = self.tokens.pop().unwrap();

            match cur_token {
                Token::VarDecl => {
                    return self.parse_var_decl()
                }

                Token::FunctionDecl => return self.parse_function_header(),

                _ => {
                    println!("Parse failed at: {:?}", cur_token);

                    return ParseResult::Failed("Failed to parse".to_string())
                }
            };
        }

        return ParseResult::Failed("Failed".to_string())
    }

}
