// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// This test verifies that array of struct initialization works correctly
// with different initializer styles:
// 1. Single element initialization in VAR declaration (only first element, rest defaults to zero)
// 2. Full initialization in VAR declaration (all elements explicitly initialized)
// 3. Multiple struct initializers in body assignment

TYPE STRUCT2 :
    STRUCT
        x1 : BOOL;
        x2 : DINT;
        x3 : DINT;
        x4 : DINT;
    END_STRUCT
END_TYPE

PROGRAM mainProg
VAR
    // Single element initializer - only arr1[0] is initialized, arr1[1] should be zeroed
    arr1 : ARRAY[0..1] OF STRUCT2 := [(x1 := TRUE, x2 := 128, x3 := 12, x4 := 421)];
    // Full initializer - both elements explicitly initialized
    arr2 : ARRAY[0..1] OF STRUCT2 := [(x1 := TRUE, x2 := 100), (x1 := FALSE, x2 := 200)];
    // Array for body assignment test
    arr3 : ARRAY[0..1] OF STRUCT2;
END_VAR
    // Multiple struct initializers in body assignment
    arr3 := [(x1 := TRUE, x2 := 500, x3 := 600, x4 := 700), (x1 := FALSE, x2 := 800, x3 := 900, x4 := 1000)];
END_PROGRAM

FUNCTION main : DINT
    mainProg();

    // Check arr1[0] - should have the initialized values
    printf('arr1[0].x1=%d$N', mainProg.arr1[0].x1); // CHECK: arr1[0].x1=1
    printf('arr1[0].x2=%d$N', mainProg.arr1[0].x2); // CHECK: arr1[0].x2=128
    printf('arr1[0].x3=%d$N', mainProg.arr1[0].x3); // CHECK: arr1[0].x3=12
    printf('arr1[0].x4=%d$N', mainProg.arr1[0].x4); // CHECK: arr1[0].x4=421

    // Check arr1[1] - should be zeroed (default initialization)
    printf('arr1[1].x1=%d$N', mainProg.arr1[1].x1); // CHECK: arr1[1].x1=0
    printf('arr1[1].x2=%d$N', mainProg.arr1[1].x2); // CHECK: arr1[1].x2=0
    printf('arr1[1].x3=%d$N', mainProg.arr1[1].x3); // CHECK: arr1[1].x3=0
    printf('arr1[1].x4=%d$N', mainProg.arr1[1].x4); // CHECK: arr1[1].x4=0

    // Check arr2[0] - should have the initialized values, unspecified fields are zero
    printf('arr2[0].x1=%d$N', mainProg.arr2[0].x1); // CHECK: arr2[0].x1=1
    printf('arr2[0].x2=%d$N', mainProg.arr2[0].x2); // CHECK: arr2[0].x2=100
    printf('arr2[0].x3=%d$N', mainProg.arr2[0].x3); // CHECK: arr2[0].x3=0
    printf('arr2[0].x4=%d$N', mainProg.arr2[0].x4); // CHECK: arr2[0].x4=0

    // Check arr2[1] - should have the initialized values, unspecified fields are zero
    printf('arr2[1].x1=%d$N', mainProg.arr2[1].x1); // CHECK: arr2[1].x1=0
    printf('arr2[1].x2=%d$N', mainProg.arr2[1].x2); // CHECK: arr2[1].x2=200
    printf('arr2[1].x3=%d$N', mainProg.arr2[1].x3); // CHECK: arr2[1].x3=0
    printf('arr2[1].x4=%d$N', mainProg.arr2[1].x4); // CHECK: arr2[1].x4=0

    // Check arr3[0] - body assignment with multiple struct initializers
    printf('arr3[0].x1=%d$N', mainProg.arr3[0].x1); // CHECK: arr3[0].x1=1
    printf('arr3[0].x2=%d$N', mainProg.arr3[0].x2); // CHECK: arr3[0].x2=500
    printf('arr3[0].x3=%d$N', mainProg.arr3[0].x3); // CHECK: arr3[0].x3=600
    printf('arr3[0].x4=%d$N', mainProg.arr3[0].x4); // CHECK: arr3[0].x4=700

    // Check arr3[1] - body assignment with multiple struct initializers
    printf('arr3[1].x1=%d$N', mainProg.arr3[1].x1); // CHECK: arr3[1].x1=0
    printf('arr3[1].x2=%d$N', mainProg.arr3[1].x2); // CHECK: arr3[1].x2=800
    printf('arr3[1].x3=%d$N', mainProg.arr3[1].x3); // CHECK: arr3[1].x3=900
    printf('arr3[1].x4=%d$N', mainProg.arr3[1].x4); // CHECK: arr3[1].x4=1000
END_FUNCTION
