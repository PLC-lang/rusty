# Structs

A struct groups named members into one value. The index keeps the members in declaration order, and codegen builds an LLVM type with the fields in that order. Member access computes an address; assignment between struct variables copies the whole value. The [init participant](../participants/06-init.md) supplies constructor code for defaults and initializers.

The example extends the instance-layout model from [POUs](00-pous.md) to nested data. It shows member defaults, a whole-struct copy, a function argument, and a struct return:

```iecst
TYPE Point:
    STRUCT
        x, y: INT;
    END_STRUCT
END_TYPE

TYPE Rect:
    STRUCT
        topLeft: Point;
        bottomRight: Point := (x := 10, y := 10);
        label: STRING[5] := 'rect';
    END_STRUCT
END_TYPE

FUNCTION area: INT
    VAR_INPUT
        r: Rect;
    END_VAR

    area := (r.bottomRight.x - r.topLeft.x) * (r.bottomRight.y - r.topLeft.y);
END_FUNCTION

FUNCTION origin: Point
    origin := (x := 0, y := 0);
END_FUNCTION

PROGRAM main
    VAR
        r1: Rect := (topLeft := (x := 1, y := 2));
        r2: Rect;
        p: Point;
        a: INT;
    END_VAR

    r2 := r1;
    r2.bottomRight.x := r1.topLeft.x + 5;
    a := area(r1);
    p := origin();
END_PROGRAM
```


## Declaration

A struct declaration contains a name and a list of variables. Each variable has a name, type, and optional initializer, as in a POU variable block.

A struct literal `(x := 10, y := 10)` is not a node kind of its own. The parser produces a parenthesized expression list of assignments, and only the resolver decides that the list is a struct value. The inline `STRING[5]` of `label` is moved out by pre-processing into the type `__Rect_label`, like every inline type (see [Index](../pipeline/02-index.md), Pre-processing).


## Index

The type index holds one record per struct. Trimmed to the struct variant of the type information:

```rust,noplayground
Struct {
    /// The type name, `Rect`
    name: TypeId,

    /// One variable entry per member, in declaration order
    members: Vec<VariableIndexEntry>,

    /// Where the struct came from: a TYPE declaration, a POU, or an internal type
    source: StructSource,
}
```

Struct members use the same variable records as POU members. Each record identifies the member, its type, its position, and any initializer in the constant store:

```
Point    { members: [ Point.x : INT @0,  Point.y : INT @1 ],                                   source: OriginalDeclaration }
Rect     { members: [ Rect.topLeft : Point @0,  Rect.bottomRight : Point @1 := ConstId(0),
                      Rect.label : __Rect_label @2 := ConstId(1) ],                             source: OriginalDeclaration }
```

The constant store keeps `(x := 10, y := 10)` as a struct expression. `main.r1` has a separate entry for its own literal. The index stores member and type names; codegen later computes sizes and offsets for the target.

Members of a struct declared in a `TYPE` block get the argument type `Input`, because the indexer reuses the variable indexing of POUs. Nothing reads that flag for a struct.

Every POU has a struct record of its own, the instance struct with the source `Pou(Program)` or `Pou(Function)`, whose members are the variables of the POU; the [POUs](00-pous.md) chapter describes it. `area` and `origin` have such a struct here although they are functions, so that `area.r` and `origin.origin` can be looked up like any member.

> [!NOTE]
> **Developer note.** The indexer registers `__Rect__init` in an unused map of default-value globals. Codegen computes defaults from the type index instead. See [Initializers](07-initializers.md).


## Annotations

A struct itself is never an expression. What the resolver annotates are member references and struct literals. A member reference is resolved left to right, each segment under the type of the one before, and the whole reference takes the annotation of its last segment (see [Resolver](../pipeline/03-resolver.md), Walking a unit). For the second statement of `main`:

```
    r2.bottomRight.x := r1.topLeft.x + 5;
    ^^                          { kind: Variable, qualified_name: "main.r2",          resulting_type: "Rect",  hint: None }
    ^^^^^^^^^^^^^^              { kind: Variable, qualified_name: "Rect.bottomRight", resulting_type: "Point", hint: None }
    ^^^^^^^^^^^^^^^^            { kind: Variable, qualified_name: "Point.x",          resulting_type: "INT",   hint: None }
                        ^^^^^^^^^^^^^^^^    { kind: Value,                            resulting_type: "DINT",  hint: "INT" }
                        ^^^^^^^^^^^^        { kind: Variable, qualified_name: "Point.x", resulting_type: "INT", hint: "DINT" }
```

The qualified name of a member is always `<struct>.<member>`, never `<variable>.<member>`: `r1.topLeft.x` and `r2.bottomRight.x` both end in `Point.x`. Codegen uses the qualified name to find the member's position in its struct; the base expression tells it which struct instance to start from.

A struct literal gets no annotation of its own, only a hint with the struct type, taken from the left side of the assignment or from the declared type of the variable. Under that hint the left side of every inner assignment is looked up as a member of the struct, and the right side is hinted with the type of that member:

```
    r1: Rect := (topLeft := (x := 1, y := 2));
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^  { kind: None,                                                    hint: "Rect" }
                 ^^^^^^^                        { kind: Variable, qualified_name: "Rect.topLeft", resulting_type: "Point", hint: None }
                            ^^^^^^^^^^^^^^^^    { kind: None,                                                    hint: "Point" }
                             ^                  { kind: Variable, qualified_name: "Point.x",      resulting_type: "INT",   hint: None }
                                  ^             { kind: Value,                                     resulting_type: "DINT",  hint: "INT" }
```

Nested literals are hinted recursively, so `(x := 1, y := 2)` inside the `topLeft` assignment is hinted `Point`. The same happens for a literal assigned in a body, `origin := (x := 0, y := 0)`. A member name the struct does not have, `z := 2`, gets no annotation at all.

A struct variable passed as an argument, `area(r1)`, is annotated like any argument: the variable `main.r1` of type `Rect` with an argument hint for parameter 0 of `area`. A struct assignment `r2 := r1` hints `r1` with `Rect`; the hint equals the type, and there is nothing to convert.


## Lowering

Two participants rewrite the tree before codegen sees it. The [init participant](../participants/06-init.md) creates `Point__ctor` and `Rect__ctor`. It splits declaration literals into assignments for individual fields. Thus `Rect__ctor` sets `self.bottomRight.x` and `.y` to 10. `main__ctor` sets `self.r1.topLeft.x := 1` and `self.r1.topLeft.y := 2`.

The [aggregate-return lowerer](../participants/09-aggregate-return.md) turns `origin` into a void function with a `VAR_IN_OUT origin: Point` parameter and gives the call site a temporary. A struct literal assigned in a body is left as it is.


## Codegen

### Layout

Each struct becomes a named LLVM type with fields in declaration order. Nested structs are embedded by value. LLVM computes padding and offsets from the target data layout. The [Codegen](../pipeline/05-codegen.md#data-types) chapter explains the two-pass creation:

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

`r1.topLeft` takes its value from the variable's literal. `bottomRight` and `label` use the defaults of `Rect`. Constructors apply these initial values again at startup, as described in [Initializers](07-initializers.md).

### Member access

A member reference is one `getelementptr` per segment, each starting from the pointer the previous segment produced. The indices are the position in the struct type, which codegen takes from the member's entry in the index. A read loads from the final address and a write stores to it:

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

### Assignment

A struct is an aggregate, so `r2 := r1` is a `memcpy` of the size of the type, computed by LLVM from the type itself. Unlike strings, nothing is cut, since both sides have the same type:

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

### Passing and returning

A by-value struct argument is passed by pointer, then copied into a local in the callee. Thus `area` works on a copy of `r1`; changing `r.topLeft.x` does not change `main.r1`. A struct return uses the in-out result pointer added by aggregate-return lowering:

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

The `Point__ctor` call at the start of `origin` is the init participant constructing the return variable. For a struct with member defaults it fills the caller's storage with those defaults before the body overwrites them.

A `VAR_IN_OUT` struct is passed as a pointer without a copy, and a `REF_TO` or `POINTER TO` a struct is an ordinary pointer whose member access adds one load in front of the address computation.


## Validation

Codegen can only lay out a type whose size is known, so a struct type must be finite. A struct that holds itself, directly or through other structs or arrays, is reported as a recursive data structure (E029) by the global validation from the index (see [Validation](../pipeline/04-validation.md), Global validation).

A member name in a literal or an access that the struct does not declare is an unresolved reference (E048). A struct assigned to a variable of a different type, or a scalar assigned to a struct, is an invalid assignment (E037); two struct types are compatible only when they are the same type. A struct literal in a declaration is checked member by member with the same rules as an assignment.


## At a glance

| Structured Text | Index | Annotation | LLVM |
|---|---|---|---|
| `TYPE Rect: STRUCT ... END_STRUCT END_TYPE` | `Struct { members, source: OriginalDeclaration }`, one variable entry per member | | `%Rect = type { ... }`, fields in declaration order |
| `topLeft: Point;` inside a struct | member entry `Rect.topLeft`, position 0 | | field 0, embedded by value |
| `r1.topLeft.x` | | each segment `Variable`, last one `Point.x` of type `INT` | one `getelementptr` per segment, then `load` or `store` |
| `(x := 1, y := 2)` | expression in the constant store, if in a declaration | no annotation, hint `Point`; members resolved under that hint | folded into the instance constant; in a body, `memcpy` from a constant or `insertvalue` chain |
| `r2 := r1` | | `r1` hinted `Rect` | `memcpy` of `sizeof(%Rect)` |
| `area(r1)` | `area.r` of type `Rect` | `r1` hinted as argument 0 | `ptr`, copied into a local `%Rect` in the callee |
| `FUNCTION origin: Point` | `origin.origin` return member of type `Point` | | void function with a `ptr` result parameter |
