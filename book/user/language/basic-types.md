# Basic Types

Numbers, bits, and truth values. Every other type in the language is built from these.


## Integers

Eight integer types, four signed and four unsigned:

| Type | Size | Range |
|---|---|---|
| `SINT` | 8 bit | -128 to 127 |
| `USINT` | 8 bit | 0 to 255 |
| `INT` | 16 bit | -32 768 to 32 767 |
| `UINT` | 16 bit | 0 to 65 535 |
| `DINT` | 32 bit | -2 147 483 648 to 2 147 483 647 |
| `UDINT` | 32 bit | 0 to 4 294 967 295 |
| `LINT` | 64 bit | -9 223 372 036 854 775 808 to 9 223 372 036 854 775 807 |
| `ULINT` | 64 bit | 0 to 18 446 744 073 709 551 615 |

`DINT` is the type to reach for. It is the type that the compiler gives to a whole-number literal, and only a literal too large for 32 bits becomes a `LINT`.

A literal can be written in another base, and `_` between two digits groups them:

```iecst
i1: DINT := 42;
i2: DINT := 2#101010;     (* binary *)
i3: DINT := 8#52;         (* octal *)
i4: DINT := 16#2A;        (* hexadecimal *)
i5: DINT := 1_000_000;
```

Integer division cuts towards zero and never produces a fraction: `7 / 2` is `3` and `-7 / 2` is `-3`. `MOD` gives the rest of that division, so `7 MOD 2` is `1` and `-7 MOD 2` is `-1`.


## BOOL

`BOOL` holds `TRUE` or `FALSE` and takes one byte. It is the type of every condition in `IF`, `WHILE`, and `UNTIL`.

An integer in a condition is accepted and counts as true when it is not zero. The compiler warns and asks you to add an `=` or a `<>` operator, so that the test says what it means:

```iecst
IF level THEN        (* accepted, with a warning *)
IF level <> 0 THEN   (* the same test, and it says so *)
```


## Bit strings

Four types that mean "a sequence of bits", not "a number":

| Type | Size |
|---|---|
| `BYTE` | 8 bit |
| `WORD` | 16 bit |
| `DWORD` | 32 bit |
| `LWORD` | 64 bit |

Use them for flags, masks, and values that come from hardware. `AND`, `OR`, `XOR`, and `NOT` work on them bit by bit, and the [hardware access](hardware-access.md) chapter shows how to read a single bit out of one.

```iecst
VAR
    flags: BYTE := 2#0000_1100;
    mask: BYTE := 16#0F;
    result: BYTE;
END_VAR

result := flags AND mask;   (* 2#0000_1100 *)
```


## Reals

`REAL` is 32 bits wide and `LREAL` is 64. A literal with a decimal point or an exponent is a real:

```iecst
r1: REAL := 1.5;
r2: LREAL := 1.0e-9;
```

A `REAL` keeps about seven decimal digits, which is less than a `DINT` needs. A large whole number does not survive a trip through a `REAL`: `123456789` comes back as `123456792`.


## Conversion between types

The compiler converts a value to a wider type of the same family by itself:

```iecst
VAR
    small: INT := 300;
    big: DINT;
END_VAR

big := small;   (* fine, every INT fits in a DINT *)
```

The other direction also compiles, but the value can change, so the compiler warns:

```iecst
VAR
    small: INT := 300;
    tiny: SINT;
END_VAR

tiny := small;   (* warning[E067]: Implicit downcast from 'INT' to 'SINT'. *)
```

Here `tiny` keeps the low eight bits of 300, which is 44. Write the conversion yourself when the narrowing is intended. The standard library has a function for every pair of types, named after them:

```iecst
tiny := INT_TO_SINT(small);
```

> [!NOTE]
> The conversion functions live in the standard library, so a project that calls one, for example `INT_TO_SINT`, must link `iec61131std`. See [Linking and Libraries](../building/linking.md).

A value also crosses between the two families. An integer becomes a real without any warning, although a large `DINT` loses digits on the way. A real becomes an integer with the same downcast warning, and the fraction is cut off. Two standard functions say which result you want:

```iecst
VAR
    r: REAL := 2.7;
    n: DINT;
END_VAR

n := REAL_TO_DINT(r);   (* 3, the nearest whole number *)
n := TRUNC_DINT(r);     (* 2, the fraction is cut off *)
```

A type name with `#` in front of a value states the type of that value. On a literal it decides how the literal is read, and on a variable it converts:

```iecst
x := DINT#16#2A;    (* the literal 16#2A, as a DINT *)
y := DINT#small;    (* small, converted to DINT *)
```


## What's next

The next chapter is about [text](text.md): the two string types, their length, and what you can do with them.
