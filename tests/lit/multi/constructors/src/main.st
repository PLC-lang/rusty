VAR_GLOBAL
    childInst : child;
END_VAR


FUNCTION main : DINT
    childInst();
    main := childInst.y + childInst.x; // Should be 36 (18 + 18)
    printf('%d$N', main);
END_FUNCTION
