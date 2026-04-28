FUNCTION_BLOCK FbC IMPLEMENTS IA, IB, IC
    METHOD describeA : DINT
        printf('FbC::describeA$N');
        describeA := 3;
    END_METHOD

    METHOD describeB : DINT
        printf('FbC::describeB$N');
        describeB := 30;
    END_METHOD

    METHOD describeC : DINT
        printf('FbC::describeC$N');
        describeC := 300;
    END_METHOD
END_FUNCTION_BLOCK
