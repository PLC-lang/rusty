FUNCTION main : DINT
VAR
    s : STRING[16];
END_VAR
    s := make_label(0);
    // CHECK: label[0] = zero
    printf('label[0] = %s$N', s);

    s := make_label(7);
    // CHECK: label[7] = item
    printf('label[7] = %s$N', s);
END_FUNCTION
