// RUN: %COMPILE_IR %s --ir -g | %CHECK_IR %s

// Test that typed enums (BYTE, INT, LINT) generate correct debug info with proper base types

TYPE ByteStatus : BYTE (off, on, error);
END_TYPE

TYPE IntLevel : INT (low, medium, high);
END_TYPE

TYPE LintCounter : LINT (zero, one, two);
END_TYPE

VAR_GLOBAL
    status : ByteStatus := off;
    level : IntLevel := low;
    counter : LintCounter := zero;
END_VAR

PROGRAM prog
    status := on;
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION

// CHECK: !DICompositeType(tag: DW_TAG_enumeration_type, name: "ByteStatus"
// CHECK-SAME: baseType: ![[BYTE_BASE:[0-9]+]]
// CHECK-SAME: size: 8, align: 8

// CHECK: ![[BYTE_BASE]] = !DIBasicType(name: "BYTE", size: 8, encoding: DW_ATE_unsigned

// CHECK: !DICompositeType(tag: DW_TAG_enumeration_type, name: "IntLevel"
// CHECK-SAME: baseType: ![[INT_BASE:[0-9]+]]
// CHECK-SAME: size: 16, align: 16

// CHECK: ![[INT_BASE]] = !DIBasicType(name: "INT", size: 16, encoding: DW_ATE_signed

// CHECK: !DICompositeType(tag: DW_TAG_enumeration_type, name: "LintCounter"
// CHECK-SAME: baseType: ![[LINT_BASE:[0-9]+]]
// CHECK-SAME: size: 64, align: 64

// CHECK: ![[LINT_BASE]] = !DIBasicType(name: "LINT", size: 64, encoding: DW_ATE_signed
