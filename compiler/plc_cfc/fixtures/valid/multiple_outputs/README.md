# multiple_outputs

An official IDE export exercising blocks with **several outputs**, only some of them wired —
covering every output shape at once:

- an **FB instance** (`myInstance`) with three outputs: `a`→`localA`, `b` **unconnected**, `c`→`localB`;
- a **function** (`myFunction`) that has *both* a return value and `VAR_OUTPUT` pins, called purely
  for those pins: its **return is unconnected**, `a`→`localA`, `b` **unconnected**.

```text
   +---- myFunctionBlock (myInstance) ----+ (0)
   |                                    a |--->  localA  (1)
   |                                    b |        (unconnected)
   |                                    c |--->  localB  (2)
   +--------------------------------------+

   +-------------- myFunction ------------+ (3)
   |                           myFunction |        (return, unconnected)
   |                                    a |--->  localA  (4)
   |                                    b |        (unconnected)
   +--------------------------------------+

   (unconnected)  an output pin with no outgoing wire
   (0)..(4)       evaluation-priority badges shown by the IDE
```

- `mainProgram.cfc` — the program under test (the FBD network above).
- `myFunctionBlock.st` / `myFunction.st` — the callees.

A wired output is evaluated once into a temp the sink then reads; an unconnected output is
emitted empty (`b => `); and the function's **unconnected return** means the call simply stands
alone (no `temp := …`), because the function is invoked only for its output pins. So the
network means:

```text
myInstance(a => temp_0, b => , c => temp_1);
localA := temp_0;
localB := temp_1;
myFunction(a => temp_2, b => );
localA := temp_2;
```

Note the two `localA` sinks (priorities 1 and 4) write the same variable twice — the diagram
is faithful to the export; ordering is by `priorityInNetwork`.
