f = open("trunc_int.st", "w")

types_real = ["REAL", "LREAL"]
types_int = ["SINT", "USINT", "INT", "UINT", "DINT", "UDINT", "LINT", "ULINT"]

generic = """
(********************
*
* Converts a floating point number to {0}
*
********************)
FUNCTION TRUNC_{0}<T: ANY_REAL> : {0}
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION
"""

generic_impl = """
(********************
*
* Converts {1} to {0}
*
********************)
FUNCTION TRUNC_{0}__{1} : {0}
    VAR_INPUT
        in : {1};
    END_VAR

    TRUNC_{0}__{1} := in;
END_FUNCTION
"""

for int in types_int:
    f.write(generic.format(int))
    f.write("\n")

    for real in types_real:
        f.write(generic_impl.format(int, real))
        f.write("\n")