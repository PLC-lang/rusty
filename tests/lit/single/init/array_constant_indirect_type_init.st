// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE IntArrayBase : ARRAY[1..3] OF DINT := [11, 22, 33]; END_TYPE
TYPE IntArrayAlias : IntArrayBase; END_TYPE
TYPE IntArrayAlias2 : IntArrayAlias; END_TYPE

VAR_GLOBAL CONSTANT
    g_base : IntArrayBase;
    g_alias : IntArrayAlias;
    g_alias2 : IntArrayAlias2;
END_VAR

FUNCTION main : DINT
    printf('g_base=%d,%d,%d$N', g_base[1], g_base[2], g_base[3]); // CHECK: g_base=11,22,33
    printf('g_alias=%d,%d,%d$N', g_alias[1], g_alias[2], g_alias[3]); // CHECK: g_alias=11,22,33
    printf('g_alias2=%d,%d,%d$N', g_alias2[1], g_alias2[2], g_alias2[3]); // CHECK: g_alias2=11,22,33

    main := 0;
END_FUNCTION
