What: `myAdd` with a negated input pin (`in1`) and a negated return pin. The
input value is inverted at the argument (`in1 := NOT a`); the return is captured
raw into its temporary and inverted at the read (`r := NOT __out_myAdd_1`), matching
how output negation is applied for stateful blocks.

Illustrated:
```
        myAdd (0)
      +--------------------+
a --o | in1          myAdd | o--> r (1)
b --> | in2   myAddDoubled |
      +--------------------+
      (o = negated pin; myAddDoubled unread)
```
