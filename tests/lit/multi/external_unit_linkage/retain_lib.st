FUNCTION_BLOCK FbRetain
VAR RETAIN
    value : DINT := 4;
END_VAR
END_FUNCTION_BLOCK

PROGRAM RetainProgram
VAR RETAIN
    count : DINT := 1;
END_VAR
    count := count + 1;
END_PROGRAM

VAR_GLOBAL
    retainFb : FbRetain;
END_VAR
