// Exercises the C variadic contracts from variadic_contracts.c against the implicit terminator
// that every untyped (`args : ...`) call carries. Each group states what would break without it,
// or what must stay unchanged because of it.

FUNCTION main : DINT
VAR
    a : STRING := 'alpha';
    b : STRING := 'beta';
    c : STRING := 'gamma';
    d : STRING := 'delta';
    e : STRING := 'epsilon';
    f : STRING := 'zeta';
    g : STRING := 'eta';
    h : STRING := 'theta';
    terminator : POINTER TO BYTE := NULL;
    count : DINT;
    first : DINT := 1;
    second : DINT := 2;
END_VAR
    // A terminator-directed callee reads past the argument list when the call carries no
    // variadic argument at all. This is the shape that segfaults without the terminator.
    SentinelPrinter('empty:');
    // CHECK: empty:

    SentinelPrinter('two:', a, b);
    // CHECK: two: alpha beta

    SentinelPrinter('literal:', 'inline');
    // CHECK: literal: inline

    // More arguments than the ABI passes in registers, so the terminator lands on the stack.
    SentinelPrinter('spill:', a, b, c, d, e, f, g, h);
    // CHECK: spill: alpha beta gamma delta epsilon zeta eta theta

    // A null written by the caller ends the list where the caller put it. The implicit
    // terminator sits behind it and is never read, so terminating twice is harmless.
    SentinelPrinter('explicit:', a, terminator, b);
    // CHECK: explicit: alpha

    // A count-directed callee reads exactly `count` arguments and never sees the terminator.
    CountPrinter(0);
    // CHECK: count=0
    CountPrinter(2, a, b);
    // CHECK: count=2 alpha beta

    // Exactly one terminator arrives, whatever the number of arguments before it.
    count := ArgsBeforeTerminator('none');
    printf('before terminator: %d$N', count);
    // CHECK: before terminator: 0
    count := ArgsBeforeTerminator('two', a, b);
    printf('before terminator: %d$N', count);
    // CHECK: before terminator: 2

    // A by-ref variadic block passes every argument as a pointer, terminator included.
    RefSentinelPrinter('byref:', first, second);
    // CHECK: byref: 1 2

    // Mixed register classes: a pointer, a 32-bit integer, a 64-bit integer and a REAL that C
    // default argument promotion widens to double, followed by the terminator.
    FormatPrinter('mixed: %s %d %lld %.1f$N', a, DINT#42, LINT#9, REAL#1.5);
    // CHECK: mixed: alpha 42 9 1.5

    // More floating point arguments than the ABI passes in vector registers. The terminator is
    // an integer-class argument, so it does not change the vector register count the ABI reports.
    FormatPrinter('floats: %.1f %.1f %.1f %.1f %.1f %.1f %.1f %.1f %.1f$N',
        REAL#1.5, REAL#2.5, REAL#3.5, REAL#4.5, REAL#5.5, REAL#6.5, REAL#7.5, REAL#8.5, REAL#9.5);
    // CHECK: floats: 1.5 2.5 3.5 4.5 5.5 6.5 7.5 8.5 9.5

    // Typed variadics keep their exact argument list. A terminator would be read as data here.
    TypedCountPrinter(3, DINT#1, DINT#2, DINT#3);
    // CHECK: typed: 1 2 3
    SizedPrinter(DINT#10, DINT#20, DINT#30);
    // CHECK: sized: 10 20 30

    // A format string without conversion specifiers consumes no argument at all.
    printf('plain$N');
    // CHECK: plain
END_FUNCTION
