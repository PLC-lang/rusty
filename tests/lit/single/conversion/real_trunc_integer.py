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

template = "    printf('%d$N', {0}_TRUNC_{1}(4.9));  // CHECK: 4\n"

f.write("// RUN: (%COMPILE %s && %RUN) | %CHECK %s\n")
f.write("FUNCTION main\n")
f.write(template.format("LREAL", "REAL"))
for type_from in types_from:
    for type_to in types_to:
        f.write(template.format(type_from, type_to))
f.write("END_FUNCTION")