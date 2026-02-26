// RUN: %COMPILE_IR %s --ir -g | %CHECK_IR %s

// Test that unsigned vs signed enums generate correct isUnsigned flag in DIEnumerator
// Signed types (INT, DINT, LINT) should NOT have isUnsigned flag
// Unsigned types (BYTE, WORD, DWORD, LWORD, UINT, UDINT, ULINT) should have isUnsigned: true

TYPE SignedEnum : INT (neg := -10, zero := 0, pos := 10);
END_TYPE

TYPE UnsignedEnum : BYTE (small, medium, large);
END_TYPE

TYPE DefaultEnum : (a, b, c);  // DINT by default (signed)
END_TYPE

VAR_GLOBAL
    signed_val : SignedEnum := zero;
    unsigned_val : UnsignedEnum := small;
    default_val : DefaultEnum := a;
END_VAR

PROGRAM prog
    signed_val := pos;
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION

// Signed enum should NOT have isUnsigned flag
// CHECK: !DICompositeType(tag: DW_TAG_enumeration_type, name: "SignedEnum"
// CHECK-SAME: baseType: ![[SIGNED_BASE:[0-9]+]]
// CHECK: ![[SIGNED_BASE]] = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed

// CHECK: !DIEnumerator(name: "neg", value: -10)
// CHECK-NOT: !DIEnumerator(name: "neg", value: -10, isUnsigned: true)
// CHECK: !DIEnumerator(name: "zero", value: 0)
// CHECK-NOT: !DIEnumerator(name: "zero", value: 0, isUnsigned: true)
// CHECK: !DIEnumerator(name: "pos", value: 10)
// CHECK-NOT: !DIEnumerator(name: "pos", value: 10, isUnsigned: true)

// Unsigned enum SHOULD have isUnsigned: true
// CHECK: !DICompositeType(tag: DW_TAG_enumeration_type, name: "UnsignedEnum"
// CHECK-SAME: baseType: ![[UNSIGNED_BASE:[0-9]+]]
// CHECK: ![[UNSIGNED_BASE]] = !DIBasicType(name: "BYTE", size: 8, encoding: DW_ATE_unsigned

// CHECK: !DIEnumerator(name: "small", value: 0, isUnsigned: true)
// CHECK: !DIEnumerator(name: "medium", value: 1, isUnsigned: true)
// CHECK: !DIEnumerator(name: "large", value: 2, isUnsigned: true)

// Default enum (DINT) should NOT have isUnsigned flag
// CHECK: !DICompositeType(tag: DW_TAG_enumeration_type, name: "DefaultEnum"
// CHECK-SAME: baseType: ![[DEFAULT_BASE:[0-9]+]]
// CHECK: ![[DEFAULT_BASE]] = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed

// CHECK: !DIEnumerator(name: "a", value: 0)
// CHECK-NOT: !DIEnumerator(name: "a", value: 0, isUnsigned: true)
