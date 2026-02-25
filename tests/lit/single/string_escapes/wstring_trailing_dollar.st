// RUN: not %COMPILE %s
// A trailing '$' with nothing following it is an invalid escape sequence
// and is a compile error per IEC 61131-3 (E124).

FUNCTION main : DINT
VAR
    s : WSTRING[10] := "hello$";
END_VAR
    main := 0;
END_FUNCTION
