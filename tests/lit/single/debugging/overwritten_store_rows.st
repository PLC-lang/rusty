// REQUIRES: system-linux

// End-to-end: an unoptimized build keeps a line-table row for every statement, which is
// what lets a debugger stop on it. The locals start at known values, so the backend computes
// both results ahead of time, and `x2 := t1` is overwritten before anything reads it. Without
// volatile key stores the backend drops that write, and the statement loses its row.

// RUN: %PLC --debug -Onone -c -o %t.x86_64.o %s
// RUN: readelf -W --debug-dump=decodedline %t.x86_64.o | %CHECK_IR %s
// RUN: %PLC --target aarch64-linux-gnu --debug -Onone -c -o %t.aarch64.o %s
// RUN: readelf -W --debug-dump=decodedline %t.aarch64.o | %CHECK_IR %s
// RUN: %PLC --target armv7-linux-gnueabihf --debug -Onone -c -o %t.armv7.o %s
// RUN: readelf -W --debug-dump=decodedline %t.armv7.o | %CHECK_IR %s

FUNCTION overwritten : INT
VAR
    x1 : INT := 4;
    x2 : INT := 2;
    t1 : INT;
    t2 : INT;
END_VAR
    t1 := -x1 + x2; // CHECK-DAG: .st{{ +}}[[@LINE]]{{ +}}{{[0-9a-fx]+}}{{.*}} x{{$}}
    x2 := t1;       // CHECK-DAG: .st{{ +}}[[@LINE]]{{ +}}{{[0-9a-fx]+}}{{.*}} x{{$}}
    t2 := x1 - x2;  // CHECK-DAG: .st{{ +}}[[@LINE]]{{ +}}{{[0-9a-fx]+}}{{.*}} x{{$}}
    x2 := t2;       // CHECK-DAG: .st{{ +}}[[@LINE]]{{ +}}{{[0-9a-fx]+}}{{.*}} x{{$}}
END_FUNCTION
