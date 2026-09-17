# Control Flow

Five statements decide what runs: two that choose, and three that repeat. Three more end a pass, a loop, or a POU early.


## IF

`IF` tests a condition and runs the block of the first test that is true:

```iecst
IF level > high THEN
    valve := Closed;
ELSIF level < low THEN
    valve := Open;
ELSE
    valve := Hold;
END_IF
```

`ELSIF` and `ELSE` are optional, and `ELSIF` can repeat. The condition is a `BOOL`; see [basic types](basic-types.md#bool) for what happens when you write a number.


## CASE

`CASE` chooses on one value. A branch can name one value, a list, or a range, and `ELSE` catches the rest:

```iecst
CASE step OF
    0:        motor := 0;
    1, 2:     motor := 50;
    3..6:     motor := 100;
ELSE
    motor := 0;
END_CASE
```

The value can be an integer or an enumeration, which is what makes `CASE` the natural shape for a state machine:

```iecst
TYPE State: (Idle, Running, Stopped);
END_TYPE

PROGRAM Machine
    VAR
        current: State;
        start, stop: BOOL;
    END_VAR

    CASE current OF
        Idle:    IF start THEN current := Running; END_IF
        Running: IF stop  THEN current := Stopped; END_IF
        Stopped: current := Idle;
    END_CASE
END_PROGRAM
```

Only the matching branch runs. There is no fall-through between branches.


## FOR

`IF` and `CASE` choose once. The next three statements repeat. `FOR` counts a variable from one value to another, and `BY` sets the step, which can be negative:

```iecst
FOR i := 1 TO 10 DO
    total := total + samples[i];
END_FOR

FOR i := 10 TO 1 BY -3 DO   (* 10, 7, 4, 1: four passes *)
    total := total + i;
END_FOR
```

The end value is included. The counter is an ordinary variable of the POU, and after the loop it keeps the value that ended it: after `FOR i := 1 TO 3`, `i` is `4`, and after `FOR i := 10 TO 1 BY -3`, `i` is `-2`. When the body never runs, the counter keeps the start value.


## WHILE and REPEAT

`FOR` needs the number of passes in advance. `WHILE` and `REPEAT` test a condition instead. `WHILE` tests before the body, so the body can run zero times. `REPEAT` tests after it, so the body always runs at least once. `UNTIL` states when to stop, and its condition takes no semicolon:

```iecst
WHILE remaining > 0 DO
    remaining := remaining - 1;
END_WHILE

REPEAT
    attempts := attempts + 1;
UNTIL attempts >= 3
END_REPEAT
```

A loop whose condition never becomes false never ends. There is no watchdog in the language.


## EXIT, CONTINUE and RETURN

`EXIT` leaves the loop, `CONTINUE` starts the next pass, and both act on the innermost loop only:

```iecst
FOR i := 1 TO 10 DO
    IF samples[i] = 0 THEN
        CONTINUE;
    END_IF
    IF samples[i] > limit THEN
        EXIT;
    END_IF
    total := total + samples[i];
END_FOR
```

`RETURN` leaves the POU at once. In a function, assign the result before you return:

```iecst
FUNCTION Divide: DINT
    VAR_INPUT
        a, b: DINT;
    END_VAR

    IF b = 0 THEN
        Divide := 0;
        RETURN;
    END_IF

    Divide := a / b;
END_FUNCTION
```


## What's next

The next chapter writes code that others can call: [functions](functions.md).
