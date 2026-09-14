# Enums

An enum gives names to integer values. `TYPE Color: (Red, Green, Blue := 5); END_TYPE` defines three variants of `Color`. Codegen uses the underlying integer type, `DINT` by default, and folds variant references into constant values.

The example shows explicit values, type defaults, and different ways to name a variant. It also includes a cross-enum assignment to show the validation rules:

```iecst
TYPE Color: (Red, Green, Blue := 5); END_TYPE

TYPE State: (Open := 1, Closed := 4, Idle, Running) BYTE := Closed; END_TYPE

TYPE Door: (Open := 8, Closed := 16); END_TYPE

FUNCTION isGreen: BOOL
    VAR_INPUT
        color: Color;
    END_VAR

    isGreen := color = Green;
END_FUNCTION

PROGRAM main
    VAR
        paint: Color;
        state: State;
        mode: (Manual, Auto) := Auto;
        flag: BOOL;
    END_VAR

    paint := Blue;
    paint := Color#Red;
    paint := Color.Green;
    state := Door#Closed;
    state := Closed;
    flag := isGreen(paint);
    flag := paint = Green;
    flag := state <> Idle;
END_PROGRAM
```


## Declaration

The enum node records its name, underlying integer type, and variant list. The default underlying type is `DINT`. Each variant is either an identifier such as `Red` or an assignment such as `Blue := 5`.

The two spellings of the underlying type, `: BYTE (...)` in the standard and `(...) BYTE` after the list, produce the same node. A default written after the list, `:= Closed`, becomes the initializer of the type declaration. `mode: (Manual, Auto)` is an inline type, which pre-processing at the start of the index stage moves out as `__main_mode` (see [Index](../pipeline/02-index.md), Pre-processing).

Pre-processing also rewrites every variant list so that each variant has an explicit value. A bare first variant gets the literal `0`, and every later bare variant gets the expression `<Enum>#<previous> + 1`, a typed reference to the variant before it:

```diff
-TYPE Color: (Red, Green, Blue := 5); END_TYPE
+TYPE Color: (Red := 0, Green := Color#Red + 1, Blue := 5); END_TYPE
-TYPE State: (Open := 1, Closed := 4, Idle, Running) BYTE := Closed; END_TYPE
+TYPE State: (Open := 1, Closed := 4, Idle := State#Closed + 1, Running := State#Idle + 1) BYTE := Closed; END_TYPE
```

From here on no stage has to count variants; every value is an expression like any other initializer.


## Index

The type index holds one record per enum type. Trimmed to the enum variant of the type information:

```rust
Enum {
    /// The enum's own name
    name: TypeId,

    /// The underlying integer type: DINT by default, BYTE for State
    referenced_type: TypeId,

    /// One variable entry per variant, in declaration order
    variants: Vec<VariableIndexEntry>,
}
```

Each variant is a constant global variable entry, such as `Color.Red`. Its value expression is stored with the enum's underlying type as the evaluation target. The index registers the variant under its enum and under its bare name, so `Red` can resolve without a qualifier.

After indexing and constant evaluation the project holds four enum types and eleven variants:

```
Color        DINT   Red := 0   Green := 1   Blue := 5                    default: Red (0)
State        BYTE   Open := 1  Closed := 4  Idle := 5   Running := 6     default: Closed (4), explicit
Door         DINT   Open := 8  Closed := 16                              default: Open (8)
__main_mode  DINT   Manual := 0   Auto := 1                              default: Manual (0)
```

Constant evaluation folds the generated expressions, `Color#Red + 1` to `1` and `State#Idle + 1` to `6` (see [Index](../pipeline/02-index.md), Constant evaluation). Its last step gives every enum type without an explicit default one: the variant whose value is zero, or the first variant if no value is zero. `Door` therefore defaults to `Open`, value 8, not to 0. The default is stored as the initial value of the type and points at the same constant-store entry as the variant.

The variable entries know only the type name: `main.paint` is of type `Color`, and `main.mode` is of type `__main_mode` with the initializer `Auto`, which evaluates to `1`.


## Annotations

A variant in a body resolves to its variable entry, and the annotation records that it is a constant global of the enum type. The hint is the type of the target, so a variant of the wrong enum gets a hint that differs from its own type. For the body of `main`:

```
    paint := Blue;
    ^^^^^                    { kind: Variable, qualified_name: "main.paint",   resulting_type: "Color", hint: None }
             ^^^^            { kind: Variable, qualified_name: "Color.Blue",   resulting_type: "Color", hint: "Color", constant: true }

    paint := Color#Red;
             ^^^^^           { kind: Type,     type_name: "Color" }
                   ^^^       { kind: Variable, qualified_name: "Color.Red",    resulting_type: "Color", constant: true }
             ^^^^^^^^^       { kind: Value,                                    resulting_type: "Color", hint: "Color" }

    paint := Color.Green;
             ^^^^^           { kind: Type,     type_name: "Color" }
             ^^^^^^^^^^^     { kind: Variable, qualified_name: "Color.Green",  resulting_type: "Color", hint: "Color", constant: true }

    state := Door#Closed;
             ^^^^^^^^^^^     { kind: Value,                                    resulting_type: "Door",  hint: "State" }

    state := Closed;
             ^^^^^^          { kind: Variable, qualified_name: "State.Closed", resulting_type: "State", hint: "State", constant: true }

    flag := paint = Green;
            ^^^^^^^^^^^^^    { kind: Value,                                    resulting_type: "BOOL",  hint: "BOOL" }
            ^^^^^            { kind: Variable, qualified_name: "main.paint",   resulting_type: "Color", hint: None }
                    ^^^^^    { kind: Variable, qualified_name: "Color.Green",  resulting_type: "Color", hint: None }

    flag := state <> Idle;
            ^^^^^            { kind: Variable, qualified_name: "main.state",   resulting_type: "State", hint: "UDINT" }
                     ^^^^    { kind: Variable, qualified_name: "State.Idle",   resulting_type: "State", hint: "UDINT" }
```

The example uses three forms of variant access: `Color#Red`, `Color.Green`, and the unqualified `Closed`.

`Color#Red` is a cast expression: the base `Color` is annotated as a type, and because that type is an enum, the target `Red` is looked up among its variants only. The expression as a whole is a value of type `Color`. `Color.Green` is an ordinary member access: the base resolves to the type `Color`, and the member lookup, which tries the members of a container first, falls back to the variants when the container is an enum.

`Closed` without a qualifier goes through the normal name lookup (see [Resolver](../pipeline/03-resolver.md), Walking a unit): a member of the current POU first, then a global. Inside a POU the member lookup also searches the variants of every enum type one of the POU's variables has. That is why `Closed` in `main` finds `State.Closed` and not `Door.Closed`: `main` has a variable of type `State` and none of type `Door`. Without such a variable, the global lookup takes the first variant of that name in declaration order.

Comparisons are promoted through the underlying type. `paint = Green` compares two `DINT` values and needs no hints. `state <> Idle` compares two `BYTE` values, and the resolver promotes both to the bigger of the underlying type and `DINT`, which for the unsigned `BYTE` is `UDINT`. The argument `paint` in `isGreen(paint)` is hinted to the parameter type `Color` like any argument, and the variant initializers are annotated too, with the underlying type as hint, so the values of `State` are hinted `BYTE`.

> [!NOTE]
> Names are case-insensitive, so a variable `color: Color` shadows the type name in `Color.Green`: the base resolves to the variable `main.color`, and the variant is then found through the variable's type. The result is the same entry, which is why the example uses `paint` for the variable.


## Lowering

Enums keep their representation during lowering. The [init participant](../participants/06-init.md) creates an empty constructor unless the type declares an explicit default. Thus `State__ctor` stores `4`. A variable initializer such as `mode := Auto` becomes an assignment in `main__ctor`.


## Codegen

### Layout

An enum type becomes the LLVM integer of its underlying type, `i32` for `Color` and `i8` for `State`; the type itself leaves no trace in the module. Every variant becomes a global constant named by its qualified name, and the instance of `main` gets its default values from the types:

```llvm
%main = type { i32, i8, i32, i8 }

@main_instance = global %main { i32 0, i8 4, i32 1, i8 0 }
@__main_mode.Auto = unnamed_addr constant i32 1
@Color.Red = unnamed_addr constant i32 0
@Color.Green = unnamed_addr constant i32 1
@Color.Blue = unnamed_addr constant i32 5
@State.Open = unnamed_addr constant i8 1
@State.Closed = unnamed_addr constant i8 4
@State.Idle = unnamed_addr constant i8 5
@State.Running = unnamed_addr constant i8 6
@Door.Open = unnamed_addr constant i32 8
@Door.Closed = unnamed_addr constant i32 16
@__main_mode.Manual = unnamed_addr constant i32 0
```

`paint` starts at `0`, the `Red` default; `state` at `4`, the explicit `Closed`; `mode` at `1` from its own initializer. The variant globals carry no debug information, so a debugger does not list them as variables.

### Assignment and comparison

A variant is a constant, so codegen never loads its global; it folds the value into the instruction. The five assignments of `main` are five stores of immediates, and `Door#Closed` into a `State` becomes `store i8 16` with the value cut to the underlying type. A comparison with a variant compares against the immediate; when the underlying type is smaller than `DINT`, the variable is widened first, as the hints said:

```llvm
  store i32 5, ptr %paint, align 4
  store i32 0, ptr %paint, align 4
  store i32 1, ptr %paint, align 4
  store i8 16, ptr %state, align 1
  store i8 4, ptr %state, align 1
  %load_paint1 = load i32, ptr %paint, align 4
  %tmpVar = icmp eq i32 %load_paint1, 1
  %load_state = load i8, ptr %state, align 1
  %2 = zext i8 %load_state to i32
  %tmpVar2 = icmp ne i32 %2, 5
```

### Passing

An enum parameter is its integer: `isGreen` is `define i8 @isGreen(i32 %0)`, and the call passes the loaded `i32`. Nothing distinguishes it from a `DINT` parameter.


## Validation

The declaration is checked for an integer underlying type (E122; `REAL` or `TIME` are rejected) and for an empty variant list (E028).

Enum assignment diagnostics are warnings or informational messages by default. They do not stop codegen unless the severity configuration changes them. See [Severity and reporting](../pipeline/04-validation.md#severity-and-reporting).

A value not accepted as a constant of the target enum produces E091. This includes an integer variable or a variant of another enum. Narrowing the underlying type also produces E067, as in `state := Door#Closed`. A matching integer literal produces a suggestion such as `Replace 1 with Green` (E092); an unmatched value produces E040.

Two enum types are the same type only when their names are equal, so a copy of a variant list under a second name is a different type. An enum assigned to an integer variable is not checked at all.


## At a glance

| Structured Text | Index | Annotation | LLVM |
|---|---|---|---|
| `TYPE Color: (Red, Green); END_TYPE` | enum type, underlying `DINT`, two variants with constant-store values | | none; variables are `i32` |
| `(...) BYTE` or `: BYTE (...)` | underlying `BYTE` | | `i8` |
| `Red` as a variant | constant global entry `Color.Red`, also findable by bare name | Variable `Color.Red`, constant, type `Color` | `@Color.Red = constant i32 0`, folded to an immediate |
| `Color#Red` | | base Type `Color`, whole a Value of type `Color` | immediate |
| `Color.Red` | | base Type `Color`, member Variable `Color.Red` | immediate |
| `mode: (Manual, Auto)` | pre-processed type `__main_mode` | | `i32`, constants `@__main_mode.Manual` |
| `x: Color` without initializer | type default: zero variant, else the first | | initial value of the type's default |
| `a = b` on enums | | operands hinted to the promoted underlying type | `icmp` on the underlying integer |
