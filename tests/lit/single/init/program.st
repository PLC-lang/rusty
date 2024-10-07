// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    gs: STRING;
END_VAR

PROGRAM prog
VAR
    s AT gs: STRING;
END_VAR
    s := 'Hello world!';

    // CHECK: Hello world!
    printf('%s$N', REF(gs));
END_PROGRAM

FUNCTION main: DINT
    prog();
END_FUNCTION
