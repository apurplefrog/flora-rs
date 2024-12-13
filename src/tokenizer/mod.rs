pub mod tokenizer;

pub fn tokenize(chars: Vec<char>) -> Vec<String> {
    crate::tokenizer::tokenizer::stringify_token_chars(chars)
}
