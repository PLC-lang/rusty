f = open("to_num.st", "w")

types = [
    "SINT",
    "USINT",
    "INT",
    "UINT",
    "DINT",
    "UDINT",
    "LINT",
    "ULINT",
    "LREAL",
    "REAL",

    # "BIT",
    "BOOL",
    "BYTE",
    "WORD",
    "DWORD",
    "LWORD",

    "STRING",
    "WSTRING",
]

# time, ltime, date, ldate, dt, ldt, tod, ltod

template = """(********************
*
* Converts any other numerical value to {0}
*
*********************)
FUNCTION TO_{0}<T: ANY_NUM> : {0}
    VAR_INPUT
        in : T;
    END_VAR
END_FUNCTION

"""

src = """(********************
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

src_same = """(********************
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

for type_a in types:
    f.write(template.format(type_a))
    for type_b in types:
        # Skip something like "TO_INT__STRING" 
        if type_b == "STRING" or type_b == "WSTRING":
            continue

        if type_a == type_b:
            f.write(src_same.format(type_b, type_a))
        else:
            f.write(src.format(type_b, type_a))