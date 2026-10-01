// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK %s
//
// A call whose generic type stays unresolved names that type by its symbol in an invalid assignment,
// for a built-in generic function as well as for a generic function of the user.

FUNCTION foo<U: ANY> : U
VAR_INPUT
    in : U;
END_VAR
END_FUNCTION

FUNCTION main
VAR
    arr : ARRAY[0..5] OF INT;
    arr2 : ARRAY[0..5] OF INT;
END_VAR

// CHECK: {{.*}}error[E037]{{.*}}cannot assign 'U' to 'ARRAY[0..5] OF INT'{{.*}}
arr2 := MOVE(SOURCE := arr);

// CHECK: {{.*}}error[E037]{{.*}}cannot assign 'U' to 'ARRAY[0..5] OF INT'{{.*}}
arr2 := foo(SOURCE := arr);
END_FUNCTION
