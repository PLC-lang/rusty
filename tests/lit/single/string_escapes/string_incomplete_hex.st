// RUN: not %COMPILE %s
// STRING hex escapes require exactly 2 hex digits ($hh). '$A' provides only
// one hex digit before end-of-string, which is an incomplete escape sequence
// and is a compile error per IEC 61131-3 (E124).

FUNCTION main : DINT
VAR
    s : STRING[10] := '$A';
END_VAR
    main := 0;
END_FUNCTION
