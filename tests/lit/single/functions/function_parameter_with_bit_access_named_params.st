// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION FLIP
    VAR_INPUT
        in: BYTE;
    END_VAR
    VAR_OUTPUT
        out : BYTE;
    END_VAR
    out := NOT(in);
END_FUNCTION

FUNCTION main
    VAR
        wordVar : WORD;
    END_VAR

    wordVar := 0;
    FLIP(in := 2#01010101, out => wordVar.%b0);

    // CHECK: wordVar=170
    PRINTF('wordVar=%d$N', wordVar);
END_FUNCTION
