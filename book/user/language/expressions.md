# Expressions and Operators

An expression computes a value from literals, variables, and calls.


## Arithmetic

`+`, `-`, `*`, `/`, `MOD`, and `**` work on numbers. Division of integers cuts towards zero, and `MOD` gives the rest of that division; the [basic types](basic-types.md) chapter gives the rule for negative operands.

`**` is the power operator. It calls the standard library, so a project that uses it must link `iec61131std` (see [Linking and Libraries](../building/linking.md)), and it computes in floating point even when both sides are integers. The result is a `REAL`, so an integer target needs a conversion, and a result that a `REAL` cannot hold exactly loses digits:

```iecst
VAR
    r: REAL;
    n: DINT;
END_VAR

r := 2 ** 10;                 (* 1024.0 *)
n := REAL_TO_DINT(2 ** 10);   (* 1024 *)
n := REAL_TO_DINT(7 ** 11);   (* 1977326720, not 1977326743 *)
```


## Comparison

`=`, `<>`, `<`, `>`, `<=`, and `>=` compare two values of the same kind and produce a `BOOL`. They work on numbers, on enumerations, and on text, where they compare character by character. A comparison of two values that do not belong together, such as a text and a number, is rejected.

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

An expression that mixes operators from more than one of these groups needs a rule for which operator binds first. Operators bind in this order, the strongest first. Operators of the same level group from left to right, so `10 - 3 - 2` is `5`.

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

So `a + b * c` multiplies first, and `x < 1 AND y > 2` compares first. A sign is part of the base of a power, so `-2 ** 2` is `4`. Parentheses override every rule and are worth writing wherever a reader would have to think.


## Mixed types

Precedence says how an expression is grouped. The types of its operands say in which type it is computed, and that type does not come from the target of the assignment. Every integer narrower than `DINT` is widened to `DINT` first, and a pair of different types is computed in the wider of the two. The result then moves to the target: a move to a wider type is silent, and a move to a narrower one compiles with a warning, because the value can change.

```iecst
VAR
    i: INT := 300;
    j: INT := 200;
    d: DINT;
    k: INT;
    s: SINT;
END_VAR

d := i * j;   (* 60000, because the product is computed in 32 bits *)
k := i * j;   (* -5536, the same product cut down to the 16 bits of k *)
s := i;       (* warning[E067]: Implicit downcast from 'INT' to 'SINT'. *)
```

An integer and a real mix the same way, and the expression is then computed in the real type. The [basic types](basic-types.md) chapter shows what each conversion costs.


## Calls in expressions

A function call is an expression, so it can appear anywhere a value can:

```iecst
level := MIN(measured, maximum) + offset;
```

`MIN` comes from the standard library, so this line needs `iec61131std` as well.

A call of a function block instance is a statement, not an expression: it runs the instance, and you read the results from the instance afterwards. The [function blocks](function-blocks.md) chapter shows this.


## What's next

Expressions compute values. The next chapter decides which statements run, with [control flow](control-flow.md).
