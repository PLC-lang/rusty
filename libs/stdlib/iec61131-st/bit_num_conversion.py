f = open("gen.txt", "w")

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
]

src = "printf('%d$N', TO_{0}({1}#10));       // CHECK: 10\n"

for type_a in types:
    # f.write(template.format(type_a))
    for type_b in types:
        f.write(src.format(type_a, type_b))
    f.write("\n")