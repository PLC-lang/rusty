// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Test recursive type alias broken by REFERENCE TO
TYPE type1 : type2; END_TYPE
TYPE type2 : REFERENCE TO type1; END_TYPE

FUNCTION main : DINT
    VAR
        var1 : type1;
        var2 : type2;
    END_VAR

    var1 := 42;
    var2 REF= var1;

    // CHECK: var1: 42
    printf('var1: %d$N', var1);

    // CHECK: var2: 42
    printf('var2: %d$N', var2);

    // Modify through reference
    var2 := 100;

    // CHECK: Modified var1: 100
    printf('Modified var1: %d$N', var1);

    main := 0;
END_FUNCTION
