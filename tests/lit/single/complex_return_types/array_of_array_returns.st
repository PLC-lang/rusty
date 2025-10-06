// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// XFAIL: *
// Fails because of #1389
FUNCTION foo : ARRAY[0..1] OF ARRAY[0..1] OF INT
    VAR
        result : ARRAY[0..1] OF ARRAY[0..1] OF INT;
    END_VAR
    result[0][0] := 5;
    result[0][1] := 10;
    result[1][0] := 15;
    result[1][1] := 20;
    foo := result;
END_FUNCTION

FUNCTION bar : ARRAY[0..1] OF ARRAY[0..1] OF INT
    VAR
        data : ARRAY[0..1] OF ARRAY[0..1] OF INT;
    END_VAR
    data := foo();
    bar := data;
END_FUNCTION

FUNCTION baz : ARRAY[0..2] OF STRING[20]
    VAR
        texts : ARRAY[0..2] OF STRING[20];
    END_VAR
    texts[0] := 'One';
    texts[1] := 'Two';
    texts[2] := 'Three';
    baz := texts;
END_FUNCTION

FUNCTION main
    VAR
        numbers : ARRAY[0..1] OF ARRAY[0..1] OF INT;
        strings : ARRAY[0..2] OF STRING[20];
    END_VAR
    numbers := bar();
    strings := baz();

    // CHECK: 5 10 15 20
    printf('%d %d %d %d$N', numbers[0][0], numbers[0][1], numbers[1][0], numbers[1][1]);

    // CHECK: One Two Three
    printf('%s %s %s$N', REF(strings[0]), REF(strings[1]), REF(strings[2]));
END_FUNCTION
