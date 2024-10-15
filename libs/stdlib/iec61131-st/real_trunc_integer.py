f = open("real_trunc_integer.st", "w")

types_from = [
    "REAL",
    "LREAL",
]

types_to = [
    "SINT",
    "USINT",
    "INT",
    "UINT",
    "DINT",
    "UDINT",
    "LINT",
    "ULINT",
]

template = """(********************
*
* Converts a {0} to {1}
*
*********************)
FUNCTION {0}_TRUNC_{1} : {1}
    VAR_INPUT
        in : {0};
    END_VAR

    {0}_TRUNC_{1} := in;
END_FUNCTION

"""


f.write(template.format("LREAL", "REAL"))

for type_from in types_from:
    for type_to in types_to:
        f.write(template.format(type_from, type_to))