# Built-in Functions

The compiler knows these functions itself. You declare nothing, you include no file, and you link no library: the compiler writes the code at the place of the call. This is what separates them from the [standard library](standard-library.md), which is a set of ordinary declarations and an object file that the linker needs. The one exception is arithmetic on date and time values, see [Arithmetic](#arithmetic).

Their names are reserved. A function of your own that takes one of them is rejected; see [Source Files](../language/source-files.md).


## The functions

The parameter names in the second column are the names a call can use.

### Addresses and memory

| Call | Parameters | Result | Gives |
|---|---|---|---|
| `ADR(in)` | `in: ANY` | `LWORD` | The address of a variable, as a number |
| `REF(in)` | `in: ANY` | `REF_TO` the type of `in` | A reference to a variable |
| `SIZEOF(in)` | `in: ANY` | `ULINT` | The size of the argument's type, in bytes |
| `MOVE(in)` | `in: ANY` | The type of `in` | The value of `in` |

### Selection

| Call | Parameters | Result | Gives |
|---|---|---|---|
| `SEL(G, IN0, IN1)` | `G: BOOL`, `IN0` and `IN1: ANY` | The type of the two values | `IN0` when `G` is `FALSE`, `IN1` when it is `TRUE` |
| `MUX(K, ...)` | `K: DINT`, then two or more values | The type of the values | The value at index `K`, counted from zero |

### Array bounds

| Call | Parameters | Result | Gives |
|---|---|---|---|
| `LOWER_BOUND(arr, dim)` | `arr: ARRAY[*]`, `dim: ANY_INT` | `DINT` | The first index of dimension `dim` |
| `UPPER_BOUND(arr, dim)` | `arr: ARRAY[*]`, `dim: ANY_INT` | `DINT` | The last index of dimension `dim` |

`arr` takes an array of any size only, not an array of a fixed size. [Functions](../language/functions.md#arrays-of-any-size) shows both in a loop.

### Arithmetic

| Call | Parameters | Result | Gives |
|---|---|---|---|
| `ABS(IN)` | `IN: ANY_NUM` | The type of `IN` | The value without its sign |
| `ADD(...)` | Two or more numbers or date and time values | The biggest argument type, or the date and time result | The sum |
| `MUL(...)` | Two or more numbers or durations | The biggest argument type, or the date and time result | The product |
| `SUB(IN1, IN2)` | `IN1` and `IN2: ANY` | The biggest argument type, or the date and time result | `IN1 - IN2` |
| `DIV(IN1, IN2)` | `IN1` and `IN2: ANY` | The biggest argument type, or the date and time result | `IN1 / IN2` |
| `MOD(IN1, IN2)` | `IN1` and `IN2: ANY` | The biggest argument type | `IN1 MOD IN2` |

With a date or time argument, the first four take the combinations of [Time and Date](../language/time.md#calculating), folded from the left, and the result has the type that table gives: `ADD(stamp, T#1s, T#2s)` is `stamp + T#1s + T#2s`, a `DATE_AND_TIME`, and `SUB(day1, day2)` is a `TIME`. The compiler carries these operations out with the standard library, such as `ADD_DT_TIME`, so such a call needs `iec61131std` linked. An argument that is neither a number nor part of a defined combination is rejected (E156).

### Comparison

| Call | Parameters | Result | Gives |
|---|---|---|---|
| `GT(...)` | Two or more `ANY_ELEMENTARY` | `BOOL` | Whether every argument is greater than the next |
| `GE(...)` | Two or more `ANY_ELEMENTARY` | `BOOL` | Whether every argument is greater than or equal to the next |
| `EQ(...)` | Two or more `ANY_ELEMENTARY` | `BOOL` | Whether every argument is equal to the next |
| `LE(...)` | Two or more `ANY_ELEMENTARY` | `BOOL` | Whether every argument is less than or equal to the next |
| `LT(...)` | Two or more `ANY_ELEMENTARY` | `BOOL` | Whether every argument is less than the next |
| `NE(IN1, IN2)` | `IN1` and `IN2: ANY_ELEMENTARY` | `BOOL` | Whether the two arguments differ |

A call with more than two arguments compares each neighbouring pair and combines the results with `AND`, so `GT(a, b, c)` is `(a > b) AND (b > c)`.

### Bitwise

| Call | Parameters | Result | Gives |
|---|---|---|---|
| `AND(...)` | Two or more `ANY_BIT` or `ANY_INT` | The biggest argument type | The arguments combined with `AND` |
| `OR(...)` | Two or more `ANY_BIT` or `ANY_INT` | The biggest argument type | The arguments combined with `OR` |
| `XOR(...)` | Two or more `ANY_BIT` or `ANY_INT` | The biggest argument type | The arguments combined with `XOR` |
| `NOT(IN)` | `IN: ANY_BIT` or `ANY_INT` | The type of `IN` | `NOT IN` |

For a `BOOL` the result is the logical one, for any other type it is bit by bit, the same as the operator of the same name.

### Bit shifts

| Call | Parameters | Result | Gives |
|---|---|---|---|
| `SHL(IN, n)` | `IN: ANY`, `n: UDINT` | The type of `IN` | `IN` shifted left by `n` bits |
| `SHR(IN, n)` | `IN: ANY`, `n: UDINT` | The type of `IN` | `IN` shifted right by `n` bits |


## Names that are also operators

`AND`, `OR`, `XOR`, `MOD`, and `NOT` are operator keywords as well as function names. A keyword directly followed by `(`, where no operand stands before it, is the function:

```iecst
flag := AND(a, b, c);   (* the function, the same as a AND b AND c *)
flag := a AND (b);      (* the operator, `a` stands before it *)
rest := MOD(IN1 := total, IN2 := size);
```

`NOT(x)` with one positional argument stays the operator applied to `(x)`, with the same result. With a named argument, as in `NOT(IN := x)`, or with more than one argument, it is the function.

Because they are keywords, these names are not available for a function of your own. `FUNCTION AND` is rejected with E007.


## Calling by name

A parameter with a name in the table above accepts `name := value`, like the parameter of any other function:

```iecst
result := SEL(TRUE, first, second);
result := SEL(G := TRUE, IN0 := first, IN1 := second);
result := SEL(IN1 := second, IN0 := first, G := TRUE);
```

The three calls are the same call. The order of named arguments does not matter, because each name says which parameter it fills.

The two forms may also be mixed. A named argument claims the parameter its name says, and the positional arguments fill the parameters left over, in the order you write them:

```iecst
result := SUB(IN2 := offset, raw);   (* the same as SUB(raw, offset) *)
```

This is the rule for every call, not only for a built-in. `plc explain E132` describes it, and `plc explain E131` describes the one mix the compiler rejects: a positional argument whose parameter a *later* named argument also names.


## A variadic parameter has no name

`ADD`, `MUL`, `AND`, `OR`, `XOR`, `GT`, `GE`, `EQ`, `LE`, and `LT` take all of their arguments through one variadic parameter, and `MUX` takes everything after `K` through one. A variadic parameter has no name to call it by, so these arguments are positional:

```iecst
sum := ADD(a, b, c);          (* correct *)
value := MUX(K := 1, a, b);   (* correct, K is not variadic *)
```

A name for such an argument is rejected:

```
error[E048]: Could not resolve reference to args
error[E089]: Invalid call parameters
```

`SEL` looks like `MUX` but is not variadic, so all three of its parameters have a name.

The [standard library](standard-library.md) overloads `ADD` and `MUL` with a named first parameter `IN1` and a variadic rest. With the library linked, `ADD(IN1 := a, b)` is a valid call as well, and every argument after the first stays positional.


## How many arguments

`ADD`, `MUL`, `AND`, `OR`, `XOR`, `GT`, `GE`, `EQ`, `LE`, and `LT` are extensible: they take two arguments or more.

`SUB`, `DIV`, `MOD`, and `NE` take exactly two, which is the one place where `NE` behaves unlike the other five comparisons:

```
error[E032]: this POU takes 2 arguments but 3 arguments were supplied
```

The rest take the number of parameters their row lists.
