// Stand-in for an ADD operator block until plc_cfc models those (see ../OPEN.md).
FUNCTION MyAdd : DINT
VAR_INPUT
    in1 : DINT;
    in2 : DINT;
END_VAR
    MyAdd := in1 + in2;
END_FUNCTION
