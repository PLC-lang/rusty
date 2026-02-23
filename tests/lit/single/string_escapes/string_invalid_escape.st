// RUN: not %COMPILE %s
// XFAIL: *
// '$Q' is not a valid named escape (not $N/$R/$T/$L/$P/$$/$$') and 'Q' is not
// a hex digit, so this should be a compile error per IEC 61131-3.
// Currently the compiler silently keeps '$Q' as the two literal characters.

FUNCTION main : DINT
VAR
    s : STRING[20] := 'test$Qtest';
END_VAR
    main := 0;
END_FUNCTION
