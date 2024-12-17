use super::token_types::*;

pub fn advance_tokens(basic_tokens: Vec<BasicToken>) -> Vec<AdvancedToken> {
    let mut advanced_tokens = Vec::new();
    let mut token_index = 0;
    while token_index < basic_tokens.len() {
        let mut token_char_index = 0;
        match &basic_tokens[token_index] {
            BasicToken::Operator(operator) => {
                let mut operator_chars = operator.chars();
                while let Some(nth_char) = operator_chars.nth(token_char_index) {
                    if nth_char == '[' {
                        advanced_tokens.push(AdvancedToken::OpeningSquareBracket);
                    } else if nth_char == ']' {
                        advanced_tokens.push(AdvancedToken::ClosingSquareBracket);
                    } else if nth_char == '{' {
                        advanced_tokens.push(AdvancedToken::OpeningBrace);
                    } else if nth_char == '}' {
                        advanced_tokens.push(AdvancedToken::ClosingBrace);
                    } else if nth_char == '<' {
                        if let Some(next_char) = operator_chars.nth(token_char_index) {
                            if next_char == '=' {
                                advanced_tokens.push(AdvancedToken::LessThanOrEqual);
                            }
                            token_char_index += 1;
                        } else {
                            advanced_tokens.push(AdvancedToken::LessThan);
                        }
                    } else if nth_char == '>' {
                        if let Some(next_char) = operator_chars.nth(token_char_index) {
                            if next_char == '=' {
                                advanced_tokens.push(AdvancedToken::GreaterThanOrEqual);
                            }
                            token_char_index += 1;
                        } else {
                            advanced_tokens.push(AdvancedToken::GreaterThan);
                        }
                    } else if nth_char == '-' {
                        if let Some(next_char) = operator_chars.nth(token_char_index) {
                            if next_char == '>' {
                                advanced_tokens.push(AdvancedToken::Then);
                            } else if next_char == '=' {
                                advanced_tokens.push(AdvancedToken::SubtractAndSet)
                            }
                            token_char_index += 1;
                        } else {
                            advanced_tokens.push(AdvancedToken::Subtract);
                        }
                    } else if nth_char == '+' {
                        if let Some(next_char) = operator_chars.nth(token_char_index) {
                            if next_char == '=' {
                                advanced_tokens.push(AdvancedToken::AddAndSet);
                            }
                            token_char_index += 1;
                        } else {
                            advanced_tokens.push(AdvancedToken::Add);
                        }
                    }
                    token_char_index += 1;
                }
            }

            BasicToken::Word(word) => {
                advanced_tokens.push(AdvancedToken::I8(-1));
            }

            BasicToken::Number(number) => {
                advanced_tokens.push(AdvancedToken::I8(-1));
            }
        }
        token_index += 1;
    }

    advanced_tokens
}
