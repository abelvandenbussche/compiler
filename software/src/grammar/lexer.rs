use EBNFToken::*;
use std::str::FromStr;

use crate::parsing::role::Role;

/// The possible ebnf token types
#[derive(PartialEq, Eq, Debug)]
pub enum EBNFToken {
    /// An identifier that is an reference to another rule
    Identifier(String),
    /// Literal characters
    Literal(String),
    /// Role definitions
    Role(Role),
    /// [
    LBracket,
    /// ]
    RBracket,
    /// ]
    LBrace,
    /// {
    RBrace,
    /// (
    LParentheses,
    /// )
    RParenthese,
    /// =
    Equals,
    /// ,
    Comma,
    /// ;
    Semicolon,
    /// |
    Pipe,
}

/// Lexes the EBNF into a sequence of tokens
pub fn lex(ebnf: String) -> Vec<EBNFToken> {
    // Preprocessing
    const SPECIAL_CHARS: [char; 10] = ['[', ']', '(', ')', '{', '}', '=', ',', ';', '|'];
    let mut processed_ebnf = space_special_chars(&ebnf, &SPECIAL_CHARS);

    // Adding a space so that it always finishes the last token
    processed_ebnf.push(' ');

    // The tokens that have already been lexed
    let mut tokens = vec![];
    // The currently in progress string for quotes and references
    let mut in_progress = String::new();
    // Keeps track of whether we are quoting
    let mut quoting = false;
    // Keeps track of whether the next character should be escaped
    let mut escaped = false;
    for c in processed_ebnf.chars() {
        // When we are quoting we accept every character
        if quoting {
            if !escaped && c == '\\' {
                escaped = true;
                continue;
            }
            if c == '"' && !escaped {
                let stripped = remove_quotes(&in_progress);
                quoting = false;
                in_progress = String::new();
                tokens.push(Literal(stripped));
            } else {
                in_progress.push(c);
                escaped = false;
            }
            continue;
        }
        // Don't include whitespace characters except for spaces
        if c.is_whitespace() && c != ' ' {
            continue;
        }
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
            '|' => Some(Pipe),
            '"' => {
                quoting = true;
                None
            }
            ' ' => {
                if !in_progress.is_empty() {
                    // The end of an identifier
                    let done = if in_progress.starts_with('@') {
                        if let Ok(role) = Role::from_str(&in_progress) {
                            Role(role)
                        } else {
                            panic!("Invalid role name: {in_progress}");
                        }
                    } else {
                        Identifier(in_progress)
                    };
                    in_progress = String::new();
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
    let mut escaped = false;
    // Putting spaces in the string
    for c in original.chars() {
        if quoting {
            processed.push(c);
            if c == '\\' {
                escaped = true;
                continue;
            }
            if c == '"' && !escaped {
                quoting = false;
            }
            escaped = false;
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
        if !processed.is_empty()
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
    str.chars().nth(index)
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
    fn simple_ebnf_lexing() {
        assert_eq!(
            lex("foo = bar;".to_owned()),
            vec![
                Identifier("foo".to_owned()),
                Equals,
                Identifier("bar".to_owned()),
                Semicolon,
            ]
        );
        assert_eq!(
            lex("foo = \"Hello world\" | [bar];".to_owned()),
            vec![
                Identifier("foo".to_owned()),
                Equals,
                Literal("Hello world".to_owned()),
                Pipe,
                LBracket,
                Identifier("bar".to_owned()),
                RBracket,
                Semicolon,
            ]
        );
        assert_eq!(
            lex("foo = bar | \"something\", ([{}])".to_owned()),
            vec![
                Identifier("foo".to_owned()),
                Equals,
                Identifier("bar".to_owned()),
                Pipe,
                Literal("something".to_owned()),
                Comma,
                LParentheses,
                LBracket,
                LBrace,
                RBrace,
                RBracket,
                RParenthese
            ]
        );
    }
    #[test]
    fn identifier_lexing() {
        assert_eq!(
            lex("foo =".to_owned()),
            vec![Identifier("foo".to_owned()), Equals]
        );
        assert_eq!(
            lex("FoO =".to_owned()),
            vec![Identifier("FoO".to_owned()), Equals]
        );
        assert_eq!(
            lex("FOO =".to_owned()),
            vec![Identifier("FOO".to_owned()), Equals]
        );
    }
    #[test]
    fn role_lexing() {
        assert_eq!(
            lex("@var_ident = foo;".to_owned()),
            vec![
                Role(Role::VariableIdentifier),
                Equals,
                Identifier("foo".to_owned()),
                Semicolon,
            ]
        );
    }
    #[test]
    #[should_panic]
    fn invalid_role_lexing() {
        // This should panic because "@var_idant" is not an existing role name, thus this would be
        lex("@var_idant = foo;".to_owned());
    }
    #[test]
    fn ignore_whitespace() {
        assert_eq!(
            lex("    foo   =something|    \"everything\";".to_owned()),
            vec![
                Identifier("foo".to_owned()),
                Equals,
                Identifier("something".to_owned()),
                Pipe,
                Literal("everything".to_owned()),
                Semicolon,
            ]
        );
    }
    #[test]
    fn escape_test() {
        assert_eq!(
            lex(r#"foo = "\"";"#.to_owned()),
            vec![
                Identifier("foo".to_owned()),
                Equals,
                Literal("\"".to_owned()),
                Semicolon
            ]
        );
    }
    #[test]
    fn spacing_with_escaping() {
        let input_string = r#"foo = "\"", {bar};"#;
        let specials = vec!['{', '}'];
        assert_eq!(
            space_special_chars(input_string, &specials),
            r#"foo = "\"", { bar } ;"#.to_owned()
        );
    }
}
