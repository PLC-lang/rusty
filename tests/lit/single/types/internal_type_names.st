// RUN: %COMPILE %s --error-format=clang 2> %t.err
// RUN: %CHECK %s --input-file=%t.err
// RUN: %RUN
// CHECK: {{.*}}warning[E158]: Type name '__Alias' starts with '__', a prefix for compiler-generated types
// CHECK: {{.*}}warning[E158]: Type name '__Struct' starts with '__', a prefix for compiler-generated types
// CHECK: {{.*}}warning[E158]: Type name '__Enum' starts with '__', a prefix for compiler-generated types
// CHECK: {{.*}}warning[E158]: Type name '__Array' starts with '__', a prefix for compiler-generated types
// CHECK-NOT: warning

TYPE
    __Alias : DINT;
    __Struct : STRUCT field : DINT; END_STRUCT
    __Enum : (First, Second);
    __Array : ARRAY[0..1] OF DINT;
END_TYPE

FUNCTION main : DINT
    VAR
        value : __Alias := 7;
        record : __Struct;
        choice : __Enum := First;
        items : __Array;
    END_VAR

    record.field := value;
    items[0] := record.field;
    main := items[0] - 7;
END_FUNCTION
