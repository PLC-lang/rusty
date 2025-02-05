// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK foo
VAR
    s : STRING;
END_VAR
METHOD baz
    s := 'hello';
END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK bar EXTENDS foo
    s := 'world';
END_FUNCTION_BLOCK

FUNCTION main: DINT
VAR
    s: STRING;
    fb: bar;
END_VAR
    fb.baz(); // CHECK: hello
    fb(); // CHECK: world
END_FUNCTION
