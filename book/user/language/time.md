# Time and Date

Four things can be measured: how long something takes, which day it is, which moment of a day it is, and which exact point in time it is. The language has a type for each, in two families.

The short family is 32 bits wide, the long family is 64 bits and starts with `L`:

| Short | Long | Holds | What the short type counts |
|---|---|---|---|
| `TIME` | `LTIME` | A duration | milliseconds |
| `DATE` | `LDATE` | A day | seconds since 1970-01-01 UTC |
| `TIME_OF_DAY` | `LTIME_OF_DAY` | A moment of a day | milliseconds since midnight |
| `DATE_AND_TIME` | `LDATE_AND_TIME` | A point in time | seconds since 1970-01-01 UTC |

Each long type counts the same thing as the short type next to it, but in nanoseconds. Each type also has a short name: `T`, `LT`, `D`, `LD`, `TOD`, `LTOD`, `DT`, `LDT`.

A part of a value that is finer than what the type counts is lost. `T#500us` is a `TIME` of zero, and `DT#1999-12-31-23:59:59.999` is the same `DATE_AND_TIME` as `DT#1999-12-31-23:59:59`. The long types count in nanoseconds and keep both.

The short types are unsigned. `DATE` and `DATE_AND_TIME` reach from 1970-01-01 to 2106-02-07, and `TIME` reaches `T#49d17h2m47s295ms`, about 49 days. A literal outside that range compiles with a warning and wraps around, so `D#1969-12-31` is a day in February 2106 and `T#49d17h2m47s296ms` is zero. The long types are signed and reach about 292 years to either side of 1970, from `LD#1677-09-22` to `LD#2262-04-11`. A long literal outside that range is rejected.


## Literals

A literal starts with the name of its type and `#`:

```iecst
VAR
    cycle: TIME := T#10ms;
    startup: TIME := TIME#2d4h6m8s10ms;
    day: DATE := D#2024-05-02;
    moment: TIME_OF_DAY := TOD#23:59:59.999;
    stamp: DATE_AND_TIME := DT#1999-12-31-23:59:59.999;
END_VAR
```

A duration is a sequence of segments, in the order `d`, `h`, `m`, `s`, `ms`, `us`, `ns`. You leave out the ones you do not need, and a segment can have a fraction:

```iecst
T#2d4h          (* two days and four hours *)
T#2d4.2h        (* a segment may be fractional *)
T#90s           (* a segment may exceed its usual range *)
```

The order is not optional. A literal that changes it, such as `T#4h2d`, is rejected.

In a moment of a day or a point in time, only the seconds can have a fraction. A date has no fraction at all.


## Calculating

Durations add and subtract, and they compare:

```iecst
VAR
    a: TIME := T#1s;
    b: TIME := T#500ms;
    total: TIME;
END_VAR

total := a + b;        (* 1500 ms *)
IF a > b THEN          (* TRUE *)
```

A duration also multiplies and divides by a number, which is how you scale a cycle time.

A negative duration does not fit into the unsigned `TIME`. A literal such as `T#-10s` still compiles, but the compiler warns about an underflow and the value wraps around to a large positive duration. Use `LTIME` when a duration must be able to go below zero: `LT#-10s` is a negative `LTIME` and gets no warning.

To read a time value as a number, convert it with a cast. `DINT#cycle` gives the milliseconds of a `TIME`, because that is what the type holds.

The standard library converts between the families and to text, for example `TIME_TO_LTIME` and `TIME_TO_STRING`, and it brings the timers `TON`, `TOF`, and `TP`, which are function blocks. The [standard library reference](../reference/standard-library.md) lists them.


## What's next

The next chapter builds bigger types out of the ones so far: [arrays, structs, and enumerations](composite-types.md).
