// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION method_with_aggregagte_return: ARRAY[0..1] OF STRING
VAR_INPUT
    in: STRING;
    in2: STRING;
END_VAR
    method_with_aggregagte_return[0] := in;
    method_with_aggregagte_return[1] := in2;
END_FUNCTION

FUNCTION main : DINT
VAR
    ret : ARRAY[0..1] OF STRING;
END_VAR
    ret := method_with_aggregagte_return('Hello', 'World!');
    printf('%s$N', ADR(ret[0])); //CHECK Hello
    printf('%s$N', ADR(ret[1])); //CHECK World!
END_FUNCTION
