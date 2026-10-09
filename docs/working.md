# Compiler workings
## General overview
```mermaid
flowchart TD
A[/Source code/]
B[[Tokenization]]
H[[Match tree]]
C[[AST]]
G[Catch syntax errors]
D[[IR]]
E[[Optimise]]
F[\Machine code\]

AB[/EBNF grammar rules/]
BB[[Grammar parsing]]

A --> B
B --> H
H --> C
C --> D
C -.-> G
D --> E
E --Not Done--> D
E --Done--> F

AB --> BB
BB --> H
```

## Grammar parsing
```mermaid 
flowchart TD
lexing[Lex EBNF grammar file into tokens]
rule_gen[Generate rules from tokens]

lexing --> rule_gen
```

## Source code lexing
```mermaid
flowchart TD

```
