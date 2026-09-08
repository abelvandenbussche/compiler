const EBNF: &str = include_str!("../../../docs/grammar.ebnf");
pub enum EBNFTokens {
    Identifier(String),
    LBracket,     // [
    RBracket,     // ]
    LBrace,       // {
    RBrace,       // }
    LParentheses, // (
    RParenthese,  // )
    Equals,       // =
    Comma,        // ,
    Semicolon,    // ;
}

/// Lexes the EBNF into a sequence of tokens
pub fn lex() -> Vec<EBNFTokens> {
    let mut tokens = vec![];
    let mut in_progress = String::new();
    for c in EBNF.chars() {
        in_progress.push(c);
        // Checking if it means anything yet
        let result = match in_progress.as_str() {
            "[" => Some(EBNFTokens::LBracket),
            "]" => Some(EBNFTokens::RBracket),
            "{" => Some(EBNFTokens::LBrace),
            "}" => Some(EBNFTokens::RBrace),
            "(" => Some(EBNFTokens::LParentheses),
            ")" => Some(EBNFTokens::RParenthese),
            "=" => Some(EBNFTokens::Equals),
            ";" => Some(EBNFTokens::Semicolon),
            "," => Some(EBNFTokens::Comma),
            _ => {
                let stripped = remove_quotes(&in_progress);
                // We got an identifier on our hands
                if c == ' ' || stripped != in_progress {
                    Some(EBNFTokens::Identifier(stripped))
                } else {
                    None
                }
            }
        };
        if let Some(token) = result {
            tokens.push(token);
        }
    }
    tokens
}

/// Removes the quotation marks round a string if they are there otherwise returns the input
/// untouched
fn remove_quotes(to_strip: &str) -> String {
    if to_strip.starts_with('\"') && to_strip.ends_with('\"') {
        let stripped: String = to_strip.chars().skip(1).take(to_strip.len() - 2).collect();
        stripped
    } else {
        to_strip.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::grammar::lexer::remove_quotes;

    #[test]
    fn strip_quotes() {
        let input = "\"Hello world\"";
        assert_eq!(remove_quotes(input), "Hello world");
    }
    #[test]
    fn do_not_strip_quotes() {
        let input = "Hello world";
        assert_eq!(remove_quotes(input), input);
    }
    #[test]
    fn strip_1_set_of_quotes() {
        let input = "\"\"Hello world\"\"";
        assert_eq!(remove_quotes(input), "\"Hello world\"");
    }
}
