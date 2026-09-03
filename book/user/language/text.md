# Text

Two types hold text. `STRING` stores UTF-8 bytes and its literals stand between single quotation marks. `WSTRING` stores UTF-16 and its literals stand between double quotation marks.

```iecst
VAR
    name  : STRING := 'motor';
    label : WSTRING := "Motor";
END_VAR
```


## Length

A text variable has a fixed capacity, which you write in brackets. `STRING[20]` holds 20 characters, and the storage is one element longer, for the terminator that marks the end. Without a length, the capacity is 80 characters.

The capacity is the whole story about what fits. An assignment that does not fit is cut off, and nothing reports it:

```iecst
VAR
    long  : STRING[20] := 'abcdefghij';
    short : STRING[5];
END_VAR

short := long;   (* 'abcde' *)
```

So declare the capacity that the value needs. The compiler cannot warn about a text that grows only while the program runs.


## Escape sequences

`$` starts an escape in both string kinds. The letter can be upper case or lower case.

| Sequence | Meaning |
|---|---|
| `$L`, `$N` | Line feed |
| `$P` | Form feed |
| `$R` | Carriage return |
| `$T` | Tabulator |
| `$$` | A dollar sign |
| `$'` | A single quotation mark, inside a `STRING` |
| `$"` | A double quotation mark, inside a `WSTRING` |
| `$XX` | The character with that hexadecimal code, four digits in a `WSTRING` |

```iecst
message : STRING := 'Line 1$NLine 2';
price   : STRING := 'costs $$5';
```


## Comparison

The comparison operators work on text and compare it character by character:

```iecst
IF name = 'motor' THEN
```

`<` and `>` order two texts the way a dictionary does, by the first character that differs.

> [!NOTE]
> Comparison of text calls the standard library, so a project that compares text must link `iec61131std`. See [Linking](../building/linking.md).


## Working with text

The standard library provides the operations of the standard. These are the ones you reach for first:

| Function | Result |
|---|---|
| `LEN(in)` | The number of characters |
| `CONCAT(in1, in2, ...)` | The texts joined |
| `LEFT(in, l)`, `RIGHT(in, l)` | The first or last `l` characters |
| `MID(in, l, p)` | `l` characters, starting at position `p` |
| `FIND(in1, in2)` | The position of `in2` inside `in1`, or `0` |
| `INSERT`, `DELETE`, `REPLACE` | A copy with a part added, removed, or exchanged |

Positions count from `1`. Note the order of the arguments of `MID`: the length comes before the position.

```iecst
VAR
    source : STRING[20] := 'abcdefghij';
    part : STRING[20];
END_VAR

part := MID(source, 3, 2);   (* 'bcd' *)
```

The [standard library reference](../reference/standard-library.md) lists the other families.


## What's next

The next chapter covers the types for [time and date](time.md), which every control program needs.
