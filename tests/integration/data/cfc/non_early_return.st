
FUNCTION main : DINT
    VAR
        my_x : DINT := 1;
        early : early_return;
    END_VAR

    early.x := my_x;
    early();

    main := early.x;
END_PROGRAM