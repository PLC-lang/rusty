# action_call

A block that targets an **action**. An action is a restricted method: it has no
inputs, outputs, in-outs, or return value — it only runs against the internal
state of the instance it is called on. The block therefore carries a *qualified*
`typeName` (`function_block_0.myAction`) alongside its `instanceName`, and no pins.

```text
                  +-- function_block_0.myAction --+ (0)
   myInstance --->|                               |
                  +-------------------------------+

   function_block_0.myAction   the action, called on instance myInstance
   (0)                         evaluation-priority badge shown by the IDE
```

- `mainProgram.cfc` — the program under test; it owns the instance `myInstance : function_block_0`.
- `function_block_0.st` — the function block whose `myAction` it calls.

Because the type name is qualified, the call targets the action as a member of the
instance — `myInstance.myAction` — rather than the function block itself. With no
pins, the call takes no arguments:

```text
myInstance.myAction();
```
