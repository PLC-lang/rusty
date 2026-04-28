// RUN: %COMPILE_IR %s | %CHECK_IR %s

// Test that enums with explicit values generate correct DIEnumerator entries

TYPE HttpStatus : INT (
    ok := 200,
    not_found := 404,
    server_error := 500
);
END_TYPE

TYPE Powers : DINT (
    one := 1,
    ten := 10,
    hundred := 100,
    thousand := 1000
);
END_TYPE

VAR_GLOBAL
    http_code : HttpStatus := ok;
    power : Powers := one;
END_VAR

PROGRAM prog
    http_code := not_found;
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION

// CHECK: !DICompositeType(tag: DW_TAG_enumeration_type, name: "HttpStatus"
// CHECK-SAME: elements: ![[HTTP_ELEMENTS:[0-9]+]]

// CHECK: ![[HTTP_ELEMENTS]] = !{![[H0:[0-9]+]], ![[H1:[0-9]+]], ![[H2:[0-9]+]]}
// CHECK: ![[H0]] = !DIEnumerator(name: "ok", value: 200)
// CHECK: ![[H1]] = !DIEnumerator(name: "not_found", value: 404)
// CHECK: ![[H2]] = !DIEnumerator(name: "server_error", value: 500)

// CHECK: !DICompositeType(tag: DW_TAG_enumeration_type, name: "Powers"
// CHECK-SAME: elements: ![[POWER_ELEMENTS:[0-9]+]]

// CHECK: ![[POWER_ELEMENTS]] = !{![[P0:[0-9]+]], ![[P1:[0-9]+]], ![[P2:[0-9]+]], ![[P3:[0-9]+]]}
// CHECK: ![[P0]] = !DIEnumerator(name: "one", value: 1)
// CHECK: ![[P1]] = !DIEnumerator(name: "ten", value: 10)
// CHECK: ![[P2]] = !DIEnumerator(name: "hundred", value: 100)
// CHECK: ![[P3]] = !DIEnumerator(name: "thousand", value: 1000)
