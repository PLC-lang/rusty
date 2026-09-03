# Known Bugs

Bugs found while researching the compiler for this documentation. These bugs were worked around during the research, not fixed. Every entry in the first section was reproduced against the compiler; the "Needs human input" section holds the entries that did not reproduce, or where the expected behavior is a judgment call.

Each entry uses this format:

```
<severity, P0 to P4> <file>:<line>:<column>: <title>
<description: what was observed, what was expected, and the cause>
```
// Reproducible example:
<minimal ST program that shows the problem, with comments for observed and expected behavior>
```
```

Severity: P0 wrong code or data loss, P1 crash or compile abort on valid input, P2 wrong diagnostic or missing diagnostic, P3 confusing behavior or inconsistent CLI, P4 cosmetic.

Entries are ordered by severity, P0 first. Add a new entry at the end of its severity group, so the file stays ordered. Do not append new entries at the end of a section. Separate entries with one blank line.

---

P0 src/validation/variable.rs:390:1: User names with the reserved double underscore prefix are accepted without a diagnostic
Placed at the top of the file on the maintainer's request (Volkan, 2026-09-11), so that it is not forgotten; by the severity scale it would be a P2. Every name the compiler generates starts with `__` (`__PI_0_0`, `__describe_return`, `__main_ptr__ctor`, `__vtable_Buffer`, ...), and the prefix is the only thing that keeps generated names apart from user names, but no validation checks user identifiers for it. A program that declares `__Buffer`, `__limit`, `__Speed`, `__global`, `__main`, and `__foo` passes `plc --check` and codegen without a single diagnostic. When a user name collides with a generated one, the failure is indirect and confusing: a user type `__describe_return` next to `FUNCTION describe : STRING[80]` is reported as E004 "Ambiguous datatype" at both locations, and a user global `__PI_0_0` next to `sensor AT %IX0.0 : BOOL` produces E037 "cannot assign 'DINT' to ': BOOL'" with no source location at all. Expected: at least a warning for every user-declared identifier (variable, type, POU, enum variant) that starts with `__`, so that the reserved prefix is visible to the user before a collision happens.
```
// Reproducible example:
TYPE __describe_return : DINT; END_TYPE

FUNCTION describe : STRING[80]
    describe := 'x';
END_FUNCTION

VAR_GLOBAL
    __PI_0_0 : DINT;
    sensor AT %IX0.0 : BOOL;
END_VAR

PROGRAM __main
    VAR
        __foo : DINT;   // accepted silently; expected a warning about the reserved prefix
    END_VAR

    __foo := __PI_0_0;
    // observed: E004 "__describe_return: Ambiguous datatype" and an E037 without location
    // expected: a diagnostic on each `__` name that names the reserved prefix
END_PROGRAM
```

P0 compiler/plc_lowering/src/reference_to_return.rs:475:5: Call to a REFERENCE TO function in an IF or WHILE condition is moved into the body
`IF pick(v) > 2 THEN r := 1; END_IF`, where `pick : REFERENCE TO INT`, lowers to a condition that reads `__pick_return_val_1` while the `REF=` setup and the call `pick(__pick_return_val_1, v)` are placed in front of the first body statement (visible with `--ast-lowered`). The condition dereferences the null-initialized temporary; a binary built with `-O none` segfaults, optimized builds fold the undefined load and print `0`. Cause: `visit_statement_list` wraps the statement it visits with the pre-statements queued so far, and the statements queued while walking the enclosing `IF`'s condition are still pending when the first body statement is visited. A `WHILE` condition shows the same inside the desugared guard `IF`. Expected: the generated statements in front of the enclosing `IF` or `WHILE`, as the aggregate-return lowerer does.
```
// Reproducible example:
FUNCTION pick : REFERENCE TO INT
VAR_IN_OUT
    v : INT;
END_VAR
    pick REF= v;
END_FUNCTION

FUNCTION main : DINT
VAR
    v : INT := 5;
    r : DINT := 0;
END_VAR
    IF pick(v) > 2 THEN
        r := 1;
    END_IF
    printf('%d$N', r);
    // -O none: segfaults at runtime; default level: prints 0, expected 1
END_FUNCTION
```

P0 compiler/plc_lowering/src/reference_to_return.rs:377:21: Return-value parameter is not the first parameter when the input block is not first
For `FUNCTION pick : REFERENCE TO INT VAR_IN_OUT a : INT; END_VAR VAR_INPUT b : INT; END_VAR`, the generated `__pick_return_val` is inserted at position 0 of the `VAR_INPUT` block, which is the second block, or appended as a new block when no `VAR_INPUT` exists (line 361), so the function's parameter list is `(a, __pick_return_val, b)`. The rewritten call passes the temporary first: `pick(__pick_return_val_1, v, 10)`. Both `a` and the temporary are pointers, so no diagnostic is reported; `v` receives the address of the temporary and `b` receives `v`, and the program prints `v=10 r=10` instead of `v=13 r=13`. Expected: the parameter position matches the argument position, for example by inserting the return variable into the first parameter block.
```
// Reproducible example:
FUNCTION pick : REFERENCE TO INT
VAR_IN_OUT
    a : INT;
END_VAR
VAR_INPUT
    b : INT;
END_VAR
    a := a + b;
    pick REF= a;
END_FUNCTION

FUNCTION main : DINT
VAR
    v : INT := 3;
    r : INT;
END_VAR
    r := pick(v, 10);
    printf('v=%d r=%d$N', v, r);
    // prints v=10 r=10, expected v=13 r=13
END_FUNCTION
```

P0 src/codegen/generators/expression_generator.rs:1698:9: A fixed array passed to a function block's VLA parameter is not wrapped
For `FUNCTION_BLOCK fb VAR_IN_OUT io : ARRAY[*] OF DINT; END_VAR io[0] := 1; END_FUNCTION_BLOCK` called as `inst(io := arr)` or `inst(arr)` with `arr : ARRAY[0..2] OF DINT`, the caller stores the raw array address into the `io` member (`store ptr %arr, ptr %1`), while the body reads that member as a pointer to the VLA struct `%__fb_io = type { ptr, [2 x i32] }` and dereferences the first `ptr` field. The array's first two elements are read as a data pointer and the next two as bounds, so `io[0] := 1` writes through a garbage address and the program segfaults. Calls to functions and methods with the same parameter wrap the array into a stack-allocated VLA struct first (`%vla_struct`, the `is_vla` branch in `generate_argument_by_ref`, see the `pass` test in `src/tests/adr/vla_adr.rs`); the stateful call path (`generate_stateful_pou_arguments` to `generate_call_struct_argument_assignment`) stores the argument without the wrap. The validator accepts `VAR_IN_OUT` VLAs in function blocks (`validate_vla` allows `(FunctionBlock, InOut)`, E044 is not raised), so this is valid input. Expected: the same wrap as for functions, or E044 for function blocks.
```
// Reproducible example:
FUNCTION_BLOCK fb
VAR_IN_OUT
    io : ARRAY[*] OF DINT;
END_VAR
    io[0] := 1;
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    inst : fb;
    arr : ARRAY[0..2] OF DINT;
END_VAR
    inst(io := arr);
    printf('%d$N', arr[0]);
    // segfaults at runtime, expected 1
    // plc --ir: main stores `store ptr %arr, ptr %1`, no %vla_struct wrap
END_FUNCTION
```

P0 compiler/plc_lowering/src/loops.rs:358:17: FOR loop whose end bound is the maximum of the counter type never terminates
`FOR u := 253 TO 255 DO n := n + 1; END_FOR` with `u : USINT`, and likewise `FOR s := 125 TO 127 DO` with `s : SINT` or `FOR s := -126 TO -128 BY -1 DO`, loop forever; a guard `IF n > 10 THEN EXIT; END_IF` in the body shows that the body runs more than three times (the program prints `11`), and without the guard the program hangs. The desugarer increments the counter at the top of every iteration after the first (line 317) and then exits on `counter > end` (line 358) or `counter < end` for the decrementing direction. When the counter holds the end value, the increment wraps to the minimum of the type, `255 + 1` becomes `0`, so the exit comparison is never true. A loop over the full range of a small integer type, `FOR i := 0 TO 255` over a `USINT` table index, is common in PLC code. Expected: the loop runs three times and stops, for example by comparing the counter against the end before adding the step.
```
// Reproducible example:
FUNCTION main : DINT
VAR
    u : USINT;
    n : DINT := 0;
END_VAR
    FOR u := 253 TO 255 DO
        n := n + 1;
        IF n > 10 THEN
            EXIT;
        END_IF
    END_FOR
    printf('%d$N', n);
    // prints 11, expected 3; without the guard the program hangs
END_FUNCTION
```

P0 src/validation/statement.rs:1258:8: Shorter string passed to a by-reference STRING parameter is overwritten past its end
`FUNCTION grow : DINT VAR_IN_OUT s : STRING; END_VAR s := 'abcdefghij'; END_FUNCTION` called as `grow(io)` with `io : STRING[4]` passes `--check` without a diagnostic and writes the ten characters plus terminator (`memcpy ... i32 11`) into the five-byte variable, so the neighbouring locals in `main` are overwritten; depending on the stack layout and optimization level the program prints the overflowed string, shows a corrupted neighbour, or dies with a bus error (observed at `-O none`). `VAR_INPUT {ref} s : STRING` and a `VAR_IN_OUT` of a function block behave the same; a `VAR_OUTPUT` string is copied out bounded and is fine. The callee stores with its declared type, so an assignment inside `grow` is bounded by the capacity of `STRING`, 80, whatever the caller's size. The by-reference argument check that rejects `ARRAY[0..2] OF DINT` for an `ARRAY[0..9] OF DINT` parameter returns early for every aggregate type (line 1258) and leaves strings to the general assignment rule, which accepts any two strings of the same encoding because a by-value copy stops at the smaller capacity. Expected: E037 for a by-reference string argument whose size differs from the parameter's size, as for arrays.
```
// Reproducible example:
FUNCTION grow : DINT
VAR_IN_OUT
    s : STRING;
END_VAR
    s := 'abcdefghij';
END_FUNCTION

FUNCTION main : DINT
VAR
    io : STRING[4];
    str : STRING[3];
END_VAR
    str := 'xyz';
    grow(io);
    printf('%s|%s$N', REF(io), REF(str));
    // plc --check: no diagnostic, expected E037
    // -O none: bus error at runtime; default level: prints abcdefghij|xyz (11 bytes written into a 5 byte variable)
END_FUNCTION
```

P0 src/codegen/generators/statement_generator.rs:572:17: CASE range on an unsigned selector is compared as signed
`CASE b OF 100..200: ... ELSE ... END_CASE` with `b : BYTE := 150` takes the `ELSE` branch; the same with `u : USINT := 150`, and `CASE w OF 30000..50000` with `w : WORD := 40000`, also take `ELSE`, while a `DINT` selector matches. The IR shows `icmp sge i8 %load_b, 100` and `icmp sle i8 %load_b, -56` (and `icmp sle i16 %load_w, -15536` for the `WORD`): the range bounds are generated in the selector's type and compared with signed predicates, so `150` is `-106` and `200` is `-56`. `generate_case_range_condition` calls `create_llvm_int_binary_expression` with `None` for the signedness (lines 572 and 588), and the helper emits signed predicates (it does so for every ordering compare, see the separate entry on unsigned comparisons). Single case values are matched by a `switch` on the bit pattern and are correct, as are `>=` and `<=` written in an `IF` on operands promoted to `DINT`. Expected: `range ok`; the compare must use unsigned predicates for unsigned selectors.
```
// Reproducible example:
FUNCTION main : DINT
VAR
    b : BYTE := 150;
    w : WORD := 40000;
    d : DINT := 150;
END_VAR
    CASE b OF
        100..200: printf('b range ok$N');
        ELSE printf('b else$N');
    END_CASE
    CASE w OF
        30000..50000: printf('w range ok$N');
        ELSE printf('w else$N');
    END_CASE
    CASE d OF
        100..200: printf('d range ok$N');
        ELSE printf('d else$N');
    END_CASE
    // prints "b else", "w else", "d range ok"; expected "b range ok", "w range ok", "d range ok"
END_FUNCTION
```

P0 src/codegen/generators/expression_generator.rs:2351:17: Ordering comparison of unsigned integers is compiled as a signed compare
`ud > ud2` with `ud : UDINT := 4000000000` and `ud2 : UDINT := 5` is `FALSE`, and so are `l > l2` with `l : ULINT := 18446744073709551615` and `l2 : ULINT := 5`, `ud > 2147483647`, and `l > 1`; `ud > 3000000000` is `TRUE` only because the 64-bit literal widens both sides to `LINT`. The IR reads `icmp sgt i32 %load_ud, %load_ud2` and `icmp sgt i64 %load_l, %load_l2`. `generate_binary_expression` computes whether the operands are signed (line 393) and passes it to `create_llvm_int_binary_expression`, but the helper uses the signed predicates `SLT`, `SGT`, `SLE`, and `SGE` unconditionally (lines 2351 to 2363) and only consults the flag for division and modulo. `USINT`, `BYTE`, `UINT`, and `WORD` operands are not affected because the resolver promotes them to `DINT` first; every comparison between `UDINT`, `DWORD`, `ULINT`, or `LWORD` values above the signed maximum of their width is wrong. Expected: `ULT`, `UGT`, `ULE`, and `UGE` when neither operand is signed.
```
// Reproducible example:
FUNCTION main : DINT
VAR
    ud : UDINT := 4000000000;
    ud2 : UDINT := 5;
    l : ULINT := 18446744073709551615;
    l2 : ULINT := 5;
END_VAR
    printf('%d %d %d %d %d$N', ud > ud2, l > l2, ud > 2147483647, l > 1, ud > 3000000000);
    // prints 0 0 0 0 1, expected 1 1 1 1 1
END_FUNCTION
```

P0 src/codegen/generators/expression_generator.rs:393:29: Division or modulo of an unsigned value by a literal is signed
`ud / 2` with `ud : UDINT := 4000000000` yields `4147483648` instead of `2000000000`; the IR reads `sdiv i32 %load_ud, 2`, while `ud / ud2` with an `UDINT` divisor yields `udiv` and the correct result. The literal `2` is a `DINT` value, and `generate_binary_expression` treats the operation as signed as soon as either operand type is signed (line 393), so the unsigned dividend is reinterpreted as `-294967296`. `ULINT` is not affected: `l / 2` for the maximum `ULINT` emits `udiv i64` and yields the correct `9223372036854775807`, because the literal takes the 64-bit unsigned type as hint, while for `UDINT` the 32-bit `DINT` literal type wins. `MOD` uses the same flag: `ud MOD 3` yields `0` instead of `1` and additionally reports the warning E067 `Implicit downcast from 'UDINT' to 'DINT'`, which `/` does not report. Expected: a non-negative literal takes the signedness of the unsigned operand, or the operation is performed in a type wide enough for both, as the promotion rules for the smaller unsigned types already do.
```
// Reproducible example:
FUNCTION main : DINT
VAR
    ud : UDINT := 4000000000;
    ud2 : UDINT := 2;
END_VAR
    printf('%u %u %u$N', ud / 2, ud / ud2, ud MOD 3);
    // prints 4147483648 2000000000 0, expected 2000000000 2000000000 1
    // plc --ir: `sdiv i32 %load_ud, 2` for the literal divisor, `udiv` for ud2
END_FUNCTION
```

P0 src/validation/statement.rs:279:15: TIME and DATE literals between 2^31 and 2^32 units wrap silently to negative values
`t := T#25d` stores `-2134967296` and `T#24d20h31m23s648ms` stores `-2147483648`, one past `T#24d20h31m23s647ms` which stores `2147483647`; `d := D#2050-01-01` stores `-1770359296`. No diagnostic is reported by `--check` or the build, and the values misbehave at run time: `T#25d > T#1s` is `FALSE`, `T#20d + T#10d` is `-1702967296`, and `D#2050-01-01 > D#2020-01-01` is `FALSE`. Codegen represents `TIME` and `DATE` as `i32` (`%t = alloca i32`, milliseconds and seconds), but the literal bound check in `validate_date_literal_bounds` accepts every value up to `u32::MAX` and reports E148 only above that (line 279), or for negative values, so the range between `2^31` and `2^32` is accepted and then truncated by the `i32` store. Expected: E148 for every literal that the 32-bit signed representation cannot hold, or a representation that matches the check.
```
// Reproducible example:
FUNCTION main : DINT
VAR
    t : TIME := T#25d;
    t2 : TIME := T#24d20h31m23s648ms;
    t3 : TIME := T#24d20h31m23s647ms;
    d : DATE := D#2050-01-01;
    d2 : DATE := D#2020-01-01;
    s : TIME;
END_VAR
    s := T#20d + T#10d;
    printf('%d %d %d %d %d$N', t, t2, t3, d, s);
    // prints -2134967296 -2147483648 2147483647 -1770359296 -1702967296
    printf('%d %d$N', t > T#1s, d > d2);
    // prints 0 0, expected 1 1 (or E148 at compile time)
END_FUNCTION
```

P0 compiler/plc_lowering/src/array_lowering.rs:458:1: Array literal of function block instances clears the method table pointer of the listed elements
`fbs : ARRAY[0..2] OF Acc := [(total := 1)]`, where `Acc` has a method `get`, passes `--check`, but `viaBase(ADR(fbs[0]))` with `FUNCTION viaBase : DINT VAR_INPUT p : POINTER TO Acc; END_VAR viaBase := p^.get();` segfaults; the same call on the unlisted element `fbs[2]`, on an array without a literal, or on a single instance `f : Acc := (total := 1)` works, and the direct call `fbs[0].get()` works because it does not go through the table. The full literal `[(total := 1), (total := 2), (total := 3)]` crashes for every element, in a function, a program, and for a global. The IR of `main` shows `call @__main_fbs__ctor(ptr %fbs)`, which sets the `__vtable` pointer of every element, followed by `memcpy(%fbs[0], @.const_init, sizeof(%Acc))` with `@.const_init = constant %Acc { ptr null, i32 1 }`. `lower_array_elements` turns the literal into one assignment per listed element, `fbs[0] := (total := 1)`, and places them after the constructor call; codegen materializes the struct literal as a complete constant whose unmentioned `__vtable` member takes its default `null` (`src/codegen/generators/expression_generator.rs:2705`), and the copy overwrites the table pointer. An array of structs that contain a function block, `ha : ARRAY[0..1] OF H := [(n := 1), (n := 2)]` with `H` holding an `Acc`, crashes the same way, while the single struct `h : H := (n := 5)` is correct because its constant is copied before `H__ctor` runs. Expected: the element assignment sets only the listed members, or the table pointer is written after the literal is copied.
```
// Reproducible example:
FUNCTION_BLOCK Acc
VAR_INPUT
    total : DINT;
END_VAR
    METHOD get : DINT
        get := total;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION viaBase : DINT
VAR_INPUT
    p : POINTER TO Acc;
END_VAR
    viaBase := p^.get();
END_FUNCTION

FUNCTION main : DINT
VAR
    fbs : ARRAY[0..2] OF Acc := [(total := 1)];
    f : Acc := (total := 1);
END_VAR
    printf('%d$N', fbs[0].get());
    // prints 1
    printf('%d$N', viaBase(ADR(f)));
    // prints 1
    printf('%d$N', viaBase(ADR(fbs[2])));
    // prints 0
    printf('%d$N', viaBase(ADR(fbs[0])));
    // segfaults, expected 1
END_FUNCTION
```

P0 src/codegen/generators/statement_generator.rs:567:32: CASE selector with side effects is evaluated once per range bound
`CASE next() OF 1..2: ... 3..4: ... 5: ... ELSE ... END_CASE`, where `next` increments a global counter and returns it, calls `next` five times and ends in the `ELSE` branch although the first value is `1`. The `switch` on the selector is generated once, but the range branches are generated as compare chains and `generate_case_range_condition` regenerates the selector expression for the lower bound (line 567) and again for the upper bound (line 587), so every range costs two more evaluations and each compare sees a different value. Without ranges the selector is evaluated once. Expected: the selector is evaluated once and its value reused for every range test.
```
// Reproducible example:
VAR_GLOBAL
    counter : DINT := 0;
END_VAR

FUNCTION next : DINT
    counter := counter + 1;
    next := counter;
END_FUNCTION

FUNCTION main : DINT
    CASE next() OF
        1..2: printf('one-two$N');
        3..4: printf('three-four$N');
        5: printf('five$N');
        ELSE printf('else$N');
    END_CASE
    printf('counter=%d$N', counter);
    // prints "else" and counter=5, expected "one-two" and counter=1
END_FUNCTION
```

P0 src/codegen/generators/expression_generator.rs:369:9: Reading a REAL constant initialized from a negative integer constant yields the value interpreted as unsigned
A `VAR_GLOBAL CONSTANT R : REAL := A;` where `A : DINT := -7` is another constant (also `R : REAL := A / B`, `R : REAL := A + B`, `R : LREAL := A`, or from an INT constant) is defined correctly in the IR as `float -7.0`, but every read of `R` in a POU body is replaced by constant propagation with the folded integer literal of the initializer and then converted to float as an unsigned value: `REAL := A` prints 4294967296.0 (2^32 - 7 rounded to float), `LREAL := A / B` prints 4294967293.0, and `REAL := I1` with `I1 : INT := -7` prints 65529.0 (2^16 - 7). Expected -7.0, -3.0 and -7.0. A literal initializer (`R : REAL := -7`) and a non-CONSTANT global with the same initializer are correct because they are not propagated. The cause is in `generate_expression_value`: for a constant variable it calls `generate_constant_expression`, which fetches the resolved (folded) constant statement of the REAL variable, an integer literal node created by the const evaluator, and generates it with `generate_expression_value(const_expression)` (line 369); the caller then casts that i32/i16 constant to the REAL target via `cast_if_needed`, and because the folded literal node has no signed integer annotation, `cast_constant` in `src/codegen/llvm_typesystem.rs:272` takes the zero-extended (unsigned) path. Reproduce with the standard link command at any optimization level.
```
// Reproducible example:
VAR_GLOBAL CONSTANT
    A : DINT := -7;
    B : DINT := 2;
    I1 : INT := -7;
    R3 : REAL := A;
    R4 : LREAL := A / B;
    R8 : REAL := I1;
END_VAR

FUNCTION main : DINT
    printf('%f %f %f$N', REAL_TO_LREAL(R3), R4, REAL_TO_LREAL(R8));
    main := 0;
END_FUNCTION
// observed: 4294967296.000000 4294967293.000000 65529.000000
// expected: -7.000000 -3.000000 -7.000000
```

P0 compiler/plc_lowering/src/initializer.rs:387:9: Enum members of function blocks and stack structs are initialized to 0 instead of the first enumerator
For `TYPE Color : (Red := 1, Green := 5, Blue := 10); END_TYPE` (no explicit type default), a variable of type Color that has no initializer gets the first enumerator (Red = 1) when it is a global, a global struct member, a PROGRAM member or a plain function local, but it gets 0 (a value that is not a member of the enum) when it is a member of a function block instance, a struct nested in a function block, or a struct allocated in a function's VAR block. Expected: the same value (1) everywhere. The static initializer path in codegen (`llvm_index::find_associated_initial_value`, fed by `data_type_generator`) uses the first enumerator when the enum type has no explicit default, but the lowered constructor path does not: `visit_user_type_declaration` only emits `self := <default>` in the enum's `__ctor` when `user_type.initializer` is `Some` (line 387), so `Color__ctor` is empty, and stack instances are `memset` to 0 before `Fb__ctor` / `S__ctor` call that empty constructor. Reproduce with the standard link command at any optimization level; the IR shows `@g = global i32 1` next to an empty `define void @Color__ctor`.
```
// Reproducible example:
TYPE Color : (Red := 1, Green := 5, Blue := 10); END_TYPE
TYPE S : STRUCT
    c : Color;
END_STRUCT END_TYPE

VAR_GLOBAL
    g : Color;
END_VAR

FUNCTION_BLOCK Fb
VAR
    col : Color;
    s : S;
END_VAR
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    fb : Fb;
    loc : Color;
    ls : S;
END_VAR
    printf('global=%d fb=%d fbstruct=%d local=%d localstruct=%d$N', g, fb.col, fb.s.c, loc, ls.c);
    main := 0;
END_FUNCTION
// observed: global=1 fb=0 fbstruct=0 local=1 localstruct=0
// expected: global=1 fb=1 fbstruct=1 local=1 localstruct=1
```

P0 src/typesystem.rs:1576:16: Power operator on two DINT operands is computed in single-precision REAL
`d := b7 ** e11` with `b7 : DINT := 7`, `e11 : DINT := 11` and `d : DINT` prints `1977326720` instead of `1977326743`, and `d := big ** 1` with `big : DINT := 123456789` prints `123456792`; no diagnostic is reported. The parser rewrites `a ** b` into a call to the generic stdlib function `EXPT<T: ANY_REAL, U: ANY_NUM> : T`. The generic binder seeds `T` with `REAL` and folds the concrete argument type into it with `get_bigger_type`; that rule promotes a mixed integer and real pair to `LREAL` only when one side is wider than 32 bits, so `DINT` and `REAL` yield `REAL`, and the IR calls `EXPT__REAL__DINT(float, i32)` and converts the result back with `fptosi`. A `REAL` has a 24-bit mantissa, so every integer result above 16777216 is rounded. `LINT` operands take the `LREAL` path and are exact. Expected: an integer base selects `LREAL` when the integer type has 32 bits or more (a `REAL` cannot represent every `DINT`), or `**` on integer operands is rejected with a diagnostic, as IEC 61131-3 defines the base of `**` as `ANY_REAL`. Reproduce with the default command line at any optimization level.
```
// Reproducible example:
FUNCTION main : DINT
VAR
    d : DINT;
    b7 : DINT := 7;
    e11 : DINT := 11;
    big : DINT := 123456789;
END_VAR
    d := b7 ** e11;
    printf('%d$N', d); // prints 1977326720, expected 1977326743
    d := big ** 1;
    printf('%d$N', d); // prints 123456792, expected 123456789
    // plc --ir: %call = call float @EXPT__REAL__DINT(float %0, i32 %load_e11)
END_FUNCTION
```

P0 src/codegen/generators/expression_generator.rs:3042:18: TIME literal multiplied or divided by a number is emitted in nanoseconds instead of milliseconds
`t := T#1s * 2` stores `2000000000`, `t := T#1s * n` (n : INT := 2) stores `2000000000`, `t := T#1s / 2` stores `500000000` and `t := T#3s / 2` stores `3647483648` (the i32 wrap of 1.5e9 + garbage), where 2000, 2000, 500 and 1500 are expected. `t + T#1s`, `t * 3` and `T#1s + T#500ms` are correct, so only a temporal literal that is a direct operand of `*` or `/` with a numeric operand is affected. The compiler stores short `TIME` and `TOD` as 32-bit milliseconds and scales the parsed nanosecond literal in `create_temporal_const_int` according to the literal's type hint; in `T#1s * 2` the resolver gives the literal the numeric hint of the arithmetic result (a DINT), so the match falls into the `_ => value_in_nanos` arm at line 3042 and the raw nanosecond count is emitted and truncated to i32. Every TIME-scaling pattern common in PLC code (`T#100ms * count`, `cycle / 2`) silently produces a value 1000000 times too large modulo 2^32. Reproduce with the default command line at any optimization level; `--ir` shows `mul i32 1000000000, %load_n` for `T#1s * n`.
```
// Reproducible example:
FUNCTION main : DINT
VAR
    t : TIME;
    n : INT := 2;
END_VAR
    t := T#1s * 2;
    printf('%u$N', t);      // prints 2000000000, expected 2000
    t := T#1s * n;
    printf('%u$N', t);      // prints 2000000000, expected 2000
    t := T#1s / 2;
    printf('%u$N', t);      // prints 500000000, expected 500
    t := T#3s / 2;
    printf('%u$N', t);      // prints 3647483648, expected 1500
    printf('%u$N', TOD#01:00:00 * 2); // prints 1634811904, expected 7200000
    main := 0;
END_FUNCTION
```

P0 src/resolver.rs:2888:49: Arithmetic between two real literals assigned to an LREAL is computed in REAL precision
`lr := 1.0 / 3.0` stores `0.33333334326744080` (the f32 quotient widened to double, IR constant `0x3FD5555560000000`), `lr := 100000000.0 + 1.0` stores `100000000.0`, `lr := 22.0 / 7.0` stores `3.14285707473754883` and `lr := 1.0E10 * 1.0E10 * 1.0E10 * 1.0E10` stores `inf`, where `0.33333333333333331`, `100000001.0`, `3.14285714285714279` and `1e40` are expected because the target is LREAL. `lr := 0.1` alone and `lr := x / 3.0` with x : LREAL are correct. The resolver types every real literal that fits in f32 as REAL through `get_real_type_name_for` (resolver.rs:3048), and the LREAL context override on the line above it only consults `ctx.lhs`, which is set for a direct assignment but not for the operands of a binary expression; the expression therefore gets type REAL, codegen emits the operation on `float` and widens the rounded result with `fpext`. Real PLC code such as scaling constants (`lr := 4.0 * 3.14159265358979 / 180.0`) silently loses precision or overflows. Reproduce with the default command line at any optimization level.
```
// Reproducible example:
FUNCTION main : DINT
VAR
    lr : LREAL;
    x : LREAL := 100000000.0 + 1.0;
END_VAR
    lr := 1.0 / 3.0;
    printf('%.17f$N', lr);   // prints 0.33333334326744080, expected 0.33333333333333331
    lr := 100000000.0 + 1.0;
    printf('%f %f$N', lr, x); // prints 100000000.000000 100000000.000000, expected 100000001.000000 twice
    lr := 1.0E10 * 1.0E10 * 1.0E10 * 1.0E10;
    printf('%g$N', lr);      // prints inf, expected 1e+40
    main := 0;
END_FUNCTION
```

P0 src/codegen/generators/llvm.rs:459:9: WSTRING initializer longer than the declared size is emitted untruncated and overflows the variable
`gw : WSTRING[4] := "wideglobal";` produces `@gw = global [5 x i16] [i16 119, ... 10 elements]`, a ten-element constant for a five-element global, and the program prints `LEN(gw) = 12`, the whole literal followed by the neighbouring global's bytes; a local `lw : WSTRING[4] := "widelocal"` gets a nine-element `@__main.lw__init` constant and prints `LEN(lw) = 6` with no terminator inside the variable. The equivalent `gs : STRING[4] := 'narrowglobal'` is truncated to `narr` as IEC 61131-3 requires. `create_const_utf16_string` collects the full `encode_utf16()` output and only pads with zeros up to `len`; it never cuts the vector to `len - 1` characters, unlike `create_const_utf8_string` at line 442, which slices `value.as_bytes()[..min(value.len(), len - 1)]`. The caller in `generate_string_literal_for_type` computes the bounded `str_len` correctly, so only the helper is wrong. Expected: the constant has exactly `len` elements, the first `len - 1` characters of the literal plus the terminator, and `LEN(gw)` prints 4. Reproduce with the default command line at any optimization level, or with `plc --ir` which fails with `constant expression type mismatch: got type '[10 x i16]' but expected '[5 x i16]'`.
```
// Reproducible example:
// observed: 12 4 6 7   (LEN(gw), LEN(gs), LEN(lw), guard)
// expected: 4 4 4 7
VAR_GLOBAL
    gw : WSTRING[4] := "wideglobal";
    gs : STRING[4] := 'narrowglobal';
END_VAR
FUNCTION main : DINT
VAR
    lw : WSTRING[4] := "widelocal";
    guard : DINT := 7;
END_VAR
    printf('%d %d %d %d$N', LEN(gw), LEN(gs), LEN(lw), guard);
    main := 0;
END_FUNCTION
```

P0 src/resolver/const_evaluator.rs:723:9: Sized string initialized from a longer STRING constant receives the constant's full-size value
With `VAR_GLOBAL CONSTANT CSTR : STRING := 'constant_text'; END_VAR`, a local `c3 : STRING[3] := CSTR;` gets the initializer constant `@__main.c3__init = unnamed_addr constant [4 x i8] c"constant_text\00...\00"`, an 81-byte value in a 4-byte constant, and prints `cons` with `LEN(c3) = 4` (no terminator inside the variable; the optimized build prints `cons` followed by garbage and `LEN = 8`). A global `g3 : STRING[3] := CSTR;` becomes `@g3 = global [4 x i8] c"constant_text\00..."` and prints the whole `constant_text` with `LEN = 13`. The run-time assignment `a3 := CSTR;` and a struct member `m : STRING[3] := CSTR` correctly yield `con`. `resolve_const_reference` returns `statement.clone()` of the constant's own resolved literal, so the cloned node keeps the AST id of CSTR's initializer; in codegen `generate_string_literal_for_type` asks `get_type_hint_info_for` for that id and receives CSTR's type `STRING` (size 80), so it builds an 81-byte constant for a variable of type `STRING[3]`. Expected: the initializer is truncated to the declared size, `con` with `LEN = 3`, as IEC 61131-3 requires for string assignment. Reproduce with the default command line at any optimization level, or `plc --ir`, which fails with `constant expression type mismatch: got type '[81 x i8]' but expected '[4 x i8]'`.
```
// Reproducible example:
// observed (-O none): cons|4|5  then  constant_text|13|6
// observed (default): cons<garbage>|8|5  then  constant_text|13|6
// expected:           con|3|5  then  con|3|6
VAR_GLOBAL CONSTANT
    CSTR : STRING := 'constant_text';
END_VAR
VAR_GLOBAL
    g3 : STRING[3] := CSTR;
    gguard : DINT := 6;
END_VAR
FUNCTION main : DINT
VAR
    c3 : STRING[3] := CSTR;
    guard : DINT := 5;
END_VAR
    printf('%s|%d|%d$N', REF(c3), LEN(c3), guard);
    printf('%s|%d|%d$N', REF(g3), LEN(g3), gguard);
    main := 0;
END_FUNCTION
```

P1 compiler/plc_driver/src/pipelines/property.rs:21:5: Property read through a property passes --check but aborts codegen
`n := a.kid.value;`, where `kid` is a `PROPERTY_GET` of function block type on the outer block and `value` is a property of that inner type, passes `--check` without a diagnostic, but `--ir` aborts with `error: Could not resolve reference to value`. In the first resolver pass `a.kid` is annotated as a property, and a property annotation carries no type, so `value` stays unresolved. The lowerer rewrites `a.kid` to `a.__get_kid()` once in `post_annotate` and re-annotates; `value` now gets a property annotation, but the lowerer does not run again and never rewrites it. Validation does not report the reference, and codegen fails on the plain member reference. Expected: `a.__get_kid().__get_value()` is generated (repeat the rewrite until no property annotation is left), or `--check` reports E048.
```
// Reproducible example:
// plc --check: no diagnostic
// plc --ir: error: Could not resolve reference to value
FUNCTION_BLOCK Inner
VAR
    v : DINT := 5;
END_VAR
    PROPERTY_GET value : DINT
        value := v;
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK Outer
VAR
    inner : Inner;
END_VAR
    PROPERTY_GET kid : Inner
        kid := inner;
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    a : Outer;
    n : DINT;
END_VAR
    n := a.kid.value;
    printf('%d$N', n);
END_FUNCTION
```

P1 compiler/plc_lowering/src/initializer.rs:200:5: VAR_EXTERNAL variable of a struct type aborts codegen
`FUNCTION_BLOCK Fb VAR_EXTERNAL gS : S; END_VAR END_FUNCTION_BLOCK` with `S` a struct passes `--check` (only the warning E106 "VAR_EXTERNAL blocks have no effect"), but `--ir` aborts with `error: Could not resolve reference to Fb.gS` and the note "error occurred while generating initialization code for type 'Fb'". A scalar `VAR_EXTERNAL gS : DINT;` compiles. The init participant skips only constant blocks and global include or external blocks; a `VAR_EXTERNAL` block inside a POU is walked, and the struct member gets `S__ctor(self.gS)` although `gS` is not a member of the instance. Expected: `VAR_EXTERNAL` blocks are skipped, as the validator says they have no effect.
```
// Reproducible example:
// plc --check: only warning E106
// plc --ir: error: Could not resolve reference to Fb.gS
TYPE S : STRUCT
    x : DINT;
END_STRUCT END_TYPE

VAR_GLOBAL
    gS : S;
END_VAR

FUNCTION_BLOCK Fb
VAR_EXTERNAL
    gS : S;
END_VAR
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    fb : Fb;
END_VAR
    fb();
END_FUNCTION
```

P1 src/validation/variable.rs:632:5: Retained program variable named like its type is rejected with E099
`PROGRAM Main VAR fb : Fb; END_VAR END_PROGRAM`, where `Fb` has a `VAR RETAIN` block, or `VAR RETAIN int : INT;`, fails with `error[E099]: REFERENCE TO variables can not reference other variables`; renaming the variable to `inst` compiles. The retain participant replaces the variable's type with a generated alias pointer type to `__Main_fb__retain : Fb`; the validator then looks up the pointer's target type name `Fb` among the members of `Main` with `find_member`, ignoring case, and finds the variable `fb` itself. Expected: compiles; the check must not apply to the generated pointer types, or must compare against variables only when the target is not a type.
```
// Reproducible example:
// plc --check: error[E099]: REFERENCE TO variables can not reference other variables (at `fb : Fb`)
// the same error for: PROGRAM Main VAR RETAIN int : INT; END_VAR END_PROGRAM
FUNCTION_BLOCK Fb
VAR RETAIN
    x : INT;
END_VAR
END_FUNCTION_BLOCK

PROGRAM Main
VAR
    fb : Fb;
END_VAR
END_PROGRAM
```

P1 compiler/plc_lowering/src/retain.rs:103:25: Retain global is defined by an include unit
With `-i hdr.st`, where `hdr.st` declares `PROGRAM Prog VAR RETAIN x : INT := 7; END_VAR END_PROGRAM`, the object generated from `main.st` defines `__Prog_x__retain` (`nm` shows `D`, the IR has `@__Prog_x__retain = global i16 7, section ".retain"`), while `Prog_instance` stays a declaration (`U`, `external global`). The retain block the participant creates always has internal linkage, whatever the linkage of the unit, although the generated pointer type does take the container linkage. Expected: in an include unit the header contributes only a declaration; the definition belongs to the library that implements `Prog`. Two definitions result when linked against that library.
```
// Reproducible example:
// plc -c main.st -i hdr.st -o main.o; nm main.o shows "D __Prog_x__retain" and "U Prog_instance"
// --- file: hdr.st ---
PROGRAM Prog
VAR RETAIN
    x : INT := 7;
END_VAR
END_PROGRAM

// --- file: main.st ---
FUNCTION main : DINT
    Prog();
    main := Prog.x;
END_FUNCTION
```

P1 src/lowering/calls.rs:120:13: Global initializer that calls an aggregate-returning function panics the compiler
`VAR_GLOBAL g : STRING := greet(); END_VAR`, where `greet` returns a `STRING`, aborts with `internal error: entered unreachable code: Statement lists should exist at this point` instead of a diagnostic, already with `--check`. The aggregate-return lowerer walks global variable blocks, meets the call, and tries to queue the allocation in front of the current statement, but no statement scope exists outside an implementation. The scalar case `g : DINT := getInt();` reports `E033 Unresolved constant ... Call-statement 'getInt' in initializer is not constant`. Expected: the same E033 diagnostic for the aggregate case.
```
// Reproducible example:
// plc --check: thread 'main' panicked at src/lowering/calls.rs:120:13:
// internal error: entered unreachable code: Statement lists should exist at this point
FUNCTION greet : STRING
    greet := 'hi';
END_FUNCTION

VAR_GLOBAL
    g : STRING := greet();
END_VAR

FUNCTION main : DINT
    main := 0;
END_FUNCTION
```

P1 compiler/plc_lowering/src/initializer.rs:280:5: Bare member names inside an array literal initializer are not qualified with self
`PROGRAM prog VAR seed : DINT := 7; vars : ARRAY[0..2] OF DINT := [seed, 2, seed]; END_VAR END_PROGRAM` passes `--check`, but `--ir` fails with `Could not resolve reference to seed` while generating the constructor of `prog`. The init participant qualifies a flat reference initializer with `self.`, but not the elements of an array literal, so the array lowerer produces `self.vars[0] := seed` inside `prog__ctor`, where `seed` does not resolve. The same happens for a function block. Expected: `self.vars[0] := self.seed;`.
```
// Reproducible example:
// plc --check: no diagnostic
// plc --ir: error: Could not resolve reference to seed (initialization code for type 'prog')
PROGRAM prog
VAR
    seed : DINT := 7;
    vars : ARRAY[0..2] OF DINT := [seed, 2, seed];
END_VAR
END_PROGRAM

FUNCTION main : DINT
    printf('%d %d %d$N', prog.vars[0], prog.vars[1], prog.vars[2]);
END_FUNCTION
```

P1 compiler/plc_lowering/src/array_lowering.rs:435:17: Global array with a runtime element is not lowered in the unit constructor
`VAR_GLOBAL gseed : DINT := 1; garr : ARRAY[0..2] OF DINT := [gseed, 2, 3]; END_VAR` passes `--check`, but `--ir` fails with `Builder error: ... invalid use of function-local name` on `@.const_init = private unnamed_addr constant [3 x i32] [i32 %load_gseed, i32 2, i32 3]`. The left side of the assignment in the unit constructor is a bare name, and the bare-name branch of `find_lhs_type_name` searches only the members of the enclosing POU with `find_member`, not the globals, so the literal is not lowered and codegen tries to build a constant from a variable load. Expected: `garr[0] := gseed;` and the remaining element assignments in the unit constructor.
```
// Reproducible example:
// plc --check: no diagnostic
// plc --ir: error: Builder error: invalid use of function-local name
VAR_GLOBAL
    gseed : DINT := 1;
    garr : ARRAY[0..2] OF DINT := [gseed, 2, 3];
END_VAR

FUNCTION main : DINT
    printf('%d %d %d$N', garr[0], garr[1], garr[2]);
END_FUNCTION
```

P1 compiler/plc_lowering/src/array_lowering.rs:232:5: Constant repeat count from a POU-local constant is not rewritten in the constructor
`PROGRAM prog VAR CONSTANT N : DINT := 3; END_VAR VAR arr : ARRAY[0..2] OF DINT := [(N)(1)]; END_VAR END_PROGRAM` passes `--check`, but `--ir` fails with `cannot generate call statement for ParenExpression { expression: ReferenceExpr { kind: Member(Identifier { name: "N" }) ... }`. The body pass of `rewrite_const_multiplied_initializers` resolves the multiplier as a member of `implementation.type_name`, which for the generated constructor is `prog__ctor`, not `prog`, so the `(N)(1)` call node stays. A global constant, or a local constant inside a `FUNCTION`, works and prints `1 1 1`. Expected: the node is rewritten to the repetition `3(1)`.
```
// Reproducible example:
// plc --check: no diagnostic
// plc --ir: error: cannot generate call statement for ParenExpression (initialization code for type 'prog')
PROGRAM prog
VAR CONSTANT
    N : DINT := 3;
END_VAR
VAR
    arr : ARRAY[0..2] OF DINT := [(N)(1)];
END_VAR
END_PROGRAM

FUNCTION main : DINT
    printf('%d %d %d$N', prog.arr[0], prog.arr[1], prog.arr[2]);
END_FUNCTION
```

P1 compiler/plc_lowering/src/array_lowering.rs:170:9: Array literal assignment inside a control statement is not lowered
`IF TRUE THEN loc := [gseed, 5, 6]; END_IF` in a function passes `--check`, but `--ir` fails with `Builder error: ... invalid use of function-local name` on `@.const_init = ... [i32 %load_gseed, i32 5, i32 6]`; the same assignment at the top level of the body compiles and prints `4 5 6`. `lower_literal_arrays` inspects only the top-level statements of an implementation and does not descend into the bodies of `IF`, `CASE`, or loops. Expected: the assignment is lowered like a top-level one.
```
// Reproducible example:
// plc --check: no diagnostic
// plc --ir: error: Builder error: invalid use of function-local name
VAR_GLOBAL
    gseed : DINT := 4;
END_VAR

FUNCTION main : DINT
VAR
    loc : ARRAY[0..2] OF DINT;
END_VAR
    IF TRUE THEN
        loc := [gseed, 5, 6];
    END_IF
    printf('%d %d %d$N', loc[0], loc[1], loc[2]);
END_FUNCTION
```

P1 src/resolver/const_evaluator.rs:488:21: Member initializer that names an inherited constant fails with E033
`FUNCTION_BLOCK X VAR CONSTANT c : DINT := 10; END_VAR END_FUNCTION_BLOCK FUNCTION_BLOCK Y EXTENDS X VAR o : DINT := c; END_VAR END_FUNCTION_BLOCK` aborts with `error[E033]: Unresolved constant `o` variable` at the initializer, while the same initializer inside one block compiles. The inheritance lowerer rewrites the initializer to `__X.c` in `post_annotate` (visible in `--ast-lowered`), the array lowerer and the init participant then re-index the project, which runs `evaluate_constants` again, and the constant evaluator takes the flat name of the base, `__X`, as the scope for `find_variable`; `__X` is a member of `Y`, not a POU, so the lookup finds nothing, the initializer stays unresolved and validation reports E033. Expected: the constant resolves as before the rewrite, for example by resolving the base `__X` to its type `X` before the member lookup.
```
// Reproducible example:
FUNCTION_BLOCK X
    VAR CONSTANT
        c : DINT := 10;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK Y EXTENDS X
    VAR
        o : DINT := c;    // error[E033]: Unresolved constant `o` variable
    END_VAR
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        y : Y;
    END_VAR
    printf('%d$N', y.o);    // expected: 10
END_FUNCTION
```

P1 src/lowering/polymorphism/dispatch/pou.rs:143:13: Method calls inside ACTION bodies bypass the method table
`FUNCTION_BLOCK Base METHOD name : DINT name := 1; END_METHOD END_FUNCTION_BLOCK ACTIONS ACTION show printf('%d$N', name()); END_ACTION END_ACTIONS`, with `FUNCTION_BLOCK Child EXTENDS Base` overriding `name` to return `2`, prints `1` for `child.show()`, while a method of `Base` with the same call `name()` prints `2`. In `visit_implementation` the dispatch lowerer sets `in_method_or_function_block` only for `PouType::FunctionBlock` and `PouType::Method`; `PouType::Action` falls into the `_ => None` arm, so the candidate check for calls inside bodies never fires, the call stays direct and is bound to `Base.name` at compile time. Expected: the call in the action goes through the method table like the call in the method, since an action body runs on the same instance as the function block body.
```
// Reproducible example:
FUNCTION_BLOCK Base
    METHOD name : DINT
        name := 1;
    END_METHOD
    METHOD viaMethod
        printf('%d$N', name());
    END_METHOD
END_FUNCTION_BLOCK
ACTIONS
    ACTION show
        printf('%d$N', name());
    END_ACTION
END_ACTIONS

FUNCTION_BLOCK Child EXTENDS Base
    METHOD name : DINT
        name := 2;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        child : Child;
    END_VAR
    child.show();         // prints 1, expected 2
    child.viaMethod();    // prints 2
END_FUNCTION
```

P1 src/lowering/polymorphism/dispatch/pou.rs:287:13: Method call on a `VAR_IN_OUT` parameter of a function block type is bound statically
`FUNCTION viaInOut : DINT VAR_IN_OUT r : Rect; END_VAR viaInOut := r.area(); END_FUNCTION` called with a `Square EXTENDS Rect` whose `area` is overridden returns the result of `Rect.area` (`6` instead of `9` for `w := 3`), while the same call through a `VAR_INPUT r : REFERENCE TO Rect` parameter returns `9`. The candidate check accepts a plain member base (`Member(member), None`) only when its annotation `is_reference_to`, that is when the auto-deref kind is `AutoDerefType::Reference`; `VAR_IN_OUT` parameters are auto-deref pointers of the `Default` kind and are not dispatched. Expected: a call through any by-reference parameter of a class or function block type goes through the method table, because the compiler accepts a derived instance for the parameter.
```
// Reproducible example:
FUNCTION_BLOCK Rect
    VAR
        w : DINT := 3;
    END_VAR
    METHOD area : DINT
        area := w * 2;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Square EXTENDS Rect
    METHOD area : DINT
        area := w * w;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION viaInOut : DINT
    VAR_IN_OUT
        r : Rect;
    END_VAR
    viaInOut := r.area();
END_FUNCTION

FUNCTION viaRef : DINT
    VAR_INPUT
        r : REFERENCE TO Rect;
    END_VAR
    viaRef := r.area();
END_FUNCTION

FUNCTION main : DINT
    VAR
        s : Square;
    END_VAR
    printf('%d$N', viaInOut(s));    // prints 6, expected 9
    printf('%d$N', viaRef(s));      // prints 9
END_FUNCTION
```

P1 src/lowering/polymorphism/table/interface.rs:180:13: Interface table type from an include file is defined again in the consumer object
With `INTERFACE Shape` in `shape.st`, `FUNCTION_BLOCK Rect IMPLEMENTS Shape` in `rect.st`, `plc -c rect.st shape.st -o librect.o` followed by `plc main.st -i rect.st -i shape.st librect.o` fails with `ld.lld: error: duplicate symbol: __itable_Shape__ctor` and `____itable_Shape_area__ctor`, both defined in `librect.o` and in the object of the included `shape.st`. `itable_definition_linkage` gives the `__itable_Shape` struct `Internal` linkage when the unit that declares the interface contains no implementing POU (the fallthrough `LinkageType::Internal`), regardless of the unit's own linkage, so the init participant emits the struct constructor in both objects. A plain `STRUCT` in an include file does not have this problem. Expected: the itable type of an include unit takes the unit's linkage.
```
// Reproducible example:
// plc -c rect.st shape.st -o librect.o
// plc main.st -i rect.st -i shape.st librect.o    (ld.lld: duplicate symbol: __itable_Shape__ctor, ____itable_Shape_area__ctor)
// --- file: shape.st ---
INTERFACE Shape
    METHOD area : DINT
    END_METHOD
END_INTERFACE

// --- file: rect.st ---
FUNCTION_BLOCK Rect IMPLEMENTS Shape
    VAR
        w : DINT := 3;
    END_VAR
    METHOD area : DINT
        area := w * 2;
    END_METHOD
END_FUNCTION_BLOCK

// --- file: main.st ---
FUNCTION main : DINT
    VAR
        r : Rect;
    END_VAR
    printf('%d$N', r.area());    // expected: 6
END_FUNCTION
```

P1 compiler/plc_driver/src/pipelines/participant.rs:446:17: CFC POU with `EXTENDS` cannot reach inherited members
A `.cfc` function block whose text declaration reads `FUNCTION_BLOCK CfcChild EXTENDS Base` and whose network assigns the inherited output `baseOut` does not compile: `--check` reports `E048: Could not resolve reference to baseOut` on every access `c.baseOut` from other code, and a build with no such access fails in codegen with `error: Could not resolve reference to __Base` and no location; the same declaration and body written in Structured Text compile. `--ast-lowered` shows the `CfcChild` POU with `variable_blocks: []` while its body was rewritten to `__Base.baseOut := 42`. The CFC participant replaces the parse step's unit with a fresh parse of the declaration at `post_index`, after the `pre_index` participants ran, so the `__Base` member the inheritance lowerer inserted is gone. Any other `pre_index` rewrite of the declaration (properties, `REFERENCE TO` returns) is lost the same way. Expected: the transpiled body is placed into the unit the participants already processed, or the swapped unit is sent through the `pre_index` hooks.
```
// Reproducible example:
// plc main.st CfcChild.cfc ...    -> error[E048]: Could not resolve reference to baseOut (at c.baseOut in main.st)
// --- file: main.st ---
FUNCTION_BLOCK Base
    VAR_OUTPUT
        baseOut : DINT;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK StChild EXTENDS Base
    baseOut := 42;
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        c : CfcChild;
        s : StChild;
    END_VAR
    c();
    s();
    printf('%d %d$N', c.baseOut, s.baseOut);    // expected: 42 42
END_FUNCTION

// --- file: CfcChild.cfc ---
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:FunctionBlock xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="CfcChild">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <bmx:TextDeclaration>FUNCTION_BLOCK CfcChild EXTENDS Base</bmx:TextDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="42" globalId="1">
                    <ppx:RelPosition x="250" y="140"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="2">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="baseOut" globalId="3">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="0"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="400" y="140"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="2"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:FunctionBlock>
```

P1 src/codegen/generators/pou_generator.rs:771:33: A function reads past the end of a shorter string argument
For `FUNCTION greet : STRING VAR_INPUT who : STRING; END_VAR` called as `greet('bob')`, the caller passes the literal's own constant, `@utf08_literal_1 = [4 x i8] c"bob\00"`, and the callee copies `(size - 1) * char_width` bytes of the parameter type into its local: `call void @llvm.memcpy.p0.p0.i64(ptr %who, ptr %1, i64 80, ...)`. The copy length in the by-value aggregate prologue comes from the declared parameter type, never from the argument, so 76 bytes past the end of the constant are read. The same happens for a `STRING[5]` variable passed to a `STRING` parameter (`call void @greet(ptr %__greet1, ptr %s)` followed by the same 80 byte copy). The result is correct as long as the bytes after the terminator are ignored and the read does not cross into an unmapped page. Reproduce with `plc --ir` on the snippet. Expected: copy the smaller of the argument's and the parameter's size, or pass the literal in a buffer of the parameter's size.
```
// Reproducible example:
// plc main.st --ir -o -
// observed in greet: call void @llvm.memcpy.p0.p0.i64(ptr align 1 %who, ptr align 1 %1, i64 80, i1 false)
// callers: call void @greet(ptr %__greet0, ptr @utf08_literal_1)   ; [4 x i8] c"bob\00"
//          call void @greet(ptr %__greet1, ptr %s)                 ; s is STRING[5], 6 bytes
FUNCTION greet : STRING
    VAR_INPUT
        who : STRING;
    END_VAR
    greet := who;
END_FUNCTION

FUNCTION main : DINT
    VAR
        s : STRING[5] := 'hi';
        r : STRING;
    END_VAR
    r := greet('bob');
    r := greet(s);
    printf('%s$N', REF(r));
END_FUNCTION
```

P1 compiler/plc_lowering/src/array_lowering.rs:282:9: Repeated array initializer with a bare constant count aborts codegen
`VAR_GLOBAL CONSTANT MAX : DINT := 3; END_VAR` and `rep : ARRAY[1..MAX] OF DINT := [MAX(7)];` pass `--check` but abort codegen: inside a POU with `error: Unknown type: __MAX__T.` (note "error occurred while generating initialization code for type 'main'"), and as a global with `error: Cannot generate Literal for CallStatement { operator: ReferenceExpr { kind: Member(Identifier { name: "MAX" }) ... }`. The parser produces a call node `MAX(7)` for the repetition; `try_rewrite_call_as_multiplied` in the array lowerer rewrites such calls into `MultipliedStatement`s only when the operator is a `ParenExpression`, `[(MAX)(7)]`, and leaves the bare spelling as a call to a nonexistent POU `MAX` that codegen cannot handle. `[3(7)]` and `[(MAX)(7)]` compile and run. Expected: both spellings compile, or the bare spelling is rejected by the validator with a message that names the accepted form.
```
// Reproducible example:
VAR_GLOBAL CONSTANT
    MAX : DINT := 3;
END_VAR

FUNCTION main : DINT
    VAR
        rep : ARRAY[1..MAX] OF DINT := [MAX(7)];    // error: Unknown type: __MAX__T.
        // rep : ARRAY[1..MAX] OF DINT := [(MAX)(7)];    compiles, prints 7
        // rep : ARRAY[1..MAX] OF DINT := [3(7)];        compiles, prints 7
    END_VAR
    printf('%d$N', rep[3]);    // expected: 7
END_FUNCTION
```

P1 src/resolver.rs:2672:17: Compiler panics when a VLA variable is referenced outside its declaring POU
The resolver hints every reference to a VLA with the array type behind the struct and looks the variable up by name among the members of the current POU (`get_pou_members(pou)` filtered by name); when the name is not a member, it hits `unreachable!()` and the compiler aborts with `thread '<unnamed>' panicked at src/resolver.rs:2672:17: internal error: entered unreachable code`, under `--check` as well as when building. Three inputs reach it: a global `VAR_GLOBAL g : ARRAY[*] OF DINT; END_VAR` (the unit constructor calls `__global_g__ctor(g)`), a function block or program with a VLA in a block other than `VAR_IN_OUT` (the constructor generated by the init participant references `self.inp`), and, on valid input, the caller reading the parameter back with `total := inst.io[0]` after `inst(io := arr)`. In the first two cases the validator would report E044 (Invalid POU for VLA), but the panic happens during annotation and the diagnostic is never shown. Expected: no panic; resolve the VLA through the annotation's qualified name, or fall back to no hint, and let validation report the misplaced declarations.
```
// Reproducible example:
// Case 3 (valid input): plc --check main.st -> panicked at src/resolver.rs:2672:17
FUNCTION_BLOCK FB
    VAR_IN_OUT
        io : ARRAY[*] OF DINT;
    END_VAR
    io[0] := 5;
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        inst : FB;
        arr : ARRAY[0..2] OF DINT;
        total : DINT;
    END_VAR
    inst(io := arr);
    total := inst.io[0];
    printf('%d$N', total);    // expected: 5
END_FUNCTION

// Case 1: same panic (validator should report E044)
// VAR_GLOBAL
//     g : ARRAY[*] OF DINT;
// END_VAR

// Case 2: same panic (validator should report E044)
// FUNCTION_BLOCK FB2
//     VAR_INPUT
//         inp : ARRAY[*] OF DINT;
//     END_VAR
// END_FUNCTION_BLOCK
```

P1 src/codegen/generators/expression_generator.rs:2705:13: Struct literal for a function block with an initialized VAR_TEMP aborts codegen
`FUNCTION_BLOCK Acc VAR total : DINT := 100; END_VAR VAR_TEMP t : DINT := 4; END_VAR END_FUNCTION_BLOCK` used in `FUNCTION_BLOCK Outer VAR a : ARRAY[0..1] OF Acc := [(total := 1), (total := 2)]; END_VAR END_FUNCTION_BLOCK` passes `--check`, but building aborts with `error: Cannot generate literal initializer for 'Acc.t': Value cannot be derived` and the note "error occurred while generating initialization code for type 'Outer'". The same block without the array literal, and the same literal without the `VAR_TEMP` initializer, compile and run correctly. `generate_literal_struct` starts from every member of the `DataTypeInformation::Struct` of `Acc`, which includes the temporary `t`, and for each member the literal does not mention it looks up an LLVM initial value; a temporary has no slot in the instance struct (the LLVM type generation filters `VariableType::Temp`) and no associated value, so the lookup fails and the error is raised. Expected: members that are not part of the instance struct (temporaries) are skipped, as the LLVM type generation already skips them.
```
// Reproducible example:
FUNCTION_BLOCK Acc
    VAR
        total : DINT := 100;
    END_VAR
    VAR_TEMP
        t : DINT := 4;    // remove the initializer and the build succeeds
    END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK Outer
    VAR
        a : ARRAY[0..1] OF Acc := [(total := 1), (total := 2)];    // error: Cannot generate literal initializer for 'Acc.t'
    END_VAR
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        o : Outer;
    END_VAR
    printf('%d$N', o.a[1].total);    // expected: 2
END_FUNCTION
```

P1 src/codegen/generators/expression_generator.rs:2747:25: Struct literal that names an inherited member of a function block aborts codegen
`c : Child := (x := 7)`, where `x` is declared in `Base` and `Child EXTENDS Base`, passes `--check` (with only E049 warnings) and aborts codegen with `error: Expected 2 fields for Struct Child, but found 3.` The inheritance lowerer turns the member into `__Base.x` for statements, but the assignments inside an initializer literal are not rewritten (the lowered AST still holds `x` with no base), so the literal generator counts `x` as a third field next to `__Base` and the child's own member. Expected: the literal initializes the member inside `__Base`, or a diagnostic says that inherited members cannot be set in a literal.
```
// Reproducible example:
FUNCTION_BLOCK Base
VAR
    x : DINT;
END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK Child EXTENDS Base
VAR
    y : DINT;
END_VAR
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    c : Child := (x := 7);
END_VAR
    // --check passes; build aborts with
    // error: Expected 2 fields for Struct Child, but found 3.
    printf('%d$N', c.x);
END_FUNCTION
```

P1 src/codegen/generators/expression_generator.rs:2659:21: Default value on an interface method parameter aborts codegen when a call omits that parameter
`INTERFACE Calc METHOD compute : DINT VAR_INPUT a : DINT; b : DINT := 10; END_VAR END_METHOD END_INTERFACE`, implemented by `Adder`, called through `c : Calc` as `c.compute(a := 3)`, passes `--check` but the build aborts with `error: no type hint available for 10` at the location of the literal. The same call with `b` passed explicitly, and a program that declares the interface without calling it, build without error. The dispatch lowerer builds the argument list of the interface call from the interface method's parameters, and for the omitted `b` it takes the initializer from the interface declaration; that initializer belongs to a method without a body, so it was never annotated, and the type hint lookup fails. Expected: initializers of interface method parameters are used with a type, or rejected with a diagnostic.
```
// Reproducible example:
// plc --check: no diagnostic
// build: error: no type hint available for 10 at main.st:5:16
INTERFACE Calc
    METHOD compute : DINT
    VAR_INPUT
        a : DINT;
        b : DINT := 10;
    END_VAR
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK Adder IMPLEMENTS Calc
    METHOD compute : DINT
    VAR_INPUT
        a : DINT;
        b : DINT;
    END_VAR
        compute := a + b;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    ad : Adder;
    c : Calc;
END_VAR
    c := ad;
    printf('%d$N', c.compute(a := 3));
    // c.compute(a := 3, b := 1) builds and prints 4
END_FUNCTION
```

P1 src/codegen/generators/expression_generator.rs:3090:89: MOVE on a string panics the compiler
`s3 := MOVE(s);` with `s : STRING` and `s3 : STRING[3]` passes `--check` and panics codegen with `Found ArrayValue(...) but expected PointerValue variant` at expression_generator.rs:3090:89. The `MOVE` builtin (src/builtins.rs:275) returns `generate_expression(actual_param)` as an RValue, that is the loaded `[81 x i8]` value; for an aggregate, `generate_store` expects the address of the source for the bounded memcpy and calls `into_pointer_value()` on it. Expected: `MOVE` returns the address for aggregates, as a plain reference does, or a diagnostic restricts `MOVE` to scalars.
```
// Reproducible example:
FUNCTION main : DINT
VAR
    s : STRING := 'hello';
    s3 : STRING[3];
END_VAR
    // --check passes; build panics:
    // thread panicked at src/codegen/generators/expression_generator.rs:3090:89:
    // Found ArrayValue(...) but expected PointerValue variant
    s3 := MOVE(s);
    printf('%s$N', REF(s3));
END_FUNCTION
```

P1 src/codegen/generators/expression_generator.rs:2821:17: Nested array literal for a multi-dimensional array aborts codegen
A variable declared as `ARRAY[0..1, 0..1] OF INT := [[1, 2], [3, 4]]` passes `--check` without any diagnostic, but code generation aborts with `error: Expected array type but found: INT`. The same nested literal works for an array of arrays (`ARRAY[0..1] OF ARRAY[0..1] OF INT`) and the flat form `[1, 2, 3, 4]` works for the multi-dimensional array, so the user expects either generated code or a proper diagnostic. The cause is a mismatch between validation and codegen: `statement_to_array_length` in `src/validation/array.rs:175` counts the nested literals recursively (4 elements, no E043/E127), but `generate_literal_array_value` in `src/codegen/generators/expression_generator.rs:2821` flattens the initializer with `flatten_expression_list`, which only flattens `ExpressionList`, `MultipliedStatement` and `ParenExpression` nodes and leaves the inner `[1, 2]` literals as elements. Each inner literal is then generated with the element type INT as type hint, and the array-type check at line 2801 fails. Reproduce with `plc main.st -i "/workspace/tests/lit/util/*.pli" -liec61131std -L/workspace/output/lib -i "/workspace/output/include/*.st" --linker cc -o main.out` (any optimization level); `plc main.st --check` reports nothing.
```
// Reproducible example:
FUNCTION main : DINT
VAR
    m : ARRAY[0..1, 0..1] OF INT := [[1, 2], [3, 4]];
END_VAR
    // observed: error: Expected array type but found: INT at main.st:3:37 (--check passes)
    // expected: prints 1 2 3 4, or a validation diagnostic
    printf('%d %d %d %d$N', m[0,0], m[0,1], m[1,0], m[1,1]);
    main := 0;
END_FUNCTION
```

P1 src/validation/statement.rs:2218:36: Method call named like a builtin is validated with the builtin's argument count
A method call `acc.add(1)` on a function block instance, where the method `add` takes one input, fails `--check` with `error[E032]: this POU takes 2 arguments but 1 argument was supplied`; `b.sel(1)` on a method named `sel` reports `takes 3 arguments`. Renaming the method (for example to `plus`) compiles and runs. The call validator looks up a builtin validation routine with `builtins::get_builtin(fn_ident.get_flat_reference_name())`, and `get_flat_reference_name` (compiler/plc_ast/src/ast.rs:1337) returns only the last member segment of a qualified reference, so the qualifier `acc` is ignored and the builtin `ADD` (or `SEL`, `MUL`, `SUB`, `DIV`, `MUX`, ...) validation in src/builtins.rs runs against the user method's argument list and emits E032. The annotation of the operator already names the method (`Acc.add`), so the builtin validation must only run when the operator actually resolves to the builtin. Expected: the program compiles and prints `3`. Reproduce with `plc --check main.st` or the normal compile command at `-O none` or the default level.
```
// Reproducible example:
FUNCTION_BLOCK Acc
VAR
    total : DINT;
END_VAR
METHOD add : DINT
VAR_INPUT
    p : DINT;
END_VAR
    total := total + p;
    add := total;
END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    acc : Acc;
    r : DINT;
END_VAR
    r := acc.add(1);
    r := acc.add(2);
    // observed: error[E032]: this POU takes 2 arguments but 1 argument was supplied (compilation aborted)
    // expected: compiles and prints 3
    printf('%d$N', r);
    main := 0;
END_FUNCTION
```

P1 src/codegen/generators/expression_generator.rs:2201:25: Pointer minus pointer passes --check but aborts codegen
`diff := p - q;` with `p, q : POINTER TO INT` passes `plc main.st --check` with exit code 0 and no diagnostic, but a full build stops with `error: '-' operation must contain one int type` and no object file is produced. `generate_binary_expression` (line 409) routes every pointer/pointer binary expression to `create_llvm_binary_expression_for_pointer`, which only handles `Plus` and `Minus` when exactly one operand is an integer and otherwise returns the codegen error at line 2200; the validator has no rule for a pointer/pointer `+` or `-`, so the program reaches codegen. Expected: either a validation diagnostic reported by `--check` or a pointer difference in elements. Reproduce with the default command line at any optimization level.
```
// Reproducible example:
FUNCTION main : DINT
VAR
    arr : ARRAY[0..9] OF INT;
    p : POINTER TO INT;
    q : POINTER TO INT;
    diff : LINT;
END_VAR
    p := ADR(arr[3]);
    q := ADR(arr[0]);
    diff := p - q;
    // plc main.st --check: exit 0, no diagnostic
    // plc main.st ... -o main.out: error: '-' operation must contain one int type, compilation aborted
END_FUNCTION
```

P1 src/resolver.rs:2896:21: Comparing a pointer with NULL passes --check but aborts codegen
`IF p = NULL THEN ... END_IF`, `b := p = NULL;`, `b := NULL = p;` and `IF p <> NULL THEN` with `p : POINTER TO INT` pass `plc main.st --check` with exit code 0, but a full build stops with `error: no type hint available for NULL` and produces no output. The resolver ignores the `NULL` literal (`_ => {}` at line 2896) and the binary-expression visitor does not hint its operands with the other side's pointer type, so the literal carries neither a type nor a type hint; `generate_binary_expression` (src/codegen/generators/expression_generator.rs:385 and 387) calls `get_type_hint_for` on both operands and fails. Only `p := NULL` works, because the assignment hints the right side with the left type. A null check before a dereference is one of the most common pointer patterns in PLC code. Expected: the compare is generated as an `icmp eq` against a null pointer, or `--check` rejects the expression. Reproduce with the default command line at any optimization level.
```
// Reproducible example:
FUNCTION main : DINT
VAR
    p : POINTER TO INT;
    b : BOOL;
END_VAR
    p := NULL;
    IF p = NULL THEN
        b := TRUE;
    END_IF
    // plc main.st --check: exit 0, no diagnostic
    // plc main.st ... -o main.out: error: no type hint available for NULL, compilation aborted
END_FUNCTION
```

P1 src/codegen/generators/variable_generator.rs:154:29: Global initializer expression that overflows its type aborts codegen with "Builder position is not set"
A global variable such as `X : SINT := 100 + 100;` (with or without CONSTANT) produces the expected E039 overflow warning and then codegen aborts with `error: Builder error: Builder position is not set.` instead of producing a binary. The expected behavior is either a hard error at validation or a wrapped constant like the one emitted for the literal `X : SINT := 200` (which compiles to `i8 -56`). The cause is that `const_evaluator::evaluate_with_target_hint` returns `UnresolvableKind::Overflow` for the folded result, so the constant expression stays as the unevaluated `100 + 100` binary expression; `generate_global_variable` then calls `generate_expression` through a context-free `ExpressionCodeGenerator` on that binary expression, which needs an instruction builder positioned in a basic block, and inkwell fails because there is none. Reproduce with `plc main.st --ir -o -` or the full link command at any optimization level.
```
// Reproducible example:
VAR_GLOBAL
    X : SINT := 100 + 100;
END_VAR

FUNCTION main : DINT
    printf('%d$N', X);
    main := 0;
END_FUNCTION
// observed: warning[E039] This will overflow for type SINT, then
//           error: Builder error: Builder position is not set.  (no binary produced)
// expected: either a compile error, or -56 printed like the literal form X : SINT := 200
```

P1 src/resolver/const_evaluator.rs:837:18: TIME literal multiplied or divided by a number in an initializer aborts with E033
`t : TIME := T#1s * 2;` (or `T#1s / 2`, or a `VAR_GLOBAL CONSTANT` of the same form) aborts compilation with `error[E033]: Unresolved constant 't' variable: Cannot evaluate LiteralTime { nanos: 1000000000, negative: false } * LiteralInteger { value: 2 }` at `--check` and at build time, while `t : TIME := T#1s + T#500ms` compiles. The `arithmetic_expression!` macro in the constant evaluator only has arms for Integer and Real literal pairs, so a `Time` (or `Date`, `TimeOfDay`, `DateAndTime`) literal combined with an integer or real literal falls into the `_ => cannot_eval_error!` arm at line 837 and the unresolved initializer becomes a hard error. Expected: the initializer evaluates to `T#2s` (2000 ms), as `TIME * ANY_NUM` is a valid IEC 61131-3 operation and compiles in statement position. Reproduce with `plc main.st --check`.
```
// Reproducible example:
FUNCTION main : DINT
VAR
    t : TIME := T#1s * 2;   // error[E033]: Unresolved constant `t` variable: Cannot evaluate LiteralTime { nanos: 1000000000, negative: false } * LiteralInteger { value: 2 }
    u : TIME := T#1s + T#500ms; // compiles
END_VAR
    printf('%u %u$N', t, u); // expected 2000 1500
    main := 0;
END_FUNCTION
```

P1 src/resolver.rs:3327:13: Recursive unqualified call of a method from its own body aborts codegen
`METHOD fact : DINT VAR_INPUT n : DINT; END_VAR ... fact := n * fact(n - 1); END_METHOD` inside a function block passes `--check` but the build aborts with `error: no type hint available for fact(n - 1)`; when the recursive call is first stored in a local (`r := fact(n - 1);`) the compiler panics at `src/codegen/generators/expression_generator.rs:1577` (`explicit panic`, missing `Argument` hint). The call operator is resolved with `call_operator_scopes`, whose `FunctionsOnly` step only accepts `index.find_pou(name)` entries that are functions; a method is indexed as `Fact.fact` and is not a function, so the step finds nothing and the default scopes resolve the bare name `fact` to the method's own return variable (a `DINT`). The call statement therefore carries a `Variable` annotation, `get_call_name` yields no POU, the arguments never get `Argument` hints, and codegen fails. The same recursion in a `FUNCTION fact : DINT` works because `find_pou("fact")` finds the function before the return variable is considered, and `THIS^.fact(n - 1)` works because the qualified lookup finds the method. Expected: the unqualified recursive call resolves to the enclosing method and prints 120. Reproduce with the default command line at any optimization level.
```
// Reproducible example:
FUNCTION_BLOCK Fact
METHOD fact : DINT
VAR_INPUT
    n : DINT;
END_VAR
    IF n <= 1 THEN
        fact := 1;
    ELSE
        fact := n * fact(n - 1); // build: error: no type hint available for fact(n - 1)
    END_IF
END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    f : Fact;
END_VAR
    printf('%d$N', f.fact(5)); // expected 120
    main := 0;
END_FUNCTION
```

P2 compiler/plc_diagnostics/src/reporter/codespan.rs:149:13: Rich reporter drops the diagnostic for a POU named like a builtin
Declaring `FUNCTION add : DINT ... END_FUNCTION` (also seen with `ADR`, `SIZEOF`, `MUX`, `SEL`, `LOWER_BOUND`, `REF`) aborts with only "Compilation aborted due to critical errors" and no diagnostic. With `--error-format clang` the expected `error[E004]: add: Duplicate symbol.` at `main.st:1:10` is printed (plus a second E004 with no location for the builtin, and with the stdlib includes a third one at `output/include/arithmetic_functions.st:25:10`), so the diagnostic exists. The duplicate check in `src/validation/global.rs:42` only treats undefined or internal locations as builtin; a builtin function has a real text range in the `<builtin>` source, so a normal duplicate diagnostic is created with a secondary "see also" location in `<builtin>`. That source is never registered with the diagnostician, its file handle does not resolve, the codespan emit call returns an error, and the error is swallowed at codespan.rs:149 because the main location is not internal. Expected: a diagnostic such as "add can not be used as a name because it is a built-in function" pointing at the user's declaration, in every error format.
```
// Reproducible example:
// plc --check main.st
// default format prints only: error: Compilation aborted due to critical errors.
// --error-format clang prints: main.st:1:10: error[E004]: add: Duplicate symbol.
FUNCTION add : DINT
VAR_INPUT
    a : DINT;
END_VAR
    add := a;
END_FUNCTION

FUNCTION main : DINT
    main := 0;
END_FUNCTION
```

P2 compiler/plc_driver/src/pipelines/participant.rs:493:5: FOR loop type checks are unreachable in the compiler
`FOR r := 0.0 TO 1.0 DO END_FOR` with `r : REAL`, and `FOR i := 0 TO TRUE DO END_FOR`, compile without a diagnostic. The validator's check that counter, start, end, and step are integers (`validate_for_loop`, E094, src/validation/statement.rs:2491) and the resolver's hinting of start, end, and step with the counter's type (src/resolver.rs:2089) only handle `ForLoop` nodes, but `LoopDesugarer::pre_index` (participant.rs:493) has replaced every `FOR` and `REPEAT` with a `WHILE TRUE` loop before the index, the resolver, and the validator run; the lowered AST of the program only contains `WhileLoopStatement` nodes. The unit tests for E094 pass because `parse_and_validate_buffered` does not run the participants. Expected: `error[E094]: Expected an integer value, got REAL` at the loop header, as the tests describe.
```
// Reproducible example:
FUNCTION main : DINT
VAR
    r : REAL;
    i : DINT;
END_VAR
    // plc --check: no diagnostic, exit 0; expected E094 for both loops
    FOR r := 0.0 TO 1.0 DO
    END_FOR
    FOR i := 0 TO TRUE DO
    END_FOR
    main := 0;
END_FUNCTION
```

P2 compiler/plc_lowering/src/reference_to_return.rs:722:25: Plain assignment to a REFERENCE TO return variable is dropped silently
`pick := a;` inside `FUNCTION pick : REFERENCE TO INT` is replaced by an `EmptyStatement` in `visit_assignment` (line 722); `--check` passes and the caller reads the zero-initialized `__pick_return_val` store, so a program that expects 42 prints 0. Only `REF=` is rewritten into the return slot assignment (`visit_ref_assignment`, line 691). Expected: a diagnostic, or the same rewrite as for `REF=`.
```
// Reproducible example:
FUNCTION pick : REFERENCE TO INT
VAR_IN_OUT
    a : INT;
END_VAR
    pick := a;    // lowered to EmptyStatement, no diagnostic
END_FUNCTION

FUNCTION main : DINT
VAR
    v : INT := 42;
    r : REFERENCE TO INT;
END_VAR
    r REF= pick(v);
    printf('%d$N', r);    // prints 0, expected 42
END_FUNCTION
```

P2 src/parser.rs:1622:5: RETAIN is accepted on every block kind without a diagnostic
The parser consumes `RETAIN` for every variable block type (parser.rs:1622) and no validation exists for the modifier. `VAR_TEMP RETAIN` in a program becomes an alias to a persistent global, so the temporary keeps its value across calls (prints 1 then 2), and `VAR RETAIN` in a function or method is silently a stack variable (prints 1 then 1). Expected: a diagnostic for `RETAIN` on `VAR_TEMP` blocks and in functions and methods.
```
// Reproducible example:
PROGRAM prg
VAR_TEMP RETAIN
    t : DINT;
END_VAR
    t := t + 1;
    printf('%d$N', t);    // prints 1 then 2; a temp must not persist
END_PROGRAM

FUNCTION f : DINT
VAR RETAIN
    c : DINT;
END_VAR
    c := c + 1;
    f := c;
END_FUNCTION

FUNCTION main : DINT
    prg();
    prg();
    printf('%d$N', f());    // prints 1
    printf('%d$N', f());    // prints 1; RETAIN accepted but has no effect
END_FUNCTION
```

P2 src/lowering/generics.rs:325:17: Generic call whose type parameter is bound by no parameter reaches codegen
`FUNCTION novote<T: ANY_NUM> : T VAR_INPUT x : DINT; END_VAR END_FUNCTION` called as `d := novote(1)` passes `--check`, but codegen aborts with `error: No callable implementation associated to "novote"`. No argument is bound to a parameter of type `T`, so the generic lowerer never fills `generic_map` for `T`, `fully_resolved` is false and the call is left untouched in every pass (generics.rs:325 to 340); the validator reports nothing because the resolver attaches natures to arguments only for built-in generics. Expected: a diagnostic during validation, such as E064 for the unresolved type parameter.
```
// Reproducible example:
FUNCTION novote<T: ANY_NUM> : T
VAR_INPUT
    x : DINT;
END_VAR
END_FUNCTION

FUNCTION main : DINT
VAR
    d : DINT;
END_VAR
    // --check passes; build aborts with
    // error: No callable implementation associated to "novote"
    d := novote(1);
END_FUNCTION
```

P2 src/lowering/calls.rs:164:5: Misleading E032 for an aggregate-returning call in a local initializer
`VAR s : STRING := greet(); END_VAR` reports `E032 this POU takes 1 argument but 0 arguments were supplied` in addition to the correct `E033 Unresolved constant`. The callee's signature gained the in-out result parameter, but variable initializers are not rewritten, so the validator counts the initializer's call against the new signature. The same program with a `DINT`-returning callee reports only E033. Expected: only E033, as for a scalar-returning call.
```
// Reproducible example:
FUNCTION greet : STRING
    greet := 'hi';
END_FUNCTION

FUNCTION main : DINT
VAR
    // plc --check reports:
    // error[E032]: this POU takes 1 argument but 0 arguments were supplied
    // error[E033]: Unresolved constant `s` variable: Call-statement 'greet' in initializer is not constant.
    // expected: only E033
    s : STRING := greet();
END_VAR
    main := 0;
END_FUNCTION
```

P2 compiler/plc_lowering/src/inheritance.rs:425:9: SUPER^ in a member initializer is not lowered inside the generated constructor
`FUNCTION_BLOCK Y EXTENDS X VAR o : DINT := SUPER^.c + 5; END_VAR END_FUNCTION_BLOCK`, with `c` a constant of `X`, is rejected with `error[E033]: Unresolved constant o variable`. The lowered AST shows that the constructor `Y__ctor` generated by the init participant still contains `self.o := SUPER^.c + 5` with an unresolved `Super(derefed)` node, because `visit_super` (inheritance.rs:425) looks up the super class of the POU whose body is walked, and `Y__ctor` has none; the constant evaluator cannot evaluate `SUPER^` either. Expected: a diagnostic that `SUPER` is not allowed in an initializer, or `self.o := self.__X.c + 5` in the constructor.
```
// Reproducible example:
FUNCTION_BLOCK X
VAR CONSTANT
    c : DINT := 3;
END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK Y EXTENDS X
VAR
    o : DINT := SUPER^.c + 5;    // error[E033]: Unresolved constant `o` variable
END_VAR
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    y : Y;
END_VAR
    printf('%d$N', y.o);    // expected 8
END_FUNCTION
```

P2 src/lowering/polymorphism/dispatch/validation.rs:37:15: Assigning a pointer to an interface variable passes `--check` and aborts codegen
`shape := rectPtr;` with `rectPtr : POINTER TO Rect` and `shape : Shape` reports nothing under `--check` and aborts codegen with `error: Could not resolve reference to __itable_Shape___main_rectPtr_instance`. `validate_pou_implements_interface` returns `None` (valid) through the `?` on `index.find_pou(pou_name)` when the right-hand type is not a POU (validation.rs:37), and the assignment is then expanded with an itable instance named after the pointer type. Expected: E126, or a type mismatch diagnostic, from the check.
```
// Reproducible example:
INTERFACE Shape
METHOD area : DINT
END_METHOD
END_INTERFACE

FUNCTION_BLOCK Rect IMPLEMENTS Shape
METHOD area : DINT
    area := 6;
END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    r : Rect;
    rectPtr : POINTER TO Rect;
    shape : Shape;
END_VAR
    rectPtr := ADR(r);
    // --check passes; build aborts with
    // error: Could not resolve reference to __itable_Shape___main_rectPtr_instance
    shape := rectPtr;
    printf('%d$N', shape.area());
END_FUNCTION
```

P2 src/lowering/polymorphism/dispatch/interface.rs:347:9: Interface assignment from a base-typed place binds the base type's itable
`shape := rectPtr^;` with `rectPtr` pointing at a `Square` (which overrides `area`), or `pick := r;` inside a function with `VAR_IN_OUT r : Rect` called with a `Square`, produces a fat pointer whose table is `__itable_Shape_Rect_instance`; `shape.area()` then prints 6 (`Rect.area`) while `rectPtr^.area()` and `shape := sq; shape.area()` print 9 (`Square.area`). The itable instance is chosen from the static type of the right-hand side (`let pou_name = rhs_type.get_name();`, interface.rs:347), while the same call through `POINTER TO Rect` uses the method table of the instance. Expected: the table is derived from the instance at run time, for example through the method table, or the compiler documents that interface conversion is static.
```
// Reproducible example:
INTERFACE Shape
METHOD area : DINT
END_METHOD
END_INTERFACE

FUNCTION_BLOCK Rect IMPLEMENTS Shape
METHOD area : DINT
    area := 6;
END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Square EXTENDS Rect
METHOD area : DINT
    area := 9;
END_METHOD
END_FUNCTION_BLOCK

FUNCTION pick : Shape
VAR_IN_OUT
    r : Rect;
END_VAR
    pick := r;
END_FUNCTION

FUNCTION main : DINT
VAR
    sq : Square;
    rectPtr : POINTER TO Rect;
    shape : Shape;
END_VAR
    rectPtr := ADR(sq);
    shape := rectPtr^;
    printf('%d$N', shape.area());       // prints 6, expected 9
    printf('%d$N', rectPtr^.area());    // prints 9
    shape := pick(sq);
    printf('%d$N', shape.area());       // prints 6, expected 9
    shape := sq;
    printf('%d$N', shape.area());       // prints 9
END_FUNCTION
```

P2 compiler/plc_cfc/src/st.rs:43:19: Syntax errors in the CFC text declaration point at the wrong place in the `.cfc` file
With `doubledOut DINT;` in the `TextDeclaration`, the compiler reports `E006 Missing expected Token KeywordColon or KeywordComma` at `bad_decl.cfc:1:36`, which is inside the XML prolog `<?xml version="1.0" ...?>`, and shows that XML line as the snippet; the follow-on E006 and E007 diagnostics land at `1:41` in the same line. The declaration is lexed as its own `SourceCode` with a `SourceLocationFactory` built from the extracted text, so the offsets are relative to the declaration text, but the diagnostics carry the `.cfc` path and are rendered against the `.cfc` file. Expected: the location is offset to where the declaration text sits in the document, or the snippet is taken from the declaration text.
```
// Reproducible example:
// --- file: bad_decl.cfc ---
// plc --check bad_decl.cfc
// observed: error[E006] at bad_decl.cfc:1:36 with the XML prolog as snippet
// expected: the location of `doubledOut DINT;` inside the TextDeclaration
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="bad_decl">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <bmx:TextDeclaration>PROGRAM bad_decl
VAR
    doubledOut DINT;
END_VAR</bmx:TextDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
```

P2 src/resolver.rs:2430:13: A variable named like an enum type hijacks the cast to that enum
With `VAR color : DINT; c : Color; END_VAR` and `c := Color#Green;`, the base `Color` of the cast is resolved with `ResolvingStrategy::default_scopes()`, variables before types, and finds the local `main.color` because names are case-insensitive. The cast is then typed as `DINT`, the enum qualifier is dropped in the `ReferenceAccess::Cast` branch because the base type is not an enum, `Green` resolves as a bare enum variant, and the validator reports `warning[E091]: Value evaluated at run-time, use an enum variant from Color` on a plain variant. Renaming the variable removes the warning. The generated code is still `store i32 1`. Expected: the base of a cast is resolved as a type only, so the variable does not shadow it and no warning is reported.
```
// Reproducible example:
TYPE Color : (Red, Green, Blue); END_TYPE

FUNCTION main : DINT
VAR
    color : DINT;
    c : Color;
END_VAR
    c := Color#Green; // warning[E091]: Value evaluated at run-time, use an enum variant from `Color`
    printf('%d$N', c); // prints 1, no warning expected
END_FUNCTION
```

P2 src/validation/variable.rs:199:1: A VLA as a function return type is not rejected
`FUNCTION ret : ARRAY[*] OF DINT END_FUNCTION` passes `plc --check` without a diagnostic. The VLA placement check `validate_vla` only runs over variable blocks, and a return type is not a block. A function that uses the return, `ret := v;` with `v : ARRAY[*] OF DINT` as `VAR_IN_OUT`, called as `out := ret(arr)`, then panics in the resolver at `src/resolver.rs:2672` with `internal error: entered unreachable code`, like the other misplaced VLAs. Expected: E044 for a VLA return type.
```
// Reproducible example:
// plc --check
// observed: panic at src/resolver.rs:2672:17, internal error: entered unreachable code
// expected: error[E044] on the VLA return type
FUNCTION ret : ARRAY[*] OF DINT
VAR_IN_OUT
    v : ARRAY[*] OF DINT;
END_VAR
    ret := v;
END_FUNCTION

FUNCTION main : DINT
VAR
    arr : ARRAY[0..2] OF DINT;
    out : ARRAY[0..2] OF DINT;
END_VAR
    out := ret(arr);
END_FUNCTION
```

P2 src/validation/statement.rs:1117:16: A user-defined compare function with local variables is not recognized
`FUNCTION STRING_EQUAL : BOOL VAR_INPUT a, b : STRING; END_VAR VAR pa, pb : POINTER TO BYTE; END_VAR ... END_FUNCTION` in the project does not satisfy the check for `s1 = s2` on strings; the compiler still reports `error[E073]: Missing compare function 'FUNCTION STRING_EQUAL : BOOL VAR_INPUT a,b : STRING; END_VAR ...'`. Removing the `VAR` block makes the error disappear. `compare_function_exists` matches the member list of the implementation against a slice pattern of exactly three entries, two by-value inputs and the return, so any local, temporary, or `{ref}` input makes the pattern fail. Expected: the check looks at the inputs and the return type only.
```
// Reproducible example:
// plc --check main.st (without the stdlib includes, which define their own STRING_EQUAL)
// observed: error[E073]: Missing compare function 'FUNCTION STRING_EQUAL : BOOL VAR_INPUT a,b : STRING; END_VAR ...'
// expected: no diagnostic; removing the VAR block makes E073 disappear
FUNCTION STRING_EQUAL : BOOL
VAR_INPUT
    a : STRING;
    b : STRING;
END_VAR
VAR
    pa : POINTER TO BYTE;
    pb : POINTER TO BYTE;
END_VAR
    pa := ADR(a);
    pb := ADR(b);
    STRING_EQUAL := pa^ = pb^;
END_FUNCTION

FUNCTION main : DINT
VAR
    s1 : STRING := 'a';
    s2 : STRING := 'a';
END_VAR
    IF s1 = s2 THEN
        main := 1;
    END_IF
END_FUNCTION
```

P2 compiler/plc_header_generator/src/header_generator/header_generator_c.rs:730:33: Generated C header names a function block member with the block's name instead of its type
For `PROGRAM prg VAR ch : Child; n : DINT; END_VAR END_PROGRAM`, `plc --generate-headers` writes `typedef struct { Child ch; int32_t n; } prg_type;`, but the function block itself is emitted as `Child_type` (the POU struct name gets the `_type` suffix in `prepare_function_block`) and `Child` is the constructor function `void Child(Child_type* self);`. A C file that includes the header does not compile. The member is produced in `get_transformed_variables_from_variables`: the FB type is a user-generated struct, `get_user_type_variable` returns a `Variable` whose `data_type` is the plain type name, and the alias check pushes it unchanged without appending `_type`. Expected: `Child_type ch;`.
```
// Reproducible example:
// plc main.st --generate-headers --header-output hdr -o main.o
// observed in hdr/main.o.h:
//   typedef struct { uint64_t* __vtable; int32_t x; } Child_type;
//   typedef struct { Child ch; int32_t n; } prg_type;
//   void Child(Child_type* self);
// expected: Child_type ch;
FUNCTION_BLOCK Child
VAR
    x : DINT;
END_VAR
END_FUNCTION_BLOCK

PROGRAM prg
VAR
    ch : Child;
    n : DINT;
END_VAR
END_PROGRAM
```

P2 compiler/plc_header_generator/src/header_generator/header_generator_c.rs:117:9: Generated C header emits structs in declaration order, so a struct that uses a later struct does not compile
For `TYPE Outer : STRUCT inner : Inner; END_STRUCT END_TYPE` declared before `TYPE Inner : STRUCT v : DINT; END_STRUCT END_TYPE`, `plc --generate-headers` writes `typedef struct { Inner inner; } Outer;` and only then `typedef struct { int32_t v; } Inner;`. A C file that includes the header fails with `unknown type name 'Inner'`. Structured Text allows any declaration order, and `prepare_user_types` walks `compilation_unit.user_types` as declared; only the aliases are dependency-sorted (`resolve_alias_dependencies`), structs and function block structs are not. The same happens for a program that contains a function block declared after it. Expected: structs are ordered so that every member type is declared before its use, like the aliases.
```
// Reproducible example:
// plc order.st --generate-headers
// observed in order.h:
//   typedef struct { Inner inner; } Outer;
//   typedef struct { int32_t v; } Inner;
// expected: Inner before Outer
TYPE Outer : STRUCT
    inner : Inner;
END_STRUCT END_TYPE

TYPE Inner : STRUCT
    v : DINT;
END_STRUCT END_TYPE
```

P2 compiler/plc_header_generator/src/header_generator/header_generator_c.rs:705:37: Function pointer typedef in the generated C header is emitted before the structs it refers to
For `VAR_GLOBAL callback : __FPOINTER scale; END_VAR` where `scale` takes a `Point` struct by reference, the header starts with `typedef int32_t (*scale_ptr)(int32_t, Point*);` and defines `typedef struct { ... } Point;` further down. The typedef is pushed into `user_defined_types.aliases`, and the template renders all aliases before enums and structs, so every function pointer whose signature names a user struct or enum refers to a type that C has not seen yet. Expected: function pointer typedefs are rendered after the types they use, or the struct is forward-declared.
```
// Reproducible example:
// plc fp.st --generate-headers
// observed in fp.h: typedef int32_t (*scale_ptr)(Point*); comes before typedef struct { int32_t x; } Point;
// expected: Point first
TYPE Point : STRUCT
    x : DINT;
END_STRUCT END_TYPE

VAR_GLOBAL
    callback : __FPOINTER scale;
END_VAR

FUNCTION scale : DINT
    VAR_INPUT {ref}
        p : Point;
    END_VAR
END_FUNCTION
```

P2 compiler/plc_header_generator/src/header_generator/header_generator_c.rs:723:21: A variable of a named array type repeats the dimensions in the generated C header
With `TYPE Grid : ARRAY[0..1, 0..2] OF DINT; END_TYPE` and `VAR_GLOBAL grid : Grid; END_VAR`, the header contains `typedef int32_t Grid[2][3];` and `extern Grid grid[2][3];`. The typedef already carries the dimensions, so the extern declares a two-dimensional array of `Grid`, a different type from the one codegen emits. The array alias is resolved through `get_user_type_variable`, which returns the member as `MultidimensionalArray` with the sizes, and the alias branch at line 723 keeps that `variable_type` while it swaps the data type to the alias name `Grid`. Expected: `extern Grid grid;`. The same applies to a struct member of a named array type and to a `STRING` alias.
```
// Reproducible example:
// plc grid.st --generate-headers
// observed in grid.h: extern Grid grid[2][3];
// expected: extern Grid grid;
TYPE Grid : ARRAY[0..1, 0..2] OF DINT; END_TYPE

VAR_GLOBAL
    grid : Grid;
END_VAR
```

P3 compiler/plc_lexer/src/lexer.rs:229:17: Two diagnostics for one missing closing token
`x := (2 * 3;` produces `E006 Missing expected Token [KeywordParensClose]` and `E007 Unexpected token: expected KeywordParensClose but found ';'` at the same position `5:16`. `recover_until_close` reports the missing token when it finds that the current token closes an outer region (line 229), then `close_region` reports the same token as unexpected (line 176). Expected: one diagnostic.
```
// Reproducible example:
// plc --check
// observed: main.st:5:16: error[E006]: Missing expected Token [KeywordParensClose]
//           main.st:5:16: error[E007]: Unexpected token: expected KeywordParensClose but found ';'
// expected: one diagnostic
FUNCTION main : DINT
VAR
    x : DINT;
END_VAR
    x := (2 * 3;
END_FUNCTION
```

P3 src/index.rs:343:29: Hardware address segments are registered with the type name "32"
When a variable is declared with a hardware address such as `sensor AT %IX0.0 : BOOL`, `HardwareBinding::from_statement` stores each address segment as a constant expression whose target type is the integer constant `DINT_SIZE` converted to a string, so the arena entry reads `target_type=32`. The intent is the type name `DINT`. Because "32" is not a type, `find_effective_type_by_name` returns `None` in the constant evaluator and the overflow check is skipped for these segments: `sensor AT %IX99999999999.0 : BOOL` compiles without a diagnostic and emits `@__PI_99999999999_0`, while `x : DINT := 99999999999;` reports `warning[E039]: This will overflow for type DINT`. Expected: `DINT_TYPE`.
```
// Reproducible example:
// plc --check
// observed: no diagnostic; IR contains @__PI_99999999999_0
// expected: E039 overflow warning for the segment, as for a DINT initializer
PROGRAM main
VAR
    sensor AT %IX99999999999.0 : BOOL;
    x : DINT := 99999999999; // warning[E039]: This will overflow for type DINT
END_VAR
END_PROGRAM
```

P3 src/lowering/property.rs:174:13: Nested property set target produces a misleading follow-on error
`shapeInstance.position.x := 1`, where `position` is a property of type `Point`, correctly reports `E128 Properties can only be assigned as a whole, not through member or index access`, but is followed by `E048 Could not resolve reference to x` at the same position, although `x` is a member of `Point`. The property lowerer pushes the E128 diagnostic and returns without rewriting the target (line 174), while the property declaration was already replaced by `__get_position` and `__set_position` methods in `pre_index`. The re-annotation after lowering therefore no longer finds a member `position` on `Shape`, and the validator reports the unresolved `x`. Expected: only E128.
```
// Reproducible example:
// plc --check
// observed: main.st:22:28: error[E128]: Properties can only be assigned as a whole, not through member or index access
//           main.st:22:28: error[E048]: Could not resolve reference to x
// expected: only E128
TYPE Point : STRUCT
    x : DINT;
    y : DINT;
END_STRUCT END_TYPE

FUNCTION_BLOCK Shape
VAR
    pos : Point;
END_VAR
PROPERTY_GET position : Point
    position := pos;
END_PROPERTY
PROPERTY_SET position : Point
    pos := position;
END_PROPERTY
END_FUNCTION_BLOCK

PROGRAM main
VAR
    shapeInstance : Shape;
END_VAR
    shapeInstance.position.x := 1;
END_PROGRAM
```

P3 compiler/plc_project/src/project.rs:262:82: Nonexistent library search path panics the compiler
`plc -L /does/not/exist -o out main.st` aborts with `thread 'main' panicked at compiler/plc_project/src/project.rs:262:82: called Result::unwrap() on an Err value: path '/does/not/exist' does not exist` and a Rust backtrace hint instead of a diagnostic. `Project::with_library_paths` resolves the `-L` paths relative to the project location with `resolve_file_paths` and unwraps the result. Expected: a diagnostic that names the missing path, or the path passed through to the linker, which reports a missing directory itself.
```
// Reproducible example:
// plc -L /does/not/exist -o out main.st
// observed: panic at compiler/plc_project/src/project.rs:262:82, exit code 101
// expected: a diagnostic naming the missing path
FUNCTION main : DINT
    main := 0;
END_FUNCTION
```

P3 compiler/plc_lowering/src/reference_to_return.rs:219:56: Alias-typed REFERENCE TO return is not lowered
`TYPE IntRef : REFERENCE TO INT; END_TYPE FUNCTION pick : IntRef` is not recognized by `ReferenceToReturnContextGatherer::visit_pou`, because only an inline `DataTypeDeclaration::Definition` with `PointerType { auto_deref: Some(Reference) }` matches and a `DataTypeDeclaration::Reference` to the alias falls into the `_ => false` arm. The function keeps a pointer return and `r REF= pick(v)` fails with `E098 Invalid assignment, expected a reference`, while the inline spelling `FUNCTION pick : REFERENCE TO INT` compiles and runs. Expected: both spellings behave the same.
```
// Reproducible example:
// plc --check
// observed: main.st:15:12: error[E098]: Invalid assignment, expected a reference
// expected: compiles, like with `FUNCTION pick : REFERENCE TO INT`
TYPE IntRef : REFERENCE TO INT; END_TYPE

FUNCTION pick : IntRef
VAR_IN_OUT
    v : INT;
END_VAR
    pick REF= v;
END_FUNCTION

FUNCTION main : DINT
VAR
    v : INT := 7;
    r : REFERENCE TO INT;
END_VAR
    r REF= pick(v);
    printf('%d$N', r);
END_FUNCTION
```

P3 src/validation/pou.rs:75:12: Interface named like a builtin function is validated as a POU and gets E112
`INTERFACE Sub EXTENDS Shape END_INTERFACE` fails with `error[E112]: Method 'area' defined in interface 'Shape' is missing in POU 'Sub'` even when a function block implements `Sub` completely; renaming the interface to `SubShape` compiles. The duplicate-method check treats a container as a POU when `context.index.find_pou(container_name)` succeeds, and the builtin function `SUB` matches the case-insensitive lookup. Expected: interface names are not resolved against POUs in this check, or a name clash with a builtin is reported as such.
```
// Reproducible example:
// plc --check
// observed: main.st:6:11: error[E112]: Method `area` defined in interface `Shape` is missing in POU `Sub`
// expected: no diagnostic (renaming Sub to SubShape compiles)
INTERFACE Shape
    METHOD area : DINT
    END_METHOD
END_INTERFACE

INTERFACE Sub EXTENDS Shape
END_INTERFACE

FUNCTION_BLOCK Sq IMPLEMENTS Sub
    METHOD area : DINT
        area := 4;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    s : Sq;
END_VAR
    printf('%d$N', s.area());
END_FUNCTION
```

P3 compiler/plc_driver/src/pipelines.rs:715:17: A `.cfc` file given as include is parsed as Structured Text
`plc --check main.st myVoid.st -i function_void.cfc` fails with `E007: Unexpected token: expected StartKeyword but found <` at `function_void.cfc:1:1`, followed by one E007 per XML token. The same file given as a positional source compiles. The source type dispatch (`source.get_type()` selecting `parse_file` or `plc_cfc::parse_file`) exists only for the positional sources at line 697; the include loop at line 715 and the library include loop at line 727 always call the text parser. Expected: includes use the same dispatch, or a diagnostic says that CFC includes are not supported.
```
// Reproducible example:
// plc --check main.st myVoid.st -i function_void.cfc
// observed: error[E007]: Unexpected token: expected StartKeyword but found < (function_void.cfc:1:1) and more
// expected: the same result as `plc --check main.st myVoid.st function_void.cfc`
// --- file: main.st ---
FUNCTION main : DINT
    function_void();
END_FUNCTION

// --- file: myVoid.st ---
FUNCTION myVoid
    VAR_INPUT
        in : DINT;
    END_VAR
    VAR_OUTPUT
        out : DINT;
    END_VAR
    out := in * 2;
END_FUNCTION

// --- file: function_void.cfc ---
// any valid CFC file, for example tests/lit/cfc/blocks/function_void/function_void.cfc
```

P3 src/validation/statement.rs:1671:12: Assigning one VLA to another reports an assignment between identical types
For `values := other;` where both are `ARRAY[*] OF DINT` parameters of the same function, the diagnostic reads `error[E037]: Invalid assignment: cannot assign 'ARRAY[*] OF DINT' to 'ARRAY[*] OF DINT'`. The VLA branch (`left_type.is_vla() && right_type.is_array() && context.is_call()`) only accepts the pair inside a call and otherwise falls through to the generic invalid-assignment message, which names the same type on both sides and gives the user no hint why the assignment is rejected. Expected: a message that says VLAs cannot be assigned inside a POU body, or that the assignment is only valid as a call argument.
```
// Reproducible example:
// plc --check main.st
// observed: main.st:6:5: error[E037]: Invalid assignment: cannot assign 'ARRAY[*] OF DINT' to 'ARRAY[*] OF DINT'
// expected: a message that explains that VLA assignments are only valid as call arguments
FUNCTION copy : DINT
    VAR_IN_OUT
        values : ARRAY[*] OF DINT;
        other : ARRAY[*] OF DINT;
    END_VAR
    values := other;
END_FUNCTION

FUNCTION main : DINT
    VAR
        a : ARRAY[1..3] OF DINT;
        b : ARRAY[1..3] OF DINT;
    END_VAR
    copy(a, b);
END_FUNCTION
```

P3 src/index.rs:1564:21: An unqualified enum variant shared by two enums resolves to the first declared enum without an ambiguity diagnostic
With `TYPE Door : (Open := 8, Closed := 16); END_TYPE`, `TYPE State : (Open := 1, Closed := 4) BYTE; END_TYPE`, and `VAR_GLOBAL g : State; END_VAR`, the statement `g := Closed;` in a program stores `16` (`Door.Closed`) and reports `warning[E040]: Non-standard enum value Closed for State` plus `warning[E067]: Implicit downcast from 'Door' to 'State'`. `find_qualified_global_variable` falls back to `enum_global_variables.get_all(name)` and returns `it.first()`, which is declaration order; the target type is not consulted. Inside a POU that declares a variable of type `State`, the same `g := Closed;` stores `4` because the member lookup's enum fallback (`find_enum_variant_in_pou`) resolves to `State.Closed`, so the result depends on which unrelated variables the POU declares. Expected: prefer the variant of the target's type when one exists, or report the name as ambiguous and ask for a qualifier.
```
// Reproducible example:
// observed: prints 16, 4, 4 (with E040 and E067 warnings on line 9 only)
// expected: 4, 4, 4 or an ambiguity diagnostic for the unqualified `Closed`
TYPE Door : (Open := 8, Closed := 16); END_TYPE
TYPE State : (Open := 1, Closed := 4) BYTE; END_TYPE

VAR_GLOBAL
    g : State;
END_VAR

PROGRAM prog
    g := Closed;
END_PROGRAM

PROGRAM prog2
    VAR
        s : State;
    END_VAR
    g := Closed;
    s := Closed;
END_PROGRAM

FUNCTION main : DINT
    prog();
    printf('%d$N', g);
    prog2();
    printf('%d$N', g);
    printf('%d$N', prog2.s);
END_FUNCTION
```

P3 compiler/plc_header_generator/src/header_generator/file_helper/file_helper_c.rs:27:9: A combined header named with an extension gets a second extension
`plc --generate-headers a.st -o api.h` writes `api.h.h`, and `-o api` writes `api.h`. `combine_generated_headers` passes the `-o` value to `set_path`, which always appends `.h`, while the per-file path built by `get_header_file_information` strips the source extension first. The header guard is derived from the doubled name as well (`API_H_H_`). Expected: an existing `.h` extension is kept once, like the per-file case.
```
// Reproducible example:
// plc a.st --generate-headers -o api.h
// observed: api.h.h with guard API_H_H_
// expected: api.h
FUNCTION f : DINT
END_FUNCTION
```

P3 compiler/plc_header_generator/src/lib.rs:16:5: The header generator accepts --include-stubs but never reads it
`plc generate plc.json headers --include-stubs` is accepted, the flag is copied into `GenerateHeaderOptions::include_stubs`, and no code reads the field; the output is identical with and without it. The help text promises "generated code stubs for the library". Expected: either stubs are generated, or the flag is removed from the command line.
```
// Reproducible example:
// plc generate plc.json headers --include-stubs   (any project)
// observed: same header as without the flag, no stub file
// expected: a stub source file, or an error that the option is not supported
FUNCTION f : DINT
END_FUNCTION
```

P4 compiler/plc_diagnostics/src/diagnostics.rs:171:13: Plain-text diagnostic summary uses zero-based line and column
The "Details:" list printed after "Compilation aborted due to critical parse errors" shows `broken.st:4:12:{4:12-4:13}` for the error that the rich renderer shows at `broken.st:5:13` and that `--error-format clang` shows at `broken.st:5:13`. The `Display` impl of `Diagnostic` writes ` at: {location}`, and the `Display` impl of `CodeSpan::Range` (`compiler/plc_source/src/source_location.rs:117`) prints the raw internal zero-based line and column; the rich and clang renderers add one to both. Expected: all forms agree, one-based.
```
// Reproducible example:
// plc --check broken.st
// observed: rich diagnostic at broken.st:5:13, then
//           "Details: Unexpected token: expected expression but found ; at: broken.st:4:12:{4:12-4:13}:"
// expected: the Details line also reads broken.st:5:13
FUNCTION main : DINT
    VAR
        x : DINT;
    END_VAR
    x := 1 +;
END_FUNCTION
```

P4 src/resolver.rs:303:9: Call arguments are annotated twice
`visit_call_statement` visits the argument list at line 2703 (`self.visit_statement(&ctx, parameters)`) and then calls `annotate_arguments`, whose first statement at line 303 (`self.visit_statement(ctx, arguments_node)`) visits the same argument list again with the same context. The builtin annotation functions in `builtins.rs` (lines 864 and 953) also call `annotate_arguments` after the visit at line 2703, so every argument expression is resolved and annotated twice per call. The second pass is idempotent (the map insert overwrites the same value, string literals go into a set), so the output is correct, but the work is doubled for every call in the project and the annotate stage already runs once per participant. Expected: one visit.
```
// Reproducible example:
// No observable difference in output; the double visit is visible in src/resolver.rs
// (visit at line 2703, then annotate_arguments visits again at line 303).
FUNCTION scale : DINT
    VAR_INPUT
        value : DINT;
        factor : DINT;
    END_VAR
    scale := value * factor;
END_FUNCTION

FUNCTION main : DINT
    VAR
        i : DINT;
        small : DINT := 2;
    END_VAR
    i := scale(3, factor := small);
END_FUNCTION
```

P4 src/codegen/generators/pou_generator.rs:146:40: Initializer constant for a POU member is emitted twice
For `PROGRAM main VAR values : ARRAY[1..3] OF DINT := [1, 2, 3]; END_VAR END_PROGRAM`, `plc --ir` contains both `@__main.values__init` and `@__main.values__init.1` with the same value `[i32 1, i32 2, i32 3]`. `generate_global_constants_for_pou_members` iterates the unit's dependencies, which contain `main` twice (once as `Dependency::Datatype` from its declaration and once as `Dependency::Call` from the reference to its instance in the generated constructor), and the guard `llvm_index.find_global_value(&name).is_none()` checks the module index that was passed in, not `local_llvm_index` into which the first constant was just inserted, so LLVM renames the second global with a `.1` suffix. Expected: one constant per initialized member.
```
// Reproducible example:
// plc main.st --ir -o -
// observed:
//   @__main.values__init = unnamed_addr constant [3 x i32] [i32 1, i32 2, i32 3]
//   @__main.values__init.1 = unnamed_addr constant [3 x i32] [i32 1, i32 2, i32 3]
// expected: only @__main.values__init
PROGRAM main
    VAR
        values : ARRAY[1..3] OF DINT := [1, 2, 3];
    END_VAR
END_PROGRAM
```

P4 src/codegen.rs:534:13: Objects for an explicit target are placed in a doubly nested target directory
`plc --target x86_64-unknown-linux-gnu --build-location tb -c -o tb/prog.o scale.st prog.st` writes the intermediate object to `tb/x86_64-unknown-linux-gnu/x86_64-unknown-linux-gnu/scale.st.o`. The codegen participant (`compiler/plc_driver/src/pipelines/participant.rs:118`) and `ensure_compile_dirs` (`pipelines.rs:1093`) already append the target name to the build location when they create the compile directory, and `GeneratedModule::get_output_file` at `codegen.rs:534` appends `target.try_get_name()` a second time when it builds the output path. Expected: `tb/x86_64-unknown-linux-gnu/scale.st.o`.
```
// Reproducible example:
// plc --target x86_64-unknown-linux-gnu --build-location tb -c -o tb/prog.o scale.st prog.st
// observed: tb/x86_64-unknown-linux-gnu/x86_64-unknown-linux-gnu/scale.st.o
// expected: tb/x86_64-unknown-linux-gnu/scale.st.o
// --- file: scale.st ---
FUNCTION scale : DINT
    VAR_INPUT
        x : DINT;
    END_VAR
    scale := x * 2;
END_FUNCTION

// --- file: prog.st ---
FUNCTION main : DINT
    main := scale(2);
END_FUNCTION
```

P4 compiler/plc_ast/src/pre_processor.rs:41:5: Generic parameter types are declared again on every index round
Pre-processing appends a `UserTypeDeclaration` for every type parameter of a generic POU (`unit.user_types.extend(generic_types)` at line 41) each time it runs, and it runs in every index round that a participant triggers. After the pipeline, `plc --check main.st --ast-lowered` for a unit with one template `times_two<T>` prints ten identical `__times_two__T` entries in the unit's user types (the exact count depends on the number of index rounds). The index merges them by name, so the output is correct. Expected: one declaration.
```
// Reproducible example:
// plc --check main.st --ast-lowered | grep -c 'name: "__times_two__T"'
// observed: 10
// expected: 1
FUNCTION times_two<T: ANY_INT> : T
    VAR_INPUT
        x : T;
    END_VAR
    times_two := x * 2;
END_FUNCTION

FUNCTION main : DINT
    main := times_two(DINT#4);
END_FUNCTION
```

P4 compiler/plc_lowering/src/array_lowering.rs:72:1: Module documentation describes stripping that another participant does
The "Side effects" section of the array lowerer's module documentation (lines 72 to 78) says the pass strips the original initializer from the variable or user-type declaration. The init participant does that (`strip_non_const_array_initializers`, called at `compiler/plc_lowering/src/initializer.rs:650`, defined at line 1052); the array lowerer never removes declaration initializers, it only rewrites `(N)(value)` calls into `MultipliedStatement` nodes inside them (`rewrite_const_multiplied_initializers`). Expected: the documentation names the init participant.
```
// Reproducible example:
// Documentation only. array_lowering.rs contains no code that sets a declaration
// initializer to None; the strip lives in initializer.rs.
PROGRAM main
    VAR
        values : ARRAY[1..3] OF DINT := [3(1)];
    END_VAR
END_PROGRAM
```

P4 src/codegen/generators/pou_generator.rs:877:8: Stack variables with an initializer are stored twice per call
For `FUNCTION pick : DINT VAR local : DINT := 5; END_VAR ... END_FUNCTION`, and likewise for a `VAR_TEMP t : DINT := 4;` in a program, `plc --ir` shows `store i32 5, ptr %local` twice at the start of `@pick` and `store i32 4, ptr %t` twice at the start of `@prog`. `generate_initialization_of_local_vars` writes the constant initial value of every local, temp, and return variable when it sets up the stack slots, and the init participant has already prepended a `local := 5` assignment to the same body for the same variable (visible with `--ast-lowered`). The value is the same, so the result is correct, but every initialized stack variable costs two stores per call. Expected: one of the two mechanisms owns stack initialization.
```
// Reproducible example:
// plc main.st --ir -o -
// observed in @pick: "store i32 5, ptr %local" twice; in @prog: "store i32 4, ptr %t" twice
// expected: one store per variable
FUNCTION pick : DINT
    VAR
        local : DINT := 5;
    END_VAR
    pick := local;
END_FUNCTION

PROGRAM prog
    VAR_TEMP
        t : DINT := 4;
    END_VAR
    t := t + 1;
END_PROGRAM

FUNCTION main : DINT
    main := pick();
    prog();
END_FUNCTION
```

P4 src/index/indexer/user_type_indexer.rs:294:13: The default-instance globals in the index are never read
The indexer registers a `__<Type>__init` entry in the `global_initializers` map for every struct, array with an initializer, string with an initializer, program, function block, and class (`register_global_initializer` at `user_type_indexer.rs:294`, `:439`, `:534` and `pou_indexer.rs:158`, `:181`). Outside of `src/index/tests/index_tests.rs` nothing calls `find_global_initializer` or `get_global_initializers`; the map is only filled, merged in `Index::import`, and looked up by those two accessors. Codegen computes default values from the type index and the generated constructors, and `plc --ir` for a project with a struct `Point` and a function block `Counter` contains no `@__Point__init` or `@__Counter__init` global. Expected: the map is removed together with its registration, or codegen uses it.
```
// Reproducible example:
// plc main.st --ir -o - | grep '^@'
// observed: only @__vtable_Counter_instance and @llvm.global_ctors; no @__Point__init, no @__Counter__init
TYPE Point : STRUCT
    x : DINT := 1;
    y : DINT := 2;
END_STRUCT END_TYPE

FUNCTION_BLOCK Counter
    VAR
        n : DINT := 3;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        p : Point;
        c : Counter;
    END_VAR
    main := p.x + c.n;
END_FUNCTION
```

---

## Needs human input

Entries in this section were not confirmed. Each one starts with a `Review:` line that says what was observed and why the entry needs a decision. Move an entry back into the main section once it is confirmed, or delete it.

Review (unclear): The symptom reproduces exactly (13 for the direct call, 3 or garbage for the interface call, codegen abort when the default is on the interface). The stated cause is partly wrong: E032 is never reported for methods, `validate_argument_count` has `_ => false` for everything except functions, programs, and function blocks, so a direct call `ad.compute(a := 3)` with no default compiles too and prints garbage. The interface path differs only because it takes the parameter list, and hence the defaults, from the interface method. Whether E032 must be raised for methods, or the implementation default must be honoured, is a design decision.

P0 src/lowering/polymorphism/dispatch/interface.rs:369:5: Omitted input argument in an interface method call passes an uninitialized value
`INTERFACE Calc METHOD compute : DINT VAR_INPUT a : DINT; b : DINT; END_VAR END_METHOD END_INTERFACE`, implemented by `Adder` with `b : DINT := 10`, called as `c.compute(a := 3)` through `c : Calc`, passes `--check` and prints `3` at `-O none` and a garbage value at the default level, while the direct call `ad.compute(a := 3)` prints `13`. The IR of the interface call reads `%1 = alloca i32` followed by `%2 = load i32, ptr %1` and passes `%2` as the second argument of the indirect call. The dispatch lowerer rewrites the call into a call through the method table entry; codegen then builds the argument list from the interface method's parameters (`src/codegen/generators/expression_generator.rs:715`), which carry no default, and fills the missing one with a fresh stack slot. The arity check `validate_argument_count` (`src/validation/statement.rs:2695`) only counts arguments for functions, programs, and function blocks, so no method call, direct or via interface, reports E032 for an omitted input; a direct method call without a default also passes an uninitialized value. Declaring the default on the interface method instead aborts codegen with `no type hint available for 10`. Expected: E032 for the omitted argument, or the implementation's default value.
```
// Reproducible example:
INTERFACE Calc
    METHOD compute : DINT
    VAR_INPUT
        a : DINT;
        b : DINT;
    END_VAR
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK Adder IMPLEMENTS Calc
    METHOD compute : DINT
    VAR_INPUT
        a : DINT;
        b : DINT := 10;
    END_VAR
        compute := a + b;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    ad : Adder;
    c : Calc;
END_VAR
    c := ad;
    printf('%d$N', ad.compute(a := 3));
    // prints 13
    printf('%d$N', c.compute(a := 3));
    // prints 3 at -O none, garbage at the default level; expected 13 or E032
END_FUNCTION
```

Review (unclear): The segfault reproduces only with `-O none`. With the default optimization level (and `-O less`) the binary runs to completion, because LLVM hoists or removes the alloca. The IR analysis is correct: `--ir` shows `%6 = alloca i32` inside the loop block. The P0 severity should be reconsidered since a default build is not affected.

P0 src/codegen/generators/expression_generator.rs:1382:36: Temporary for an expression passed to a by-reference parameter is allocated inside the loop and exhausts the stack
`FOR i := 1 TO 3000000 DO acc := acc + refsum(i + 1); END_FOR` with `FUNCTION refsum : DINT VAR_INPUT {ref} a : DINT; END_VAR` segfaults with the default 8 MB stack when compiled with `-O none`; the same loop with a plain variable argument runs, and the default optimization level hides the problem because LLVM hoists the slot. The IR of `main` has `%6 = alloca i32` in the loop block `continue2`, not in `entry`: when the argument of a by-reference parameter is not an lvalue, codegen allocates a stack slot at the current insert position and stores the value there, so every iteration allocates a new slot that is only released when the function returns. Every call with a computed argument for a `VAR_INPUT {ref}` parameter grows the stack per call. Expected: the temporary is allocated once in the entry block, or the slot is released with lifetime markers as the aggregate-return temporaries are.
```
// Reproducible example:
// plc main.st ... -O none; segfaults at runtime, prints -1121226208 with -O default
FUNCTION refsum : DINT
VAR_INPUT {ref}
    a : DINT;
END_VAR
    refsum := a;
END_FUNCTION

FUNCTION main : DINT
VAR
    i : DINT;
    acc : DINT;
END_VAR
    FOR i := 1 TO 3000000 DO
        acc := acc + refsum(i + 1);
    END_FOR
    printf('%d$N', acc);
END_FUNCTION
```

Review (unclear): The segfault reproduces only with `-O none`. With the default optimization level the binary runs and prints 3000000. The IR analysis is correct: `%vla_struct` and `%vla_struct_ptr` are allocated in the loop block `continue2`, and the source already carries an XXX comment about this alloca per call. The P0 severity should be reconsidered since a default build is not affected.

P0 src/codegen/llvm_typesystem.rs:476:25: VLA wrapper struct for a fixed array argument is allocated inside the loop and exhausts the stack
`FOR i := 1 TO 3000000 DO acc := acc + vsum(arr); END_FOR` with `FUNCTION vsum : DINT VAR_IN_OUT a : ARRAY[*] OF DINT; END_VAR` and `arr : ARRAY[0..2] OF DINT` segfaults with the default 8 MB stack when compiled with `-O none`; the default optimization level hides the problem because LLVM hoists the slots. The IR of `main` has `%vla_struct = alloca %__vsum_a` and `%vla_struct_ptr = alloca %__vsum_a` in the loop block `continue2`. The cast that wraps a fixed array into the VLA struct allocates the struct at the current insert position on every call (lines 416 and 476), so a loop leaks two slots per iteration until the function returns. Expected: the wrapper slots are allocated once in the entry block and reused, or released with lifetime markers.
```
// Reproducible example:
// plc main.st ... -O none; segfaults at runtime, prints 3000000 with -O default
FUNCTION vsum : DINT
VAR_IN_OUT
    a : ARRAY[*] OF DINT;
END_VAR
    vsum := a[0];
END_FUNCTION

FUNCTION main : DINT
VAR
    i : DINT;
    acc : DINT;
    arr : ARRAY[0..2] OF DINT := [1, 2, 3];
END_VAR
    FOR i := 1 TO 3000000 DO
        acc := acc + vsum(arr);
    END_FOR
    printf('%d$N', acc);
END_FUNCTION
```

Review (unclear): The symptom reproduces exactly, but the cause is elsewhere. The flat literal escapes E033 because the init participant detects it as a non-constant array literal and moves it into the constructor before the constant evaluator sees it. That detection uses `is_const_expression` in compiler/plc_lowering/src/helper.rs, which returns true for any `Literal`, including a nested array literal, without recursing into its elements. So the nested initializer stays in the declaration and the constant evaluator reports it. The original location `const_evaluator.rs:1:1` was a placeholder; the entry now points at the helper.

P1 compiler/plc_lowering/src/helper.rs:12:9: Nested array literal with a runtime element is rejected as an unresolved constant
`nested : ARRAY[0..1] OF ARRAY[0..1] OF DINT := [[seed, 1], [2, seed]]` aborts with `error[E033]: Unresolved constant 'nested' variable: 'seed' is no const reference`, while the flat literal `flat : ARRAY[0..1] OF DINT := [seed, 1]` with the same element compiles and prints 7. The init participant moves an array initializer into the constructor only when `is_const_expression` says its elements are not constant; that helper returns true for every `Literal` node, so an inner array literal counts as constant whatever its elements contain. The nested initializer therefore stays in the declaration, and the constant evaluator, which does recurse into the inner literal, reports the runtime element. Expected: the nested literal is handled like the flat one; `is_const_expression` must recurse into array literals.
```
// Reproducible example:
// plc --check: error[E033]: Unresolved constant `nested` variable: `seed` is no const reference
// without `nested` the program compiles and prints 7
FUNCTION main : DINT
VAR
    seed : DINT := 7;
    flat : ARRAY[0..1] OF DINT := [seed, 1];
    nested : ARRAY[0..1] OF ARRAY[0..1] OF DINT := [[seed, 1], [2, seed]];
END_VAR
    printf('%d %d$N', flat[0], nested[1][1]);
END_FUNCTION
```

Review (unclear): Reproduces (THIS^.name() returns 1, name() returns 2). The behavior is enforced on purpose: the comment at the top of is_polymorphic_call_candidate says THIS^.foo() must not be lowered, and the unit test this_calls_are_untouched in the same file asserts it. Only the inner comment of Case 1 says the opposite. IEC 61131-3 binds calls through THIS dynamically (only SUPER is static), so the project has to decide whether to change the design and the test, or to fix the inner comment.

P1 src/lowering/polymorphism/dispatch/pou.rs:241:9: `THIS^.method()` is a direct call while `method()` in the same body is dispatched
`METHOD viaThis : DINT viaThis := THIS^.name(); END_METHOD` in `Base` returns `1` for a `Child` instance that overrides `name` to return `2`; the bare call `name()` in a sibling method returns `2`. The candidate check returns early for `THIS` and `THIS^` operators and for bases that are `THIS^` (the recursion for `Member` with a base hits `is_this_deref`), and the check for calls inside methods tests the base with `is_this`, which is false for the deref node `THIS^`, so `THIS^.foo()` never matches although the comment of that check says it wants to accept it. The top comment of the function and the unit test `this_calls_are_untouched` state the opposite, that `THIS^.foo()` stays untouched. Expected: both spellings dispatch through the method table, as IEC 61131-3 binds calls through `THIS` dynamically, or the difference is documented and the contradicting comment is removed.
```
// Reproducible example:
FUNCTION_BLOCK Base
    METHOD name : DINT
        name := 1;
    END_METHOD
    METHOD viaThis : DINT
        viaThis := THIS^.name();
    END_METHOD
    METHOD viaBare : DINT
        viaBare := name();
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Child EXTENDS Base
    METHOD name : DINT
        name := 2;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        child : Child;
    END_VAR
    printf('%d$N', child.viaThis());    // prints 1, expected 2
    printf('%d$N', child.viaBare());    // prints 2
END_FUNCTION
```

Review (unclear): Reproduces in part. The E048 for the undeclared name is dropped as described, but the build does not end silently: it prints a misleading `error[E037]: Invalid assignment: cannot assign 'rsion' to 'DINT'` at `ghost.cfc: Block 3`, where 'rsion' is a slice of the .cfc text at the offsets of the detached identifier location. The cause (inner identifier keeps the internal location) matches; the observed output in the entry is wrong.

P1 compiler/plc_cfc/src/st.rs:22:5: Undeclared name in a CFC source or sink loses its E048 and produces a garbage type name
A `DataSource` with `identifier="ghost"`, or a `DataSink` naming a variable the POU does not declare, ends with `error[E037]: Invalid assignment: cannot assign 'rsion' to 'DINT'` at `ghost.cfc: Block 3` followed by `error: Compilation aborted due to critical errors.`, both under `--check` and when building; no `E048` names the undeclared identifier, and `'rsion'` is a slice of the `.cfc` file text taken at the offsets of the parsed identifier. `parse_expression_at` relocates only the outer node of the parsed identifier to the block's location; the identifier node inside keeps the location of the detached text, so the E048 the resolver reports on it has no location the reporter can render and is dropped, and the type name derived from that location is garbage. Expected: `E048` for the undeclared name at `Block <id>` of the element, and no E037 with a garbage type name.
```
// Reproducible example:
// plc --check main.st ghost.cfc ...
// observed: error[E037]: Invalid assignment: cannot assign 'rsion' to 'DINT' (ghost.cfc: Block 3), no E048
// expected: error[E048]: Could not resolve reference to ghost (ghost.cfc: Block 1)
// --- file: main.st ---
FUNCTION main : DINT
    ghost();
    printf('%d$N', ghost.x);
END_FUNCTION

// --- file: ghost.cfc ---
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ppx:Program xmlns:ppx="www.iec.ch/public/TC65SC65BWG7TF10" xmlns:rxt="www.iec.ch/public/TC65SC65BWG7TF10/Recommendation" name="ghost">
    <ppx:AddData>
        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
            <bmx:TextDeclaration>PROGRAM ghost
VAR
    x : DINT;
END_VAR</bmx:TextDeclaration>
        </ppx:Data>
    </ppx:AddData>
    <ppx:MainBody>
        <ppx:BodyContent xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ppx:FBD">
            <ppx:Network>
                <ppx:FbdObject xsi:type="ppx:DataSource" identifier="ghost" globalId="1">
                    <ppx:RelPosition x="250" y="140"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointOut connectionPointOutId="2">
                        <ppx:RelPosition x="80" y="10"/>
                    </ppx:ConnectionPointOut>
                </ppx:FbdObject>
                <ppx:FbdObject xsi:type="ppx:DataSink" identifier="x" globalId="3">
                    <ppx:AddData>
                        <ppx:Data name="http://www.bachmann.at/xml/PLC" handleUnknown="implementation">
                            <EvaluationPriority priorityInNetwork="0"/>
                        </ppx:Data>
                    </ppx:AddData>
                    <ppx:RelPosition x="400" y="140"/>
                    <ppx:Size x="80" y="20"/>
                    <ppx:ConnectionPointIn>
                        <ppx:RelPosition x="0" y="10"/>
                        <ppx:Connection refConnectionPointOutId="2"/>
                    </ppx:ConnectionPointIn>
                </ppx:FbdObject>
            </ppx:Network>
        </ppx:BodyContent>
    </ppx:MainBody>
</ppx:Program>
```

Review (unclear): Reproduces only for the FOR bound. `CASE xof(mk(3)) OF` compiles and runs correctly; the lowered AST shows the `__mk` temporary placed in front of the CASE. The FOR case fails with `Could not resolve reference to __mk2`. The cause in the entry is wrong: `visit_control_statement` does walk the CASE selector and the FOR bound. The loop desugarer runs first and copies the bound expression with `end.clone()` into two exit tests (same AST ids); the aggregate lowerer then allocates a temporary in front of each exit test, but the codegen cannot resolve one of them. The exact mechanism (likely the shared AST ids of the cloned bound colliding in the annotation map) still needs confirmation.

P1 src/lowering/calls.rs:487:13: Aggregate-returning call as a FOR bound aborts codegen
`FOR i := 1 TO xof(mk(4)) DO ... END_FOR`, where `mk` returns a struct and `xof` reads its member, passes `--check` but aborts codegen with `error: Could not resolve reference to __mk2` at the location of the bound. The same call in an `IF`, `ELSIF`, `WHILE`, or `REPEAT UNTIL` condition, in a `CASE` selector, in an assignment, nested inside another call, or in a hand-written `WHILE TRUE DO IF i > xof(mk(4)) THEN EXIT; END_IF; ... END_WHILE` is lowered correctly. The loop desugarer runs before the aggregate-return lowerer and turns the FOR into a `WHILE TRUE` loop whose bound check copies the `end` expression with `end.clone()` into two exit tests (`counter > end` and `counter < end`), so both copies share their AST ids; the aggregate lowerer rewrites each copy in place into a temporary (`__mk2`, `__mk3`) allocated in front of its exit test, and the codegen fails to resolve the first one, most likely because the annotation of the shared id points at the other temporary. Expected: the temporary and the call are placed once in front of the loop, or the desugarer gives the copied bound fresh ids so each temporary resolves.
```
// Reproducible example:
TYPE S : STRUCT
    x : DINT;
END_STRUCT END_TYPE

FUNCTION mk : S
    VAR_INPUT
        v : DINT;
    END_VAR
    mk.x := v;
END_FUNCTION

FUNCTION xof : DINT
    VAR_INPUT
        s : S;
    END_VAR
    xof := s.x;
END_FUNCTION

FUNCTION main : DINT
    VAR
        i : DINT;
    END_VAR
    CASE xof(mk(3)) OF    // compiles and prints "case ok"
        3: printf('case ok$N');
    END_CASE;
    FOR i := 1 TO xof(mk(4)) DO    // error: Could not resolve reference to __mk0
        printf('%d$N', i);    // expected: 1 2 3 4
    END_FOR;
END_FUNCTION
```

Review (unclear): The symptom reproduces (E048 on p^.show(), while plain.show() and p^.method() resolve), but the cause is not the resolver. The polymorphism dispatch lowering treats every `ptr^.member()` call as a virtual method call and rewrites it to `__vtable_Plain#(p^.__vtable^).show^(p^)`; actions are not in the vtable, so `show` is unresolved after lowering. The corrected location is given in the entry. Whether actions should be dispatched directly or become virtual is a design decision.

P3 src/lowering/polymorphism/dispatch/pou.rs:269:13: Action called through a pointer is not resolved
`p^.show();` with `p : POINTER TO Plain` and `ACTION show` of `Plain` fails with `E048: Could not resolve reference to show`, while `plain.show();` on an instance resolves and `p^.m()` with a `METHOD m` works. `is_polymorphic_call_candidate` accepts any `Member` access whose base is a dereferenced pointer to a function block or class without checking that the callee is a method, so the call is rewritten to the virtual table dispatch `__vtable_Plain#(p^.__vtable^).show^(p^)` (visible with `--ast-lowered`). Actions have no vtable slot, so the re-annotation cannot resolve `show`. Expected: the action is found through the pointer like a method is, by calling it directly instead of through the vtable.
```
// Reproducible example:
// plc --check
// observed: main.st:21:8: error[E048]: Could not resolve reference to show
// expected: compiles and prints 5 twice
FUNCTION_BLOCK Plain
VAR
    n : DINT;
END_VAR
END_FUNCTION_BLOCK

ACTIONS Plain
    ACTION show
        printf('%d$N', n);
    END_ACTION
END_ACTIONS

FUNCTION main : DINT
VAR
    plain : Plain;
    p : POINTER TO Plain;
END_VAR
    p := ADR(plain);
    plain.n := 5;
    plain.show();
    p^.show();
END_FUNCTION
```

Review (unclear): IEC 61131-3 does not state how often the end and step expressions are evaluated and CODESYS also re-evaluates them, so this is a semantics choice worth a documented decision rather than a clear miscompilation; reported as P3.

P3 compiler/plc_lowering/src/loops.rs:359:17: FOR end bound and step are re-evaluated on every iteration
`FOR n := next() TO next() + 3 DO END_FOR`, where `next` increments a global counter and returns it, runs about two billion iterations and ends with `cnt=2147483645 n=2147483644` instead of `cnt=2` and four iterations; `FOR i := 1 TO 10 BY st DO st := 5; END_FOR` with `st := 2` runs two iterations instead of five; and `FOR i := 1 TO e DO e := 10; END_FOR` with `e := 3` runs ten times. The FOR desugarer clones the user's `end` expression into the exit comparison (line 359) and the `step` expression into the increment (line 321), so both are evaluated again on every iteration of the generated `WHILE TRUE` loop; only the start value is evaluated once. Expected (Siemens SCL, and the reading of the standard where the header values are fixed at loop entry): end and step are stored in temporaries before the loop, or the behavior is documented. Same output at -O none and the default level.
```
// Reproducible example:
VAR_GLOBAL
    cnt : DINT := 0;
END_VAR
FUNCTION next : DINT
    cnt := cnt + 1;
    next := cnt;
END_FUNCTION
FUNCTION main : DINT
VAR
    n : DINT;
    i : DINT;
    st : DINT := 2;
    k : DINT := 0;
END_VAR
    FOR n := next() TO next() + 3 DO END_FOR
    printf('cnt=%d n=%d$N', cnt, n);
    // prints cnt=2147483645 n=2147483644, expected cnt=2 n=6
    FOR i := 1 TO 10 BY st DO
        k := k + 1;
        st := 5;
    END_FOR
    printf('k=%d$N', k);
    // prints k=2, expected k=5 if the step is fixed at loop entry
END_FUNCTION
```

Review (unclear): Out-of-range float to integer conversion is implementation-defined in IEC 61131-3, so this is reported as P3 for the inconsistency and the optimization-dependent result, not as wrong code by itself.

P3 src/codegen/llvm_typesystem.rs:330:26: Implicit REAL to integer assignment out of range yields an optimization-dependent value while REAL_TO_INT saturates
With `r : REAL := 40000.0` and `lr : LREAL := 3.0E9`, `i := r` prints `40000` at `-O none` and `-668053464` at the default level, and `d := lr` prints `2147483647` at `-O none` and `-1499349872` at the default level; `REAL_TO_INT(r)` and `LREAL_TO_DINT(lr)` print `32767` and `2147483647` at both levels. The implicit cast in `cast_float_to_int` emits a plain `fptosi` (line 330, `fptoui` for unsigned targets), whose result is poison for values outside the target range, so LLVM folds it to an arbitrary constant when it can and the hardware saturates when it cannot; the explicit stdlib conversion functions are Rust `as` casts and saturate. Expected: the implicit narrowing behaves like the explicit conversion (saturating `llvm.fptosi.sat`), or at least does not change with the optimization level. Reproduce with the default command line at `-O none` and at the default level and compare.
```
// Reproducible example:
FUNCTION main : DINT
VAR
    i : INT;
    d : DINT;
    r : REAL := 40000.0;
    lr : LREAL := 3.0E9;
END_VAR
    i := r;
    d := lr;
    printf('%d %d$N', i, d);
    // -O none: 40000 2147483647 ; default: -668053464 -1499349872
    printf('%d %d$N', REAL_TO_INT(r), LREAL_TO_DINT(lr));
    // both levels: 32767 2147483647
    main := 0;
END_FUNCTION
```
