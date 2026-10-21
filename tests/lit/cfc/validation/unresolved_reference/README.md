What: unresolved references in a CFC diagram are reported at the diagram
element. The source `asd` feeds the sink `foo`, and neither is
declared, so the validation reports E048 twice, both at the sink block.
The sources `.global` and `[element]` nest an undeclared name in a global
access and in an array literal; each is reported at the sink it feeds.
The sources `(.parenGlobal)` and `([parenElement])` wrap the same forms in
parentheses.

Illustrated:
```
asd              --> foo (1)
.global          --> x (2)
[element]        --> values (3)
(.parenGlobal)   --> x (4)
([parenElement]) --> values (5)
```
