What: a call to another program with an input, an inout, and an output pin;
the inout writes back into the caller's variable. `main` runs two cycles with
`stepValue = 3` and checks state and output (`total = 6, result = 12`).

Illustrated:
```
stepValue -------> step [accumulator] doubled --> result (0, 1)
runningTotal <--> total
```
