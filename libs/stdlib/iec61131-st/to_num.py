f = open("to_num.st", "w")

real_types = ["REAL", "LREAL"]
bit_types = ["BOOL", "BYTE", "WORD", "DWORD", "LWORD"]

types = [
    "SINT",
    "USINT",
    "INT",
    "UINT",
    "DINT",
    "UDINT",
    "LINT",
    "ULINT",

    "REAL",
    "LREAL",

    # "BIT",
    # "BOOL",
    # "BYTE",
    # "WORD",
    # "DWORD",
    # "LWORD",

    # "STRING",
    # "WSTRING",

    # "TIME",
    # "LTIME",
    # "DATE",
    # "LDATE",
    # "DT",
    # "LDT",
    # "TOD",
    # "LTOD",
]

generic = """(********************
*
* Converts any other numerical value to {0}
*
*********************)
FUNCTION TO_{0}<T: {1}> : {0}
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION
"""

generic_impl = """(********************
*
* Converts {0} to {1}
*
*********************)
FUNCTION TO_{1}__{0} : {1}
    VAR_INPUT
        in : {0};
    END_VAR

    TO_{1}__{0} := {0}_TO_{1}(in);
END_FUNCTION
"""

generic_impl_same = """(********************
*
* Converts {0} to {1}
*
*********************)
FUNCTION TO_{1}__{0} : {1}
    VAR_INPUT
        in : {0};
    END_VAR

    TO_{1}__{0} := in;
END_FUNCTION
"""

# a real target also takes BOOL and the bit string types, so its constraint is the wider ANY
for type_from in types:
    f.write(generic.format(type_from, "ANY" if type_from in real_types else "ANY_NUM"))
    f.write("\n")

    for type_to in types:
        if type_from == type_to:
            f.write(generic_impl_same.format(type_from, type_to))
        else:
            f.write(generic_impl.format(type_from, type_to))

        f.write("\n")

# BOOL and the bit string types widen to a real by value
for type_from in bit_types:
    for type_to in real_types:
        f.write(generic_impl.format(type_from, type_to))
        f.write("\n")
