f = open("real_trunc_int.st", "w")

types_real = ["REAL", "LREAL"]
types_int = ["SINT", "USINT", "INT", "UINT", "DINT", "UDINT", "LINT", "ULINT"]

template = """
(********************
*
* Converts {0} to {1}
*
********************)
FUNCTION {0}_TRUNC_{1} : {1}
    VAR_INPUT
        in : {0};
    END_VAR

    {0}_TRUNC_{1} := in;
END_FUNCTION
"""

for real in types_real:
    for int in types_int:
        f.write(template.format(real, int))
        f.write("\n")