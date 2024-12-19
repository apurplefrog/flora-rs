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
                    match nth_char {
                        ';' => (),
                        '(' => advanced_tokens.push(AdvancedToken::OpeningBracket),
                        ')' => advanced_tokens.push(AdvancedToken::ClosingBracket),
                        '[' => advanced_tokens.push(AdvancedToken::OpeningSquareBracket),
                        ']' => advanced_tokens.push(AdvancedToken::ClosingSquareBracket),
                        '{' => advanced_tokens.push(AdvancedToken::OpeningBrace),
                        '}' => advanced_tokens.push(AdvancedToken::ClosingBrace),
                        '<' => {
                            if let Some(next_char) = operator_chars.nth(token_char_index) {
                                if next_char == '=' {
                                    advanced_tokens.push(AdvancedToken::LessThanOrEqual);
                                }
                                token_char_index += 1;
                            } else {
                                advanced_tokens.push(AdvancedToken::LessThan);
                            }
                        }
                        '>' => {
                            if let Some(next_char) = operator_chars.nth(token_char_index) {
                                if next_char == '=' {
                                    advanced_tokens.push(AdvancedToken::GreaterThanOrEqual);
                                }
                                token_char_index += 1;
                            } else {
                                advanced_tokens.push(AdvancedToken::GreaterThan);
                            }
                        }
                        '-' => {
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
                        }
                        '+' => {
                            if let Some(next_char) = operator_chars.nth(token_char_index) {
                                if next_char == '=' {
                                    advanced_tokens.push(AdvancedToken::AddAndSet);
                                }
                                token_char_index += 1;
                            } else {
                                advanced_tokens.push(AdvancedToken::Add);
                            }
                        }
                        '.' => advanced_tokens.push(AdvancedToken::MemberOf),
                        '"' => {
                            let s = match &basic_tokens[token_index - 1] {
                                BasicToken::Word(word) => word,
                                BasicToken::Number(number) => number,
                                BasicToken::Operator(operator) => operator,
                            };
                            advanced_tokens.push(AdvancedToken::String(s.to_string()));
                        }
                        _ => panic!(),
                    }
                    token_char_index += 1;
                }
            }

            BasicToken::Word(word) => match word.as_str() {
                "include" => advanced_tokens.push(AdvancedToken::Include),
                "func" => advanced_tokens.push(AdvancedToken::Function),
                "if" => advanced_tokens.push(AdvancedToken::If),
                "else" => advanced_tokens.push(AdvancedToken::Else),
                "while" => advanced_tokens.push(AdvancedToken::While),
                "var" => advanced_tokens.push(AdvancedToken::Variable),
                "const" => advanced_tokens.push(AdvancedToken::Const),
                "ret" => advanced_tokens.push(AdvancedToken::Return),
                identifier => {
                    advanced_tokens.push(AdvancedToken::Identifier(identifier.to_string()))
                }
            },

            BasicToken::Number(number) => {
                advanced_tokens.push(AdvancedToken::I32(i32::from_str_radix(number, 10).unwrap()));
            }
        }
        token_index += 1;
    }

    advanced_tokens
}
