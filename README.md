# Compiler

A final project for the sixth year of [PTI](https://www.pti.be/)\
It's a compiler that allows the user to define the syntax using EBNF syntax. This ofcourse is not that useful but it is very interesting,

## General workings
```mermaid
flowchart TD
A[/Source code/]
B[Tokens]
C[Grammar tree]
D[Abstract syntax tree]
E[IR]
F[ASM]

G[EBNF]

A --> B
B --> C
G --> C
C --> D
D --> E
E --> F
```

## General idea
```mermaid
mindmap
root((compiler))
  Procedural syntax
    EBNF
      parsing rules
      lexing rules
      assigned roles
    Predefined components
  Features
    Basic
      Variables
      Control flow
      Functions
    Advanced
      Classes / Structs
      Memory management
```

# How to use
## Install
Cloning the repository
```bash
git clone https://github.com/abelvandenbussche/compiler
cd compiler/software
```
In the [software](software/) directory:\
Build
```bash
cargo build
```
Run tests
```bash
cargo test
```
