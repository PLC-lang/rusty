// RUN: not %COMPILE %s
// XFAIL: *
// A trailing '$' with nothing following it is an invalid escape sequence
// and should be a compile error per IEC 61131-3.
// Currently the compiler silently keeps the '$' as a literal character.

FUNCTION main : DINT
VAR
    s : WSTRING[10] := "hello$";
END_VAR
    main := 0;
END_FUNCTION
