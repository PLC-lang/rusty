FUNCTION main : DINT
    VAR
        my_val : DINT := 5;
        conditional : conditional_return;
    END_VAR

    conditional.val := my_val;
    conditional();

    main := conditional.val;
END_FUNCTION
