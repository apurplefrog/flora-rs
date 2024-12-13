#[derive(Debug)]
pub enum BasicToken {
    Word(String),
    Number(String),
    Operator(String),
}
