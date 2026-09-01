// REQUIRES: system-linux

// End-to-end: every CASE body keeps a line-table row of its own, which is what lets a
// debugger bind a breakpoint to it. All four bodies here end in the same call and two of
// them are identical, so LLVM's tail merging factors them into shared blocks. A block
// carries one source location, so a merged one is attributed to line 0 and the bodies it
// came from lose their rows.

// RUN: %PLC --debug -Onone -c -o %T/%basename_t.o %s
// RUN: readelf --debug-dump=decodedline %T/%basename_t.o | %CHECK_IR %s

FUNCTION_BLOCK Counter
VAR_INPUT step : INT; reset : BOOL; END_VAR
VAR_OUTPUT total : INT; END_VAR
    IF reset THEN total := 0; ELSE total := total + step; END_IF
END_FUNCTION_BLOCK

PROGRAM branches
VAR
    c : Counter;
    sel : INT;
    out : INT;
END_VAR
    CASE sel OF
        0: c(step := 1, reset := FALSE, total => out); // CHECK-DAG: .st{{ +}}[[@LINE]]{{ +}}
        1: c(step := 2, reset := FALSE, total => out); // CHECK-DAG: .st{{ +}}[[@LINE]]{{ +}}
        2: c(step := 3, reset := TRUE, total => out);  // CHECK-DAG: .st{{ +}}[[@LINE]]{{ +}}
        3: c(step := 1, reset := FALSE, total => out); // CHECK-DAG: .st{{ +}}[[@LINE]]{{ +}}
    END_CASE
END_PROGRAM
