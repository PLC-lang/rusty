// RUN: %COMPILE %s && %RUN | %CHECK %s
// A struct literal, an array literal, and a string on a VAR_OUTPUT are applied by the lowered
// stack initializer of the function; the entry reset only zero-fills aggregates.
TYPE S : STRUCT
    x : INT;
    y : INT;
END_STRUCT END_TYPE

FUNCTION f
    VAR_OUTPUT
        out : S := (x := 7);
        arr : ARRAY[0..1] OF INT := [1, 2];
        s : STRING := 'abc';
    END_VAR
END_FUNCTION

FUNCTION main
    VAR
        v : S;
        a : ARRAY[0..1] OF INT;
        str : STRING;
    END_VAR
    v.y := 5;
    f(out => v, arr => a, s => str);
    printf('v = (%d, %d), a = [%d, %d], s = %s$N', v.x, v.y, a[0], a[1], str);
    // CHECK: v = (7, 0), a = [1, 2], s = abc
END_FUNCTION
