// Stand-in for a builtin operator block until plc_cfc models those.
// A CFC block calls this like any user function; the body is just the native ST operator.
FUNCTION MyAdd : DINT
VAR_INPUT
    in1 : DINT;
    in2 : DINT;
END_VAR
    MyAdd := in1 + in2;
END_FUNCTION
