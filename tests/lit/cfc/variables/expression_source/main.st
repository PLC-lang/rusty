FUNCTION main : DINT
    expression_source();
    // CHECK: sum = 4, scaled = -2, ready = 1, idle = 0, differs = 1
    printf('sum = %d, scaled = %d, ready = %d, idle = %d, differs = %d$N', expression_source.sum, expression_source.scaled, expression_source.ready, expression_source.idle, expression_source.differs);

    // `item` reads arr[2] before the last sink overwrites it with `a`.
    // CHECK: item = 30, arr[2] = 3
    printf('item = %d, arr[2] = %d$N', expression_source.item, expression_source.arr[2]);
END_FUNCTION
