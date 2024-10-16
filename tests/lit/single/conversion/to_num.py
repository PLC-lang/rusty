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

    "BOOL",
    "BYTE",
    "WORD",
    "DWORD",
    "LWORD",
]

template = "    printf('%d$N', TO_{0}({1}#10));       // CHECK: 10\n"

f.write("// RUN: (%COMPILE %s && %RUN) | %CHECK %s\n")
f.write("FUNCTION main\n")
for type_a in types:
    # Skip something like "STRING_TO_<NUMBER>"
    if type_a == "STRING" or type_a == "WSTRING":
        continue

    for type_b in types:
        f.write(template.format(type_a, type_b))
    f.write("\n")
f.write("END_FUNCTION")