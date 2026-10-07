FUNCTION main : DINT
VAR
    fb : FbExt;
END_VAR
    fb.value := 42;
    printf('%d$N', fb.value);
    printf('%d$N', fb.getValue());
END_FUNCTION
