# Datatypes

## Strings
### STRING
rusty treats `STRING`s as byte-arrays storing UTF-8 character bytes with a Null-terminator (0-byte) at the end. 
So a String of size n requres n+1 bytes to account for the Null-terminator.
A `STRING` literal is surrounded by single-ticks `'`.

A String has a well defined length which can be defined similar to the array-syntax. A String-variable 
`myVariable: STRING[20]` declares a byte array of length 21, to store 20 utf8 character bytes. When 
declaring a `STRING`, the length-attribute is optional. The default length is 80.

Examples
- `s1 : STRING;` - declares a String of length 80
- `s2 : STRING[20];` - declares a String of length 20
- `s3 : STRING := 'Hello World';` - declares and initializes a String of length 80, and initializes it with the utf8 characters and a null-terminator at the end
- `s4 : STRING[55] := 'Foo Baz';` - declares and initializes a String of length 55 and initializes it with the utf8 characters and a null-terminator at the end.

### WSTRING (Wide Strings)
rusty treats `WSTRING`s as byte-arrays storing UTF-16 character bytes with two Null-terminator bytes at the end. The bytes are stored in Little Endian encoding. A Wide-String of size n requres 2 * (n+1) bytes to account for the 2 byes per utf16 character and the Null-terminators. A `WSTRING` literal is surrounded by doubly-ticks `"`.

A `WSTRING` has a well defined length which can be defined similar to the array-syntax. A `WSTRING`-variable 
`myVariable: WSTRING[20]` declares a byte array of length 42, to store 20 utf16 character bytes. When 
declaring a `WSTRING`, the length-attribute is optional. The default length is 80.

Examples
- `ws1 : WSTRING;` - declares a Wide-String of length 80
- `ws2 : WSTRING[20];` - declares a Wide-String of length 20
- `ws3 : WSTRING := "Hello World";` - declares and initializes a Wide-String of length 80, and initializes it with the utf16 characters and a utf16-null-terminator at the end
- `ws4 : WSTRING[55] := "Foo Baz";` - declares and initializes a Wide-String of length 55 and initializes it with the utf8 characters and a utf16-null-terminator at the end.

## Date and Time
### DATE
The `DATE` datatype is used to represent a Date in the Gregorian Calendar. Such a value is 
stored as an i64 with a precision in milliseconds and denotes the number of milliseconds 
that have elapsed since January 1, 1970 UTC not counting leap seconds. DATE literals start 
with `DATE#` or `D#` followed by a date in the format of `yyyy-mm-dd`.

Examples
- `d1 : DATE := DATE#2021-05-02;`
- `d2 : DATE := DATE#1-12-24;`
- `d3 : DATE := D#2000-1-1;`

### DATE_AND_TIME
The `DATE_AND_TIME` datatype is used to represent a certain point in time in the Gregorian Calendar.
Such a value is stored as an `i64` with a precision in milliseconds and denotes the
number of milliseconds that have elapsed since January 1, 1970 UTC not counting leap seconds.
DATE_AND_TIME literals start with `DATE_AND_TIME#` or `DT#` followed by a date and time in the
format of `yyyy-mm-dd-hh:mm:ss`.

Note that only the seconds-segment can have a fraction denoting the milliseconds.

Examples
- `d1 : DATE_AND_TIME := DATE_AND_TIME#2021-05-02-14:20:10.25;`
- `d2 : DATE_AND_TIME := DATE_AND_TIME#1-12-24-00:00:1;`
- `d3 : DATE_AND_TIME := DT#1999-12-31-23:59:59.999;`

### TIME_OF_DAY
The `TIME_OF_DAY` datatype is used to represent a specific moment in time in a day.
Such a value is stored as an `i64` value with a precision in milliseconds and denotes the
number of milliseconds that have elapsed since January 1, 1970 UTC not counting leap seconds.
Hence this value is stored as a `DATE_AND_TIME` with the day fixed to 1970-01-01.
`TIME_OF_DAY` literals start with `TIME_OF_DAY#` or `TOD#` followed by a time in the
format of `hh:mm:ss`.

Note that only the seconeds-segment can have a fraction denoting the milliseconds.

Examples
- `t1 : TIME_OF_DAY := TIME_OF_DAY#14:20:10.25;`
- `t2 : TIME_OF_DAY := TIME_OF_DY#0:00:1;`
- `t3 : TIME_OF_DAY := TOD#23:59:59.999;`

### TIME
The `TIME` datatype is used to represent a time-span. A `TIME` value is stored as an
`i64` value with a precision in nanoseconds.
TIME literals start with `TIME#` or `T#` followed by the `TIME` segements. Supported segements are:
- `d` ... `f64` days
- `h` ... `f64` hours
- `m` ... `f64`minutes
- `s` ... `f64` seconds
- `ms` ... `f64` milliseconds
- `us` ... `f64` microseconds
- `ns` ... `u32` nanaoseconds

Note that only the last segment of a `TIME` literal can have a fraction.

Examples
- `t1 : TIME := TIME#2d4h6m8s10ms;`
- `t2 : TIME := T#2d4.2h;`
- `t3 : TIME := T#-10s4ms16ns;`