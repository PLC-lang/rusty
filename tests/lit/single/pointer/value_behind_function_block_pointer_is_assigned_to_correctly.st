// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK file_t
    VAR_INPUT
        var1: BOOL;
        var2: BOOL;
    END_VAR

    VAR_OUTPUT
        out1: BOOL;
        out2: BOOL;
    END_VAR

    out1 := var1;
    out2 := var2;
END_FUNCTION_BLOCK

FUNCTION main: DINT
    VAR
        a: BOOL;
        b: BOOL;
        file: file_t;
        FileOpen: REF_TO file_t;
    END_VAR

    FileOpen := REF(file);
    FileOpen^(var1 := FALSE, var2 := TRUE, out1 => a, out2 => b);

    // CHECK: a = 0
    // CHECK: b = 1
    printf('a = %d$N', a);
    printf('b = %d$N', b);
END_FUNCTION