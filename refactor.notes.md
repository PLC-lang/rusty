- visit_call_statement

```rust
//TODO why do we start a lhs context here???
let ctx = ctx.with_lhs(operator_qualifier.as_str());
```

- fix all TODOs

- test parse priority regarding casts
  (INT#a).b.c vs. INT#(a.b.c)


  (int)a.b.c
  -a.b.c

    MyEnum(x,y,z);

    VAR
        x : INT;
    END_VAR


  MyEnum.x