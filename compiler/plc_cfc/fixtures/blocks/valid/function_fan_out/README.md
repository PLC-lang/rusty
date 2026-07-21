What: One `myAdd` call whose return pin fans out to two sinks (`x`, `y`). The
call runs once into a single temporary; both sinks read that temporary, so a
stateless callee is never invoked twice for a fanned-out output.

Illustrated:
```
        myAdd (0)
      +--------------------+
a --> | in1          myAdd | --+--> x (1)
b --> | in2   myAddDoubled |   '--> y (2)
      +--------------------+
      (myAddDoubled unread)
```
