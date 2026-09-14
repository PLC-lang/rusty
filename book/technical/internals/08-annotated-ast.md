# Annotated AST

The resolver records meaning in a map keyed by AST node ID. An *annotation* identifies a declaration or result type. A *type hint* records the type expected where an expression is used. Validation and codegen combine this information with the index. Participants can rewrite the tree and request new annotations.

The [Resolver](../pipeline/03-resolver.md) chapter explains how the table is filled. This chapter is the reference for what it holds: one section per annotation kind, with its fields, an example, and the stages that read it.

The example produces the annotations used below. Labels are introduced separately because they come from CFC diagrams:

```iecst
TYPE Color: (Red, Green, Blue); END_TYPE
TYPE Percent: INT(0..100); END_TYPE

FUNCTION CheckRangeSigned: INT
    VAR_INPUT
        value, lower, upper: INT;
    END_VAR

    CheckRangeSigned := value;
END_FUNCTION

{external}
FUNCTION STRING_EQUAL: BOOL
    VAR_INPUT
        a, b: STRING;
    END_VAR
END_FUNCTION

FUNCTION scale: DINT
    VAR_INPUT
        value: DINT;
        factor: INT := 1;
    END_VAR

    scale := value * factor;
END_FUNCTION

FUNCTION_BLOCK Base
    VAR_INPUT
        limit: INT;
    END_VAR

    METHOD area: DINT
        area := limit;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Counter EXTENDS Base
    VAR_INPUT
        step: DINT;
    END_VAR
    VAR_OUTPUT
        count: DINT;
    END_VAR

    PROPERTY_GET scaled: DINT
        scaled := count * 10;
    END_PROPERTY

    METHOD area: DINT
        area := SUPER^.area() + step;
    END_METHOD

    count := count + step;
END_FUNCTION_BLOCK

PROGRAM logger
    VAR_INPUT
        text: STRING;
    END_VAR
END_PROGRAM

PROGRAM main
    VAR
        counter: Counter;
        i: DINT;
        pct: Percent;
        hue: Color;
        fp: __FPOINTER Base.area := ADR(Base.area);
        same: BOOL;
        text: STRING;
        r: REFERENCE TO DINT;
    END_VAR
    VAR CONSTANT
        MAX: DINT := 10;
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

The annotations below are the ones after the first resolver pass, with the accessor methods of the property lowerer already in place. Later participants rewrite some of these statements and the resolver runs again, but the kinds stay the same.

Entries store type names, which consumers resolve through the index. Most entries describe expressions; method and POU declarations also receive annotations for validation.


## Type hints

A hint is a second entry for the same node. It usually uses `Value` or `Argument` and specifies the required type: an assignment target, parameter, promoted operand, or condition type. A hint can equal the annotated type; no conversion is then needed.

```
    i := MAX;
         ^^^          Variable "DINT"   hint Value "DINT"
    pct := i;
           ^          Variable "DINT"   hint Argument "INT", position 0, pou CheckRangeSigned
```

The second line shows a hint that was replaced: `pct` is a subrange of `INT`, so the value is not hinted to `Percent` but to the parameter of the range check function the hidden call below inserts. Codegen compares annotation and hint and emits the conversion (see [Codegen](../pipeline/05-codegen.md), Expressions and statements); the validator compares them to report the implicit downcast (E067).


## Hidden function calls

An assignment to a subrange such as `INT(0..100)` can call a range-check function. If the project declares `CheckRangeSigned` or `CheckRangeUnsigned`, the resolver builds a call with the value and bounds. It annotates the call and stores it under the assigned value's node ID. The original value keeps its annotation.

When codegen stores into a subrange variable, it asks the table and generates the call in place of the value. Without the function declared, the store is plain.

```
    pct := i;
           ^          hidden: CheckRangeSigned(i, 0, 100)
```


## Annotation kinds

Each entry below shows its fields, an example, and the stages that use it.

### Value

```rust
Value {
    /// The type the expression evaluates to
    resulting_type: String,
}
```

`Value` records a result type without identifying a variable declaration. It covers literals, arithmetic, casts, call results, and array element accesses. In the example:

```
    i := scale(i, factor := INT#5);
         ^^^^^^^^^^^^^^^^^^^^^^^^^   Value "DINT"      the call takes the function's return type
                            ^^^^^    Value "INT"       the cast takes the type it names
                                ^    Value "INT"       the literal inside a cast is typed by the cast
    logger(text := 'x');
                   ^^^               Value "__STRING_1" a literal is typed by its own value
```

Consumers read the type through [`get_type`](#deriving-a-type). `Value` alone does not determine whether an expression has an address: an array element does, while `i + 1` does not. The AST form also matters to codegen.


### Variable

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

`r` is declared as `REFERENCE TO DINT`, and its type is reported as `DINT`, the type behind the reference. `auto_deref` records that one load through the pointer type `__main_r` is needed to reach that `DINT`, and the same holds for an `AT` alias and for a `VAR_IN_OUT` parameter.

This is the kind codegen uses most. It looks the address of a reference up in the LLVM index by `qualified_name`, replaces a constant variable of a scalar type by its evaluated value instead of a load, and adds the load through the pointer for `auto_deref`.

Validation uses variable annotations for constant assignments (E036), private-member access (E049), and reference assignments (E098). It also checks constants passed by reference and references in `VAR_CONFIG`.


### Function

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

The resolver uses the return type to annotate the call result. Generic lowering uses the callee and argument information to select an implementation. Aggregate-return lowering identifies calls that need result storage. Codegen uses `call_name` when present and `qualified_name` otherwise.


### FunctionPointer

```rust
FunctionPointer {
    /// The return type of the referenced function
    return_type: String,

    /// The method or function block the pointer type names, such as "Base.area"
    qualified_name: String,
}
```

The kind for the operator of an indirect call: a dereferenced variable whose type is a pointer to a method or to a function block body. Method tables are built from such pointers, so almost every entry of this kind comes from the [polymorphism lowerer](../participants/03-polymorphism.md). The example writes one by hand:

```
    i := fp^(counter);
         ^^^             FunctionPointer return "DINT", Base.area
         ^^              Variable "__main_fp", main.fp
```

Codegen generates an indirect call through the loaded pointer and takes the parameter list from the declaration `qualified_name` names. The aggregate-return lowerer treats it like `Function`.


### Type

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


### Program

```rust
Program {
    /// The program, class, or action
    qualified_name: String,
}
```

The kind for a reference to a program, and also for a reference to a class or to an action, whatever POU the action belongs to. A program has exactly one instance, so the name is enough to find its memory.

```
    logger(text := 'x');
    ^^^^^^                Program logger
```

Codegen loads the global instance of the program by `qualified_name` and passes its address to the call. The validator uses it to report an action referenced without parentheses (E095).


### Argument

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

`Argument` is used only as a hint. It connects each argument to a parameter. A positional argument carries the hint on its expression; a named argument carries it on the assignment node.

```
    counter(limit := 3, step := 2, count => i);
            ^^^^^^^^^^                 hint Argument "INT",  position 0, depth 1, pou Base
                        ^^^^^^^^^      hint Argument "DINT", position 0, depth 0, pou Counter
                                   ^^^^^^^^^^  hint Argument "DINT", position 1, depth 0, pou Counter
    i := scale(i, factor := INT#5);
               ^                       hint Argument "DINT", position 0, depth 0, pou scale
                  ^^^^^^^^^^^^^^^      hint Argument "INT",  position 1, depth 0, pou scale
```

`limit` is declared in `Base`, one `EXTENDS` step above `Counter`, so the hint says position 0 of `Base` at depth 1, not position 0 of `Counter`.

Codegen uses `pou` and `position` to find the member of the instance struct that receives the value, and `depth` to walk through the embedded base parts first. The aggregate-return lowerer reads `pou` and `position` to rewrite an output argument. The type is the hint for the conversion of the argument value, like any other hint.


### Property

```rust
Property {
    /// The accessor to call: "__get_<name>" or "__set_<name>"
    name: String,
}
```

The kind for a reference to a property, before it is lowered. The [property lowerer](../participants/02-property.md) has already turned the accessors into methods when the resolver runs. The resolver recognizes the member as a property, decides from the position of the reference whether the getter or the setter is meant, and stores the name of that accessor.

```
    i := counter.scaled;
                 ^^^^^^    Property "__get_scaled"
```

Only the property lowerer reads it: at `post_annotate` it replaces every such reference by a call to the named accessor and annotates again. A `Property` entry that survives to codegen is an error; codegen has no case for it.


### ReplacementAst

```rust
ReplacementAst {
    /// The statement to generate instead of the annotated one
    statement: AstNode,
}
```

`ReplacementAst` attaches a replacement expression without changing the original node. String equality becomes a `STRING_EQUAL` call. Other string comparisons combine `_EQUAL`, `_LESS`, and `_GREATER` calls with `NOT` and `OR`. The replacement has its own annotations.

```
    same := text = 'hello';
            ^^^^^^^^^^^^^^    ReplacementAst STRING_EQUAL(text, 'hello'),  hint "BOOL"
            ^^^^              Variable "STRING", main.text,  hint Argument "STRING", position 0, pou STRING_EQUAL
                   ^^^^^^^    Value "__STRING_5",            hint Argument "STRING", position 1, pou STRING_EQUAL
```

Codegen checks every expression for this kind first and generates the replacement instead of the node. The type of the node is the type of its replacement; see [Deriving a type](#deriving-a-type).


### Label

```rust
Label {
    /// The label the jump targets
    name: String,
}
```

The kind for a jump statement. Structured Text has no spelling for jumps and labels; they come from the [CFC participant](../participants/00-cfc.md), which renders a jump element into a jump statement and a label element into a label statement. The resolver collects every jump per POU and, once the labels of the POU are known, annotates each jump with the label it targets. Codegen looks the basic block of the label up by `name` and emits the branch.


### MethodDeclarations and Override

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

These two kinds do not sit on expressions. `MethodDeclarations` is stored under the ID of a function block, class, or interface declaration. For every method name it lists where the method is declared, as `Concrete` or `Abstract`, from the block itself up its `EXTENDS` chain and through its interfaces. `Override` is stored under the ID of a method declaration and lists the methods it overrides.

```
FUNCTION_BLOCK Base           MethodDeclarations { area: [Concrete Base.area] }
FUNCTION_BLOCK Counter        MethodDeclarations { area: [Concrete Counter.area], __get_scaled: [Concrete Counter.__get_scaled] }
    METHOD area               Override { [Concrete Base.area] }
```

Only the validator reads them: the first for the abstract-signature and implemented-methods checks (E111, E112), the second for the override checks (E112, E118).

> [!NOTE]
> **Developer note.** Two variants of the annotation enum are never produced. `Super` was meant for the `SUPER` keyword, but the inheritance lowerer rewrites `SUPER^` before it needs a type of its own, and the resolver leaves the keyword node unannotated. `None` is the default value and is not stored. The wrapper the driver puts around the map, which answers one reserved node ID with a `BOOL` value, is not read by any stage either.


## Deriving a type

Consumers use `get_type` for the annotation's type and `get_type_hint` for its hint. Both resolve a stored name through the index.

`Value`, `Variable`, and `Argument` use `resulting_type`; `Type` uses `type_name`. POU annotations use `qualified_name` to find the instance type. `ReplacementAst` uses the replacement's hint or annotation. `Label`, `Override`, `MethodDeclarations`, and `Property` have no type.

Codegen compares the annotated type with the expected type from the hint. The annotation describes the value; the hint determines any conversion needed at its use. The [Codegen](../pipeline/05-codegen.md#expressions) chapter shows the resulting instructions.


## At a glance

| Kind | Produced for | Fields | Read by |
|---|---|---|---|
| `Value` | literals, expressions, casts, call results, element accesses | type | everyone, through `get_type` |
| `Variable` | references to declared variables, members, globals, enum variants, return variables | type, qualified name, constant, argument type, auto-deref | codegen (address, constant folding, deref load), validator (E036, E049, E098, E067) |
| `Function` | call operators and references naming a function or method | return type, qualified name, generic name, call name | resolver, generic lowerer, aggregate-return lowerer, codegen |
| `FunctionPointer` | dereferenced pointer-to-method operators | return type, qualified name | aggregate-return lowerer, codegen (indirect call) |
| `Type` | the type side of a cast, a type used as qualifier | type name | resolver, validator (literal casts) |
| `Program` | references to programs, classes, actions | qualified name | codegen (instance address), validator (E095) |
| `Argument` (hint only) | every call argument | type, position, depth, declaring POU | codegen (parameter slot), aggregate-return lowerer, conversion like any hint |
| `Property` | property references before lowering | accessor name | property lowerer |
| `ReplacementAst` | string and other library-compared comparisons | the replacement statement | codegen |
| `Label` | jump statements from CFC | label name | codegen |
| `MethodDeclarations` | block, class, and interface declarations (by declaration ID) | method name to declarations | validator (E111, E112) |
| `Override` | method declarations that override (by declaration ID) | overridden methods | validator (E112, E118) |
