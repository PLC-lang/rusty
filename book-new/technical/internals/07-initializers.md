# Initializers

A declaration can give a variable or a type a value to start with: `i : DINT := 1`, `TYPE MyInt : INT := 7`, `p : Point := (y := 2)`, `ptr : POINTER TO DINT := ADR(i)`. Internally an initializer is an expression that the compiler tries to fold into a constant while it builds the symbol table. A value that folds becomes static data of the instance, and every value, folded or not, is also written again by a generated constructor at start-up, or by a statement at the start of the body when the variable lives on the stack. For each kind of initializer the chapter answers one question: which of these three places does its value reach.

This chapter follows one project through the compiler:

```iecst
VAR_GLOBAL CONSTANT
    MAX : DINT := 3;
END_VAR

VAR_GLOBAL
    gCount : DINT := MAX + 1;
END_VAR

TYPE MyInt : INT := 7; END_TYPE

TYPE Point : STRUCT
    x : DINT := 1;
    y : DINT;
END_STRUCT END_TYPE

FUNCTION_BLOCK Counter
    VAR_INPUT
        step : DINT := 1;
    END_VAR
    VAR
        count : DINT;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION pick : DINT
    VAR
        local : DINT := 5;
    END_VAR

    pick := local;
END_FUNCTION

PROGRAM main
    VAR
        i : DINT := MAX + 1;
        n : MyInt;
        p : Point := (y := 2);
        values : ARRAY[0..2] OF DINT := [1, 2, 3];
        counter : Counter := (step := 10);
        ptr : POINTER TO DINT := ADR(i);
        r : REFERENCE TO DINT REF= i;
        readings : ARRAY[0..1] OF DINT := [pick(), 2];
    END_VAR
    VAR_TEMP
        t : DINT := 4;
    END_VAR
END_PROGRAM
```


## Declaration

The parser keeps the initializer as an ordinary expression node on the variable or on the type declaration. Nothing is special about it yet: `MAX + 1` is a binary expression, `(y := 2)` is a parenthesized assignment, `[1, 2, 3]` is an array literal, `ADR(i)` is a call, and `REF= i` is stored as the reference `i` with the declaration marked as a reference initializer. Pre-processing at the start of the index stage moves inline types such as `ARRAY[0..2] OF DINT` out into named types (`__main_values`), but the initializer stays on the variable, not on the new type.


## Index

Every initializer goes into the constant store of the index, the same arena that holds array bounds and string sizes (see [Index](../pipeline/02-index.md), Constant evaluation). The store hands out an id, and the id is what the entries keep: a variable entry stores it in `initial_value`, a type stores it in `initial_value` when the type declaration has a default. Trimmed, one entry of the store is:

```rust
pub enum ConstExpression {
    /// Not evaluated yet; scope is the POU the expression was written in, lhs the variable it initializes
    Unresolved { statement: AstNode, scope: Option<String>, lhs: Option<String> },

    /// Folded to a literal, or accepted as is for a struct or array literal
    Resolved(AstNode),

    /// Cannot become static data; the reason says why and what to do instead
    Unresolvable { statement: AstNode, reason: Box<UnresolvableKind> },
}

pub enum UnresolvableKind {
    /// Not a constant, and never will be; reported by validation
    Misc(String),

    /// A literal that does not fit the target type; reported as a warning
    Overflow(String, SourceLocation),

    /// An address, which exists only after codegen has laid out the memory
    Address(InitData),
}
```

Each entry also records the target type name, so that the evaluator knows in which type to fold and whether the result fits. For the example, the store holds eighteen entries: one per initializer and four for the array bounds `0`, `2`, `0`, `1`. After evaluation, the initializers read:

```
MAX                 DINT             Resolved(3)
gCount              DINT             Resolved(4)                        folded from MAX + 1
MyInt               MyInt            Resolved(7)                        the type's default
Point.x             DINT             Resolved(1)
Counter.step        DINT             Resolved(1)
pick.local          DINT             Resolved(5)
main.i              DINT             Resolved(4)                        folded from MAX + 1
main.p              Point            Resolved(y := 2)                   struct literal, kept as written
main.values         __main_values    Resolved([1, 2, 3])                array literal, kept as written
main.counter        Counter          Resolved(step := 10)
main.ptr            __main_ptr       Unresolvable(ADR(i), Address)      "Try to re-resolve during codegen"
main.r              __main_r         Unresolvable(i, Address)           "Try to re-resolve during codegen"
main.readings       __main_readings  Unresolvable([pick(), 2], Misc)    "Call-statement 'pick' in initializer is not constant."
main.t              DINT             Resolved(4)
```

The evaluator works through the store as a queue. A literal is resolved at once, after a check that it fits the target type; a reference to a constant is replaced by that constant's resolved value, or sent to the back of the queue when the constant is not resolved yet, which is how `gCount` and `main.i` become `4` in the second pass. A reference to a variable that is not constant is unresolvable, "`x` is no const reference", unless the target is a pointer type; then it is an address. `ADR`, `REF`, and a bare reference initializing a `REFERENCE TO` are addresses too. A call to a user function is a plain unresolvable, since the compiler does not execute code at compile time. Struct and array literals are accepted as resolved without folding their elements; an element of an array literal that is not constant makes the whole literal unresolvable, as `readings` shows. A variable without an initializer has no entry at all. Its value comes from its type: the type's own default (`n : MyInt` starts at `7`), or zero.

> **Developer Note**
>
> The indexer also registers a default-instance global named `__<Type>__init` for every struct, program, function block, and class in a map of its own (`__Point__init`, `__Counter__init` here). Nothing outside unit tests reads that map; codegen computes default values from the type index and the constructors, and no such global appears in the IR. Logged in `bugs.md`.


## Annotations

The resolver visits initializers like any other expression, with the declaring POU as context, and hints them with the declared type of the variable (see [Resolver](../pipeline/03-resolver.md)). Struct literals get no annotation of their own, only the hint; their assignments resolve the member name against the target type, not against the current POU:

```
    i : DINT := MAX + 1;
                ^^^^^^^        { kind: Value,                                   resulting_type: "DINT",  hint: "DINT" }
                ^^^            { kind: Variable, qualified_name: "MAX", constant: true, resulting_type: "DINT", hint: None }

    p : Point := (y := 2);
                 ^^^^^^^^      { kind: None,                                                             hint: "Point" }
                  ^            { kind: Variable, qualified_name: "Point.y",      resulting_type: "DINT",  hint: None }
                       ^       { kind: Value,                                   resulting_type: "DINT",  hint: "DINT" }

    ptr : POINTER TO DINT := ADR(i);
                             ^^^^^^   { kind: Value,                                resulting_type: "LWORD", hint: "__main_ptr" }
                                 ^    { kind: Variable, qualified_name: "main.i",   resulting_type: "DINT",  hint: "DINT" }

    r : REFERENCE TO DINT REF= i;
                               ^      { kind: Variable, qualified_name: "main.i",   resulting_type: "DINT",  hint: "__main_r" }

TYPE MyInt : INT := 7; END_TYPE
                    ^                 { kind: Value,                                resulting_type: "DINT",  hint: "INT" }
```

`ADR(i)` is a `LWORD` value hinted to the pointer type `__main_ptr`; `REF= i` is the variable itself hinted to the reference type. Both hints tell codegen to store an address rather than a value. The elements of `[1, 2, 3]` are each hinted `DINT`, the literal as a whole `__main_values`.


## Lowering

Two participants act on initializers. The [init participant](../participants/06-init.md) turns every initializer of a stateful POU or type into an assignment inside a generated constructor, `main__ctor`, `Point__ctor`, `MyInt__ctor`, and every initializer of a function local or `VAR_TEMP` into a statement at the start of the body; it also removes the array literal of `readings` from the declaration, so that the index no longer sees an unresolvable entry there. The [array lowerer](../participants/11-array.md) then splits the assignment `self.readings := [pick(), 2]` in the constructor into one assignment per element. Everything else stays where it is: the resolved entries are still in the store, and codegen reads them for the static data.


## Codegen

**Static data.** Every global and every program instance gets a compile-time initial value. For a variable, codegen takes the resolved expression from the store; when there is none, the default of its type; when the type has none, zero. Unresolvable entries count as none. The instance of `main` and the globals are therefore complete before any code runs, apart from the two addresses and the array with the call:

```llvm
@MAX = unnamed_addr constant i32 3
@gCount = global i32 4
@main_instance = global %main { i32 4, i16 7, %Point { i32 1, i32 2 }, [3 x i32] [i32 1, i32 2, i32 3], %Counter { ptr null, i32 10, i32 0 }, ptr null, ptr null, [2 x i32] zeroinitializer }
```

`i` is `4` from the folded expression, `n` is `7` from the default of `MyInt`, `p` is `{1, 2}` from the struct's own default for `x` and the literal for `y`, and `counter` is `{null, 10, 0}` from `Counter`'s default `step := 1` overridden by the literal. A struct's default value is computed once per type from its members' initializers and reused for every instance of it. For every local or temporary member of aggregate type with an initializer, codegen also emits a constant named after the member, `__main.values__init`, that holds the value. When such a variable lives on the stack, in a function or a `VAR_TEMP` block, its slot is initialized with a `memcpy` from that constant instead of element by element; for a member of a program or function block the constant is emitted but not used, since the value is already in the instance.

**Constructor.** The generated constructor writes the same values again at start-up, and this is the only place where the three unresolvable initializers get their value. The relevant statements of `main__ctor`, trimmed:

```llvm
  store i32 4, ptr %i, align 4
  call void @MyInt__ctor(ptr %n)
  call void @Point__ctor(ptr %p)
  store i32 2, ptr %y, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %values, ptr align 1 @.const_init, i64 ptrtoint (ptr getelementptr ([3 x i32], ptr null, i32 1) to i64), i1 false)
  call void @Counter__ctor(ptr %counter)
  store i32 10, ptr %step, align 4
  store ptr %i, ptr %ptr, align 8
  store ptr %i, ptr %r, align 8
  %call = call i32 @pick()
  store i32 %call, ptr %tmpVar, align 4
  store i32 2, ptr %tmpVar27, align 4
```

The type constructor runs before the member's own literal, so `Point__ctor` sets `x := 1` and the following store sets `y := 2`. `ADR(i)` and `REF= i` become stores of the address of `i`, and `pick()` is called once, at start-up, not on every cycle. The unit constructor stores `gCount := 4` in the same way and then calls `main__ctor` on the instance (see [Codegen](../pipeline/05-codegen.md), Initialization).

**Stack.** A function local, a return variable, and a `VAR_TEMP` have no instance. Codegen stores their initial value when it creates the stack slot at the start of the body, and the statement the init participant prepended stores it again:

```llvm
define i32 @pick() {
entry:
  %pick = alloca i32, align 4
  %local = alloca i32, align 4
  store i32 5, ptr %local, align 4
  store i32 0, ptr %pick, align 4
  store i32 5, ptr %local, align 4
  ...
```

A value with a resolved entry is therefore written twice: once as static data and once by the constructor, or twice at the start of a body for a stack variable. The redundancy is harmless and LLVM removes most of it at `-O1`. Only the constructor path knows how to write addresses and call functions, and only static data survives without the constructor running, which is what an object linked into a foreign program relies on.


## Validation

The validator turns the states of the store into diagnostics. An `Unresolvable` entry with a `Misc` reason is an error at the initializer, "Unresolved constant `chosen` variable: Call-statement 'pick' in initializer is not constant." (E033); the same code is reported for a `CONSTANT` variable that has no initializer at all, and for an entry that is still `Unresolved` after evaluation, which happens when constants reference each other in a cycle. An `Overflow` reason is a warning. An `Address` reason is not an error; the validator instead checks that the pointed-to type matches the declared pointer type, and warns when the address is that of a temporary. A call in the initializer of a scalar is therefore rejected, while the same call inside an array literal is accepted, because the init participant removes that literal from the declaration before validation runs and the array lowerer turns it into element assignments.


## At a glance

| Initializer | Constant store | Static data | Constructor or body |
|---|---|---|---|
| `i : DINT := 1` | `Resolved(1)` | `i32 1` in the instance | `store i32 1` |
| `i : DINT := MAX + 1` | `Resolved(4)`, folded in a later pass | `i32 4` | `store i32 4` |
| `TYPE MyInt : INT := 7` | `Resolved(7)` on the type | default for every `MyInt` without its own initializer | `MyInt__ctor` stores 7 |
| `p : Point := (y := 2)` | `Resolved`, literal kept as written | struct constant, type default for the other members | `Point__ctor(p)` then `store` per member |
| `values : ARRAY := [1, 2, 3]` | `Resolved`, literal kept | array constant | `memcpy` from a constant |
| `counter : Counter := (step := 10)` | `Resolved`, literal kept | struct constant with the FB's defaults | `Counter__ctor` then `store` |
| `ptr := ADR(i)` | `Unresolvable(Address)` | `ptr null` | `store ptr %i` |
| `r REF= i` | `Unresolvable(Address)` | `ptr null` | `store ptr %i` |
| `readings := [pick(), 2]` | `Unresolvable(Misc)`, removed by lowering | `zeroinitializer` | one `store` per element, the call runs once |
| `local : DINT := 5` in a function | `Resolved(5)` | none | two `store` at the start of the body |
| `t : DINT := 4` in `VAR_TEMP` | `Resolved(4)` | none | two `store` at the start of the body |
| `chosen : DINT := pick()` | `Unresolvable(Misc)` | rejected, E033 | |
