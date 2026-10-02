What: One `myAdd` call whose return pin fans out to two sinks (`x`, `y`). The
call runs once into a single temporary; both sinks read that temporary, so a
stateless callee is never invoked twice for a fanned-out output.

Illustrated:
```
        myAdd (1)
      +--------------------+
a --> | in1          myAdd | --+--> x (2)
b --> | in2   myAddDoubled |   '--> y (3)
      +--------------------+
      (myAddDoubled unread)
```
