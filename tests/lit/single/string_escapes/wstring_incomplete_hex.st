// RUN: not %COMPILE %s
// WSTRING hex escapes require exactly 4 hex digits ($hhhh). '$004' provides
// only three hex digits before end-of-string, which is an incomplete escape
// sequence and is a compile error per IEC 61131-3 (E124).

FUNCTION main : DINT
VAR
    s : WSTRING[10] := "$004";
END_VAR
    main := 0;
END_FUNCTION
