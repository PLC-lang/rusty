FUNCTION main : DINT
    main := g_in_a + g_in_b;
    IF main = 33 THEN
        printf('ok$N');
    END_IF;
END_FUNCTION
