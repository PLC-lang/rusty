# Expressions and Operators

An expression computes a value from literals, variables, and calls.


## Arithmetic

`+`, `-`, `*`, `/`, `MOD`, and `**` work on numbers. Division of integers cuts towards zero, and `MOD` gives the rest of that division:

```iecst
a := 7 / 2;     (* 3 *)
b := 7 MOD 2;   (* 1 *)
c := 2 ** 10;   (* 1024 *)
```

`**` is the power operator. It calls the standard library, so a project that uses it must link `iec61131std`, and it computes in floating point even when both sides are integers.


## Comparison

`=`, `<>`, `<`, `>`, `<=`, and `>=` compare two values of the same kind and produce a `BOOL`. They work on numbers, on enumerations, and on text, where they compare character by character.

Note that the test for equality is a single `=`, because `:=` is the assignment.


## Boolean and bit operators

`AND`, `OR`, `XOR`, and `NOT` do two jobs. On `BOOL` values they are the logical operators; on the bit string types they work bit by bit. `&` is another spelling of `AND`.

```iecst
ready := motorOn AND NOT alarm;
masked := flags AND 16#0F;
```

`AND` and `OR` always evaluate both sides, also when the left side already decides the result. When the right side is a call that you want to avoid, use the short-circuit forms:

```iecst
IF valid AND_THEN check() THEN     (* check() runs only when valid is TRUE *)
IF failed OR_ELSE check() THEN     (* check() runs only when failed is FALSE *)
```


## Precedence

Operators bind in this order, the strongest first. Operators of the same level are evaluated from left to right.

| Level | Operators |
|---|---|
| 1 | `(...)`, a call, `^`, `.%X` and the other direct accesses, `TYPE#value` |
| 2 | `NOT`, unary `-`, unary `+` |
| 3 | `**` |
| 4 | `*`, `/`, `MOD` |
| 5 | `+`, `-` |
| 6 | `<`, `>`, `<=`, `>=` |
| 7 | `=`, `<>` |
| 8 | `AND`, `AND_THEN`, `&` |
| 9 | `XOR` |
| 10 | `OR`, `OR_ELSE` |

So `a + b * c` multiplies first, and `x < 1 AND y > 2` compares first. Parentheses override every rule and are worth writing wherever a reader would have to think.


## Mixed types

An expression of two different numeric types is computed in the wider of the two, and the result then goes to the target of the assignment. Widening is silent; narrowing compiles with the warning `E067`, because the value can change:

```iecst
VAR
    i: INT := 300;
    d: DINT;
    s: SINT;
END_VAR

d := i + 1;   (* fine *)
s := i;       (* warning[E067]: Implicit downcast from 'INT' to 'SINT' *)
```

Between integers and reals nothing is implicit. Convert with a standard function such as `REAL_TO_DINT`, or state the type with `DINT#value`.


## Calls in expressions

A function call is an expression, so it can appear anywhere a value can:

```iecst
level := MIN(measured, maximum) + offset;
```

A call of a function block instance is a statement, not an expression: it runs the instance, and you read the results from the instance afterwards. The [function blocks](function-blocks.md) chapter shows this.


## What's next

Expressions compute values. The next chapter decides which statements run, with [control flow](control-flow.md).
