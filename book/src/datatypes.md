# Datatypes

## Numeric types

A variety of numeric types exist with different sizes and properties complying with IEC61131.

### Overview

| Type name | Size   | Properties |
| --------- | ------ | ---------- |
| SINT      | 8 bit  | signed     |
| USINT     | 8 bit  | unsigned   |
| INT       | 16 bit | signed     |
| UINT      | 16 bit | unsigned   |
| DINT      | 32 bit | signed     |
| UDINT     | 32 bit | unsigned   |
| LINT      | 64 bit | signed     |
| ULINT     | 64 bit | unsigned   |
| REAL      | 32 bit | float      |
| LREAL     | 64 bit | float      |

When such a variable is declared without being initialized, it will
be default-initialized with a value of `0` or `0.0` respectively.

### A word on integer literals

Integer literals can be prefixed with either `2#` (binary), `8#` (octal) or `16#` (hexadecimal).
They will then be treated with regard to the respective number system.

Examples:

- `i1 : DINT := 42;` - declares and initializes a 32bit signed integer with value 42.
- `i1 : DINT := 2#101010;` - declares and initializes a 32bit signed integer with value 42.
- `i1 : DINT := 8#52;` - declares and initializes a 32bit signed integer with value 42.
- `i1 : DINT := 16#2A;` - declares and initializes a 32bit signed integer with value 42.

## Strings

### Overview

| Type name | Size | Encoding |
| --------- | ---- | -------- |
| STRING    | n+1  | UTF-8    |
| WSTRING   | 2n+2 | UTF-16   |

When such a variable is declared without being initialized, it will
be default-initialized with a value of '' or "" respectively (empty strings).

### STRING

RuSTy treats `STRING`s as byte-arrays storing UTF-8 character bytes with a Null-terminator (0-byte) at the end.
So a String of size n requres n+1 bytes to account for the Null-terminator.
A `STRING` literal is surrounded by single-ticks `'`.

A String has a well defined length which can be defined similar to the array-syntax.
A String-variable `myVariable: STRING[20]` declares a byte array of length 21, to store 20 utf8 character bytes.
When declaring a `STRING`, the length-attribute is optional. The default length is 80.

Examples:

- `s1 : STRING;` - declares a String of length 80.
- `s2 : STRING[20];` - declares a String of length 20.
- `s3 : STRING := 'Hello World';` - declares and initializes a String of length 80, and initializes it with the utf8 characters and a null-terminator at the end.
- `s4 : STRING[55] := 'Foo Baz';` - declares and initializes a String of length 55 and initializes it with the utf8 characters and a null-terminator at the end.

### WSTRING (Wide Strings)

RuSTy treats `WSTRING`s as byte-arrays storing UTF-16 character bytes with two Null-terminator bytes at the end.
The bytes are stored in Little Endian encoding.
A Wide-String of size n requres 2 * (n+1) bytes to account for the 2 byes per utf16 character and the Null-terminators.
A `WSTRING` literal is surrounded by doubly-ticks `"`.

A `WSTRING` has a well defined length which can be defined similar to the array-syntax.
A `WSTRING`-variable `myVariable: WSTRING[20]` declares a byte array of length 42, to store 20 utf16 character bytes.
When declaring a `WSTRING`, the length-attribute is optional. The default length is 80.

Examples:

- `ws1 : WSTRING;` - declares a Wide-String of length 80.
- `ws2 : WSTRING[20];` - declares a Wide-String of length 20.
- `ws3 : WSTRING := "Hello World";` - declares and initializes a Wide-String of length 80, and initializes it with the utf16 characters and a utf16-null-terminator at the end.
- `ws4 : WSTRING[55] := "Foo Baz";` - declares and initializes a Wide-String of length 55 and initializes it with the utf8 characters and a utf16-null-terminator at the end.

## Date and Time

### Overview

| Type name       | Size   | Internally stored as              |
| --------------- | ------ | --------------------------------- |
| TIME            | 32 bit | Unsigned milliseconds             |
| TIME\_OF\_DAY   | 32 bit | Unsigned milliseconds since Jan 1, 1970 UTC |
| DATE            | 32 bit | Unsigned seconds since Jan 1, 1970 UTC |
| DATE\_AND\_TIME | 32 bit | Unsigned seconds since Jan 1, 1970 UTC |
| LTIME           | 64 bit | Nanoseconds                       |
| LTIME\_OF\_DAY  | 64 bit | Nanoseconds since Jan 1, 1970 UTC |
| LDATE           | 64 bit | Nanoseconds since Jan 1, 1970 UTC |
| LDATE\_AND\_TIME| 64 bit | Nanoseconds since Jan 1, 1970 UTC |

RuSTy treats the short date/time family (`TIME`, `TIME_OF_DAY`, `DATE`, `DATE_AND_TIME`) as 32-bit unsigned values.
Short `TIME`/`TOD` use milliseconds, while short `DATE`/`DT` use seconds.
The long family (`LTIME`, `LTOD`, `LDATE`, `LDT`) remains 64-bit nanosecond precision.

### DATE

The `DATE` datatype is used to represent a Date in the Gregorian Calendar.
Such a value is stored as a `u32` with a precision in seconds and denotes the number of seconds
that have elapsed since January 1, 1970 UTC not counting leap seconds.
DATE literals start with `DATE#` or `D#` followed by a date in the format of `yyyy-mm-dd`.

Examples:

- `d1 : DATE := DATE#2021-05-02;`
- `d2 : DATE := DATE#1-12-24;`
- `d3 : DATE := D#2000-1-1;`

### DATE_AND_TIME

The `DATE_AND_TIME` datatype is used to represent a certain point in time in the Gregorian Calendar.
Such a value is stored as a `u32` with a precision in seconds and denotes the
number of seconds that have elapsed since January 1, 1970 UTC not counting leap seconds.
DATE_AND_TIME literals start with `DATE_AND_TIME#` or `DT#` followed by a date and time in the
format of `yyyy-mm-dd-hh:mm:ss`.

Note that only the seconds-segment can have a fraction denoting the milliseconds.

Examples:

- `d1 : DATE_AND_TIME := DATE_AND_TIME#2021-05-02-14:20:10.25;`
- `d2 : DATE_AND_TIME := DATE_AND_TIME#1-12-24-00:00:1;`
- `d3 : DATE_AND_TIME := DT#1999-12-31-23:59:59.999;`

### TIME_OF_DAY

The `TIME_OF_DAY` datatype is used to represent a specific moment in time in a day.
Such a value is stored as a `u32` value with a precision in milliseconds and denotes the
number of milliseconds that have elapsed since January 1, 1970 UTC not counting leap seconds.
Hence this value is stored as a `DATE_AND_TIME` with the day fixed to 1970-01-01.
`TIME_OF_DAY` literals start with `TIME_OF_DAY#` or `TOD#` followed by a time in the
format of `hh:mm:ss`.

Note that only the seconds-segment can have a fraction denoting the milliseconds.

Examples:

- `t1 : TIME_OF_DAY := TIME_OF_DAY#14:20:10.25;`
- `t2 : TIME_OF_DAY := TIME_OF_DAY#0:00:1;`
- `t3 : TIME_OF_DAY := TOD#23:59:59.999;`

### TIME

The `TIME` datatype is used to represent a time-span.
A `TIME` value is stored as a `u32` value with a precision in milliseconds.
TIME literals start with `TIME#` or `T#` followed by the `TIME` segments.

Supported segments are:

- `d` ... `f64` days
- `h` ... `f64` hours
- `m` ... `f64`minutes
- `s` ... `f64` seconds
- `ms` ... `f64` milliseconds
- `us` ... `f64` microseconds
- `ns` ... `u32` nanaoseconds

Note that only the last segment of a `TIME` literal can have a fraction.

Examples:

- `t1 : TIME := TIME#2d4h6m8s10ms;`
- `t2 : TIME := T#2d4.2h;`
- `t3 : TIME := T#-10s4ms16ns;`

### LDATE

The `LDATE` datatype is used to represent a Date in the Gregorian Calendar.
Such a value is stored as an `i64` with a precision in nanoseconds and denotes the number of nanoseconds
that have elapsed since January 1, 1970 UTC not counting leap seconds.
LDATE literals start with `LDATE#` or `LD#` followed by a date in the format of `yyyy-mm-dd`.

Examples:

- `d1 : LDATE := LDATE#2021-05-02;`
- `d2 : LDATE := LDATE#1-12-24;`
- `d3 : LDATE := LD#2000-1-1;`

### LDATE_AND_TIME

The `LDATE_AND_TIME` datatype is used to represent a certain point in time in the Gregorian Calendar.
Such a value is stored as an `i64` with a precision in nanoseconds and denotes the
number of nanoseconds that have elapsed since January 1, 1970 UTC not counting leap seconds.
LDATE_AND_TIME literals start with `LDATE_AND_TIME#` or `LDT#` followed by a date and time in the
format of `yyyy-mm-dd-hh:mm:ss`.

Note that only the seconds-segment can have a fraction denoting the milliseconds.

Examples:

- `d1 : LDATE_AND_TIME := LDATE_AND_TIME#2021-05-02-14:20:10.25;`
- `d2 : LDATE_AND_TIME := LDATE_AND_TIME#1-12-24-00:00:1;`
- `d3 : LDATE_AND_TIME := LDT#1999-12-31-23:59:59.999;`

### LTIME_OF_DAY

The `LTIME_OF_DAY` datatype is used to represent a specific moment in time in a day.
Such a value is stored as an `i64` value with a precision in nanoseconds and denotes the
number of nanoseconds that have elapsed since January 1, 1970 UTC not counting leap seconds.
Hence this value is stored as a `LDATE_AND_TIME` with the day fixed to 1970-01-01.
`LTIME_OF_DAY` literals start with `LTIME_OF_DAY#` or `LTOD#` followed by a time in the
format of `hh:mm:ss`.

Note that only the seconds-segment can have a fraction denoting the milliseconds.

Examples:

- `t1 : LTIME_OF_DAY := LTIME_OF_DAY#14:20:10.25;`
- `t2 : LTIME_OF_DAY := LTIME_OF_DAY#0:00:1;`
- `t3 : LTIME_OF_DAY := LTOD#23:59:59.999;`

### LTIME

The `LTIME` datatype is used to represent a time-span.
A `LTIME` value is stored as an `i64` value with a precision in nanoseconds.
LTIME literals start with `LTIME#` or `LT#` followed by the `LTIME` segments.

Supported segments are:

- `d` ... `f64` days
- `h` ... `f64` hours
- `m` ... `f64`minutes
- `s` ... `f64` seconds
- `ms` ... `f64` milliseconds
- `us` ... `f64` microseconds
- `ns` ... `u32` nanaoseconds

Note that only the last segment of a `LTIME` literal can have a fraction.

Examples:

- `t1 : LTIME := LTIME#2d4h6m8s10ms;`
- `t2 : LTIME := LT#2d4.2h;`
- `t3 : LTIME := LT#-10s4ms16ns;`

## Other types

The `BOOL` type can either be assigned `TRUE` or `FALSE`.
The type `__VOID` is the empty type and has an undefined size.

| Type name | Size      | Properties |
| --------- | --------- | ---------- |
| BOOL      | 8 bit     | signed     |
| \_\_VOID  | undefined |            |

Bit datatypes are defined as follows:

| Type name | Size   | Properties |
| --------- | ------ | ---------- |
| BYTE      | 8 bit  | unsigned   |
| WORD      | 16 bit | unsigned   |
| DWORD     | 32 bit | unsigned   |
| LWORD     | 64 bit | unsigned   |
