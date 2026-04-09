// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Verify that array type aliases with struct initializers produce correct runtime values.
TYPE SomeStruct : STRUCT
    field4 : USINT;
END_STRUCT
END_TYPE

TYPE SomeArray : ARRAY[1..2] OF SomeStruct; END_TYPE
TYPE MySomeArrayWInit : SomeArray := [(field4 := 2), (field4 := 2)]; END_TYPE

VAR_GLOBAL
    g_arr : MySomeArrayWInit;
END_VAR

FUNCTION main : DINT
VAR
    local_arr : MySomeArrayWInit;
END_VAR
    // Global instance should carry the type-level initializer
    printf('%d$N', g_arr[1].field4);   // CHECK: 2
    printf('%d$N', g_arr[2].field4);   // CHECK: 2

    // Local instance should also be initialized via the type ctor
    printf('%d$N', local_arr[1].field4);  // CHECK: 2
    printf('%d$N', local_arr[2].field4);  // CHECK: 2
    main := 0;
END_FUNCTION
