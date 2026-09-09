use EBNFTokens::*;
const EBNF: &str = include_str!("../../../docs/grammar.ebnf");

#[derive(PartialEq, Eq, Debug)]
pub enum EBNFTokens {
    Identifier(String),
    Literal(String),
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
    // Preprocessing
    const SPECIAL_CHARS: [char; 9] = ['[', ']', '(', ')', '{', '}', '=', ',', ';'];
    let processed_ebnf = space_special_chars(EBNF, &SPECIAL_CHARS);

    println!("processed: {processed_ebnf}");

    // Parsing
    let mut tokens = vec![];
    let mut in_progress = String::new();
    let mut quoting = false;
    for c in processed_ebnf.chars() {
        // When we are quoting we accept every character
        if quoting {
            if c == '"' {
                let stripped = remove_quotes(&in_progress);
                quoting = false;
                println!("End quote");
                in_progress = String::new();
                tokens.push(Literal(stripped));
            } else {
                in_progress.push(c);
            }
            continue;
        }
        println!("char: {c}");
        let result = match c {
            '[' => Some(LBracket),
            ']' => Some(RBracket),
            '{' => Some(LBrace),
            '}' => Some(RBrace),
            '(' => Some(LParentheses),
            ')' => Some(RParenthese),
            '=' => Some(Equals),
            ';' => Some(Semicolon),
            ',' => Some(Comma),
            '"' => {
                quoting = true;
                None
            }
            ' ' => {
                if in_progress.len() > 0 {
                    // The end of a identifier
                    let done = Identifier(in_progress);
                    in_progress = String::new();
                    println!("Identifier finished");
                    Some(done)
                } else {
                    None
                }
            }
            _ => {
                in_progress.push(c);
                None
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
    if to_strip.starts_with('\"') && to_strip.ends_with('\"') && to_strip.len() > 1 {
        let stripped: String = to_strip.chars().skip(1).take(to_strip.len() - 2).collect();
        stripped
    } else {
        to_strip.to_owned()
    }
}

/// Adds spaces to both sides of specified characters
/// Does not add spaces to the beginning or end
/// Keeps everything in quotes unchanged
fn space_special_chars(original: &str, specials: &[char]) -> String {
    let mut processed = String::new();
    let mut quoting = false;
    // Putting spaces in the string
    for c in original.chars() {
        println!("char: {c}");
        if quoting {
            processed.push(c);
            if c == '"' {
                quoting = false;
            }
            continue;
        } else if c == '"' {
            processed.push(c);
            quoting = true;
            continue;
        }
        if !specials.contains(&c) {
            processed.push(c);
            continue;
        }
        if processed.len() > 0
            && let Some(prev) = index_string(&processed, processed.len() - 1)
            && prev != ' '
        {
            processed.push(' ');
        }
        processed.push(c);
        processed.push(' ');
    }

    processed
}
fn index_string(str: &str, index: usize) -> Option<char> {
    str.chars().skip(index).next()
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn index_a_string() {
        assert_eq!(index_string("Hello world", 2), Some('l'));
        assert_eq!(index_string("Hello world", 0), Some('H'));
        assert_eq!(index_string("Hello world", 5), Some(' '));
    }
    #[test]
    fn fail_index_string() {
        assert_eq!(index_string("Hello world", 20), None);
        assert_eq!(index_string("Hello world", 11), None);
    }

    #[test]
    fn add_space_around_specials() {
        let special_chars = &['h', 'o'];
        assert_eq!(
            space_special_chars("hello world", special_chars),
            "h ell o  w o rld"
        );
        assert_eq!(
            space_special_chars("hello \"o\" world", special_chars),
            "h ell o  \"o\" w o rld"
        );
    }

    #[test]
    fn tokenize_first_line() {
        let expected = vec![
            Identifier("variable_declaration".to_owned()),
            Equals,
            Literal("let".to_owned()),
            Comma,
            Identifier("variable_identifier".to_owned()),
            Comma,
            LBracket,
            Literal(":".to_owned()),
            Comma,
            Identifier("type".to_owned()),
            RBracket,
            Comma,
            Literal("=".to_owned()),
            Comma,
            Identifier("value".to_owned()),
        ];
        let output: Vec<EBNFTokens> = lex()
            .into_iter()
            .take_while(|token| token != &Semicolon)
            .collect();
        assert_eq!(output, expected);
    }
}
