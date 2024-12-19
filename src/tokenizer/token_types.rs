#[derive(Debug)]
pub enum BasicToken {
    Word(String),
    Number(String),
    Operator(String),
}

#[derive(Debug)]
pub enum AdvancedToken {
    // Core types
    // Integers
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    // Floating Point Numbers
    F32(f32),
    F64(f64),

    // Comparisons,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    IsEqual,
    SetEqual,

    // Operators,
    Add,
    AddAndSet,
    Subtract,
    SubtractAndSet,
    Multiply,
    MultiplyAndSet,
    Divide,
    DivideAndSet,
    Modulo,
    ModuloAndSet,
    Exponent,
    ExponentAndSet,

    MemberOf,
    String(String),

    NewLine,

    // Symbols,
    Then,

    // Brackets
    OpeningBrace,
    ClosingBrace,
    OpeningBracket,
    ClosingBracket,
    OpeningSquareBracket,
    ClosingSquareBracket,

    // Core words
    Function,
    Include,
    While,
    If,
    Else,
    Variable,
    Const,
    Return,

    Identifier(String),
}
