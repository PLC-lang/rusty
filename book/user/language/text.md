# Text

Two types hold text, and two more hold one character each. `STRING` stores UTF-8 bytes and its literals stand between single quotation marks. `WSTRING` stores UTF-16 and its literals stand between double quotation marks.

```iecst
VAR
    name: STRING := 'motor';
    label: WSTRING := "Motor";
END_VAR
```


## Length

A text variable has a fixed capacity, which you write in brackets. The storage is one element longer, for the terminator that marks the end, so `STRING[20]` takes 21 bytes and `WSTRING[20]` takes 21 units of 16 bits. Without a length, the capacity is 80.

The capacity of a `STRING` counts bytes, not characters. A character outside ASCII takes two bytes or more, so a text that is not plain ASCII needs more capacity than it has characters.

The capacity is the whole story about what fits. An assignment that does not fit is cut off, and nothing reports it:

```iecst
VAR
    long: STRING[20] := 'abcdefghij';
    short: STRING[5];
END_VAR

short := long;   (* 'abcde' *)
```

So declare the capacity that the value needs. The compiler cannot warn about a text that grows only while the program runs.


## Single characters

`CHAR` holds one byte and `WCHAR` one unit of 16 bits. A literal is written like a text literal of the same kind, with one character in it:

```iecst
VAR
    letter: CHAR := 'a';
    wide: WCHAR := "b";
END_VAR
```

A character is not a text of length one, and the compiler keeps the two apart. An assignment between `CHAR` and `STRING`, or between `CHAR` and `WCHAR`, is rejected, and a text cannot be indexed to take a character out of it. A comparison with a plain literal is rejected as well, because that literal is a text; write the type in front of it, as in `letter = CHAR#'a'`. The standard library converts in both directions: `STRING_TO_CHAR` gives the first byte of a text, `CHAR_TO_STRING` makes a text of one character, and `WSTRING_TO_WCHAR`, `WCHAR_TO_WSTRING`, `CHAR_TO_WCHAR`, and `WCHAR_TO_CHAR` do the same for the other pairs.

A literal with more than one character keeps its first unit only, and the compiler does not warn. For a `CHAR` that unit is one byte, so a character outside ASCII does not fit: `'ü'` gives the first byte of its two-byte code.


## Escape sequences

A literal ends at the quotation mark that opened it, and it cannot hold a line break. `$` starts an escape that writes such a character. It works in both string kinds, and the letter after it can be upper case or lower case.

| Sequence | Meaning |
|---|---|
| `$L`, `$N` | Line feed |
| `$P` | Form feed |
| `$R` | Carriage return |
| `$T` | Tabulator |
| `$$` | A dollar sign |
| `$'` | A single quotation mark, which ends a `STRING` literal |
| `$"` | A double quotation mark, which ends a `WSTRING` literal |
| `$XX` | The character with that hexadecimal code, four digits in a `WSTRING` |

```iecst
message: STRING := 'Line 1$NLine 2';
price: STRING := 'costs $$5';
```

In a `STRING` the code of `$XX` has two digits and must be an ASCII code, because a `STRING` holds UTF-8. Write a character outside ASCII into the literal itself, as `'Grüße'`.


## Comparison

The comparison operators work on text and compare it character by character:

```iecst
IF name = 'motor' THEN
```

`<` and `>` order two texts by the first character that differs, and a text that is the start of a longer one comes first. The order is the order of the character codes, so `'Z'` comes before `'a'`.

> [!NOTE]
> Comparison of text calls the standard library, so a project that compares text must link `iec61131std`. See [Linking and Libraries](../building/linking.md).


## Working with text

The same library holds the text operations of the standard. These are the ones you reach for first:

| Function | Result |
|---|---|
| `LEN(in)` | The number of characters |
| `CONCAT(in1, in2, ...)` | The texts joined |
| `LEFT(in, l)`, `RIGHT(in, l)` | The first or last `l` characters |
| `MID(in, l, p)` | `l` characters, starting at position `p` |
| `FIND(in1, in2)` | The position of `in2` inside `in1`, or `0` |
| `INSERT(in1, in2, p)` | `in1` with `in2` put in after position `p` |
| `DELETE(in, l, p)` | `in` with `l` characters removed from position `p` |
| `REPLACE(in1, in2, l, p)` | `in1` with `l` characters from position `p` exchanged for `in2` |

Positions count from `1`. Note the order of the arguments of `MID`, `DELETE` and `REPLACE`: the length comes before the position.

```iecst
VAR
    source: STRING[20] := 'abcdefghij';
    part: STRING[20];
END_VAR

part := MID(source, 3, 2);   (* 'bcd' *)
```

The [standard library reference](../reference/standard-library.md) lists the rest of the family, and the functions that convert between the text types.


## What's next

The next chapter covers the types for [time and date](time.md), which every control program needs.
