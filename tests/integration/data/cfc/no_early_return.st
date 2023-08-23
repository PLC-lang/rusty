FUNCTION main : DINT
    VAR
        my_val : DINT := 0;
        early : early_return;
    END_VAR

    early.val := my_val;
    early();

    main := early.val;
END_FUNCTION