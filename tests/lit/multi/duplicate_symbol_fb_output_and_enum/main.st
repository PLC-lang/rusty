PROGRAM mainProg
VAR
    fb : MC_ReadStatus;
END_VAR
    fb();

    printf('%d$N', fb.StatusEnumerator);
    printf('%d$N', fb.Homing);
END_PROGRAM

FUNCTION main
    mainProg();
END_FUNCTION
