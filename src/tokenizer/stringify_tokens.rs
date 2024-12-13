use super::token_types::BasicToken;

pub fn stringify_token_chars(token_chars: Vec<char>) -> Vec<BasicToken> {
    let mut tokens: Vec<BasicToken> = Vec::new();
    let mut char_index = 0;

    while char_index < token_chars.len() {
        let mut token = String::new();

        if token_chars[char_index].is_alphabetic() {
            while token_chars[char_index].is_alphanumeric() {
                token += &token_chars[char_index].to_string();
                char_index += 1;
            }
            tokens.push(BasicToken::Word(token));
        } else if token_chars[char_index].is_numeric() {
            while token_chars[char_index].is_alphanumeric() || token_chars[char_index] == '.' {
                token += &token_chars[char_index].to_string();
                char_index += 1;
            }
            tokens.push(BasicToken::Number(token));
        } else if token_chars[char_index].is_ascii_punctuation() {
            while token_chars[char_index].is_ascii_punctuation() {
                token += &token_chars[char_index].to_string();
                char_index += 1;
            }
            tokens.push(BasicToken::Operator(token));
        } else {
            char_index += 1;
        }
    }

    tokens
}
