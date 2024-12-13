pub mod stringify_tokens;

pub fn tokenize(chars: Vec<char>) -> Vec<stringify_tokens::BasicToken> {
    stringify_tokens::stringify_token_chars(chars)
}
