PROGRAM mainProg
    VAR
        bar : STRING;
    END_VAR

    bar := 'something';
    bar();

END_PROGRAM

FUNCTION main
    mainProg();
END_FUNCTION
