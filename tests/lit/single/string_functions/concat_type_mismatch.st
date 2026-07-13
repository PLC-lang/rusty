// RUN: not %COMPILE %s --error-format clang 2>&1 | %CHECK %s
// End-to-end guard for passing non-string arguments to the generic CONCAT stdlib function.
// CONCAT is `FUNCTION CONCAT <T: ANY_STRING> : T` with a `{ref} {sized} T...` variadic. Passing a
// non-ANY_STRING argument used to compile silently and then crash in codegen: the generic call is
// lowered to the concrete `CONCAT__STRING` and the following re-annotation dropped the generic
// nature before validation could see it. Every offending argument must now be reported, while
// genuine string arguments are left alone.

FUNCTION mainProg : DINT
VAR
    x : DINT;
    y : REAL;
    z : STRING;
END_VAR
    // Every non-string argument is flagged individually; `z` is a valid STRING and is not.
    // CHECK-DAG: {{.*}}concat_type_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E062]: Invalid type nature for generic argument. DINT is no ANY_STRING
    z := CONCAT(x, z);

    // CHECK-DAG: {{.*}}concat_type_mismatch.st:[[@LINE+2]]:{{[0-9]+}}:{{.*}}error[E062]: Invalid type nature for generic argument. DINT is no ANY_STRING
    // CHECK-DAG: {{.*}}concat_type_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E062]: Invalid type nature for generic argument. REAL is no ANY_STRING
    z := CONCAT(x, y, z);
END_FUNCTION
