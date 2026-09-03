# Resolver

After indexing, every declaration is known, but the statement bodies are still trees of names. In

```iecst
PROGRAM main
    VAR
        i : DINT;
        sintVar : SINT;
    END_VAR

    sintVar := i + 1;
END_PROGRAM
```

the parser produced an assignment whose left side is a name `sintVar` and whose right side is a name `i` plus a literal `1`. Nothing says that `sintVar` is the local variable `main.sintVar`, that `i + 1` is a `DINT` expression, or that storing it into a `SINT` needs a narrowing conversion. The resolver answers these questions for every expression in the project. The driver calls this stage "annotate", and the name is exact: the resolver does not change the tree. It writes its answers into a side table, the annotation map, keyed by the id the parser gave each node. Validation reads that table to report type errors, and codegen reads it to pick instructions and insert conversions.

```mermaid
flowchart LR
    parse[Parse] --> index[Index] --> annotate[Annotate] --> validate[Validate] --> codegen[Codegen] --> link[Link]
    style annotate fill:#bfdbfe,stroke:#000,stroke-width:1px,stroke-dasharray:4 3,color:#0f172a
```

The stage receives the units and the global index. Each unit is annotated concurrently by its own visitor, which returns an annotation map, the set of names the unit depends on, and the string literals it contains. The maps are merged into one, and the few types the visitors had to create are imported into the index. The result is the annotated project that validation and codegen work on. Like the index, the map is never updated in place: whenever a participant rewrites the tree, the stage runs again and replaces it.


## Annotations and hints

For every expression the resolver records two things. The *annotation* says what the expression is: which declaration it refers to, or which type its value has. The *type hint* says what the expression should become: the type the surrounding statement expects. For `sintVar := i + 1` the literal `1` and the sum `i + 1` are annotated as `DINT`, because integer literals are `DINT` (unless they need 64 bits), and the sum receives the hint `SINT`, because that is what the left side stores. Codegen sees the hint and emits a truncation; validation compares both and warns about the implicit downcast.

Annotations come in a few kinds: a reference to a variable, a reference to a function, a reference to a type, program, or function block, a plain value such as `i + 1` or a literal that has only a resulting type, and a call argument that records which parameter it was matched to. What each kind stores is best read in `src/resolver.rs`; the [Internals](../internals/README.md) chapters show it per language construct.

Trimmed to its fields, the annotation map looks like this:

```rust
pub struct AnnotationMapImpl {
    /// What each expression is, keyed by node id
    type_map: FxIndexMap<AstId, StatementAnnotation>,

    /// What each expression should become, keyed by node id
    type_hint_map: FxIndexMap<AstId, StatementAnnotation>,

    /// Range-check calls that codegen emits in place of an assigned value
    hidden_function_calls: FxIndexMap<AstId, AstNode>,

    /// Types created while annotating, such as sized string literal types
    pub new_index: Index,

    // ... other omitted fields
}
```

The map is the only output of the stage. An expression the resolver could not resolve has no entry; the resolver itself reports nothing, and validation turns the missing entry into an "unresolved reference" diagnostic.


## Walking a unit

The resolver walks each unit from top to bottom and visits every expression: the initializers in variable blocks, the bounds in type declarations, and the statements of the bodies. Each expression is visited with the POU it sits in as its context. For

```iecst
FUNCTION_BLOCK Buffer
    VAR_INPUT
        limit : INT;
    END_VAR
END_FUNCTION_BLOCK

PROGRAM main
    VAR
        bufferInstance : Buffer;
        i : DINT := 1;
    END_VAR

    i := bufferInstance.limit;
END_PROGRAM
```

the resolver walks declarations first and bodies second: the variable blocks of every POU, then the body of every POU. Only expressions get entries, so `Buffer`, whose one variable has no initializer and whose body is empty, ends without a single entry.

In `main`, only `i` has an initializer, `1`: an integer literal is a `DINT` value, and because it initializes a `DINT` variable it is also hinted `DINT`. The body is one assignment, which the resolver visits value first, target second, and hints last. The value `bufferInstance.limit` is resolved left to right: `bufferInstance` is found as a member of `main`, the current POU, and its type `Buffer` becomes the qualifier under which `limit` is found as `Buffer.limit` of type `INT`. The whole reference takes the entry of its last part. The target `i` is found as `main.i` of type `DINT` and gets no hint. Last, the value is hinted with the target's type `DINT`, a widening that codegen emits later.

Visualized:

```
PROGRAM main
    VAR
        bufferInstance : Buffer;
        i : DINT := 1;
                    ^          { kind: Value, resulting_type: "DINT", hint: "DINT" }
    END_VAR

    i := bufferInstance.limit;
    ^                          { kind: Variable, qualified_name: "main.i",              resulting_type: "DINT",   hint: None }
         ^^^^^^^^^^^^^^        { kind: Variable, qualified_name: "main.bufferInstance", resulting_type: "Buffer", hint: None }
                        ^^^^^  { kind: Variable, qualified_name: "Buffer.limit",        resulting_type: "INT",    hint: None }
         ^^^^^^^^^^^^^^^^^^^^  { kind: Variable, qualified_name: "Buffer.limit",        resulting_type: "INT",    hint: "DINT" }
END_PROGRAM
```

This example has no name clashes, but imagine a global variable that is also named `bufferInstance`. The resolver tries a fixed order of lookups and takes the first that succeeds: a member of the current POU, then a global variable or enum variant, then a POU (program, function, or function block) of that name, then a type. The local member wins. The one exception is the operator of a call, which tries functions first: inside a function `scale`, the name `scale` is the return variable, but `scale(...)` is the function.


## Promotion

Arithmetic and comparisons combine operands of different types. The resolver decides which type the operation runs in and marks the operands that have to be converted. In

```iecst
PROGRAM main
    VAR
        sintVar : SINT;
        dintVar : DINT;
        boolVar : BOOL;
    END_VAR

    sintVar := dintVar + 1;
    boolVar := sintVar < dintVar;
END_PROGRAM
```

the first statement is an assignment, so its value `dintVar + 1` is visited first. The sum visits both operands: `dintVar` is annotated as `main.dintVar` of type `DINT`, and the literal `1` is a `DINT` value. The type of the sum is the bigger of the two operand types, and never smaller than `DINT`. Both operands already are `DINT`, so nothing is converted and `dintVar + 1` is a `DINT` value. Then the target: `sintVar` is `main.sintVar` of type `SINT`. Last, the value is hinted with the target's type: `dintVar + 1` gets the hint `SINT`. The hint sits on the sum as a whole, not on its operands; the addition runs in `DINT` and only the result is narrowed.

The second statement compares `sintVar`, a `SINT`, with `dintVar`, a `DINT`. The bigger type is `DINT`, so `sintVar` gets the hint `DINT`: it is widened before the comparison. A comparison always produces a `BOOL`, so `sintVar < dintVar` is a `BOOL` value. The target `boolVar` is `main.boolVar` of type `BOOL`, and the comparison gets the hint `BOOL`, which changes nothing here.

Visualized:

```
    sintVar := dintVar + 1;
    ^^^^^^^                             { kind: Variable, qualified_name: "main.sintVar", resulting_type: "SINT", hint: None }
               ^^^^^^^^^^^              { kind: Value,                                    resulting_type: "DINT", hint: "SINT" }
               ^^^^^^^                  { kind: Variable, qualified_name: "main.dintVar", resulting_type: "DINT", hint: None }
                         ^              { kind: Value,                                    resulting_type: "DINT", hint: None }

    boolVar := sintVar < dintVar;
    ^^^^^^^                             { kind: Variable, qualified_name: "main.boolVar", resulting_type: "BOOL", hint: None }
               ^^^^^^^^^^^^^^^^^        { kind: Value,                                    resulting_type: "BOOL", hint: "BOOL" }
               ^^^^^^^                  { kind: Variable, qualified_name: "main.sintVar", resulting_type: "SINT", hint: "DINT" }
                         ^^^^^^^        { kind: Variable, qualified_name: "main.dintVar", resulting_type: "DINT", hint: None }
```


## Calls

In

```iecst
FUNCTION scale : DINT
    VAR_INPUT
        value : DINT;
        factor : INT;
    END_VAR
END_FUNCTION

PROGRAM main
    VAR
        i : DINT;
        sintVar : SINT;
    END_VAR

    i := scale(i, factor := sintVar);
END_PROGRAM
```

the statement is an assignment, so the call is visited first. A call has an operator and an argument list. The operator `scale` is looked up with functions first and annotated as the function `scale` returning `DINT`. The arguments are visited next, with `scale` remembered as the callee. `i` resolves as usual to `main.i` of type `DINT`. `factor := sintVar` is a named argument, itself an assignment: its value `sintVar` resolves to `main.sintVar` of type `SINT`, but its target `factor` is looked up as a member of the callee, not of `main`, and resolves to the input `scale.factor` of type `INT`; `sintVar` gets the hint `INT`.

Then the arguments are matched to the parameters of `scale`. Positional arguments take the parameters in declaration order, named arguments take the parameter with their name. `i` is matched to parameter 0 and gets the hint `DINT`, `factor := sintVar` is matched to parameter 1 and gets the hint `INT`. These hints also record the parameter position, which codegen uses to place the values. The call as a whole is annotated with the return type of the function, a `DINT` value. The rest is the ordinary assignment: the target `i` is `main.i`, and the call gets the hint `DINT`.

Visualized:

```
    i := scale(i, factor := sintVar);
    ^                                 { kind: Variable, qualified_name: "main.i",       resulting_type: "DINT", hint: None }
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^  { kind: Value,                                    resulting_type: "DINT", hint: "DINT" }
         ^^^^^                        { kind: Function, qualified_name: "scale",        return_type: "DINT",    hint: None }
               ^                      { kind: Variable, qualified_name: "main.i",       resulting_type: "DINT", hint: Argument { resulting_type: "DINT", position: 0 } }
                  ^^^^^^^^^^^^^^^^^   { kind: None,                                                             hint: Argument { resulting_type: "INT", position: 1 } }
                  ^^^^^^              { kind: Variable, qualified_name: "scale.factor", resulting_type: "INT",  hint: None }
                            ^^^^^^^   { kind: Variable, qualified_name: "main.sintVar", resulting_type: "SINT", hint: "INT" }
```

The named argument `factor := sintVar` has no annotation of its own, only the hint that ties it to parameter 1; its two sides are annotated like any assignment.

A call on a function block instance follows the same path, except that the operator resolves to a variable whose type is the function block; the arguments are matched against that function block, and the call has no result type. Built-in functions such as `ADR`, `REF`, `SIZEOF`, and `MUX` carry their own annotation logic, because their result type depends on the argument.


## Literals and generated types

Literals are typed by their value: an integer is `DINT` if it fits 32 bits and `LINT` otherwise, a real is `REAL` if it fits 32 bits and `LREAL` otherwise, and a typed literal such as `INT#5` takes the type of its prefix. A string literal gets a string type sized to its length: in `text : STRING := 'hello'` the literal is annotated `__STRING_5` and hinted to `STRING`. The sized type does not exist in the index, so the resolver registers it in a small index of its own. After all units are annotated, these generated types, together with the on-demand pointer types, are imported into the global index, so that codegen can look them up like any declared type. String literals found in bodies are also collected per unit, because codegen emits them as global constants. For

```iecst
PROGRAM main
    VAR
        text : STRING := 'hello';
        large : LINT := 5_000_000_000;
        n : INT := INT#5;
    END_VAR
END_PROGRAM
```

Visualized:

```
        text : STRING := 'hello';
                         ^^^^^^^        { kind: Value, resulting_type: "__STRING_5", hint: "STRING" }
        large : LINT := 5_000_000_000;
                        ^^^^^^^^^^^^^   { kind: Value, resulting_type: "LINT",       hint: "LINT" }
        n : INT := INT#5;
                   ^^^^^                { kind: Value, resulting_type: "INT",        hint: "INT" }
```


## Dependencies

While it annotates, the visitor records every type, variable, and callable the unit refers to, following types into their members, arrays into their element type, and pointers into their target. Codegen uses this set to declare in a unit's module only what that module needs, instead of every declaration in the project. For the unit of the walk example above, the set contains, among others, the data types `main`, `Buffer`, `INT`, and `DINT`.


## Where it lives

| What | Where |
|---|---|
| Resolver | `src/resolver.rs`, `src/resolver/` |
| Built-ins | `src/builtins.rs` |


## What's next

Every expression now has a type, every name its declaration, and every conversion its hint, but nothing has checked whether the program is correct: whether every name refers to a declaration, whether an `INT` fits into a `SINT`, or whether a private member is accessed from outside. Those checks are the job of the next stage, [Validation](04-validation.md).
