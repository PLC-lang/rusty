# Resolver

After indexing, every declaration is known, but the statement bodies are still trees of names. In

```iecst
PROGRAM main
    VAR
        i: DINT;
        sintVar: SINT;
    END_VAR

    sintVar := i + 1;
END_PROGRAM
```

the parser produced an assignment: the name `sintVar` on the left, the name `i` plus the literal `1` on the right. Nothing says that `sintVar` is the local variable `main.sintVar`, that `i + 1` is a `DINT`, or that a `SINT` target needs a narrowing conversion. The resolver answers these questions for every expression of the project.

The driver calls this stage *annotate*. The resolver stores its results in an annotation map keyed by node ID. It can also attach replacement expressions without changing the original tree. Validation uses the map to check types; codegen uses it to select instructions and conversions.

```mermaid
flowchart LR
    parse[Parse] --> index[Index] --> annotate[Annotate] --> validate[Validate] --> codegen[Codegen] --> link[Link]
    style annotate fill:#bfdbfe,stroke:#000,stroke-width:1px,stroke-dasharray:4 3,color:#0f172a
```

The stage receives the units and the global index. Each unit is annotated concurrently by its own visitor, which returns an annotation map, the set of names the unit depends on, and the string literals it contains. The maps are merged into one, and the few types the visitors had to create are imported into the index. Like the index, the map is never updated in place: when a participant rewrites the tree, the stage runs again and replaces it.


## Annotations and hints

The resolver distinguishes an expression's *annotation* from its *type hint*. The annotation identifies its declaration or result type. The hint records the type expected where the expression is used. In `sintVar := i + 1`, the sum has type `DINT` and hint `SINT`. Codegen truncates the result, and validation warns about the implicit downcast.

Annotations distinguish variable, function, type, and POU references from plain expression values. Argument hints also identify matched parameters. [Annotated AST](../internals/08-annotated-ast.md) describes each kind.

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

A failed name lookup leaves the reference without an annotation. Validation reports the unresolved reference. Some containers, such as named argument assignments, also have no annotation of their own; their children and hints carry the required information.


## Walking a unit

The resolver walks each unit from top to bottom and visits every expression: the initializers in variable blocks, the bounds in type declarations, and the statements of the bodies. Each expression is visited with the POU it sits in as its context. For

```iecst
FUNCTION_BLOCK Buffer
    VAR_INPUT
        limit: INT;
    END_VAR
END_FUNCTION_BLOCK

PROGRAM main
    VAR
        bufferInstance: Buffer;
        i: DINT := 1;
    END_VAR

    i := bufferInstance.limit;
END_PROGRAM
```

the resolver walks declarations first and bodies second: the variable blocks of every POU, then the body of every POU. Only expressions get entries, so `Buffer`, whose one variable has no initializer and whose body is empty, ends without a single entry.

In `main`, only `i` has an initializer, `1`: an integer literal is a `DINT` value, and because it initializes a `DINT` variable it is also hinted `DINT`.

The body is one assignment, which the resolver visits value first, target second, and hints last. The value `bufferInstance.limit` is resolved left to right: `bufferInstance` is found as a member of `main`, the current POU, and its type `Buffer` becomes the qualifier under which `limit` is found as `Buffer.limit` of type `INT`. The whole reference takes the entry of its last part. The target `i` is found as `main.i` and gets no hint. Last, the value is hinted with the target's type `DINT`, a widening that codegen emits later.

Visualized:

```
PROGRAM main
    VAR
        bufferInstance: Buffer;
        i: DINT := 1;
                   ^          { kind: Value, resulting_type: "DINT", hint: "DINT" }
    END_VAR

    i := bufferInstance.limit;
    ^                          { kind: Variable, qualified_name: "main.i",              resulting_type: "DINT",   hint: None }
         ^^^^^^^^^^^^^^        { kind: Variable, qualified_name: "main.bufferInstance", resulting_type: "Buffer", hint: None }
                        ^^^^^  { kind: Variable, qualified_name: "Buffer.limit",        resulting_type: "INT",    hint: None }
         ^^^^^^^^^^^^^^^^^^^^  { kind: Variable, qualified_name: "Buffer.limit",        resulting_type: "INT",    hint: "DINT" }
END_PROGRAM
```

Now imagine a global variable that is also named `bufferInstance`. The resolver tries a fixed order of lookups and takes the first that succeeds: a member of the current POU, then a global variable or enum variant, then a POU (program, function, or function block), then a type. The local member wins. The one exception is the operator of a call, which tries functions first: inside a function `scale`, the name `scale` is the return variable, but `scale(...)` is the function.


## Promotion

Arithmetic and comparisons combine operands of different types. The resolver decides which type the operation runs in and marks the operands that have to be converted. In

```iecst
PROGRAM main
    VAR
        sintVar: SINT;
        dintVar: DINT;
        boolVar: BOOL;
    END_VAR

    sintVar := dintVar + 1;
    boolVar := sintVar < dintVar;
END_PROGRAM
```

the addition uses `DINT` for both operands and for its result. The assignment then gives the sum the hint `SINT`. The operands are not narrowed: codegen performs the addition first and converts its result.

The comparison widens `sintVar` from `SINT` to `DINT` through a hint on that operand. Its result has type `BOOL`, which already matches the assignment target.

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
FUNCTION scale: DINT
    VAR_INPUT
        value: DINT;
        factor: INT;
    END_VAR
END_FUNCTION

PROGRAM main
    VAR
        i: DINT;
        sintVar: SINT;
    END_VAR

    i := scale(i, factor := sintVar);
END_PROGRAM
```

the resolver visits the call before the assignment target. It identifies `scale` as a function returning `DINT`, then resolves the arguments. `i` refers to `main.i`. In `factor := sintVar`, the value refers to `main.sintVar`, but `factor` names the callee's parameter `scale.factor`. That parameter has type `INT`, so `sintVar` gets the hint `INT`.

Then the arguments are matched to the parameters of `scale`. A positional argument takes the parameter at its place, a named argument the parameter with its name. `i` is matched to parameter 0 and hinted `DINT`, `factor := sintVar` to parameter 1 and hinted `INT`. These hints also record the parameter position, which codegen uses to place the values. The call as a whole takes the return type of the function, a `DINT` value, and the rest is the ordinary assignment: the target `i` is `main.i`, and the call gets the hint `DINT`.

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

For a function block call, the operator is a variable of the block's type. Arguments match that block's parameters, and the call has no result type. Built-ins such as `REF`, array bound functions, and generic arithmetic functions have special annotation rules because their types depend on the arguments.


## Literals and generated types

Integer literals use `DINT` if they fit 32 bits and `LINT` otherwise. Real literals use `REAL` or `LREAL` by the same size rule. Typed literals use their prefix, as in `INT#5`. String literals get types sized to their contents: `'hello'` has type `__STRING_5`, even when assigned to an unsized `STRING` (i.e. `STRING[5]`).

That sized type does not exist in the index, so the resolver registers it in a small index of its own. After all units are annotated, these generated types, together with the on-demand pointer types, are imported into the global index, where codegen finds them like any declared type. String literals found in bodies are also collected per unit, because codegen emits them as global constants. For

```iecst
PROGRAM main
    VAR
        text: STRING := 'hello';
        large: LINT := 5_000_000_000;
        n: INT := INT#5;
    END_VAR
END_PROGRAM
```

Visualized:

```
        text: STRING := 'hello';
                        ^^^^^^^        { kind: Value, resulting_type: "__STRING_5", hint: "STRING" }
        large: LINT := 5_000_000_000;
                       ^^^^^^^^^^^^^   { kind: Value, resulting_type: "LINT",       hint: "LINT" }
        n: INT := INT#5;
                  ^^^^^                { kind: Value, resulting_type: "INT",        hint: "INT" }
```


## Dependencies

While it annotates, the visitor records every type, variable, and callable the unit refers to. It follows a type into its members, an array into its element type, and a pointer into its target. Codegen uses the set to declare in a module only what that module needs, instead of every declaration of the project. For the unit of the walk example above, the set holds the data types `main`, `Buffer`, `INT`, and `DINT`, among others.


## Where it lives

| What | Where |
|---|---|
| Resolver | `src/resolver.rs`, `src/resolver/` |
| Built-ins | `src/builtins.rs` |


## What's next

The resolver has recorded the names and types it could resolve, plus the required conversions. The [Validation](04-validation.md) stage now checks missing references, incompatible types, and access rules.
