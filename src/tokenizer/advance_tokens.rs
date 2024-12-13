use super::token_types::*;

pub fn advance_tokens(basic_tokens: Vec<BasicToken>) {
    let mut advanced_tokens = Vec::new();
    for basic_token in basic_tokens.iter() {
        advanced_tokens.push(match basic_token {
            BasicToken::Word(_) => advance_word_token(basic_token),
            BasicToken::Number(_) => advance_number_token(basic_token),
            BasicToken::Operator(_) => advance_operator_token(basic_token),
        });
    }
}

fn advance_word_token(word_token: &BasicToken) -> AdvancedToken {
    AdvancedToken::Function
}
fn advance_number_token(number_token: &BasicToken) -> AdvancedToken {
    AdvancedToken::F32(10.0f32)
}
fn advance_operator_token(operator_token: &BasicToken) -> AdvancedToken {
    AdvancedToken::OpeningBrace
}
