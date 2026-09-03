# Reference Expressions

A reference expression names a place: a variable, a member of a struct or function block, an element of an array, the target of a pointer, a bit inside a word. Structured Text writes them as chains, `pShape^.points[i].x`, and every link of the chain is a different kind of step. Internally, a reference is a linked list of segments, stored from the last segment back to the first, and every segment answers two questions in turn: the resolver asks what declaration or type the segment stands for, and codegen asks what address it computes. A reference is an address until a value is needed.

This chapter follows one project through the compiler:

```iecst
TYPE Point : STRUCT
    x : DINT;
    y : DINT;
END_STRUCT END_TYPE

TYPE Color : (Red, Green, Blue); END_TYPE

FUNCTION_BLOCK Shape
    VAR_INPUT
        origin : Point;
    END_VAR
    VAR_OUTPUT
        points : ARRAY[0..3] OF Point;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION bump : DINT
    VAR_IN_OUT
        target : DINT;
    END_VAR
    VAR_INPUT
        step : INT;
    END_VAR

    target := target + step;
    bump := target;
END_FUNCTION

VAR_GLOBAL
    count : DINT;
END_VAR

PROGRAM main
    VAR
        shape : Shape;
        pShape : POINTER TO Shape;
        pDint : REF_TO DINT;
        rDint : REFERENCE TO DINT;
        i : DINT;
        color : Color;
        word : WORD;
        flag : BOOL;
        count : DINT;
    END_VAR

    pShape := ADR(shape);
    pShape^.points[i].x := 1;
    pDint := REF(i);
    rDint REF= i;
    rDint := rDint + 1;
    i := bump(count, 2);
    color := Color#Red;
    i := INT#5;
    flag := word.%X0;
    i := word.%B1;
    .count := 3;
    i := pShape^.origin.x + shape.points[2].y;
END_PROGRAM
```


## Declaration

The parser has one node kind for every reference. It holds an access, the kind of step, and an optional base, the reference the step is applied to. The parser reads a chain from left to right and wraps each step around what it has so far, so the tree is stored in reverse: the root of `pShape^.points[i].x` is the member step `x`, its base is the index step `[i]`, whose base is the member step `points`, whose base is the dereference `^`, whose base is the plain name `pShape`. A plain name is a member step without a base. Stored this way, every segment carries its context: whoever looks at `x` can ask what `points[i]` is.

Six kinds of access exist:

| Access | Spelling | Holds |
|---|---|---|
| Member | `a`, `a.b` | the identifier, or a direct access such as `%X0` |
| Index | `a[i]`, `a[i, j]` | the index expression, or a list of them |
| Dereference | `p^` | nothing; the base is the pointer |
| Cast | `INT#5`, `Color#Red` | the value to cast; the base is the type name |
| Global | `.count` | the identifier, looked up in the global scope only |
| Address | none | reserved; the parser never produces it |

A cast binds tighter than the chain around it: `INT#a.b` means `INT#(a.b)`, so the parser parses the whole target first and then wraps it. `x.5` is a shorthand for `x.%X5` and becomes a bit access. Pointer declarations are inline type definitions, so pre-processing moves `POINTER TO Shape` out into the named type `__main_pShape` before indexing, like every other anonymous type.

> **Developer Note**
>
> The address access exists in the node kind, in the resolver, in codegen, and in the validator, but no spelling reaches it: the parser rejects `&i`, and no lowering creates it. The address of a variable is taken with the built-in calls `ADR` and `REF`, described below.


## Index

References themselves are not indexed; a body cannot declare anything. What the index contributes is the pointer types that make dereferencing and auto-dereferencing possible. Trimmed to the pointer variant of the type information:

```rust
Pointer {
    /// The type pointed to, by name
    inner_type_name: TypeId,

    /// None for a pointer the user dereferences with ^; otherwise the kind of implicit dereference
    auto_deref: Option<AutoDerefType>,

    /// false for POINTER TO, true for REF_TO, REFERENCE TO, and every generated pointer
    type_safe: bool,

    /// Whether the pointer targets a POU rather than data
    is_function: bool,
}
```

The auto-deref kind is what separates a pointer the program handles from a pointer the compiler handles. `Default` marks the pointers behind by-reference parameters, `Reference` marks a `REFERENCE TO` variable, and `Alias` marks an `x AT y` declaration. A variable of such a type reads and writes like its target type, and the resolver and codegen insert the dereference. For the example, the index holds these pointer types:

```
__main_pShape           { inner_type_name: "Shape", auto_deref: None,            type_safe: false }   from POINTER TO Shape
__main_pDint            { inner_type_name: "DINT",  auto_deref: None,            type_safe: true }    from REF_TO DINT
__main_rDint            { inner_type_name: "DINT",  auto_deref: Some(Reference), type_safe: true }    from REFERENCE TO DINT
__auto_pointer_to_DINT  { inner_type_name: "DINT",  auto_deref: Some(Default),   type_safe: true }    for bump.target
```

The last one is generated by the indexer: a `VAR_IN_OUT`, a `VAR_INPUT {ref}`, and a function `VAR_OUTPUT` do not keep their declared type. The member `bump.target` is registered with the type `__auto_pointer_to_DINT`, created once per target type and shared by every by-reference parameter of that type in the project. The declared type of `bump.target` survives only as the inner type of that pointer.

Enum variants are the other index entries a reference can land on. `Color.Red` is a constant global variable of type `Color` with the initializer `0`, and it is also findable under the bare name `Red`, which is what lets `color := Red` and `Color#Red` both resolve.


## Annotations

The resolver visits a reference base first, then applies the step. Each segment gets its own annotation, and the whole chain takes the annotation of its last segment (see [Resolver](../pipeline/03-resolver.md), Walking a unit, for the lookup order of a plain name). What a step produces depends on its kind:

- **Member with a base.** The type of the base becomes the qualifier, and the member is looked up inside that type, following `EXTENDS` chains and properties. The annotation is the variable entry: its qualified name, type, and auto-deref kind.
- **Member without a base.** The lookup order is a member of the current POU, then a global or enum variant, then a POU, then a type. When the reference is the operator of a call, functions are tried first.
- **Index.** The base must be an array; the step is annotated as a value of the element type. The index expression is resolved on its own and hinted to `DINT`.
- **Dereference.** The base must be a pointer without auto-deref; the step is a value of the inner type.
- **Cast.** The base is resolved as a type, and the target is resolved as a variable under that type when the type is an enum, so `Color#Red` finds `Color.Red`. The step is a value of the type; a cast literal is retyped to the cast type.
- **Global.** The identifier is looked up among global variables only; local names cannot shadow it.
- **Direct access.** A `%X`, `%B`, `%W`, `%D`, or `%L` member is a value of `BOOL`, `BYTE`, `WORD`, `DWORD`, or `LWORD`.

For the body of `main`, two statements show most of these at once:

```
    pShape^.points[i].x := 1;
    ^^^^^^                        { kind: Variable, qualified_name: "main.pShape",   resulting_type: "__main_pShape", auto_deref: None }
    ^^^^^^^                       { kind: Value,                                     resulting_type: "Shape" }
    ^^^^^^^^^^^^^^                { kind: Variable, qualified_name: "Shape.points",  resulting_type: "__Shape_points", auto_deref: None }
                   ^              { kind: Variable, qualified_name: "main.i",        resulting_type: "DINT" }
    ^^^^^^^^^^^^^^^^^             { kind: Value,                                     resulting_type: "Point" }
    ^^^^^^^^^^^^^^^^^^^           { kind: Variable, qualified_name: "Point.x",       resulting_type: "DINT" }

    i := bump(count, 2);
              ^^^^^               { kind: Variable, qualified_name: "main.count",    resulting_type: "DINT",  hint: Argument { resulting_type: "__auto_pointer_to_DINT", position: 0 } }
```

`count` resolves to the local `main.count`, not the global; the global is reachable only as `.count`. Its hint names the pointer type of the parameter, which is how codegen knows to pass the address of `count` rather than its value.

The auto-deref pointers show up as a flag on the variable annotation, not as a type. `rDint` is annotated with the resulting type `DINT`, its inner type, and `auto_deref: Reference`; inside `bump`, `target` is annotated `DINT` with `auto_deref: Default`. Every stage after the resolver treats such a variable as a `DINT` and adds one load when it needs the address:

```
    rDint REF= i;
    ^^^^^                         { kind: Variable, qualified_name: "main.rDint",    resulting_type: "DINT",  auto_deref: Some(Reference("__main_rDint")) }
    rDint := rDint + 1;
    ^^^^^                         { kind: Variable, qualified_name: "main.rDint",    resulting_type: "DINT",  auto_deref: Some(Reference("__main_rDint")) }
```

The address of a variable is not a reference step but a call. `ADR(shape)` is annotated as a call of the built-in `ADR` returning `LWORD`, an integer wide enough for any address, and hinted to `__main_pShape` by the assignment. `REF(i)` is typed: the built-in's annotation reads the argument's type and registers a pointer type to it on the fly, `__POINTER_TO_DINT`, in the resolver's own index, and the call is a value of that type. Casts and direct accesses complete the picture:

```
    pShape := ADR(shape);
              ^^^^^^^^^^          { kind: Value,                                     resulting_type: "LWORD",            hint: "__main_pShape" }
    pDint := REF(i);
             ^^^^^^               { kind: Value,                                     resulting_type: "__POINTER_TO_DINT", hint: "__main_pDint" }
    color := Color#Red;
             ^^^^^                { kind: Type,     type_name: "Color" }
                   ^^^            { kind: Variable, qualified_name: "Color.Red",     resulting_type: "Color", constant: true }
             ^^^^^^^^^            { kind: Value,                                     resulting_type: "Color",            hint: "Color" }
    i := INT#5;
         ^^^^^                    { kind: Value,                                     resulting_type: "INT",              hint: "DINT" }
    flag := word.%X0;
                 ^^^              { kind: Value,                                     resulting_type: "BOOL" }
    i := word.%B1;
              ^^^                 { kind: Value,                                     resulting_type: "BYTE",             hint: "DINT" }
    .count := 3;
    ^^^^^^                        { kind: Variable, qualified_name: "count",         resulting_type: "DINT", argument_type: Global }
```

> **Developer Note**
>
> The base of a cast is resolved with the ordinary lookup order, variables before types. A variable whose name matches the type, `color : DINT` next to `Color#Red`, is found first, and the cast is typed after the variable instead of the enum. The generated code is still right, because `Red` then resolves as a bare enum variant, but the validator reports a spurious "value evaluated at run-time" warning. Logged in `bugs.md`.


## Lowering

No participant rewrites reference expressions as such, but several rewrite what they refer to. The [inheritance lowerer](../participants/10-inheritance.md) inserts `__Base` member steps in front of inherited members, the [property lowerer](../participants/02-property.md) replaces a member step that names a property with a call, and the [polymorphism lowerer](../participants/03-polymorphism.md) turns a method call through a pointer into a dereference of a function pointer. Codegen sees only the six access kinds above.


## Codegen

Codegen computes an address for every reference and loads from it only when a value is needed (see [Codegen](../pipeline/05-codegen.md), Expressions). The address of a chain is built from the innermost segment outward, one instruction per step.

**Member.** A plain name is the address that the function setup registered for it: a member pointer into the instance struct for a program or function block variable, a stack slot for a function variable, a global otherwise. A member with a base is one `getelementptr` from the base's address, with the member's position in the struct as the index. `origin.x` is two such steps:

```llvm
  %origin = getelementptr inbounds nuw %Shape, ptr %deref8, i32 0, i32 1
  %x9 = getelementptr inbounds nuw %Point, ptr %origin, i32 0, i32 0
  %load_x = load i32, ptr %x9, align 4
```

The struct positions come from the index. A program or function block struct also skips `VAR_TEMP` variables, which live on the stack, so the position is computed rather than read from the entry.

**Index.** The array is laid out flat, and the index expression is turned into an offset from the first element: the lower bound is subtracted, and for several dimensions each index is multiplied by the number of elements the dimensions after it hold. The result is one `getelementptr` into the array type. For `points[i]` on `ARRAY[0..3]`:

```llvm
  %load_i = load i32, ptr %i, align 4
  %tmpVar = mul i32 1, %load_i
  %tmpVar1 = add i32 %tmpVar, 0
  %tmpVar2 = getelementptr inbounds [4 x %Point], ptr %points, i32 0, i32 %tmpVar1
```

The multiplication by one and the addition of zero are the general formula applied to one dimension with lower bound zero; the optimizer removes them. No bounds check is emitted.

**Dereference.** A pointer variable holds an address, so `pShape^` loads that address from the variable's slot, and the result is the address of the target. `pShape^.points` is the load followed by the member step:

```llvm
  %deref = load ptr, ptr %pShape, align 8
  %points = getelementptr inbounds nuw %Shape, ptr %deref, i32 0, i32 2
```

**Auto-dereference.** A variable whose annotation carries an auto-deref kind gets the same load inserted without a `^` in the source. In `bump`, `target` is a stack slot that holds the caller's address, and every use loads the address first. In `main`, `rDint` is a struct member that holds an address:

```llvm
  %deref3 = load ptr, ptr %rDint, align 8
  %deref4 = load ptr, ptr %rDint, align 8
  %load_rDint = load i32, ptr %deref4, align 4
  %tmpVar5 = add i32 %load_rDint, 1
  store i32 %tmpVar5, ptr %deref3, align 4
```

The left and right side of `rDint := rDint + 1` each load the pointer once. A `REF=` assignment is the one place where an auto-deref variable is not dereferenced: `rDint REF= i` stores the address of `i` into the slot, `store ptr %i, ptr %rDint`.

**Address.** `ADR(x)` and `REF(x)` generate the address of their argument and stop there, so `pShape := ADR(shape)` is one store of the member pointer: `store ptr %shape, ptr %pShape`. When the argument names a function or a method, the address of that function is taken instead. Passing `count` to the `VAR_IN_OUT` parameter of `bump` works the same way without a call to `ADR`: the argument hint names a pointer type, so codegen passes the address, `call i32 @bump(ptr %count, i16 2)`.

**Cast.** A cast produces no instruction of its own. A cast literal is created directly in the cast type and then converted by the surrounding hint like any literal, so `i := INT#5` is `store i32 5`. A cast enum variant is a reference to a constant global; the value is known at compile time, so `color := Color#Red` is `store i32 0`. Casting a non-literal, non-identifier expression is a bit cast of the value.

**Direct access.** `word.%X0` and `word.%B1` load the whole base, shift it right by the bit offset (the index times the access width), truncate to the access type, and for a bit mask the result to one bit:

```llvm
  %load_word = load i16, ptr %word, align 2
  %shift = lshr i16 %load_word, 0
  %1 = trunc i16 %shift to i8
  %2 = and i8 %1, 1
  store i8 %2, ptr %flag, align 1
```

A direct access is therefore a value, not an address. Assigning to one, `word.%X0 := TRUE`, is handled by the assignment generator as a read-modify-write of the base.

**Global.** `.count` is the global's address, `store i32 3, ptr @count`, whatever the local scope declares.


## Validation

Each access kind has its own rule in the [validator](../pipeline/04-validation.md). A member accessed through a `POINTER TO` without `^` is E141, a dereference of something that is not a pointer is E068, an index on something that is not an array is E069, and a member of a function block that is not an input or output is a private member (E049) when accessed from outside. `REF=` accepts only a pointer or an auto-deref variable on its left and a reference on its right (E098). A `POINTER TO` declaration itself is reported as type-unsafe at information severity (E015), and the validator checks the assignment of type-safe pointers for matching inner types where `POINTER TO` accepts any address.


## At a glance

| Structured Text | Index | Annotation | LLVM |
|---|---|---|---|
| `x` | the variable entry | `Variable`, qualified name and type | the registered address of the slot or member |
| `a.b` | member `b` of the type of `a` | `Variable` on `b`, chain takes it | `getelementptr` by struct position |
| `a[i]` | the array type of `a` | `Value` of the element type | offset from the lower bound, one `getelementptr` |
| `p^` | pointer type, `auto_deref: None` | `Value` of the inner type | `load ptr` from the slot |
| `r : REFERENCE TO T`, `VAR_IN_OUT` | pointer type, `auto_deref: Reference` or `Default` | `Variable` of type `T` with an auto-deref flag | an extra `load ptr` on every use |
| `ADR(x)`, `REF(x)` | built-in functions | `Value` of `LWORD`, or of a generated `__POINTER_TO_T` | the address of `x`, no load |
| `T#v` | the type `T` | `Type` on `T`, `Value` of `T` on the whole | none; the value is created in `T` |
| `w.%Xn`, `w.%Bn` | | `Value` of `BOOL`, `BYTE`, and so on | load, shift, truncate, mask |
| `.g` | the global `g` | `Variable` with `argument_type: Global` | the global's address |
