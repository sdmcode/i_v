use std::collections::HashMap;

use compiler::token::Token;

use std::clone::Clone;

#[derive(Debug, Clone)]
pub struct Expression {
    id: u32,
    pub expression_type: ExpressionType,
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

#[derive(Debug, Clone)]
pub enum ExpressionType {

    Literal(Token),

    LiteralExpression(String, Box<Expression>),

    AssignmentExpression(String, Box<Expression>),

    PrintExpression(String),

    BlockExpression(Vec<Expression>),

    VarExpression(Box<Expression>),
    ConstExpression(Box<Expression>),

    UnaryExpression(Token, Box<Expression>),
    BinaryExpression(Token, Box<Expression>, Box<Expression>),

    ConditionalExpression(Box<Expression>, Box<Expression>),

    LoopExpression(Box<Expression>),

    FunctionExpression(Box<Function>),

    FunctionHeaderExpression(FunctionHeader)
}

#[derive(Clone, Debug)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub node_count: u32,
    pub vars: HashMap<String, Expression>,

}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing: None,
            node_count: 0,
            vars: HashMap::new()
        }
    }

    pub fn new_sub(env: Environment) -> Environment {
        Environment {
            enclosing: Some(Box::new(env)),
            node_count: 0,
            vars: HashMap::new()
        }
    }

    pub fn define(&mut self, var: Variable) -> ParseResult {
        match self.vars.get(&var.ident.clone()) {
            Some(_) => return ParseResult::Failed("Variable already defined".to_string()),
            _ => {
                self.node_count += 1;
                self.vars.insert(var.ident.clone(), var.value.clone());
                return ParseResult::Success(
                    Expression::new(
                        self.node_count,
                        ExpressionType::LiteralExpression(var.ident.clone(), Box::new(var.value.clone())),
                        var.value.return_type.clone()
                    )
                )
            }
        }
    }

    pub fn assign_value(&mut self, var: Variable) -> ParseResult {
        match self.vars.get_mut(&var.ident.clone()) {
            Some(val) => {
                self.node_count += 1;
                *val = var.value.clone();
                return ParseResult::Success(
                    Expression::new(
                        self.node_count,
                        ExpressionType::LiteralExpression(var.ident.clone(), Box::new(var.value.clone())),
                        var.value.return_type.clone()
                    )
                )
            },
            _ => {
                match self.enclosing {
                    Some(ref mut env) => {
                        return env.assign_value(var)
                    },
                    _ => return ParseResult::Failed("Variable not defined".to_string())
                }
            }
        }
    }

    pub fn get_value(&mut self, var: String) -> ParseResult {
        match self.vars.get(&var) {
            Some(val) => return ParseResult::Success(val.clone()),
            _ => {
                match self.enclosing {
                    Some(ref mut env) => return env.get_value(var),
                    _ => return ParseResult::Failed("Variable doesn't exist".to_string())
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Variable {
    ident: String,
    value: Expression
}

impl Variable {
    fn new(name: String, val: Expression) -> Variable {
        Variable {
            ident: name,
            value: val
        }
    }
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
    ReturnBlock,
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Function {
    pub header: FunctionHeader,
    pub body: Box<Expression>
}

#[derive(Debug, Clone)]
pub enum ParseResult {
    Success(Expression),
    Failed(String)
}

#[derive(Clone, Debug)]
pub struct Statement {
    pub expr: Expression
}

impl Statement {
    pub fn new(ex: Expression) -> Statement {
        Statement {
            expr: ex
        }
    }
}

#[derive(Clone, Debug)]
pub struct AstProgram {
    pub statements: Vec<Statement>,
    pub node_count: u32,
    pub env: Environment
}

impl AstProgram {
    pub fn new() -> AstProgram {
        AstProgram {
            statements: vec!(),
            node_count: 0,
            env: Environment::new()
        }
    }
}

pub struct Parser {
    program: AstProgram,
    tokens: Vec<Token>,
    node_count: u32
}

impl Parser {
    pub fn new(mut toks: Vec<Token>) -> Parser {
        Parser {
            program: AstProgram::new(),
            tokens: toks,
            node_count: 0
        }
    }

    fn parse_primary(&mut self) -> ParseResult {

        let t = self.tokens.pop();

        match t.clone() {
            None => return ParseResult::Failed("Ran out of tokens".to_string()),

            Some(Token::StringLiteral(_)) | Some(Token::IntegerLiteral(_)) |
            Some(Token::FloatLiteral(_)) | Some(Token:: BooleanLiteral(_)) |
            Some(Token::CollectionLiteral) | Some(Token::RangeLiteral) |
            Some(Token::Identifier(_)) | Some(Token::Null) => {
                let rt = ReturnType::from(t.clone().unwrap());
                self.node_count += 1;

                return ParseResult::Success(Expression::new(
                        self.node_count,
                        ExpressionType::Literal(t.unwrap()),
                        rt));
            },

            Some(Token::LeftBrace) => {
                let rhs = self.parse_expression();
                match self.tokens.pop() {
                    Some(Token::RightBrace) => {
                        return rhs
                    },
                    Some(_) => return ParseResult::Failed("Expected ')'".to_string()),
                    None => return ParseResult::Failed("Ran out of tokens".to_string()),
                }
            },

            Some(t) =>  {
                println!("{:?}", t);
                return ParseResult::Failed("Expected primary expression".to_string())
            }
        }
    }

    fn parse_unary(&mut self) -> ParseResult {

        let t = self.tokens.pop();
        let rt = ReturnType::from(t.clone().unwrap());

        match t.clone() {
            None => return ParseResult::Failed("Ran out of tokens..".to_string()),

            Some(Token::Bang) | Some(Token::Subtract) => {
                let rcmp = self.parse_unary();

                match rcmp.clone() {

                    ParseResult::Success(rhs) => {
                        self.node_count += 1;

                        return ParseResult::Success(Expression::new(
                                self.node_count,
                                ExpressionType::UnaryExpression(t.unwrap(), Box::new(rhs)),
                                rt))
                    },
                    _ => return ParseResult::Failed("Failed unary".to_string())
                }
            },
            Some(_) => return self.parse_primary()
        }
    }

    fn parse_multiplication(&mut self) -> ParseResult {
        let mut cmp = self.parse_unary();

        loop {
            match cmp.clone() {
                ParseResult::Success(lr) => {

                let lhs = lr.clone();

                    let t = self.tokens.pop();
                    let rt = lhs.return_type.clone();

                    match t.clone() {
                        None => return ParseResult::Failed("Ran out of tokens..".to_string()),

                        Some(Token::Multiply) | Some(Token::Divide) => {
                            let rcmp = self.parse_unary();

                            match rcmp.clone() {

                                ParseResult::Success(rhs) => {
                                    if rt != rhs.return_type.clone() {
                                        return ParseResult::Failed("Comparing different return types!".to_string())
                                    } else {
                                        self.node_count += 1;

                                        cmp = ParseResult::Success(Expression::new(
                                                self.node_count,
                                                ExpressionType::BinaryExpression(t.unwrap(), Box::new(lhs), Box::new(rhs)),
                                                rt));
                                    }
                                },
                                _ => return ParseResult::Failed("Failed multiplication RHS".to_string())
                            }
                        },

                        Some(_) => return cmp
                    }
                },

                _ => {
                    println!("Failed multiplication");
                    return cmp
                }
            }
        }
    }

    fn parse_addition(&mut self) -> ParseResult {
        let mut cmp = self.parse_multiplication();

        loop {
            match cmp.clone() {
                ParseResult::Success(lr) => {

                let lhs = lr.clone();

                    let t = self.tokens.pop();
                    let rt = lhs.return_type.clone();

                    match t.clone() {
                        None => return ParseResult::Failed("Ran out of tokens..".to_string()),

                        Some(Token::Add) | Some(Token::Subtract) => {
                            let rcmp = self.parse_multiplication();

                            match rcmp.clone() {

                                ParseResult::Success(rhs) => {
                                    if rt != rhs.return_type.clone() {
                                        return ParseResult::Failed("Comparing different return types!".to_string())
                                    } else {
                                        self.node_count += 1;

                                        cmp = ParseResult::Success(Expression::new(
                                                self.node_count,
                                                ExpressionType::BinaryExpression(t.unwrap(), Box::new(lhs), Box::new(rhs)),
                                                rt));
                                    }
                                },
                                _ => return ParseResult::Failed("Failed addition RHS".to_string())
                            }
                        },

                        Some(_) => return cmp
                    }
                },

                _ => {
                    println!("Failed addition");
                    return cmp
                }
            }
        }
    }

    fn parse_comparison(&mut self) -> ParseResult {
        let mut cmp = self.parse_addition();

        loop {
            match cmp.clone() {
                ParseResult::Success(lr) => {

                let lhs = lr.clone();

                    let t = self.tokens.pop();
                    let rt = lhs.return_type.clone();

                    match t.clone() {
                        None => return ParseResult::Failed("Ran out of tokens..".to_string()),

                        Some(Token::GreaterThan) | Some(Token::LessThan) |
                        Some(Token:: LessThanEqual) | Some(Token:: GreaterThanEqual) => {
                            let rcmp = self.parse_addition();

                            match rcmp.clone() {

                                ParseResult::Success(rhs) => {
                                    if rt != rhs.return_type {
                                        return ParseResult::Failed("Comparing different return types!".to_string())
                                    } else {
                                        self.node_count += 1;

                                        cmp = ParseResult::Success(Expression::new(
                                                self.node_count,
                                                ExpressionType::BinaryExpression(t.unwrap(), Box::new(lhs), Box::new(rhs)),
                                                rt));
                                    }
                                },
                                _ => return ParseResult::Failed("Failed comparison RHS".to_string())
                            }
                        },

                        Some(_) => return cmp
                    }
                },

                _ => {
                    println!("Failed comparison");
                    return cmp
                }
            }
        }
    }

    fn parse_equality(&mut self) -> ParseResult {
        let mut cmp = self.parse_comparison();

        loop {

            match cmp.clone() {
                ParseResult::Success(lr) => {

                let lhs = lr.clone();

                    let t = self.tokens.pop();
                    let rt = lhs.return_type.clone();

                    match t.clone() {

                        None => {
                            return ParseResult::Failed("Ran out of tokens".to_string())
                        },

                        Some(Token::NotEquality) | Some(Token::Equality) => {
                            let rcmp = self.parse_comparison();

                            match rcmp.clone() {

                                ParseResult::Success(rhs) => {
                                    if rt != rhs.return_type {
                                        return ParseResult::Failed("Comparing different return types!".to_string())
                                    } else {
                                        self.node_count += 1;

                                        cmp = ParseResult::Success(Expression::new(
                                                self.node_count,
                                                ExpressionType::BinaryExpression(t.unwrap(), Box::new(lhs), Box::new(rhs)),
                                                rt));
                                    }
                                },
                                _ => return ParseResult::Failed("Failed equality comparison".to_string())
                            }
                        },

                        Some(_) => return cmp
                    }
                },

                _ => {
                    println!("Failed equality");
                    return cmp
                }
            }
        }
        return cmp;
    }

    fn parse_assignment(&mut self) -> ParseResult {
        let lh = self.parse_equality();

        match lh.clone() {
            ParseResult::Success(expr_l) => {

            let popped = self.tokens.clone().pop();

                match popped {
                    None => return ParseResult::Failed("Out of tokens".to_string()),

                    Some(Token::Assign) => {
                        self.tokens.pop();
                        let res = self.parse_assignment();

                        match res {
                            ParseResult::Success(rh) => {
                                if rh.return_type == expr_l.return_type {
                                    match expr_l.clone().expression_type {
                                        ExpressionType::LiteralExpression(name, v) => {
                                            self.node_count += 1;

                                            return self.program.env.define(
                                                Variable::new(name.clone(),
                                                    Expression::new(
                                                        self.node_count,
                                                        ExpressionType::AssignmentExpression(name, Box::new(rh)),
                                                        expr_l.return_type
                                                    )
                                                )
                                            )
                                        },
                                        _ => return ParseResult::Failed("Invalid assignment target".to_string())
                                    }
                                } else {
                                    return ParseResult::Failed("Mismatched types".to_string())
                                }
                            },
                            _ => return ParseResult::Failed("Failed RHS of assignment".to_string())
                        }

                    },
                    Some(_) => return lh
                }
            },
            _ => {
                println!("Failed assignment");
                return lh
            }
        }
    }

    fn parse_function_header_statement(&mut self) -> ParseResult {
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

    fn parse_var_decl_statement(&mut self) -> ParseResult {

        match self.tokens.pop() {
            None => return ParseResult::Failed("Ran out of tokens".to_string()),

            Some(Token::Identifier(ident)) => {
                let mut name = String::new();
                name = ident;

                match self.program.env.get_value(name) {

                    ParseResult::Success(val) => {
                        match self.tokens.pop() {
                            None => return ParseResult::Failed("Ran out of tokens".to_string()),

                            Some(Token::Assign) => {
                                let expctd = val.return_type;
                                let res = self.parse_expression();
                                match res {
                                    ParseResult::Success(expr) => {
                                        if expr.return_type != expctd.clone() {
                                            return ParseResult::Failed("Invalid return type".to_string());
                                        }
                                        match self.tokens.pop().unwrap() {
                                            Token::Semicolon => {
                                                self.node_count += 1;
                                                return ParseResult::Success(Expression::new(self.node_count, ExpressionType::VarExpression(Box::new(expr)), expctd))
                                            }
                                            _ => return ParseResult::Failed("Expected ';'".to_string())
                                        }
                                    }
                                    _ => return res
                                }
                            },
                            Some(_) => return ParseResult::Failed("Expected '=' after return type".to_string())
                        }
                    },
                    _ => return ParseResult::Failed("Variable not found".to_string())
                }
            },
            Some(_) => return ParseResult::Failed("Expected identifier".to_string())
        }
    }

    fn parse_print_expression(&mut self) -> ParseResult {
        match self.tokens.pop().unwrap() {
            Token::StringLiteral(str) => {
                self.node_count += 1;
                return ParseResult::Success(
                    Expression::new(
                        self.node_count,
                        ExpressionType::PrintExpression(str),
                        ReturnType::ReturnString
                    )
                )
            },
            _ => return ParseResult::Failed("Expected string after 'print'".to_string())
        }
    }

    fn parse_declaration(&mut self) -> ParseResult {

        let cur_token = self.tokens.pop().unwrap();

        match cur_token {
            Token::VarDecl => {
                let stm = self.parse_var_decl_statement();

                match stm.clone() {
                    ParseResult::Success(s) => {
                        self.push_expression_statement(s);
                        return stm.clone()
                    },

                    ParseResult::Failed(f) => {
                        println!("Failed parsing var decl: {}", f);
                        return stm.clone()
                    }
                }
            },

            Token::FunctionDecl => {
                let stm = self.parse_function_header_statement();

                match stm.clone() {
                    ParseResult::Success(s) => {
                        self.push_expression_statement(s);
                        return stm.clone()
                    },

                    ParseResult::Failed(f) => {
                        println!("Failed parsing function decl: {}", f);
                        return stm.clone()
                    }
                }
            },

            _ => return self.parse_expression_statement()
        }
    }

    fn parse_expression_statement(&mut self) -> ParseResult {

        let cur_token = self.tokens.pop().unwrap();

        match cur_token {
            Token::Print => return self.parse_print_expression(),
            Token::LeftBrace => {
                let mut exs = vec!();

                loop {
                    let next = self.tokens.clone().pop();

                    match next {
                        None => return ParseResult::Failed("Ran out of tokens".to_string()),

                        Some(Token::RightBrace) =>  {
                            self.tokens.pop();
                            self.node_count += 1;
                            return ParseResult::Success
                            (
                                Expression::new
                                (
                                    self.node_count,
                                    ExpressionType::BlockExpression(exs),
                                    ReturnType::ReturnBlock
                                )
                            )
                        },
                        Some(Token::EOF) => return ParseResult::Failed("Unexpected EOF".to_string()),
                        Some(_) => {
                            let res = self.parse_declaration();
                            match res {
                                ParseResult::Success(ex) => {
                                    self.node_count += 1;
                                    exs.push(ex);
                                },
                                _ => return res
                            }
                        }
                    }
                }
            },
            _ => return self.parse_statement()
        }

        return ParseResult::Failed("lol".to_string())
    }

    pub fn parse_statement(&mut self) -> ParseResult {
        let res = self.parse_expression();
        match res.clone() {
            ParseResult::Success(s) => {
                if self.tokens.pop().unwrap() == Token::Semicolon {
                    return res
                }
                return ParseResult::Failed("Expected ';' after expression".to_string())
            },
            _ => return res
        }
    }

    fn parse_expression(&mut self) -> ParseResult {
        match self.tokens.clone().pop() {
            Some(Token::EOF) | None => return ParseResult::Failed("Unexpected EOF".to_string()),
            _ => return self.parse_assignment()
        }
    }

    pub fn push_expression_statement(&mut self, expr: Expression) {
        let stat = Statement::new(expr);
        self.push_statement(stat);
    }

    pub fn push_statement(&mut self, stat: Statement) {
        self.program.statements.push(stat);
    }

    pub fn parse(&mut self) -> AstProgram {

        loop {

            let cur_token = self.tokens.pop();

            match cur_token {

                None => break,

                Some(Token::EOF) => {
                    break
                }

                Some(Token::Identifier(id)) => {
                    match self.program.env.get_value(id) {
                        ParseResult::Success(e) => {
                            self.push_expression_statement(e);
                        },

                        ParseResult::Failed(f) => {
                            println!("Failed parsing: {}", f);
                            return self.program.clone()
                        }
                    }
                },

                Some(_) => {
                    let stm = self.parse_expression_statement();

                    match stm {
                        ParseResult::Success(s) => {
                            self.push_expression_statement(s);
                        },

                        ParseResult::Failed(f) => {
                            println!("Failed parsing: {}", f);
                            return self.program.clone()
                        }
                    }
                }
            };
        }

        return self.program.clone()
    }

}
