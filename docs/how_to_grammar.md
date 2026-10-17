# How to use EBNF to describe language grammar
EBNF uses a simple syntax to describe the rules of grammar.
For this we use something called a <a id="rule-definition"></a>`Rule`
# How to define a rule
You define a [Rule](<how_to_grammar#How to define a rule>) by giving it a name and setting it equal to a certain pattern.\
Example:
```EBNF
DIGIT = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0";
NUMBER = DIGIT, {DIGIT};
```
## Syntax
* `{}` - One or more of contained value
* `[]` - Zero or one of contained value
* `a, b, c, ...` - Values in a sequence
* `a | b | c | ...` - One of the given values
* `"foo"` - Literal characters eg. "if" for an if-statement
* `foo` - Reference to another [Rule](<how_to_grammar#How to define a rule>)

## Differentiating between lexing and parsing rules
To differentiate between rules that describe basic things like:
numbers, strings, keywords, ...
And rules that describe things like:
Variable declarations, if statements, ...
We do this by giving lexical [Rules](<how_to_grammar#How to define a rule>) names in UPPER_SNAKE_CASE and rules for parsing names in lower_snake_case.

[Rules](<how_to_grammar#How to define a rule>) for parsing can contain references to [Rules](<how_to_grammar#How to define a rule>) for lexing but a [Rule](<how_to_grammar#How to define a rule>) for lexing can't contain references to ´rules` for parsing.
