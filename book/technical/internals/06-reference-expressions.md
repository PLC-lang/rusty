# Reference Expressions

A reference expression connects a name to a variable, member, element, pointer target, or part of a value. In `pShape^.points[i].x`, each segment performs one step. The resolver identifies its declaration or type; codegen uses it to compute an address or value. This chapter connects the member and array accesses from the preceding chapters.

The example combines member access, array indexing, pointers, casts, direct bit access, and an explicit global reference:

```iecst
TYPE Point:
    STRUCT
        x: DINT;
        y: DINT;
    END_STRUCT
END_TYPE

TYPE Color: (Red, Green, Blue); END_TYPE

FUNCTION_BLOCK Shape
    VAR_INPUT
        origin: Point;
    END_VAR
    VAR_OUTPUT
        points: ARRAY[0..3] OF Point;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION bump: DINT
    VAR_IN_OUT
        target: DINT;
    END_VAR
    VAR_INPUT
        step: INT;
    END_VAR

    target := target + step;
    bump := target;
END_FUNCTION

VAR_GLOBAL
    count: DINT;
END_VAR

PROGRAM main
    VAR
        shape: Shape;
        pShape: POINTER TO Shape;
        pDint: REF_TO DINT;
        rDint: REFERENCE TO DINT;
        i: DINT;
        paint: Color;
        word: WORD;
        flag: BOOL;
        count: DINT;
    END_VAR

    pShape := ADR(shape);
    pShape^.points[i].x := 1;
    pDint := REF(i);
    rDint REF= i;
    rDint := rDint + 1;
    i := bump(count, 2);
    paint := Color#Red;
    i := INT#5;
    flag := word.%X0;
    i := word.%B1;
    .count := 3;
    i := pShape^.origin.x + shape.points[2].y;
END_PROGRAM
```


## Declaration

The parser has one node kind for every reference. It holds an access, which is the kind of step, and an optional base, which is the reference the step is applied to.

The parser reads a chain from left to right, wrapping each new step around the previous result. Thus the root of `pShape^.points[i].x` is `x`, whose base is `[i]`, then `points`, then `^`, then `pShape`. A plain name is a member step with no base. Each step can inspect its base to find the type it operates on.

Six kinds of access exist:

| Access | Spelling | Holds |
|---|---|---|
| Member | `a`, `a.b` | the identifier, or a direct access such as `%X0` |
| Index | `a[i]`, `a[i, j]` | the index expression, or a list of them |
| Dereference | `p^` | nothing; the base is the pointer |
| Cast | `INT#5`, `Color#Red` | the value to cast; the base is the type name |
| Global | `.count` | the identifier, looked up in the global scope only |
| Address | none | reserved; the parser never produces it |

A cast takes only the name that follows the `#`, and the chain goes on from the cast: `INT#a.b` means `(INT#a).b`, so a member step after a cast applies to the result of the cast. `x.5` is a shorthand for `x.%X5` and becomes a bit access. Pointer declarations are inline type definitions, so pre-processing moves `POINTER TO Shape` out into the named type `__main_pShape` before indexing, like every other anonymous type.

> [!NOTE]
> **Developer note.** The address access exists in the node kind, in the resolver, in codegen, and in the validator, but no spelling reaches it: the parser rejects `&i`, and no lowering creates it. The address of a variable is taken with the built-in calls `ADR` and `REF`, described below.


## Index

The index does not record body references. It supplies their declarations and types, including the pointer types used for explicit and automatic dereferencing:

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

The auto-dereference marker distinguishes explicit pointers from implicit references. `Default` marks by-reference parameters, `Reference` marks `REFERENCE TO`, and `Alias` marks `x AT y`. These variables read and write through their pointers without an explicit `^`:

```
__main_pShape           { inner_type_name: "Shape", auto_deref: None,            type_safe: false }   from POINTER TO Shape
__main_pDint            { inner_type_name: "DINT",  auto_deref: None,            type_safe: true }    from REF_TO DINT
__main_rDint            { inner_type_name: "DINT",  auto_deref: Some(Reference), type_safe: true }    from REFERENCE TO DINT
__auto_pointer_to_DINT  { inner_type_name: "DINT",  auto_deref: Some(Default),   type_safe: true }    for bump.target
```

The indexer creates pointer types for `VAR_IN_OUT`, `VAR_INPUT {ref}`, and function `VAR_OUTPUT` parameters. Thus `bump.target` uses `__auto_pointer_to_DINT`. By-reference parameters with the same target type share this generated type, whose inner type preserves the declared `DINT`.

Enum variants are the other index entries a reference can land on. `Color.Red` is a constant global variable of type `Color` with the initializer `0`, and it is also findable under the bare name `Red`, which is what lets `paint := Red` and `Color#Red` both resolve.


## Annotations

The resolver visits the base of a reference first, then applies the step. Each segment gets its own annotation, and the whole chain takes the annotation of its last segment (see [Resolver](../pipeline/03-resolver.md), Walking a unit, for the lookup order of a plain name). What a step produces depends on its kind:

- **Member with a base.** The type of the base becomes the qualifier, and the member is looked up inside that type, following `EXTENDS` chains and properties. The annotation is the variable entry: its qualified name, type, and auto-deref kind.
- **Member without a base.** The lookup order is a member of the current POU, then a global or enum variant, then a POU, then a type. When the reference is the operator of a call, functions are tried first.
- **Index.** The base must be an array; the step is annotated as a value of the element type. The index expression is resolved on its own and gets no hint; codegen widens it to `DINT` before the offset arithmetic.
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

The address of a variable is not a reference step but a call. `ADR(shape)` is annotated as a call of the built-in `ADR` returning `LWORD`, an integer wide enough for any address, and the assignment hints it to `__main_pShape`. `REF(i)` is typed: its annotation reads the type of the argument and registers a pointer type to it on the fly, `__POINTER_TO_DINT`, in the resolver's own index, and the call is a value of that type. Casts and direct accesses complete the picture:

```
    pShape := ADR(shape);
              ^^^^^^^^^^          { kind: Value,                                     resulting_type: "LWORD",            hint: "__main_pShape" }
    pDint := REF(i);
             ^^^^^^               { kind: Value,                                     resulting_type: "__POINTER_TO_DINT", hint: "__main_pDint" }
    paint := Color#Red;
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


## Lowering

No participant rewrites reference expressions as such, but several rewrite what they refer to. The [inheritance lowerer](../participants/10-inheritance.md) inserts `__<Base>` member steps in front of inherited members. The [property lowerer](../participants/02-property.md) replaces a member step that names a property with a call. The [polymorphism lowerer](../participants/03-polymorphism.md) turns a method call through a pointer into a dereference of a function pointer. Codegen sees only the six access kinds above.


## Codegen

For a reference to storage, codegen computes the address from the base outward and loads only when a value is needed. Casts and direct accesses have separate value rules, described below. See [Codegen](../pipeline/05-codegen.md#expressions).

### Member

A plain name is the address the function setup registered for it: a member pointer into the instance struct for a program or function block variable, a stack slot for a function variable, a global otherwise. A member with a base is one `getelementptr` from the address of the base, with the position of the member in the struct as the index. `origin.x` is two such steps:

```llvm
  %origin = getelementptr inbounds nuw %Shape, ptr %deref8, i32 0, i32 1
  %x9 = getelementptr inbounds nuw %Point, ptr %origin, i32 0, i32 0
  %load_x = load i32, ptr %x9, align 4
```

The struct positions come from the index. A program or function block struct also skips `VAR_TEMP` variables, which live on the stack, so the position is computed rather than read from the entry.

### Index

Array access subtracts the lower bound from each index, then multiplies by the lengths of later dimensions. The sum is the flat offset used by `getelementptr`. For `points[i]` on `ARRAY[0..3]`:

```llvm
  %load_i = load i32, ptr %i, align 4
  %tmpVar = mul i32 1, %load_i
  %tmpVar1 = add i32 %tmpVar, 0
  %tmpVar2 = getelementptr inbounds [4 x %Point], ptr %points, i32 0, i32 %tmpVar1
```

The multiplication by one and the addition of zero are the general formula applied to one dimension with lower bound zero; the optimizer removes them. No bounds check is emitted.

### Dereference

A pointer variable holds an address, so `pShape^` loads that address from the variable's slot, and the result is the address of the target. `pShape^.points` is the load followed by the member step:

```llvm
  %deref = load ptr, ptr %pShape, align 8
  %points = getelementptr inbounds nuw %Shape, ptr %deref, i32 0, i32 2
```

### Auto-dereference

A variable whose annotation carries an auto-deref kind gets the same load inserted without a `^` in the source. In `bump`, `target` is a stack slot that holds the caller's address, and every use loads the address first. In `main`, `rDint` is a struct member that holds an address:

```llvm
  %deref3 = load ptr, ptr %rDint, align 8
  %deref4 = load ptr, ptr %rDint, align 8
  %load_rDint = load i32, ptr %deref4, align 4
  %tmpVar5 = add i32 %load_rDint, 1
  store i32 %tmpVar5, ptr %deref3, align 4
```

The left and right side of `rDint := rDint + 1` each load the pointer once. A `REF=` assignment is the one place where an auto-deref variable is not dereferenced: `rDint REF= i` stores the address of `i` into the slot, `store ptr %i, ptr %rDint`.

### Address

`ADR(x)` and `REF(x)` generate the address of their argument and stop there, so `pShape := ADR(shape)` is one store of the member pointer: `store ptr %shape, ptr %pShape`. When the argument names a function or a method, the address of that function is taken instead. Passing `count` to the `VAR_IN_OUT` parameter of `bump` works the same way without a call to `ADR`: the argument hint names a pointer type, so codegen passes the address, `call i32 @bump(ptr %count, i16 2)`.

### Cast

A typed literal is created in the named type, then converted as required by its hint. Thus `i := INT#5` produces `store i32 5`. An enum variant can be folded into a constant, so `paint := Color#Red` produces `store i32 0`. A cast of a non-literal, non-identifier expression uses a bit cast.

### Direct access

`word.%X0` and `word.%B1` load the whole base, shift it right by the bit offset (the index times the access width), truncate to the access type, and for a bit mask the result to one bit:

```llvm
  %load_word = load i16, ptr %word, align 2
  %shift = lshr i16 %load_word, 0
  %1 = trunc i16 %shift to i8
  %2 = and i8 %1, 1
  store i8 %2, ptr %flag, align 1
```

A direct access is therefore a value, not an address. Assigning to one, `word.%X0 := TRUE`, is handled by the assignment generator as a read-modify-write of the base.

### Global

`.count` is the global's address, `store i32 3, ptr @count`, whatever the local scope declares.


## Validation

[Validation](../pipeline/04-validation.md) checks each access kind. Missing `^` on a `POINTER TO` member access is E141. Dereferencing a non-pointer is E068; indexing a non-array is E059. Access to a private function block member from outside is E049. For `REF=`, the left side must be a pointer or auto-dereferenced variable, and the right side a reference (E098).

The parser reports a `POINTER TO` declaration as type-unsafe (E015), but that code is registered as ignored, so nothing is printed. The validator checks an assignment between type-safe pointers for matching inner types (E090), where `POINTER TO` accepts any address.


## At a glance

| Structured Text | Index | Annotation | LLVM |
|---|---|---|---|
| `x` | the variable entry | `Variable`, qualified name and type | the registered address of the slot or member |
| `a.b` | member `b` of the type of `a` | `Variable` on `b`, chain takes it | `getelementptr` by struct position |
| `a[i]` | the array type of `a` | `Value` of the element type | offset from the lower bound, one `getelementptr` |
| `p^` | pointer type, `auto_deref: None` | `Value` of the inner type | `load ptr` from the slot |
| `r: REFERENCE TO T`, `VAR_IN_OUT` | pointer type, `auto_deref: Reference` or `Default` | `Variable` of type `T` with an auto-deref flag | an extra `load ptr` on every use |
| `ADR(x)`, `REF(x)` | built-in functions | `Value` of `LWORD`, or of a generated `__POINTER_TO_T` | the address of `x`, no load |
| `T#v` | the type `T` | `Type` on `T`, `Value` of `T` on the whole | none; the value is created in `T` |
| `w.%Xn`, `w.%Bn` | | `Value` of `BOOL`, `BYTE`, and so on | load, shift, truncate, mask |
| `.g` | the global `g` | `Variable` with `argument_type: Global` | the global's address |
