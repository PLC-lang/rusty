# Standard Library

`iec61131std` provides the functions and function blocks of IEC 61131-3. This page says what is in it and which file declares each family. For the signature of a single function, read that file.


## Using it

A release installs the library and its declarations:

| File | Contents |
|---|---|
| `/usr/share/plc/include/*.st` | The declarations |
| `/usr/lib/<triplet>/libiec61131std.so` | The implementation, also as `libiec61131std.a` |

```bash
plc main.st -i "/usr/share/plc/include/*.st" -l iec61131std -o app --linker=cc
```

Some parts of the language call the library by themselves, so link it also when your own code names none of these functions:

- `**` calls `EXPT`
- a comparison of `STRING` or `WSTRING` calls the string functions


## Families

| Family | Declared in | Contains |
|---|---|---|
| Arithmetic | `arithmetic_functions.st` | `SQRT`, `LN`, `LOG`, `EXP`, `SIN`, `COS`, `TAN`, `ASIN`, `ACOS`, `ATAN`, `ATAN2`, `EXPT`, the variadic `ADD` and `MUL`, and the constants `PI_REAL`, `FRAC_PI_2_REAL`, `FRAC_PI_4_REAL`, `E_REAL`, `INF_REAL`, and `NAN_REAL`, each also in an `LREAL` form such as `PI_LREAL` |
| Numerical | `numerical_functions.st` | `ABS` |
| Selectors | `selectors.st` | `MAX`, `MIN`, `LIMIT` |
| Bit shifts | `bit_shift_functions.st` | `ROL`, `ROR` |
| Endianness | `endianness_conversion_functions.st` | `TO_BIG_ENDIAN`, `TO_LITTLE_ENDIAN`, and back |
| Validation | `validation_functions.st` | `IS_VALID`, `IS_VALID_BCD` |
| Text | `string_functions.st` | `LEN`, `LEFT`, `RIGHT`, `MID`, `CONCAT`, `INSERT`, `DELETE`, `REPLACE`, `FIND`, and the comparisons |
| Text conversion | `string_conversion.st` | Between `STRING`, `WSTRING`, `CHAR`, and `WCHAR` |
| Timers | `timers.st` | `TP`, `TON`, `TOF`, each also in a `_TIME` and an `_LTIME` form |
| Counters | `counters.st` | `CTU`, `CTD`, `CTUD`, each also with the suffix `_INT`, `_DINT`, `_UDINT`, `_LINT`, or `_ULINT` |
| Edges | `flanks.st` | `R_TRIG`, `F_TRIG` |
| Bistable | `bistable_functionblocks.st` | `SR`, `RS` |
| Date and time | `date_time_numeric_functions.st` | Adding and subtracting durations, dates, and times of day, and `MUL_TIME` and `DIV_TIME` |
| Date and time | `date_time_conversion.st` | Between the date and time types, and between the short and long families |
| Date and time | `date_time_extra_functions.st` | `CONCAT_DATE`, `CONCAT_TOD`, the `SPLIT_` family that takes such a value apart again, and `DAY_OF_WEEK` |
| Numeric conversion | `num_conversion.st` | `<TYPE>_TO_<TYPE>` for every pair of numeric types |
| Bit conversion | `bit_conversion.st` | Between the bit string types and `BOOL`, and between them and `CHAR` and `WCHAR` |
| Bit and number | `bit_num_conversion.st` | Between `BOOL` and the bit string types on one side and the integer and real types on the other |
| Truncation | `trunc_int.st` | `TRUNC_<TYPE>`, which cuts the fraction of a real |
| Truncation | `real_trunc_int.st` | `REAL_TRUNC_<TYPE>` and `LREAL_TRUNC_<TYPE>`, the same for one source type each |
| Generic conversion | `to_num.st`, `to_bit.st`, `to_string.st`, `to_date_time.st` | `TO_<TYPE>`, one generic function per target type |
| Text output and input | `extra_functions.st` | `<TYPE>_TO_STRING` and `<TYPE>_TO_WSTRING`, the `STRING_TO_<TYPE>` family that reads a value back, `<TYPE>_TO_BOOL` for the integer and real types, `TRUNC`, and `TIME()`, which gives the time since midnight |

The text conversion family covers eight of the twelve directions: each of the four text types converts to two of the other three. `STRING` to `WCHAR`, `WSTRING` to `CHAR`, `CHAR` to `WSTRING`, and `WCHAR` to `STRING` do not exist.

`<TYPE>_TO_STRING` covers most types, but not all of them. There is no `INT_TO_STRING`, `SINT_TO_STRING`, `WORD_TO_STRING`, or `BOOL_TO_STRING`, and the generic `TO_STRING` has no version for those types either, so a call compiles and then fails at the link step. Convert such a value to a wider type first, for example with `INT_TO_DINT`. `<TYPE>_TO_WSTRING` covers the same types except the unsigned integers.


## Two forms of conversion

The library provides the same conversion twice. `INT_TO_DINT(x)` names both types, and `TO_DINT(x)` is generic and takes the source type from the argument. The generic form calls the named one, so both give the same result, and the named one saves a call.

Where a conversion can lose information, the name says so: `TRUNC_DINT` cuts the fraction of a real, and `REAL_TO_DINT` rounds it.


## Numeric casts

The conversions between `BOOL`, the bit string types, the integers, and the reals follow five rules. None of them reports a diagnostic, and none of them faults at run time.

- **A real rounds to the nearest integer, and a tie goes away from zero.** `REAL_TO_BYTE(2.5)` is 3 and `REAL_TO_BYTE(-2.5)` is 253, not the 2 and 254 that rounding to even would give.
- **A real to a bit string type wraps.** The rounded value becomes a signed 64-bit integer and keeps the low bits of the target width, so `REAL_TO_BYTE(300.0)` is 44 and `REAL_TO_BYTE(-1.0)` is 255. A value that does not fit a signed 64-bit integer, `NaN` and the infinities included, becomes 2^63 first: `REAL_TO_BYTE(NAN_REAL)` is 0 and `REAL_TO_LWORD(1.0E20)` is 9223372036854775808. `LWORD` alone keeps a value between 2^63 and 2^64 exact, so `LREAL_TO_LWORD(1.0E19)` is 10000000000000000000 while `LREAL_TO_DWORD(1.0E19)` is 0. A real to a signed or unsigned integer type saturates instead: `REAL_TO_USINT(300.0)` is 255.
- **Anything to `BOOL` is `FALSE` for exactly zero and `TRUE` for every other value.** `INT_TO_BOOL(256)`, `BYTE_TO_BOOL(2)`, `REAL_TO_BOOL(0.5)`, and `REAL_TO_BOOL(NAN_REAL)` are all `TRUE`. It is not a test of the lowest bit, and a fraction is not cut first.
- **`BOOL` and the bit string types widen to a real by value.** `BOOL`, `BYTE`, and `WORD` fit a `REAL` exactly, and `DWORD` fits an `LREAL` exactly. A `REAL` keeps 24 bits of mantissa, so `LWORD_TO_REAL(16777217)` and `DWORD_TO_REAL(16777217)` are both 16777216. These conversions have the named form only: the generic `TO_REAL` and `TO_LREAL` take the numeric types, so a `STRING` or a `TIME` argument is still rejected at compile time.
- **No conversion reinterprets bits.** `DWORD_TO_REAL(1073741824)` is 1073741824.0, and `REAL_TO_DWORD(2.0)` is 2.
