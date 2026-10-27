use crate::{
    grammar::{
        rules_to_map,
        types::{GrammarNode, Rule},
    },
    lexing::Token,
    parsing::{MatchNode, build_syntax_tree, tokens_match_rule},
};

#[test]
fn tokens_matching_rule() {
    let program_rule = Rule {
        head: "program".to_owned(),
        body: Some(GrammarNode::Sequence(vec![
            GrammarNode::Reference("FOO".to_owned()),
            GrammarNode::Reference("BAR".to_owned()),
        ])),
    };
    let input_tokens = vec![
        Token {
            head: "FOO".to_owned(),
            body: "Hello".to_owned(),
        },
        Token {
            head: "BAR".to_owned(),
            body: " world!".to_owned(),
        },
        Token {
            head: "FOOBAR".to_owned(),
            body: "This should not be included".to_owned(),
        },
    ];
    let rules_map = rules_to_map(vec![program_rule]);
    assert_eq!(
        tokens_match_rule(&input_tokens, &rules_map["program"], &rules_map),
        Some((
            vec![
                MatchNode::Token(input_tokens[0].clone()),
                MatchNode::Token(input_tokens[1].clone())
            ],
            2
        ))
    );
}

#[test]
fn token_matching() {
    let rules = vec![
        Rule {
            head: "program".to_owned(),
            body: Some(GrammarNode::Sequence(vec![
                GrammarNode::Choice(vec![
                    GrammarNode::Reference("FOO".to_owned()),
                    GrammarNode::Reference("BAR".to_owned()),
                ]),
                GrammarNode::ZeroOrMore(Box::new(GrammarNode::Reference("SPACE".to_owned()))),
                GrammarNode::Optional(Box::new(GrammarNode::Reference("BAR".to_owned()))),
            ])),
        },
        Rule {
            head: "FOO".to_owned(),
            body: Some(GrammarNode::Literal("Hello".to_owned())),
        },
        Rule {
            head: "BAR".to_owned(),
            body: Some(GrammarNode::Literal("world".to_owned())),
        },
        Rule {
            head: "SPACE".to_owned(),
            body: Some(GrammarNode::Literal(" ".to_owned())),
        },
    ];
    let tokens1 = vec![
        Token {
            head: "FOO".to_owned(),
            body: "hello".to_owned(),
        },
        Token {
            head: "SPACE".to_owned(),
            body: " ".to_owned(),
        },
        Token {
            head: "BAR".to_owned(),
            body: "world".to_owned(),
        },
    ];
    let tokens2 = vec![
        Token {
            head: "BAR".to_owned(),
            body: "world".to_owned(),
        },
        Token {
            head: "SPACE".to_owned(),
            body: " ".to_owned(),
        },
        Token {
            head: "SPACE".to_owned(),
            body: " ".to_owned(),
        },
        Token {
            head: "BAR".to_owned(),
            body: "world".to_owned(),
        },
    ];
    let rules_map = rules_to_map(rules);
    let result1 = build_syntax_tree(tokens1.clone(), rules_map.clone(), None);
    let result2 = build_syntax_tree(tokens2.clone(), rules_map, None);
    assert_eq!(
        result1,
        Ok(MatchNode::Rule {
            name: "program".to_owned(),
            children: vec![
                MatchNode::Token(tokens1[0].clone()),
                MatchNode::Token(tokens1[1].clone()),
                MatchNode::Token(tokens1[2].clone())
            ]
        })
    );
    assert_eq!(
        result2,
        Ok(MatchNode::Rule {
            name: "program".to_owned(),
            children: vec![
                MatchNode::Token(tokens2[0].clone()),
                MatchNode::Token(tokens2[1].clone()),
                MatchNode::Token(tokens2[2].clone()),
                MatchNode::Token(tokens2[3].clone())
            ]
        })
    );
}
#[test]
fn grouping_test() {
    let rules = vec![
        Rule {
            head: "program".to_owned(),
            body: Some(GrammarNode::Choice(vec![
                GrammarNode::Grouping(Box::new(GrammarNode::Sequence(vec![
                    GrammarNode::Reference("FOO".to_owned()),
                    GrammarNode::Reference("BAR".to_owned()),
                ]))),
                GrammarNode::Reference("BAR".to_owned()),
            ])),
        },
        Rule {
            head: "FOO".to_owned(),
            body: Some(GrammarNode::Literal("foo".to_owned())),
        },
        Rule {
            head: "BAR".to_owned(),
            body: Some(GrammarNode::Literal("bar".to_owned())),
        },
    ];
    let rule_map = rules_to_map(rules);
    let foo_token = Token {
        head: "FOO".to_owned(),
        body: "foo".to_owned(),
    };
    let bar_token = Token {
        head: "BAR".to_owned(),
        body: "bar".to_owned(),
    };
    // Valid
    let tokens1 = vec![foo_token.clone(), bar_token.clone()];
    // Valid
    let tokens2 = vec![bar_token.clone()];
    // Invalid
    let tokens3 = vec![foo_token.clone()];
    assert_eq!(
        build_syntax_tree(tokens1.clone(), rule_map.clone(), None),
        Ok(MatchNode::Rule {
            name: "program".to_owned(),
            children: tokens1
                .iter()
                .map(|i| MatchNode::Token(i.clone()))
                .collect()
        })
    );
    assert_eq!(
        build_syntax_tree(tokens2.clone(), rule_map.clone(), None),
        Ok(MatchNode::Rule {
            name: "program".to_owned(),
            children: tokens2
                .iter()
                .map(|i| MatchNode::Token(i.clone()))
                .collect()
        })
    );
    assert!(build_syntax_tree(tokens3, rule_map, None).is_err());
}
