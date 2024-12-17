pub mod advance_tokens;
pub mod stringify_tokens;
pub mod token_types;

use advance_tokens::advance_tokens;

use token_types::AdvancedToken;
use token_types::BasicToken;

pub fn tokenize(chars: Vec<char>) -> Vec<AdvancedToken> {
    let strings = stringify_tokens::stringify_token_chars(chars);
    println!("[");
    for s in strings.iter() {
        println!("\t{:?}", s);
    }
    println!("]");

    let adv_strings = advance_tokens(strings);
    println!("[");
    for s in adv_strings.iter() {
        println!("\t{:?}", s);
    }
    println!("]");
    adv_strings
}
