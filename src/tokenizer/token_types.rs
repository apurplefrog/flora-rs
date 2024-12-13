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

    // Operators,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    IsEqual,
    SetEqual,

    // Brackets
    OpeningBrace,
    ClosingBrace,
    OpeningBracket,
    ClosingBracket,
    OpeningSquarBracket,
    ClosingSquareBracket,

    // Core words
    Function,
    Include,
    While,
    If,
    Else,
}
