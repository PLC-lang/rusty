// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    arr : ARRAY[0..3] OF STRING := ['a', 'b', 'c', 'd'];
    alias AT arr : ARRAY[0..3] OF STRING;
END_VAR

FUNCTION main: DINT
    __init();

    // CHECK: a, b, c, d
    printf('%s, %s, %s, %s$N', REF(alias[0]), REF(alias[1]), REF(alias[2]), REF(alias[3]));
END_FUNCTION