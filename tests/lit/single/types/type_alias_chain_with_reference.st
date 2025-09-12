// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Test longer recursive chain broken by REF_TO
TYPE type1 : type2; END_TYPE
TYPE type2 : type3; END_TYPE
TYPE type3 : REF_TO type1; END_TYPE

FUNCTION main : DINT
    VAR
        var1 : type1;
        var3 : type3;
    END_VAR

    var1 := 42;
    var3 := REF(var1);

    // CHECK: var1: 42
    printf('var1: %d$N', var1);

    // CHECK: var3: 42
    printf('var3: %d$N', var3^);

    // Modify through reference chain
    var3^ := 200;

    // CHECK: Modified var1: 200
    printf('Modified var1: %d$N', var1);

    main := 0;
END_FUNCTION
