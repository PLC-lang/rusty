What: `myAdd` called with only its return pin consumed (`sum`); the `myAddDoubled`
output pin is present but unwired. Only consumed outputs get a temporary, so the
call captures just the return and passes the dangling output as an empty argument
(`myAddDoubled => `) to satisfy the full parameter list.

Illustrated:
```
        myAdd (0)
      +--------------------+
a --> | in1          myAdd | --> sum (1)
b --> | in2   myAddDoubled | --> (unread)
      +--------------------+
```
