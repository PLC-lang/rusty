// RUN: %COMPILE_IR %s --ir -g | %CHECK_IR %s

// Test that inline (anonymous) enum variables generate proper debug info
// These are enums declared directly in VAR blocks without TYPE declaration

VAR_GLOBAL
    state : DINT (idle, running, stopped);
    mode : BYTE (manual, automatic);
    priority : INT (low := 1, medium := 5, high := 10);
END_VAR

PROGRAM prog
    state := running;
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION

// Inline enum for 'state' variable should generate a unique type name
// CHECK: !DICompositeType(tag: DW_TAG_enumeration_type, name: "__global_state"
// CHECK-SAME: baseType: ![[STATE_BASE:[0-9]+]]
// CHECK-SAME: elements: ![[STATE_ELEMENTS:[0-9]+]]

// CHECK: ![[STATE_BASE]] = !DIBasicType(name: "DINT", size: 32, encoding: DW_ATE_signed

// CHECK: ![[STATE_ELEMENTS]] = !{![[S0:[0-9]+]], ![[S1:[0-9]+]], ![[S2:[0-9]+]]}
// CHECK: ![[S0]] = !DIEnumerator(name: "idle", value: 0)
// CHECK: ![[S1]] = !DIEnumerator(name: "running", value: 1)
// CHECK: ![[S2]] = !DIEnumerator(name: "stopped", value: 2)

// Inline enum for 'mode' variable
// CHECK: !DICompositeType(tag: DW_TAG_enumeration_type, name: "__global_mode"
// CHECK-SAME: baseType: ![[MODE_BASE:[0-9]+]]
// CHECK-SAME: size: 8

// CHECK: ![[MODE_BASE]] = !DIBasicType(name: "BYTE", size: 8, encoding: DW_ATE_unsigned

// Inline enum for 'priority' with explicit values
// CHECK: !DICompositeType(tag: DW_TAG_enumeration_type, name: "__global_priority"
// CHECK-SAME: elements: ![[PRIO_ELEMENTS:[0-9]+]]

// CHECK: ![[PRIO_ELEMENTS]] = !{![[PR0:[0-9]+]], ![[PR1:[0-9]+]], ![[PR2:[0-9]+]]}
// CHECK: ![[PR0]] = !DIEnumerator(name: "low", value: 1)
// CHECK: ![[PR1]] = !DIEnumerator(name: "medium", value: 5)
// CHECK: ![[PR2]] = !DIEnumerator(name: "high", value: 10)
