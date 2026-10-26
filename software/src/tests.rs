use crate::{
    grammar::{RuleSet, get_rules, rules_to_map},
    lexing::{Lexer, Token},
    parsing::{MatchNode, build_syntax_tree},
};

#[test]
fn full_parsing1() {
    let ebnf = r#"DIGIT = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0";
LETTER = "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z";


EQUALS = "=";
UNDERSCORE = "_";
SEMICOLON = ";";
QUOTATION_MARK = "\"";
SPACE = " ";

valid_char = LETTER | UNDERSCORE;
identifier = valid_char, {valid_char};
variable_decl = identifier, SPACE, EQUALS, SPACE, QUOTATION_MARK, {valid_char | SPACE}, QUOTATION_MARK; 

program = {variable_decl};
"#;
    let source_code = r#"var = "test""#;

    // Grammar lexing
    let RuleSet {
        lexing: lexing_rules,
        parsing: parsing_rules,
        ..
    } = get_rules(ebnf.to_owned());

    // Source code lexing
    let mut lexer = Lexer::new(source_code.to_owned(), lexing_rules);
    let mut tokens = vec![];
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    // Source code parsing
    let output = build_syntax_tree(tokens, rules_to_map(parsing_rules), None);
    assert_eq!(
        output,
        Ok(MatchNode::Rule {
            name: "program".to_owned(),
            children: vec![MatchNode::Rule {
                name: "variable_decl".to_owned(),
                children: vec![
                    MatchNode::Rule {
                        name: "identifier".to_owned(),
                        children: vec![
                            MatchNode::Rule {
                                name: "valid_char".to_owned(),
                                children: vec![MatchNode::Token(Token {
                                    head: "LETTER".to_owned(),
                                    body: "v".to_owned()
                                })]
                            },
                            MatchNode::Rule {
                                name: "valid_char".to_owned(),
                                children: vec![MatchNode::Token(Token {
                                    head: "LETTER".to_owned(),
                                    body: "a".to_owned()
                                })]
                            }
                        ]
                    },
                    MatchNode::Token(Token {
                        head: "SPACE".to_owned(),
                        body: " ".to_owned()
                    }),
                    MatchNode::Token(Token {
                        head: "EQUALS".to_owned(),
                        body: "=".to_owned()
                    }),
                    MatchNode::Token(Token {
                        head: "SPACE".to_owned(),
                        body: " ".to_owned()
                    }),
                    MatchNode::Token(Token {
                        head: "QUOTATION_MARK".to_owned(),
                        body: "\"".to_owned()
                    }),
                    MatchNode::Rule {
                        name: "valid_char".to_owned(),
                        children: vec![MatchNode::Token(Token {
                            head: "LETTER".to_owned(),
                            body: "t".to_owned()
                        })]
                    },
                    MatchNode::Rule {
                        name: "valid_char".to_owned(),
                        children: vec![MatchNode::Token(Token {
                            head: "LETTER".to_owned(),
                            body: "e".to_owned()
                        })]
                    },
                    MatchNode::Rule {
                        name: "valid_char".to_owned(),
                        children: vec![MatchNode::Token(Token {
                            head: "LETTER".to_owned(),
                            body: "s".to_owned()
                        })]
                    },
                    MatchNode::Rule {
                        name: "valid_char".to_owned(),
                        children: vec![MatchNode::Token(Token {
                            head: "LETTER".to_owned(),
                            body: "t".to_owned()
                        })]
                    },
                    MatchNode::Token(Token {
                        head: "QUOTATION_MARK".to_owned(),
                        body: "\"".to_owned()
                    }),
                ]
            }]
        })
    );
}
