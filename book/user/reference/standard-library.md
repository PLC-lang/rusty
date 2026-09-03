# Standard Library

`iec61131std` provides the functions and function blocks of IEC 61131-3. This page says what is in it and where each family is declared. For the signature of a single function, read the declaration file.


## Using it

A release installs the library and its declarations:

| File | Contents |
|---|---|
| `/usr/share/plc/include/*.st` | The declarations, one file per family |
| `/usr/lib/<triplet>/libiec61131std.so` | The implementation |

```bash
plc main.st -i "/usr/share/plc/include/*.st" -l iec61131std -o app --linker=cc
```

Some parts of the language call the library by themselves, so link it also when your own code names none of these functions:

- `**` calls `EXPT`
- a comparison of `STRING` or `WSTRING` calls the string functions


## Families

| Family | Declared in | Contains |
|---|---|---|
| Arithmetic | `arithmetic_functions.st` | `SQRT`, `LN`, `LOG`, `EXP`, `SIN`, `COS`, `TAN`, `ASIN`, `ACOS`, `ATAN`, `ATAN2`, `EXPT`, and the variadic `ADD` and `MUL` |
| Numerical | `numerical_functions.st` | `ABS` |
| Selectors | `selectors.st` | `MAX`, `MIN`, `LIMIT` |
| Bit shifts | `bit_shift_functions.st` | `ROL`, `ROR` |
| Endianness | `endianness_conversion_functions.st` | `TO_BIG_ENDIAN`, `TO_LITTLE_ENDIAN`, and back |
| Validation | `validation_functions.st` | `IS_VALID`, `IS_VALID_BCD` |
| Text | `string_functions.st` | `LEN`, `LEFT`, `RIGHT`, `MID`, `CONCAT`, `INSERT`, `DELETE`, `REPLACE`, `FIND`, and the comparisons |
| Text conversion | `string_conversion.st` | Between `STRING`, `WSTRING`, `CHAR`, and `WCHAR` |
| Timers | `timers.st` | `TP`, `TON`, `TOF`, each also in a `_TIME` and an `_LTIME` form |
| Counters | `counters.st` | `CTU`, `CTD`, `CTUD`, each for every integer type |
| Edges | `flanks.st` | `R_TRIG`, `F_TRIG` |
| Bistable | `bistable_functionblocks.st` | `SR`, `RS` |
| Date and time | `date_time_numeric_functions.st` | Adding and subtracting durations, dates, and times of day |
| Date and time | `date_time_conversion.st` | Between the date and time types, and between the short and long families |
| Date and time | `date_time_extra_functions.st` | `CONCAT_DATE`, `CONCAT_TOD`, and their variants |
| Numeric conversion | `num_conversion.st` | `<TYPE>_TO_<TYPE>` for every pair of numeric types |
| Bit conversion | `bit_conversion.st` | Between the bit string types |
| Bit and number | `bit_num_conversion.st` | Between the bit string types and the numeric types |
| Truncation | `trunc_int.st`, `real_trunc_int.st` | `TRUNC` and the `TRUNC_<TYPE>` family, which cut the fraction |
| Generic conversion | `to_num.st`, `to_bit.st`, `to_string.st`, `to_date_time.st` | `TO_<TYPE>`, one generic function per target type |
| Text output | `extra_functions.st` | `<TYPE>_TO_STRING` for every type |


## Two forms of conversion

The library provides the same conversion twice. `INT_TO_DINT(x)` names both types, and `TO_DINT(x)` is generic and takes the source type from the argument. Both compile to the same code; the generic form is shorter, and the explicit form is clearer in a long expression.

Where a conversion can lose information, the name says so: `TRUNC_DINT` cuts the fraction of a real, and `REAL_TO_DINT` rounds it.
