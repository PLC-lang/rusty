# Annotated AST

The parser produces a tree of names, and the tree is never changed to carry meaning. Meaning lives in a side table: for every expression node, keyed by the id the parser gave it, the resolver stores an *annotation* that says what the expression is (the variable `main.i`, the function `scale`, a value of type `DINT`) and, where needed, a *type hint* that says what the expression must become before it is used. Validation and codegen never look a name up again; they ask the table. The [Resolver](../pipeline/03-resolver.md) chapter explains how the table is filled. This chapter is the reference for what it holds: one section per annotation kind, with its fields, an example, and the stages that read it.

The running example produces every kind at least once:

```iecst
TYPE Color : (Red, Green, Blue); END_TYPE
TYPE Percent : INT(0..100); END_TYPE

FUNCTION CheckRangeSigned : INT
    VAR_INPUT
        value, lower, upper : INT;
    END_VAR

    CheckRangeSigned := value;
END_FUNCTION

{external}
FUNCTION STRING_EQUAL : BOOL
    VAR_INPUT
        a, b : STRING;
    END_VAR
END_FUNCTION

FUNCTION scale : DINT
    VAR_INPUT
        value : DINT;
        factor : INT := 1;
    END_VAR

    scale := value * factor;
END_FUNCTION

FUNCTION_BLOCK Base
    VAR_INPUT
        limit : INT;
    END_VAR

    METHOD area : DINT
        area := limit;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Counter EXTENDS Base
    VAR_INPUT
        step : DINT;
    END_VAR
    VAR_OUTPUT
        count : DINT;
    END_VAR

    PROPERTY_GET scaled : DINT
        scaled := count * 10;
    END_PROPERTY

    METHOD area : DINT
        area := SUPER^.area() + step;
    END_METHOD

    count := count + step;
END_FUNCTION_BLOCK

PROGRAM logger
    VAR_INPUT
        text : STRING;
    END_VAR
END_PROGRAM

PROGRAM main
    VAR
        counter : Counter;
        i : DINT;
        pct : Percent;
        hue : Color;
        fp : __FPOINTER Base.area := ADR(Base.area);
        same : BOOL;
        text : STRING;
        r : REFERENCE TO DINT;
    END_VAR
    VAR CONSTANT
        MAX : DINT := 10;
    END_VAR

    i := MAX;
    counter(limit := 3, step := 2, count => i);
    i := scale(i, factor := INT#5);
    i := counter.area();
    logger(text := 'x');
    hue := Color#Red;
    i := fp^(counter);
    pct := i;
    same := text = 'hello';
    i := counter.scaled;
    r REF= i;
END_PROGRAM
```

The annotations below are the ones after the first resolver pass, with the property lowerer's accessor methods already in place. Later participants rewrite some of these statements and the resolver runs again; the kinds stay the same.


The map's fields are shown in the [Resolver](../pipeline/03-resolver.md) chapter. Every entry is one of the kinds below; a kind that describes a typed thing carries the type as a name, and the type itself is one more lookup in the index. Two declaration nodes are annotated too: a POU declaration and a method declaration carry annotations about their methods, keyed by the id of the declaration node.


## Value

```rust
Value {
    /// The type the expression evaluates to
    resulting_type: String,
}
```

The kind for anything that has a type but no home: a literal, an arithmetic or logical expression, a cast, the result of a call, an array element access. In the example:

```
    i := scale(i, factor := INT#5);
         ^^^^^^^^^^^^^^^^^^^^^^^^^   Value "DINT"      the call takes the function's return type
                         ^^^^^       Value "INT"       the cast takes the type it names
                             ^       Value "INT"       the literal inside a cast is typed by the cast
    logger(text := 'x');
                   ^^^               Value "__STRING_1" a literal is typed by its own value
```

Every consumer reads it through `get_type`, described below; nobody matches on the kind itself. A `Value` is an rvalue: codegen cannot take its address, and validation rejects `ADR(...)` of one (E066).


## Variable

```rust
Variable {
    /// The type name of the variable
    resulting_type: String,

    /// The declaration it refers to, such as "main.i" or "Base.limit"
    qualified_name: String,

    /// Declared in a CONSTANT block, or an enum variant
    constant: bool,

    /// Input, Output, InOut, Local, Temp, Global, Return, by value or by reference
    argument_type: ArgumentType,

    /// Set when reading the variable reads through a pointer: REFERENCE TO, AT alias, or VAR_IN_OUT
    auto_deref: Option<AutoDerefType>,
}
```

The kind for a reference that resolves to a declared variable, wherever it is declared: a local, a member reached through an instance, a global, an enum variant, the return variable of a function. A qualified reference `a.b.c` gets the entry of its last segment, and every segment has an entry of its own.

```
    i := MAX;
    ^                Variable "DINT",  main.i,      constant: false, Local
         ^^^         Variable "DINT",  main.MAX,    constant: true,  Local
    counter(limit := 3, step := 2, count => i);
    ^^^^^^^          Variable "Counter", main.counter                 the operator of a function block call
            ^^^^^    Variable "INT",   Base.limit,  Input             looked up in the callee, not in main
    hue := Color#Red;
                 ^^^ Variable "Color", Color.Red,   constant: true,  Global
    r REF= i;
    ^                Variable "DINT",  main.r,      auto_deref: Reference("__main_r")
```

`r` is declared as `REFERENCE TO DINT`, and its type is reported as `DINT`, the type behind the reference; `auto_deref` records that reaching that `DINT` takes one load through the pointer type `__main_r`, and the same holds for an `AT` alias and for a `VAR_IN_OUT` parameter.

This is the kind codegen uses most. The address of a reference is looked up in the LLVM index by `qualified_name`; a constant variable of a scalar type is replaced by its evaluated value instead of a load; `auto_deref` adds the load through the pointer. The validator matches on it for assignments to constants (E036), access to private members through `argument_type` (E049), the target of `ADR` and `REF=` (E066), constants passed to by-reference parameters, and the reference in a `VAR_CONFIG` block.


## Function

```rust
Function {
    /// The declared return type, or VOID
    return_type: String,

    /// The function or method, such as "scale" or "Counter.area"
    qualified_name: String,

    /// For a call to a generic function: the template's name
    generic_name: Option<String>,

    /// For a call to a generic function: the concrete implementation to call, when it differs
    call_name: Option<String>,
}
```

The kind for the operator of a call to a function or method, and for a bare reference to one. The resolver looks the operator up with functions first, so inside `scale` the name `scale` as an operator is the function and everywhere else the return variable.

```
    i := scale(i, factor := INT#5);
         ^^^^^                       Function return "DINT", scale
    i := counter.area();
                 ^^^^                Function return "DINT", Counter.area
```

The resolver itself reads it to give the call its `Value`. The generic lowerer reads `qualified_name` and `return_type` to derive the concrete implementation and writes `call_name`; the aggregate-return lowerer reads it to find calls that return an aggregate; codegen takes the callee from `call_name` if set, otherwise from `qualified_name`.


## FunctionPointer

```rust
FunctionPointer {
    /// The return type of the referenced function
    return_type: String,

    /// The method or function block the pointer type names, such as "Base.area"
    qualified_name: String,
}
```

The kind for the operator of an indirect call: a dereferenced variable whose type is a pointer to a method or a function block body. Method tables are built from such pointers, so almost every entry of this kind is produced by the [polymorphism lowerer](../participants/03-polymorphism.md); the example writes one by hand:

```
    i := fp^(counter);
         ^^^             FunctionPointer return "DINT", Base.area
         ^^              Variable "__main_fp", main.fp
```

Codegen generates an indirect call through the loaded pointer and takes the parameter list from the declaration `qualified_name` names. The aggregate-return lowerer treats it like `Function`.


## Type

```rust
Type {
    /// The name of the type
    type_name: String,
}
```

The kind for a reference that names a type: the left side of a cast `INT#5` or `Color#Red`, a function block type used as a qualifier, a data type in an expression position.

```
    i := scale(i, factor := INT#5);
                            ^^^      Type "INT"
    hue := Color#Red;
           ^^^^^                     Type "Color"
```

The resolver reads it to type the right side of the cast. The validator reads it to check a literal against the type it is cast to: a value that does not fit the type or a literal kind the type cannot take (E053, E054, E061).


## Program

```rust
Program {
    /// The program, class, or action
    qualified_name: String,
}
```

The kind for a reference to a program, and also for a reference to a class or to an action of a program. A program has exactly one instance, so the name is enough to find its memory.

```
    logger(text := 'x');
    ^^^^^^                Program logger
```

Codegen loads the global instance of the program by `qualified_name` and passes its address to the call. The validator uses it to report an action referenced without parentheses (E095).


## Argument

```rust
Argument {
    /// The declared type of the parameter
    resulting_type: String,

    /// The position of the parameter among the declared parameters of its POU
    position: usize,

    /// How many EXTENDS steps lie between the called block and the block that declares the parameter
    depth: usize,

    /// The block that declares the parameter, which may be a base of the called block
    pou: String,
}
```

This kind is only ever a hint. It sits on every argument of a call, positional or named, and ties the argument to the parameter it was matched to. For a named argument the hint sits on the assignment node; for a positional one on the expression.

```
    counter(limit := 3, step := 2, count => i);
            ^^^^^^^^^^                 hint Argument "INT",  position 0, depth 1, pou Base
                        ^^^^^^^^^      hint Argument "DINT", position 0, depth 0, pou Counter
                                   ^^^^^^^^^^  hint Argument "DINT", position 1, depth 0, pou Counter
    i := scale(i, factor := INT#5);
               ^                       hint Argument "DINT", position 0, depth 0, pou scale
                  ^^^^^^^^^^^^^^^      hint Argument "INT",  position 1, depth 0, pou scale
```

`limit` is declared in `Base`, one `EXTENDS` step above `Counter`, so the hint says position 0 of `Base` at depth 1, not position 0 of `Counter`. Codegen uses `pou` and `position` to find the member of the instance struct that receives the value, and `depth` to walk through the embedded base parts first. The aggregate-return lowerer reads `pou` and `position` to rewrite output arguments. The type serves as the hint for the conversion of the argument value, like any other hint.


## Property

```rust
Property {
    /// The accessor to call: "__get_<name>" or "__set_<name>"
    name: String,
}
```

The kind for a reference to a property, before it is lowered. The [property lowerer](../participants/02-property.md) has already turned the accessors into methods when the resolver runs; the resolver recognizes the member as a property, decides from the position of the reference whether the getter or the setter is meant, and stores the accessor's name.

```
    i := counter.scaled;
                 ^^^^^^    Property "__get_scaled"
```

Only the property lowerer reads it: at `post_annotate` it replaces every such reference by a call to the named accessor and annotates again. A `Property` entry that survives to codegen is an error; codegen has no case for it.


## ReplacementAst

```rust
ReplacementAst {
    /// The statement to generate instead of the annotated one
    statement: AstNode,
}
```

The kind that swaps one expression for another without touching the tree. The resolver uses it for comparisons on types that have no comparison instruction: `text = 'hello'` on two strings becomes a call to `STRING_EQUAL`, and `<>`, `<=`, `>=` become `NOT`, `OR` combinations of the `_EQUAL`, `_LESS`, and `_GREATER` calls. The replacement statement is itself annotated in full.

```
    same := text = 'hello';
            ^^^^^^^^^^^^^^    ReplacementAst STRING_EQUAL(text, 'hello'),  hint "BOOL"
            ^^^^              Variable "STRING", main.text,  hint Argument "STRING", position 0, pou STRING_EQUAL
                   ^^^^^^^    Value "__STRING_5",            hint Argument "STRING", position 1, pou STRING_EQUAL
```

Codegen checks every expression for this kind first and generates the replacement instead of the node. The type of the node is the type of its replacement; see Deriving a type below.


## Label

```rust
Label {
    /// The label the jump targets
    name: String,
}
```

The kind for a jump statement. Structured Text has no spelling for jumps and labels; they come from the [CFC participant](../participants/00-cfc.md), which renders a jump element into a jump statement and a label element into a label statement. The resolver collects every jump per POU and, once the POU's labels are known, annotates each jump with the label it targets. Codegen looks the label's basic block up by `name` and emits the branch.


## MethodDeclarations and Override

```rust
MethodDeclarations {
    /// Method name to every declaration of it in the block, its bases, and its interfaces
    declarations: FxHashMap<String, Vec<MethodDeclarationType>>,
}

Override {
    /// Every method this method overrides, in the bases and interfaces
    definitions: Vec<MethodDeclarationType>,
}
```

These two kinds are not on expressions. `MethodDeclarations` is stored under the id of a function block, class, or interface declaration and lists, for every method name, where the method is declared, as `Concrete` or `Abstract`, from the block itself up its `EXTENDS` chain and through its interfaces. `Override` is stored under the id of a method declaration and lists the methods it overrides.

```
FUNCTION_BLOCK Base           MethodDeclarations { area: [Concrete Base.area] }
FUNCTION_BLOCK Counter        MethodDeclarations { area: [Concrete Counter.area], __get_scaled: [Concrete Counter.__get_scaled] }
    METHOD area               Override { [Concrete Base.area] }
```

Only the validator reads them: the first for the abstract-signature and implemented-methods checks (E111, E112), the second for the override checks (E112, E118).

> **Developer Note**
>
> Two variants of the annotation enum are never produced. `Super` was meant for the `SUPER` keyword, but `SUPER^` is rewritten by the inheritance lowerer before it needs a type of its own, and the resolver leaves the keyword node unannotated. `None` is the default value and is not stored. The wrapper the driver puts around the map, which answers one reserved node id with a `BOOL` value, is not read by any stage either.


## Type hints

A hint is a second entry for the same node and uses the same kinds, in practice `Value` and `Argument`. It says what the expression must become where it is used: the type of the assignment target, the type of the parameter, the wider operand type of a binary expression, `BOOL` for the condition of a control statement. A hint on a node whose annotation names the same type is harmless and common.

```
    i := MAX;
         ^^^          Variable "DINT"   hint Value "DINT"
    pct := i;
           ^          Variable "DINT"   hint Argument "INT", position 0, pou CheckRangeSigned
```

The second line shows the hint being replaced: `pct` is a subrange of `INT`, so the value is not hinted to `Percent` but to the parameter of the range check function that the hidden call below inserts. Codegen compares annotation and hint and emits the conversion (see [Codegen](../pipeline/05-codegen.md), Expressions and statements); the validator compares them to report the implicit downcast (E067).


## Hidden function calls

A subrange type such as `INT(0..100)` is checked at run time. When a value is assigned to a variable of such a type and a function `CheckRangeSigned` or `CheckRangeUnsigned` is declared in the project, the resolver builds the call `CheckRangeSigned(value, 0, 100)`, annotates it like any call, and stores it in the hidden-call table under the id of the assigned value. The value keeps its own annotation. Codegen, when it stores into a subrange variable, asks the table and generates the call in place of the value; without the function declared, the store is plain.

```
    pct := i;
           ^          hidden: CheckRangeSigned(i, 0, 100)
```


## Deriving a type

Every consumer asks the map for a type through two helpers. `get_type` returns the type behind the annotation, and `get_type_hint` the type behind the hint; both resolve the name through the index. The name is taken from the kind: `resulting_type` for `Value`, `Variable`, and `Argument`; `type_name` for `Type`; `qualified_name` for `Program`, `Function`, and `FunctionPointer`, where the type of a POU is its instance struct; for `ReplacementAst` the hint, else the annotation, of the replacement statement. `Label`, `Override`, `MethodDeclarations`, and `Property` have no type.

The type an expression is generated in is the hint when there is one and the annotation otherwise. That is the rule the codegen chapter states as "the hint decides the conversion": the annotation says what the value is, the hint says what it becomes, and codegen emits the cast between the two.


## At a glance

| Kind | Produced for | Fields | Read by |
|---|---|---|---|
| `Value` | literals, expressions, casts, call results, element accesses | type | everyone, through `get_type` |
| `Variable` | references to declared variables, members, globals, enum variants, return variables | type, qualified name, constant, argument type, auto-deref | codegen (address, constant folding, deref load), validator (E036, E049, E066, E067) |
| `Function` | call operators and references naming a function or method | return type, qualified name, generic name, call name | resolver, generic lowerer, aggregate-return lowerer, codegen |
| `FunctionPointer` | dereferenced pointer-to-method operators | return type, qualified name | aggregate-return lowerer, codegen (indirect call) |
| `Type` | the type side of a cast, a type used as qualifier | type name | resolver, validator (literal casts) |
| `Program` | references to programs, classes, program actions | qualified name | codegen (instance address), validator (E095) |
| `Argument` (hint only) | every call argument | type, position, depth, declaring POU | codegen (parameter slot), aggregate-return lowerer, conversion like any hint |
| `Property` | property references before lowering | accessor name | property lowerer |
| `ReplacementAst` | string and other library-compared comparisons | the replacement statement | codegen |
| `Label` | jump statements from CFC | label name | codegen |
| `MethodDeclarations` | block, class, and interface declarations (by declaration id) | method name to declarations | validator (E111, E112) |
| `Override` | method declarations that override (by declaration id) | overridden methods | validator (E112, E118) |
