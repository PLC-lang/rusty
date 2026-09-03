# Structs

A struct groups named members of any type into one value. Internally a struct is a memory layout and nothing more: the index keeps its members in declaration order, every access to a member is an address computation from the start of the struct, and every operation on the whole struct is a copy of its full size. A struct has no code of its own; the only code that exists for it is the constructor a participant generates from its initializers.

This chapter follows one project through the compiler:

```iecst
TYPE Point : STRUCT
    x, y : INT;
END_STRUCT END_TYPE

TYPE Rect : STRUCT
    topLeft : Point;
    bottomRight : Point := (x := 10, y := 10);
    label : STRING[5] := 'rect';
END_STRUCT END_TYPE

FUNCTION area : INT
    VAR_INPUT
        r : Rect;
    END_VAR

    area := (r.bottomRight.x - r.topLeft.x) * (r.bottomRight.y - r.topLeft.y);
END_FUNCTION

FUNCTION origin : Point
    origin := (x := 0, y := 0);
END_FUNCTION

PROGRAM main
    VAR
        r1 : Rect := (topLeft := (x := 1, y := 2));
        r2 : Rect;
        p : Point;
        a : INT;
    END_VAR

    r2 := r1;
    r2.bottomRight.x := r1.topLeft.x + 5;
    a := area(r1);
    p := origin();
END_PROGRAM
```


## Declaration

The parser turns `TYPE Rect : STRUCT ... END_STRUCT END_TYPE` into a user type declaration whose data type is a struct node: the name and a list of variables, each with its name, type, and optional initializer, exactly like the variables of a `VAR` block. A struct literal `(x := 10, y := 10)` is not a node kind of its own; the parser produces a parenthesized expression list of assignments, and only the resolver decides that the list is a struct value. The inline `STRING[5]` of `label` is moved out by pre-processing into the type `__Rect_label`, like every inline type (see [Index](../pipeline/02-index.md), Pre-processing).


## Index

The type index holds one record per struct. Trimmed to the struct variant of the type information:

```rust
Struct {
    /// The type name, `Rect`
    name: TypeId,

    /// One variable entry per member, in declaration order
    members: Vec<VariableIndexEntry>,

    /// Where the struct came from: a TYPE declaration, a POU, or an internal type
    source: StructSource,
}
```

The members are ordinary variable entries, the same record that describes a variable of a POU: a name, a qualified name such as `Rect.bottomRight`, the type name, the position in the parent, and the id of the initializer in the constant store, if there is one. For the example the index holds:

```
Point    { members: [ Point.x : INT @0,  Point.y : INT @1 ],                                   source: OriginalDeclaration }
Rect     { members: [ Rect.topLeft : Point @0,  Rect.bottomRight : Point @1 := ConstId(0),
                      Rect.label : __Rect_label @2 := ConstId(1) ],                             source: OriginalDeclaration }
```

The struct literal `(x := 10, y := 10)` is stored in the constant store as the expression it is; the constant evaluator marks it resolved but does not fold it, since it is not a scalar. `main.r1` likewise carries the id of its literal. The index knows nothing about sizes or offsets: `Rect` is a list of member names and type names, and the layout is computed by codegen from the target's data layout.

Members of a struct declared in a `TYPE` block get the argument type `Input`, because the indexer reuses the variable indexing of POUs. Nothing reads that flag for a struct.

Every POU has a struct record of its own, the instance struct with the source `Pou(Program)` or `Pou(Function)`, whose members are the POU's variables; the [POUs](00-pous.md) chapter describes it. `area` and `origin` have such a struct here even though they are functions, so that `area.r` and `origin.origin` can be looked up like any member.

> **Developer Note**
>
> The indexer also registers a global variable `__Rect__init` of type `Rect` for every struct, in a map of default-value globals. Nothing reads that map any more: codegen computes the initial value of a type directly and the init participant generates constructors. The entries survive as dead data in the index.


## Annotations

A struct itself is never an expression; what the resolver annotates are member references and struct literals. A member reference is resolved left to right, each segment under the type of the one before, and the whole reference takes the annotation of its last segment (see [Resolver](../pipeline/03-resolver.md), Walking a unit). For the second statement of `main`:

```
    r2.bottomRight.x := r1.topLeft.x + 5;
    ^^                          { kind: Variable, qualified_name: "main.r2",          resulting_type: "Rect",  hint: None }
    ^^^^^^^^^^^^^^              { kind: Variable, qualified_name: "Rect.bottomRight", resulting_type: "Point", hint: None }
    ^^^^^^^^^^^^^^^^            { kind: Variable, qualified_name: "Point.x",          resulting_type: "INT",   hint: None }
                        ^^^^^^^^^^^^^^^^^^   { kind: Value,                            resulting_type: "DINT",  hint: "INT" }
                        ^^^^^^^^^^^^        { kind: Variable, qualified_name: "Point.x", resulting_type: "INT", hint: "DINT" }
```

The qualified name of a member is always `<struct>.<member>`, never `<variable>.<member>`: `r1.topLeft.x` and `r2.bottomRight.x` both end in `Point.x`. Codegen uses the qualified name to find the member's position in its struct; the base expression tells it which struct instance to start from.

A struct literal gets no annotation of its own, only a hint with the struct type, taken from the left side of the assignment or the declared type of the variable. Under that hint the left sides of the inner assignments are looked up as members of the struct, and the right sides are hinted with the member's type:

```
    r1 : Rect := (topLeft := (x := 1, y := 2));
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  { kind: None,                                                    hint: "Rect" }
                  ^^^^^^^                        { kind: Variable, qualified_name: "Rect.topLeft", resulting_type: "Point", hint: None }
                             ^^^^^^^^^^^^^^^^    { kind: None,                                                    hint: "Point" }
                              ^                  { kind: Variable, qualified_name: "Point.x",      resulting_type: "INT",   hint: None }
                                   ^             { kind: Value,                                     resulting_type: "DINT",  hint: "INT" }
```

Nested literals are hinted recursively, so `(x := 1, y := 2)` inside the `topLeft` assignment is hinted `Point`. The same happens for a literal assigned in a body, `origin := (x := 0, y := 0)`. A member name that the struct does not have, `z := 2`, simply gets no annotation and is reported as an unresolved reference (E048) by validation.

A struct variable passed as an argument, `area(r1)`, is annotated like any argument: the variable `main.r1` of type `Rect` with an argument hint for parameter 0 of `area`. A struct assignment `r2 := r1` hints `r1` with `Rect`; the hint equals the type, and there is nothing to convert.


## Lowering

No participant rewrites structs themselves. The [init participant](../participants/06-init.md) generates a constructor `Point__ctor` and `Rect__ctor` per struct, decomposes every struct literal in a declaration into one assignment per leaf, and moves them into the constructor of the type or of the enclosing POU: `Rect__ctor` holds `self.bottomRight.x := 10` and `self.bottomRight.y := 10`, and `main__ctor` holds `self.r1.topLeft.x := 1` and `self.r1.topLeft.y := 2`. The [aggregate-return lowerer](../participants/09-aggregate-return.md) turns `origin` into a void function with a `VAR_IN_OUT origin : Point` parameter and gives the call site a temporary. A struct literal assigned in a body is left as it is.


## Codegen

**Layout.** A struct becomes a named LLVM struct type with one field per member, in declaration order, and a nested struct is embedded by value (see [Codegen](../pipeline/05-codegen.md), Data types, for the two-pass creation). A struct has no padding of its own; LLVM places the fields according to the target's data layout, which is where the size and the offsets finally come from.

```llvm
%main = type { %Rect, %Rect, %Point, i16 }
%Rect = type { %Point, %Point, [6 x i8] }
%Point = type { i16, i16 }
```

The initial value of a struct type is a constant struct whose fields are the evaluated initializers of the members, or the initial value of the member's type, or zero. A variable with a struct literal gets that literal folded into the constant of its instance:

```llvm
@main_instance = global %main {
    %Rect { %Point { i16 1, i16 2 }, %Point { i16 10, i16 10 }, [6 x i8] c"rect\00\00" },
    %Rect { %Point zeroinitializer, %Point { i16 10, i16 10 }, [6 x i8] c"rect\00\00" },
    %Point zeroinitializer,
    i16 0 }
```

`r1` shows the two sources side by side: `topLeft` comes from the variable's own literal, `bottomRight` and `label` from the defaults of `Rect`. The constructors write the same values once more at start-up, so every initialized member is stored twice, statically and by `Rect__ctor` or `main__ctor`.

**Member access.** A member reference is one `getelementptr` per segment, each starting from the pointer the previous segment produced. The indices are the position in the struct type, which codegen takes from the member's entry in the index. A read loads from the final address and a write stores to it:

```llvm
  %bottomRight = getelementptr inbounds nuw %Rect, ptr %r2, i32 0, i32 1
  %x = getelementptr inbounds nuw %Point, ptr %bottomRight, i32 0, i32 0
  %topLeft = getelementptr inbounds nuw %Rect, ptr %r1, i32 0, i32 0
  %x1 = getelementptr inbounds nuw %Point, ptr %topLeft, i32 0, i32 0
  %load_x = load i16, ptr %x1, align 2
  %1 = sext i16 %load_x to i32
  %tmpVar = add i32 %1, 5
  %2 = trunc i32 %tmpVar to i16
  store i16 %2, ptr %x, align 2
```

**Assignment.** A struct is an aggregate, so `r2 := r1` is a `memcpy` of the size of the type, computed by LLVM from the type itself. Unlike strings, nothing is cut, since both sides have the same type:

```llvm
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %r2, ptr align 1 %r1, i64 ptrtoint (ptr getelementptr (%Rect, ptr null, i32 1) to i64), i1 false)
```

A struct literal assigned in a body takes one of two paths. When every member value is a constant, the literal becomes a private constant global and is copied from there, so `origin := (x := 0, y := 0)` is a `memcpy` from `@.const_init`. When a member is a run-time value, the struct is built in registers with one `insertvalue` per assigned member and stored as a whole:

```llvm
  %load_n = load i16, ptr %n, align 2
  %1 = insertvalue %Point undef, i16 %load_n, 0
  %2 = insertvalue %Point %1, i16 2, 1
  store %Point %2, ptr %q, align 2
```

In both paths the members the literal does not name are filled with their initial value from the type, so a literal always produces a complete struct.

**Passing and returning.** A struct argument to a function is a pointer, and the callee copies the whole struct into a local of its own before the body runs; `area` therefore works on a copy of `r1`, and a write to `r.topLeft.x` inside `area` would not reach `main`. A struct result travels through the in-out pointer the aggregate-return lowerer added, into a zeroed temporary the caller provides:

```llvm
define i16 @area(ptr %0) {
entry:
  %area = alloca i16, align 4
  %r = alloca %Rect, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %r, ptr align 1 %0, i64 ptrtoint (ptr getelementptr (%Rect, ptr null, i32 1) to i64), i1 false)
  ...

define void @origin(ptr %0) {
entry:
  %origin = alloca ptr, align 8
  store ptr %0, ptr %origin, align 8
  %deref = load ptr, ptr %origin, align 8
  call void @Point__ctor(ptr %deref)
  %deref1 = load ptr, ptr %origin, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %deref1, ptr align 1 @.const_init, i64 ptrtoint (ptr getelementptr (%Point, ptr null, i32 1) to i64), i1 false)
  ret void
}

define void @main(ptr %0) {
  ...
  %call = call i16 @area(ptr %r1)
  %__origin0 = alloca %Point, align 8
  call void @llvm.memset.p0.i64(ptr align 1 %__origin0, i8 0, i64 ptrtoint (ptr getelementptr (%Point, ptr null, i32 1) to i64), i1 false)
  call void @origin(ptr %__origin0)
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %p, ptr align 1 %__origin0, i64 ptrtoint (ptr getelementptr (%Point, ptr null, i32 1) to i64), i1 false)
  ...
```

The `Point__ctor` call at the start of `origin` is the init participant constructing the return variable; for a struct with member defaults it fills the caller's storage with those defaults before the body overwrites them. `VAR_IN_OUT` structs are passed as pointers without a copy, and a `REF_TO` or `POINTER TO` a struct is an ordinary pointer whose member access adds one load in front of the address computation.

## Validation

A struct type must be finite: a struct that contains itself, directly or through other structs or arrays, is reported as a recursive data structure (E029) by the global validation from the index (see [Validation](../pipeline/04-validation.md), Global validation). A member name in a literal or an access that the struct does not declare is an unresolved reference (E048). Assigning a struct to a variable of a different type, or a scalar to a struct, is an invalid assignment (E037); two struct types are compatible only when they are the same type. Struct literals in declarations are checked member by member with the same rules as assignments.


## At a glance

| Structured Text | Index | Annotation | LLVM |
|---|---|---|---|
| `TYPE Rect : STRUCT ... END_STRUCT END_TYPE` | `Struct { members, source: OriginalDeclaration }`, one variable entry per member | | `%Rect = type { ... }`, fields in declaration order |
| `topLeft : Point;` inside a struct | member entry `Rect.topLeft`, position 0 | | field 0, embedded by value |
| `r1.topLeft.x` | | each segment `Variable`, last one `Point.x` of type `INT` | one `getelementptr` per segment, then `load` or `store` |
| `(x := 1, y := 2)` | expression in the constant store, if in a declaration | no annotation, hint `Point`; members resolved under that hint | folded into the instance constant; in a body, `memcpy` from a constant or `insertvalue` chain |
| `r2 := r1` | | `r1` hinted `Rect` | `memcpy` of `sizeof(%Rect)` |
| `area(r1)` | `area.r` of type `Rect` | `r1` hinted as argument 0 | `ptr`, copied into a local `%Rect` in the callee |
| `FUNCTION origin : Point` | `origin.origin` return member of type `Point` | | void function with a `ptr` result parameter |
