// RUN: not %COMPILE %s
// '$Q' is not a valid named escape (not $N/$R/$T/$L/$P/$$/$$') and 'Q' is not
// a hex digit, so this is a compile error per IEC 61131-3 (E124).

FUNCTION main : DINT
VAR
    s : STRING[20] := 'test$Qtest';
END_VAR
    main := 0;
END_FUNCTION
