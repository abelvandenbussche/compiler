# Flowchart
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
