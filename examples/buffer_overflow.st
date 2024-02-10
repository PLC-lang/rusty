FUNCTION main : DINT
    buf_overflow();
END_FUNCTION

FUNCTION buf_overflow : DINT
VAR
    i : DINT;
    buf : ARRAY [0..2] OF BYTE;
END_VAR

FOR i := 0 TO 3 DO
    buf[i] := 1;
END_FOR
END_FUNCTION