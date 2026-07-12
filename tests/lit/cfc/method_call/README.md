# method_call

A block calling a **method** on a function block instance, with an input pin
and a consumed return pin. Like an action, the block's `typeName` carries an
`<fb>.<method>` qualifier and the call is lowered to `<instance>.<method>`;
unlike an action, a method has pins — and its return pin is named after the
*method* (`getValue`), not the dotted `typeName` (`myFb.getValue`), so the
return-pin detection and the temporary's name must use the callable's own name.

> **Anticipated shape:** the IDE does not export method blocks yet. This
> fixture models the expected export by analogy with `action_call` (dotted
> `typeName` + `instanceName`) and `function_call` (pins, return pin named
> after the callable). Cross-check against a real export once the IDE
> supports it.

```text
                     +-- myFb.getValue --+ (1)
   localOffset ----->| offset   getValue |--->  localResult  (2)
                     +-------------------+

   myFb.getValue  the method, called on instance myInstance
   (1),(2)        evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `myFb.st` — the function block declaring `METHOD getValue : DINT`.

The network means:

```text
__getValue_res_0 := myInstance.getValue(offset := localOffset);
localResult := __getValue_res_0;
```
