// RUN: not %COMPILE %s
// XFAIL: *
// STRING hex escapes require exactly 2 hex digits ($hh). '$A' provides only
// one hex digit before end-of-string, which is an incomplete escape sequence
// and should be a compile error per IEC 61131-3.
// Currently the compiler silently keeps '$' and 'A' as literal characters.

FUNCTION main : DINT
VAR
    s : STRING[10] := '$A';
END_VAR
    main := 0;
END_FUNCTION
