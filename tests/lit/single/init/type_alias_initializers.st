// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Comprehensive test for type aliases with initializers (enum, struct, array).
TYPE SomeEnum : (E1, E2, E3); END_TYPE
TYPE MySomeEnumWInit : SomeEnum := E2; END_TYPE

TYPE SomeStruct : STRUCT
    field2 : DINT;
    field3 : BOOL;
    field4 : USINT;
END_STRUCT
END_TYPE
TYPE MySomeStructWInit : SomeStruct := (field3 := FALSE, field2 := 9); END_TYPE

TYPE SomeArray : ARRAY[1..2] OF SomeStruct; END_TYPE
TYPE MySomeArrayWInit : SomeArray := [(field4 := 2), (field4 := 2)]; END_TYPE

// Aliases without custom initializers (should inherit defaults)
TYPE MySomeEnum : SomeEnum; END_TYPE
TYPE MySomeStruct : SomeStruct; END_TYPE
TYPE MySomeArray : SomeArray; END_TYPE

FUNCTION main : DINT
VAR
    e : MySomeEnumWInit;
    s : MySomeStructWInit;
    a : MySomeArrayWInit;
    e_default : MySomeEnum;
    s_default : MySomeStruct;
    a_default : MySomeArray;
END_VAR
    // Enum alias with initializer (E1=0, E2=1)
    printf('%d$N', e);            // CHECK: 1

    // Struct alias with initializer
    printf('%d$N', s.field2);     // CHECK: 9
    printf('%d$N', s.field3);     // CHECK: 0

    // Array alias with struct initializer
    printf('%d$N', a[1].field4);  // CHECK: 2
    printf('%d$N', a[2].field4);  // CHECK: 2

    // Default enum (should be 0 / E1)
    printf('%d$N', e_default);    // CHECK: 0

    // Default struct fields (should be 0)
    printf('%d$N', s_default.field2);  // CHECK: 0

    // Default array element (should be 0)
    printf('%d$N', a_default[1].field4);  // CHECK: 0

    main := 0;
END_FUNCTION
