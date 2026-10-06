What: sources that negate their value with a unary minus. `-t_inc` feeds the
first input of a builtin `DIV` block, `-(t_inc)` feeds a sink directly, and the
literal `-5` does the same. `main` checks all three results (`-10 / 2 = -5`,
`-10`, `-5`).

Illustrated:
```
-t_inc   --> IN1 [DIV] --> quotient (0, 1)
rc       --> IN2
-(t_inc) --> negated (3)
-5       --> literal (4)
```
