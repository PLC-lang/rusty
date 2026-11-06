PROGRAM Prog
VAR_OUTPUT
    runs : DINT;
    words : ARRAY[0..1] OF STRING[5] := ['ab', 'cd'];
END_VAR
    METHOD twice : DINT
    VAR_INPUT
        p : STRING[10];
    END_VAR
        twice := runs * 2;
    END_METHOD

    runs := runs + 1;
END_PROGRAM

ACTIONS Prog
    ACTION reset
        runs := 0;
    END_ACTION
END_ACTIONS
