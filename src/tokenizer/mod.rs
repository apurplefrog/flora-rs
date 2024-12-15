pub mod advance_tokens;
pub mod stringify_tokens;
pub mod token_types;

use super::tokenizer::token_types::BasicToken;

pub fn tokenize(chars: Vec<char>) -> Vec<BasicToken> {
    let strings = stringify_tokens::stringify_token_chars(chars);
    println!("[");
    for s in strings.iter() {
        println!("\t{:?}", s);
    }
    println!("]");
    strings
}
