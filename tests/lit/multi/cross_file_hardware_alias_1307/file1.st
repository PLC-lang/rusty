VAR_GLOBAL
    foo AT %IX1.2.3.4 : BOOL;
END_VAR

FUNCTION main : DINT
    // Confirm the alias is wired by writing through `foo` and reading it back.
    // file2.st declares `bar` at the same hardware address, so the two names
    // must share the same backing storage (`__PI_1_2_3_4`).
    foo := TRUE;
    IF foo THEN
        printf('aliased$N');
    END_IF;
    main := 0;
END_FUNCTION
