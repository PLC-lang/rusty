// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK fb_with_method
VAR_TEMP
    ret : STRING;
END_VAR
    METHOD method_with_aggregagte_return: STRING
    VAR_INPUT
        in: STRING;
    END_VAR
        method_with_aggregagte_return := in;
    END_METHOD

    ret := method_with_aggregagte_return('Hello');
    printf('%s$N', ADR(ret)); // CHECK: Hello
END_FUNCTION_BLOCK

FUNCTION main
VAR
    fb: fb_with_method;
    ret : STRING;
END_VAR

    fb();
    ret := fb.method_with_aggregagte_return('World!');
    printf('%s$N', ADR(ret)); // CHECK: World!
END_FUNCTION
