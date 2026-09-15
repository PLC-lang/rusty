// RUN: %COMPILE_IR %s | %CHECK_IR %s

// A branch body that starts with instructions carrying no location of their own (the
// argument stores of a call) must be attributed to the body's own line, not to the
// location of whatever was generated before it. A body that shares its line with the
// enclosing condition or end keyword gets no line-table row of its own, and a debugger
// cannot then place a breakpoint on it.

FUNCTION_BLOCK Counter
VAR_INPUT step : INT; END_VAR
VAR_OUTPUT total : INT; END_VAR
    total := total + step;
END_FUNCTION_BLOCK

PROGRAM branches
VAR
    c : Counter;
    sel : INT;
    out : INT;
END_VAR
    CASE sel OF
        0: c(step := 1, total => out);
        1: c(step := 2, total => out);
    END_CASE
    IF sel > 0 THEN
        c(step := 3, total => out);
    END_IF
END_PROGRAM

FUNCTION main
    branches();
END_FUNCTION

// Every instruction of a body shares the body's own location, argument stores included.
// CHECK-LABEL: define void @branches
// CHECK: case:
// CHECK: store i16 1, ptr %{{[0-9]+}}, align 2, !dbg ![[CASE0:[0-9]+]]
// CHECK: call void @Counter(ptr %c), !dbg ![[CASE0]]
// CHECK: case1:
// CHECK: store i16 2, ptr %{{[0-9]+}}, align 2, !dbg ![[CASE1:[0-9]+]]
// CHECK: call void @Counter(ptr %c), !dbg ![[CASE1]]
// CHECK: condition_body:
// CHECK: store i16 3, ptr %{{[0-9]+}}, align 2, !dbg ![[IFBODY:[0-9]+]]
// CHECK: call void @Counter(ptr %c), !dbg ![[IFBODY]]

// And that location is the body's source line.
// CHECK-DAG: ![[CASE0]] = !DILocation(line: [[@LINE-25]]
// CHECK-DAG: ![[CASE1]] = !DILocation(line: [[@LINE-25]]
// CHECK-DAG: ![[IFBODY]] = !DILocation(line: [[@LINE-23]]
