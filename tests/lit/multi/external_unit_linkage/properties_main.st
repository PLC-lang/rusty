FUNCTION main : DINT
VAR
    base : FbBase;
    child : FbChild;
    counter : Counter;
    iface : IValue;
END_VAR
    base.value := 42;
    printf('%d$N', base.value);

    child.value := 5;
    printf('%d$N', child.value);

    printf('%d$N', counter.count);

    iface := child;
    printf('%d$N', iface.value);
END_FUNCTION
