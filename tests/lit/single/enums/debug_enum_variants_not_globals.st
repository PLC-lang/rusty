// RUN: %COMPILE_IR %s --ir -g | %CHECK_IR %s

// Test that enum variants are NOT included in global variable debug info
// but the enum type itself and variables using it ARE included
// Variants like Color.red should not appear as global variables

TYPE Color : (red, green, blue);
END_TYPE

TYPE Status : BYTE (inactive, active, error);
END_TYPE

VAR_GLOBAL
    current_color : Color := red;
    current_status : Status := active;
END_VAR

PROGRAM prog
    current_color := green;
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION

// The global variables should have debug info (in order they appear in IR)
// CHECK: !DIGlobalVariable(name: "current_color"
// CHECK: !DICompositeType(tag: DW_TAG_enumeration_type, name: "Color"
// CHECK: !DIEnumerator(name: "red", value: 0)
// CHECK: !DIEnumerator(name: "green", value: 1)
// CHECK: !DIEnumerator(name: "blue", value: 2)

// CHECK: !DIGlobalVariable(name: "current_status"
// CHECK: !DICompositeType(tag: DW_TAG_enumeration_type, name: "Status"
// CHECK: !DIEnumerator(name: "inactive", value: 0
// CHECK: !DIEnumerator(name: "active", value: 1
// CHECK: !DIEnumerator(name: "error", value: 2

// Enum variants should NOT appear as global variables
// CHECK-NOT: !DIGlobalVariable(name: "red"
// CHECK-NOT: !DIGlobalVariable(name: "green"
// CHECK-NOT: !DIGlobalVariable(name: "blue"
// CHECK-NOT: !DIGlobalVariable(name: "inactive"
// CHECK-NOT: !DIGlobalVariable(name: "active"
// CHECK-NOT: !DIGlobalVariable(name: "error"
