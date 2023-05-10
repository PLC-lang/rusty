# create/open file
f = open("gen.txt", "w")

# list of data types
types = ["LREAL", "REAL", "LINT", "DINT", "INT",
         "SINT", "ULINT", "UDINT", "UINT", "USINT"]

src = """(********************
*
* Converts {0} to {1}
*
*********************)
FUNCTION {0}_TO_{1} : {1}
VAR_INPUT
	in : {0};
END_VAR
	{0}_TO_{1} := in;
END_FUNCTION

"""

# loop over types
for type_a in types:
    for type_b in types:
        if type_a != type_b:
            f.write(src.format(type_a, type_b))
