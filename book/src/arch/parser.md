# Parser
 
 The role of the parser is to turn source-code which is fed as a string (in the form of files) into a tree-representation of that source-code.
 This tree is typically called the *Abstract Syntax Tree (AST)*.
 The step of parsing consists of two distinct stages. 
 The first one is the *lexical analysis (Lexer)* which is performed by a lexer.
 After lexing we perform the *syntactical analysis (Parser)* to construct the syntax tree.

```ignore
                                                                   ┌──┐
       ┌──────────────┐                                            │  │
       │              │                                            └──┘
       │  Source Code │                                            /  \
       │              │   ┌─────────┐        ┌──────────┐         /    \
       │  ──────────  │   │         │        │          │     ┌──┐      ┌──┐
       │              ├───►  Lexer  │        │  Parser  ├────►│  │      │  │
       │  ─────────   │   │         │        │          │     └──┘      └──┘
       │              │   └────┬────┘        └──────────┘      /\        /\
       │  ────        │        │                  ▲           /  \      /  \
       │              │        │                  │        ┌──┐ ┌──┐ ┌──┐ ┌──┐
       │  ────────    │        ▼                  │        │  │ │  │ │  │ │  │
       │              │   ┌───────────────────────┴──┐     └──┘ └──┘ └──┘ └──┘
       │              │   │                          │
       └──────────────┘   │  ┌───┐ ┌───┐ ┌───┐ ┌───┐ │       Abstract Syntax
                          │  │ T │ │ T │ │ T │ │...│ │            Tree
                          │  └───┘ └───┘ └───┘ └───┘ │
                          │                          │
                          └──────────────────────────┘
                                 Token-Stream
```

## Lexer

The lexer performs the lexical analysis.
This step turns the source-string into a sequence of well known tokens.
The Lexer (or sometimes also called *tokenizer*) splits the source-string into *tokens* (or *words*).
Each token has a distinct type which corresponds to a grammar's element.
Typical token-types are keywords, numbers, identifiers, brackets, dots, etc.
So with the help of this token-stream it is much easier for the parser to spot certain patterns.
E.g. a floating-point number consists of the token-sequence: *number*, *dot*, *number*.

The lexer is implemented in the `lexer`-module.
It uses the [logos](https://github.com/maciejhirsz/logos) crate to create a lexer that is able to identify all different terminal-symbols.
Compared to other languages, Structured Text has a quite high number of keywords and other tokens, so RuSTy's lexer identifies a quite large number of different tokens.

### Discussion: RuSTy-Lexer

The logos crate uses [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html) to generate the code required to  lex the source-string.
The number of tokens identified by the RuSTy-lexer is quite high, so as of january 2022 the rust sdk for vs-code (rust-analyzer) reports problem with the number of macro-generated tokens (*macro invocation exceeds token limit...*).

The tokens identified by the lexer follow the formal definition provided by the IEC61131-3 (2013) standard.

Following strategies increase the number of tokens and should be reconsidered :
- case insensitivity
- optional underscores in keywords (e.g. `END_IF` == `ENDIF`)
- unrolled tokens instead of grouping tokens (e.g. `KEYWORD_TRUE` & `KEYWORD_FALSE` instead of `KEYWORD_BOOL`)
- etc.

## Parser

The parser takes the token stream and creates the corresponding AST that represents the source code in a structured, hierarchical way.
The parser is implemented in the `parser` module whereas the model for the AST is implemented in the `ast` module.

### AST - Abstract Syntax Tree

The abstract syntax tree is a tree representation of the source code.
Some parser implementations use a generic tree-data-structure consisting of `Nodes` which can have an arbitrary number of children.
These nodes usually have dynamic properties like a type and an optional value and sometimes they even have dynamic properties stored in a map to make this representation even more flexible.

While this approach needs very little source code we decided to favour a less flexible approach.
The RuSTy-AST models every single ast-node as its own *struct* with all necessary fields including the possible child-nodes.
While this approach needs much more code and hand-written changes, its benefits lie in the clearness and simplicity of the data-structure.
Every element of the AST is easily identified, debugged and understood. 
E.g. while in a generic node based AST it is easily possible to have a binary-statement with no, one, or seven child-nodes, the RuSTy-AST enforces the structure of every node. So the RuSTy-Binary-Statement has exactly two children.
It is impossible to construct it differently.

#### Example

So an assignment `a := 3;` will be parsed with the help of the following Structures :
```rs
struct Reference {
   name: string
}

struct LiteralInteger {
   value: i128
}

struct Assignment {
   left: Box<AstStatement>,
   right: Box<AstStatement>
}
``` 

### Recursive Descent Parser

There are a lot of different frameworks to generate parsers from formal grammars.
While they generate highly optimized parsers we felt we wanted more control and more understanding of the parsing process and the resulting AST.
The fact that at that point in time we were pretty new to rust itself, writing the parser by hand also gave us more practice and a stronger feeling of control and understanding.
Using a parser-generator framework will definitely be an option for future improvements.

As for now, the parser is a hand-written [recursive descent parser](https://en.wikipedia.org/wiki/Recursive_descent_parser) inside the `parser`-module. 

As the parser reads the token stream `Reference`, `KeywordEquals`, `Number`, `Semicolon` it instantiates the corresponding syntax tree: 
```ignore
                      ┌─────────────────┐
                      │   Assignment    │
                      └──────┬──┬───────┘
                   left      │  │     right 
                 ┌───────────┘  └──────────┐
                 ▼                         ▼
        ┌──────────────────┐     ┌──────────────────┐
        │    Reference     │     │  LiteralInteger  │
        ├──────────────────┤     ├──────────────────┤
        │    name: 'a'     │     │    value: '3'    │
        └──────────────────┘     └──────────────────┘
```