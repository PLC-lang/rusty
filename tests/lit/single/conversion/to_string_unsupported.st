// RUN: not %COMPILE %s 2>&1 | %CHECK %s
// A type with no TO_STRING monomorphization must not silently mis-dispatch to TO_STRING__STRING
// (which reinterprets the argument's bytes and crashes at runtime). Instead the missing
// monomorphization is materialized as an external declaration and surfaces as a link-time undefined
// symbol — a clean, uniform failure for scalar and aggregate returns alike. A user STRUCT never gets
// a monomorph, so this stays valid as follow-up PRs add the remaining scalar conversions.
TYPE MyStruct : STRUCT x : DINT; END_STRUCT END_TYPE

FUNCTION main : DINT
VAR
    st : MyStruct;
    s : STRING;
END_VAR
    // CHECK: {{.*}}TO_STRING__MyStruct{{.*}}
    s := TO_STRING(st);
END_FUNCTION
