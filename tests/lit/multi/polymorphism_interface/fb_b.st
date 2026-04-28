FUNCTION_BLOCK FbB IMPLEMENTS IA, IB
    METHOD describeA : DINT
        printf('FbB::describeA$N');
        describeA := 2;
    END_METHOD

    METHOD describeB : DINT
        printf('FbB::describeB$N');
        describeB := 20;
    END_METHOD
END_FUNCTION_BLOCK
