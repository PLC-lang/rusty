// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    ps: STRING := 'Hello world!';
    ps2: STRING;
END_VAR

FUNCTION_BLOCK foo
VAR
    s AT ps: STRING;
END_VAR
VAR_TEMP
    s2 AT ps2: STRING;
END_VAR
    s2 := 'Hello from me, too';

    // CHECK: Hello world!
    printf('%s$N', REF(s));
    // CHECK: Hello from me, too
    printf('%s$N', REF(s2));
END_FUNCTION_BLOCK

FUNCTION main: DINT
VAR
    fb: foo;
END_VAR
    fb();
END_FUNCTION