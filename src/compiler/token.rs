#[derive(Debug, PartialEq, Clone)]
pub enum Token {

    // Helpers
    Illegal,
    EOF,

    // Operators
    Assign,

    Add,
    Subtract,
    Multiply,
    Divide,

    Or,
    And,

    LogicalOr,
    LogicalAnd,

    ShiftLeft,
    ShiftRight,

    Xor,

    Modulo,

    // Logic
    LessThan,
    GreaterThan,

    LessThanEqual,
    GreaterThanEqual,

    Bang,

    Equality,
    NotEquality,

    // Delimiters
    Dot,
    Comma,

    LeftParenthesis,
    RightParenthesis,

    LeftBrace,
    RightBrace,

    Semicolon,

    Colon,

    Quote,

    // Control flow
    If,
    Else,

    Return,

    Match,

    For,
    While,

    Comment,

    // Object

    Super,

    Struct,

    Function,

    VarDecl,
    ConstDecl,

    Identifier(String),

    DotDot,

    RangeLiteral,

    StringLiteral(String),
    IntegerLiteral(i32),
    FloatLiteral(f64),
    BooleanLiteral(bool),

    CollectionLiteral,

    StringDecl,
    IntegerDecl,
    FloatDecl,
    BooleanDecl,

    CollectionDecl,

    StructDecl,

    FunctionDecl,

    VoidDecl,

    Null,

    Error(String),

    Print,

}

impl Default for Token {
    fn default() -> Token {
        return Token::Illegal
    }
}

pub fn lookup(ident: &str) -> Token {
    match ident {
        "fn" => Token::FunctionDecl,
        "const" => Token::ConstDecl,
        "var" => Token::VarDecl,
        "match" => Token::Match,
        "return" => Token::Return,
        "super" => Token::Super,
        "if" => Token::If,
        "else" => Token::Else,
        "null" => Token::Null,
        "for" => Token::For,
        "while" => Token::While,
        "true" => Token::BooleanLiteral(true),
        "false" => Token::BooleanLiteral(false),
        "void" => Token::VoidDecl,
        "collection" => Token::CollectionDecl,
        "int" => Token::IntegerDecl,
        "float" => Token::FloatDecl,
        "string" => Token::StringDecl,
        "bool" => Token::BooleanDecl,
        "struct" => Token::StructDecl,
        "print" => Token::Print,

        _ => Token::Identifier(ident.to_string()),
    }
}

#[test]
fn test_lookup() {
    assert_eq!(lookup("fn"), Token::FunctionDecl);
}
