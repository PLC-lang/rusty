# Parser Recovery Issues

This file tracks unique parser recovery failures found while improving syntax diagnostics.

## Summary

Status as of this pass: 31 unique recovery cases are documented, fixed, and covered by focused
parser tests.

The recurring root cause was that local parse recovery only knew about the current closing token.
When a syntax error made that close token unreachable, recovery consumed later declarations or
entries and produced cascading diagnostics. The parser now has shared recovery boundaries for:

- Top-level declarations: `TYPE`, `FUNCTION`, `FUNCTION_BLOCK`, `PROGRAM`, `CLASS`, `INTERFACE`,
  global/config variable blocks, and action blocks.
- Member declarations: methods and properties inside class/function-block/interface contexts.
- Statement block ends and enclosing POU/member ends.
- Repeated element starts inside lists, such as variable declarations and `VAR_CONFIG` entries.
- Delimited expression/type regions where a missing comma, close delimiter, or keyword should not
  consume the next useful element.

The implementation follows the same broad approach as Coil's `recovery.rs`: centralize construct
starts and recovery boundaries, then let narrow parser regions stop when they see a sensible next
construct instead of blindly eating input until EOF or a distant close token.

## Wrong `END_*` after `TYPE` struct consumes next top-level declaration

Status: fixed, tested

Minimal reproducer:

```iecst
TYPE Position:
    STRUCT
        x: DINT;
    END_STRUCT
END_POSITION

FUNCTION_BLOCK FbA
END_FUNCTION_BLOCK
```

Current bad diagnostic behavior:

The parser treated `END_POSITION` as the start of another type declaration inside the open
`TYPE` block. It then consumed the following `FUNCTION_BLOCK` while trying to recover the
invented type declaration, producing cascading errors in unrelated code.

Expected improved behavior:

Report the wrong `END_POSITION` near the malformed type declaration, then recover before the
following `FUNCTION_BLOCK` so that the function block is parsed independently.

Recovery boundary:

After a malformed top-level `TYPE` declaration, recover at the next top-level declaration keyword
such as `FUNCTION_BLOCK`, `FUNCTION`, `PROGRAM`, `CLASS`, `TYPE`, global variable blocks, actions,
or interfaces.

## Missing `END_TYPE` before next top-level declaration

Status: fixed, tested

Minimal reproducer:

```iecst
TYPE Position:
    STRUCT
        x: DINT;
    END_STRUCT

FUNCTION main
END_FUNCTION
```

Current bad diagnostic behavior:

The parser remained inside the `TYPE` block and could consume the following top-level declaration
while recovering.

Expected improved behavior:

Report the missing `END_TYPE` and leave the following `FUNCTION` available to the top-level parser.

Recovery boundary:

The next top-level declaration keyword.

## Missing `END_STRUCT` and `END_TYPE` before next top-level declaration

Status: fixed, tested

Minimal reproducer:

```iecst
TYPE Position:
    STRUCT
        x: DINT;

FUNCTION main
END_FUNCTION
```

Current bad diagnostic behavior:

The inner `STRUCT` recovery consumed `FUNCTION main` and `END_FUNCTION` while searching for
`END_STRUCT`, then the outer `TYPE` recovery reported again at EOF.

Expected improved behavior:

Report the missing `END_STRUCT` and `END_TYPE` at `FUNCTION main`, then parse the function
independently.

Recovery boundary:

The enclosing `END_TYPE` or the next top-level declaration keyword.

## Missing semicolon between struct members

Status: fixed, tested

Minimal reproducer:

```iecst
TYPE Position:
    STRUCT
        x: DINT
        y: DINT;
    END_STRUCT
END_TYPE
```

Current bad diagnostic behavior:

The parser reported the missing semicolon at `y: DINT`, but the recovery consumed that whole
member while searching for the semicolon, so `y` was dropped from the recovered struct AST.

Expected improved behavior:

Report the missing semicolon at `y`, stop at the next `identifier :` variable declaration start,
and parse `y` as the next struct member.

Recovery boundary:

The next variable declaration start, following Coil's `element_start` recovery pattern.

## Missing colon before a variable or struct member type

Status: fixed, tested

Minimal reproducers:

```iecst
TYPE Position:
    STRUCT
        x INT;
        y: INT;
    END_STRUCT
END_TYPE
```

```iecst
FUNCTION_BLOCK FbA
    VAR
        x INT;
        y: INT;
    END_VAR
END_FUNCTION_BLOCK
```

Current bad diagnostic behavior:

The declaration parser treated the type name after `x` as another variable name. It then reported
multiple missing colon/comma diagnostics and finally reported the semicolon as an unexpected
datatype token.

Expected improved behavior:

Report one missing `:` at the apparent type name, parse `x` with that type, and continue with the
following declaration.

Recovery boundary:

After the first variable name in a declaration line, an identifier-like token followed by a
datatype suffix or declaration terminator can be treated as the datatype boundary for missing-colon
recovery.

## Missing enum close paren before next top-level declaration

Status: fixed, tested

Minimal reproducer:

```iecst
TYPE Color:
    (
        Red,
        Green

FUNCTION main
END_FUNCTION
```

Current bad diagnostic behavior:

The enum list recovery consumed `FUNCTION main` and `END_FUNCTION` while searching for `)`, then
reported the missing enum close, declaration semicolon, and `END_TYPE` at EOF.

Expected improved behavior:

Report the missing enum close, declaration semicolon, and `END_TYPE` at `FUNCTION main`, then
parse the function independently.

Recovery boundary:

The enclosing `END_TYPE` or the next top-level declaration keyword.

## Missing comma between enum elements

Status: fixed, tested

Minimal reproducer:

```iecst
TYPE Color:
    (
        Red
        Green,
        Blue
    );
END_TYPE
```

Current bad diagnostic behavior:

After parsing `Red`, the enum element list stopped because there was no comma. The enclosing enum
region then consumed `Green, Blue` while searching for `)`, producing a broad diagnostic and
dropping the later enum elements from the recovered AST.

Expected improved behavior:

Report the missing comma at `Green`, then continue parsing `Green` and `Blue` as enum elements.

Recovery boundary:

The next identifier-like enum element start inside an enum list.

## Missing `OF` in array type before next top-level declaration

Status: fixed, tested

Minimal reproducer:

```iecst
TYPE Matrix:
    ARRAY [1..10]

FUNCTION main
END_FUNCTION
```

Current bad diagnostic behavior:

The array range parser searched for `OF` without the enclosing type recovery boundaries. It
consumed `FUNCTION main` and `END_FUNCTION` while recovering the array type, then reported the
remaining type declaration errors at EOF.

Expected improved behavior:

Report the missing `OF` at `FUNCTION main`, keep `FUNCTION main` available to the top-level
parser, and parse the function independently.

Recovery boundary:

The enclosing `END_TYPE` or the next top-level declaration keyword.

## Missing `]` in array range before `OF`

Status: fixed, tested

Minimal reproducer:

```iecst
TYPE Matrix:
    ARRAY [1..10 OF DINT;
END_TYPE
```

Current bad diagnostic behavior:

The parser reported that `OF` was not `]`, then consumed `OF` during recovery. The next diagnostic
was shifted onto `DINT`, as if the element type were an unexpected token instead of the parser
continuing after a recoverable missing bracket.

Expected improved behavior:

Report the missing `]` at `OF`, keep `OF` as the array grammar boundary, and continue parsing the
array element type.

Recovery boundary:

The array `OF` separator, without consuming it while reporting the missing bracket.

## Missing comma in delimited expression list

Status: fixed, tested

Minimal reproducers:

```iecst
TYPE Matrix:
    ARRAY [1..10 20..30] OF DINT;
END_TYPE
```

```iecst
FUNCTION_BLOCK FbA
    VAR
        xs: ARRAY [1..10 20..30] OF DINT;
        y: DINT;
    END_VAR
END_FUNCTION_BLOCK
```

```iecst
FUNCTION_BLOCK FbA
    VAR
        xs: Arr := [1 2];
        y: DINT;
    END_VAR
END_FUNCTION_BLOCK
```

```iecst
FUNCTION_BLOCK FbA
    VAR
        p: Position := (x := 1 y := 2);
        z: DINT;
    END_VAR
END_FUNCTION_BLOCK
```

```iecst
PROGRAM main
    buz(a,b c);
END_PROGRAM
```

Current bad diagnostic behavior:

Delimited expression lists stopped after the first element unless an explicit comma was present.
In array bounds this made the second range look like a missing `]` and then unexpected trailing
input before `OF`. In array literals, struct initializers, and call arguments, the following
expression looked like a missing closing delimiter or was dropped from the recovered AST.

Expected improved behavior:

Report a missing comma at the next expression element, preserve the following element in the
expression list, and continue parsing the array type or initializer normally.

Recovery boundary:

Inside `(...)` and `[...]` expression-list regions, an expression-start token after a completed
expression is treated as the next list element with a missing comma.

## Missing `OF` in variable array type before next variable

Status: fixed, tested

Minimal reproducer:

```iecst
FUNCTION_BLOCK FbA
    VAR
        xs: ARRAY [1..10]
        y: DINT;
    END_VAR
END_FUNCTION_BLOCK
```

Current bad diagnostic behavior:

The array range recovery inside `xs` did not know about variable declaration element starts. It
consumed `y: DINT` while searching for `OF`, then produced duplicate `OF` and datatype
diagnostics at the following semicolon.

Expected improved behavior:

Report the missing `OF` at `y`, report the missing semicolon for `xs` at the same boundary, and
parse `y` as the next variable declaration.

Recovery boundary:

The next `identifier :` variable declaration start inside a variable block.

## Missing `]` in string size before next declaration

Status: fixed, tested

Minimal reproducers:

```iecst
FUNCTION_BLOCK FbA
    VAR
        s: STRING[10
        x: DINT;
    END_VAR
END_FUNCTION_BLOCK
```

```iecst
TYPE Name:
    STRING[10

FUNCTION other
END_FUNCTION
```

Current bad diagnostic behavior:

The string-size expression recovery searched only for `]`, so a malformed `STRING[10` could
consume the following variable declaration or top-level declaration while looking for the missing
bracket.

Expected improved behavior:

Report the missing `]` at the next declaration boundary, report the enclosing missing semicolon or
`END_TYPE` where appropriate, and keep the following declaration available to the enclosing parser.

Recovery boundary:

The next `identifier :` variable declaration start in variable/member contexts, or the next
top-level declaration keyword in a type declaration context.

## Missing comma in interface inheritance or implementation list

Status: fixed, tested

Minimal reproducers:

```iecst
INTERFACE IBase
END_INTERFACE

INTERFACE IOther
END_INTERFACE

INTERFACE IFoo EXTENDS IBase IOther
END_INTERFACE
```

```iecst
INTERFACE IBase
END_INTERFACE

INTERFACE IOther
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IBase IOther
END_FUNCTION_BLOCK
```

Current bad diagnostic behavior:

The parser accepted adjacent interface names as if a comma were present, so malformed `EXTENDS`
and `IMPLEMENTS` lists produced no syntax diagnostic.

Expected improved behavior:

Report the missing comma at the second interface name and continue parsing the list.

Recovery boundary:

The next identifier-like interface name in the comma-separated list.

## Missing delimiter in generic parameter list

Status: fixed, tested

Minimal reproducers:

```iecst
FUNCTION test<T: ANY R: ANY_NUM> : R
END_FUNCTION
```

```iecst
FUNCTION test<T: ANY, R: ANY_NUM : R
END_FUNCTION
```

Current bad diagnostic behavior:

When a comma was missing between generic bindings, the parser stopped the generic list and reported
the next valid binding (`R: ANY_NUM`) as an unexpected token while looking for `>`. When the final
`>` was missing before the function return type, recovery could continue to `END_FUNCTION` and
emit duplicate `OperatorGreater` diagnostics.

Expected improved behavior:

Report a missing comma at the next generic binding start, preserve both generic bindings in the
recovered AST, and report a missing `>` at the return-type `:` without consuming the rest of the
function declaration.

Recovery boundary:

Inside a generic parameter list, recover at the next `identifier :` generic binding start or at the
return-type `:` that follows the list.

## Missing base type after `REF_TO`

Status: fixed, tested

Minimal reproducers:

```iecst
TYPE RefInt:
    REF_TO

FUNCTION other
END_FUNCTION
```

```iecst
FUNCTION_BLOCK FbA
    VAR
        r: REF_TO
        x: DINT;
    END_VAR
END_FUNCTION_BLOCK
```

Current bad diagnostic behavior:

When `REF_TO` or `POINTER TO` was missing its base type, nested datatype parsing did not know about
the enclosing recovery boundaries. At top level it reported an unexpected datatype token at the
next declaration, and in a variable block it could consume the next variable name as the missing
base type before reporting at the colon.

Expected improved behavior:

Report the missing base datatype at the recovery boundary, leave the following top-level
declaration or variable declaration available to its enclosing parser, and avoid swallowing the
next declaration as the reference base type.

Recovery boundary:

The next top-level declaration keyword or the next `identifier :` variable declaration start,
depending on the enclosing context.

## Missing expression delimiter before statement semicolon

Status: fixed, tested

Minimal reproducers:

```iecst
FUNCTION main
    foo(1, 2;
END_FUNCTION
```

```iecst
FUNCTION main
    x := arr[1;
END_FUNCTION
```

```iecst
FUNCTION_BLOCK FbA
    VAR
        p: Position := (x := 1, y := 2;
        z: DINT;
    END_VAR
END_FUNCTION_BLOCK
```

Current bad diagnostic behavior:

Expression subregions recovered only to their own closing delimiter. When a call, subscript, or
initializer was missing a closing `)` or `]` before the statement semicolon, the parser reported
both a missing delimiter and an unexpected semicolon for the same source location.

Expected improved behavior:

Report only the missing delimiter at the semicolon, leave the semicolon for the enclosing
statement or variable declaration, and continue parsing the following statement or variable.

Recovery boundary:

The statement semicolon and enclosing statement/declaration block boundaries.

## Missing delimiter in variable initializer before next variable

Status: fixed, tested

Minimal reproducers:

```iecst
FUNCTION_BLOCK FbA
    VAR
        p: Position := (x := 1
        z: DINT;
    END_VAR
END_FUNCTION_BLOCK
```

```iecst
FUNCTION_BLOCK FbA
    VAR
        p: DINT := foo(1
        z: DINT;
    END_VAR
END_FUNCTION_BLOCK
```

```iecst
FUNCTION_BLOCK FbA
    VAR
        p: DINT := arr[1
        z: DINT;
    END_VAR
END_FUNCTION_BLOCK
```

Current bad diagnostic behavior:

Delimited expression recovery in initializers searched only for the missing `)` or `]` and broad
expression stop tokens. Without a semicolon after the bad initializer, it consumed the next
`identifier :` variable declaration as part of the initializer and emitted duplicate delimiter
diagnostics.

Expected improved behavior:

Report the missing delimiter at the next variable declaration boundary, report the missing
semicolon for the malformed declaration at the same boundary, and parse the following variable
independently.

Recovery boundary:

The next `identifier :` variable declaration start while recovering from a delimited expression
region.

## Missing control-statement header delimiter before body

Status: fixed, tested

Minimal reproducers:

```iecst
FUNCTION main
    IF x
        y := 1;
    END_IF
    z := 2;
END_FUNCTION
```

```iecst
FUNCTION main
    FOR i := 0 10 DO
        y := i;
    END_FOR
    z := 2;
END_FUNCTION
```

Current bad diagnostic behavior:

Missing `THEN` in an `IF` header and missing `TO` in a `FOR` header aborted the current control
statement parsing path. The parser then treated the body or terminator as unrelated top-level
statements, producing cascades through `END_IF`/`END_FOR` and the following statement.

Expected improved behavior:

Report the missing header delimiter at the first token where it should have appeared, then keep
parsing the control statement body and following statement.

Recovery boundary:

The next body statement after `IF` condition when `THEN` is missing, and the final bound
expression after the `FOR` start expression when `TO` is missing.

## Missing `OF` in `CASE` header before first label

Status: fixed, tested

Minimal reproducer:

```iecst
FUNCTION main
    CASE x
        1: y := 1;
    END_CASE
    z := 2;
END_FUNCTION
```

Current bad diagnostic behavior:

The parser aborted the `CASE` statement as soon as `OF` was missing. The first label and
`END_CASE` were then parsed as unrelated statements, producing cascades through the case terminator
and following statement.

Expected improved behavior:

Report the missing `OF` at the first label and continue parsing the `CASE` arms.

Recovery boundary:

The first case label after the selector expression.

## Missing `:` in `CASE` selection before body or next label

Status: fixed, tested

Minimal reproducers:

```iecst
FUNCTION main
    CASE x OF
        1 y := 1;
        2: y := 2;
    END_CASE
    z := 3;
END_FUNCTION
```

```iecst
FUNCTION main
    CASE x OF
        1
        2: y := 2;
    END_CASE
    z := 3;
END_FUNCTION
```

Current bad diagnostic behavior:

When a case selection missed the `:` delimiter, the parser used the statement semicolon as the
next recovery point or treated the next label as an unexpected statement token. That either
dropped the malformed arm body or prevented the following case label from being parsed as a
separate arm. In the first-arm case, a later statement could also receive a misleading
`Missing Case-Condition` diagnostic.

Expected improved behavior:

Report the missing `:` at the first token that starts the arm body or the next `case-label:`, keep
the malformed arm local, and continue parsing later case arms and following statements.

Recovery boundary:

Inside a `CASE` statement, recover a missing selection delimiter at a plausible statement start,
the next `case-label:` start, `ELSE`, `END_CASE`, or an enclosing statement/top-level boundary.

## Missing `END_REPEAT` after `UNTIL` condition before next statement

Status: fixed, tested

Minimal reproducer:

```iecst
FUNCTION main
    REPEAT
        x := 1;
    UNTIL x > 5
    z := 2;
END_FUNCTION
```

Current bad diagnostic behavior:

The parser treated the following `z := 2;` statement as part of recovery while looking for
`END_REPEAT`, then also reported again at the enclosing `END_FUNCTION`.

Expected improved behavior:

Report the missing `END_REPEAT` at `z`, leave `z := 2;` available to the enclosing function body,
and avoid duplicate recovery diagnostics for the same missing terminator.

Recovery boundary:

The next statement after the `UNTIL` condition.

## Missing POU end before next top-level declaration

Status: fixed, tested

Minimal reproducer:

```iecst
FUNCTION_BLOCK FbA
    VAR
        x: DINT;
    END_VAR

FUNCTION main
END_FUNCTION
```

Current bad diagnostic behavior:

The parser remains inside `FUNCTION_BLOCK FbA` and attempts to parse `FUNCTION main` as a
statement in the function block body. It reports several expression and semicolon diagnostics
before finally reporting that `END_FUNCTION` did not match `END_FUNCTION_BLOCK`.

Expected improved behavior:

Report the missing `END_FUNCTION_BLOCK` at the next top-level `FUNCTION` boundary and parse
`FUNCTION main` independently.

Recovery boundary:

The next top-level declaration keyword after a POU header, variable block, method/property block,
or implementation body when the expected `END_*` for the current POU has not appeared.

## Missing `END_VAR` before next top-level declaration

Status: fixed, tested

Minimal reproducer:

```iecst
FUNCTION_BLOCK FbA
    VAR
        x: DINT;

FUNCTION main
END_FUNCTION
```

Current bad diagnostic behavior:

The parser stayed inside the variable block, consumed `FUNCTION main` while searching for
`END_VAR`, and then also reported against `END_FUNCTION`.

Expected improved behavior:

Report the missing `END_VAR` at the `FUNCTION` boundary, then let the enclosing POU recovery
report the missing `END_FUNCTION_BLOCK` without consuming `FUNCTION main`.

Recovery boundary:

The next top-level declaration keyword.

## Missing `END_VAR` before next member declaration

Status: fixed, tested

Minimal reproducer:

```iecst
FUNCTION_BLOCK FbA
    VAR
        x: DINT;

    METHOD second
    END_METHOD
END_FUNCTION_BLOCK
```

Current bad diagnostic behavior:

The parser stayed inside the variable block, consumed `METHOD second` and `END_METHOD` while
searching for `END_VAR`, then cascaded again at `END_FUNCTION_BLOCK`.

Expected improved behavior:

Report the missing `END_VAR` at `METHOD second` and parse the method as the next member.

Recovery boundary:

The next member declaration keyword or enclosing POU/member end token.

## Missing semicolon between variables in `VAR` block

Status: fixed, tested

Minimal reproducer:

```iecst
FUNCTION_BLOCK FbA
    VAR
        x: DINT
        y: DINT;
    END_VAR
END_FUNCTION_BLOCK
```

Current bad diagnostic behavior:

The parser reported the missing semicolon at `y: DINT`, but recovery consumed the following
variable while looking for the semicolon, so `y` was dropped from the recovered variable block.

Expected improved behavior:

Report the missing semicolon at `y`, stop at the next `identifier :` variable declaration start,
and parse `y` as the next variable.

Recovery boundary:

The next variable declaration start.

## Missing `END_VAR` in `VAR_CONFIG` before next top-level declaration

Status: fixed, tested

Minimal reproducer:

```iecst
VAR_CONFIG
    main.x AT %QX0.0 : BOOL;

FUNCTION main
END_FUNCTION
```

Current bad diagnostic behavior:

The parser stayed inside `VAR_CONFIG`, consumed `FUNCTION main` and `END_FUNCTION` while looking
for `END_VAR`, and then reported another `END_VAR` error at EOF.

Expected improved behavior:

Report the missing `END_VAR` at `FUNCTION main` and parse the function independently.

Recovery boundary:

The next top-level declaration keyword.

## Missing semicolon in `VAR_CONFIG` entry before next top-level declaration

Status: fixed, tested

Minimal reproducers:

```iecst
VAR_CONFIG
    main.x AT %QX0.0 : BOOL

FUNCTION main
END_FUNCTION
```

```iecst
VAR_CONFIG
    main.x AT %QW0 : DINT
    main.y AT %QW1 : INT;
END_VAR
```

Current bad diagnostic behavior:

The parser stayed inside the unterminated config entry, consumed `FUNCTION main` and
`END_FUNCTION` while searching for the entry semicolon, then also reported `END_VAR` at EOF. It
also consumed later `qualified.name AT ...` config entries in the same block as trailing text from
the bad entry.

Expected improved behavior:

Report the missing semicolon at `FUNCTION main` or the next config entry, report the still-missing
`END_VAR` at the same boundary when needed, and parse the following top-level declaration or config
entry independently.

Recovery boundary:

The enclosing `END_VAR`, the next top-level declaration keyword, or the next config entry start
(`qualified.name AT ...`).

## Missing statement block terminator before next top-level declaration

Status: fixed, tested

Minimal reproducers:

```iecst
FUNCTION main
    IF TRUE THEN
        x := 1;

FUNCTION other
END_FUNCTION
```

```iecst
FUNCTION main
    FOR i := 0 TO 10 DO
        x := i;

FUNCTION other
END_FUNCTION
```

```iecst
FUNCTION main
    WHILE TRUE DO
        x := 1;

FUNCTION other
END_FUNCTION
```

```iecst
FUNCTION main
    REPEAT
        x := 1;

FUNCTION other
END_FUNCTION
```

```iecst
FUNCTION main
    CASE x OF
        1: y := 1;

FUNCTION other
END_FUNCTION
```

Current bad diagnostic behavior:

The parser stayed inside the open statement block, parsed the following `FUNCTION` as a statement,
and emitted expression and semicolon cascades before reaching `END_FUNCTION`.

Expected improved behavior:

Report the missing statement terminator (`END_IF`, `END_FOR`, `END_WHILE`, `UNTIL`, or
`END_CASE`) and the missing enclosing `END_FUNCTION` at the `FUNCTION other` boundary, then
parse `FUNCTION other` independently.

Recovery boundary:

The enclosing POU end token or the next top-level declaration keyword, based on the IEC statement
block grammar.

## Missing `END_METHOD` before next member declaration

Status: fixed, tested

Minimal reproducer:

```iecst
FUNCTION_BLOCK FbA
    METHOD first

    METHOD second
    END_METHOD
END_FUNCTION_BLOCK
```

Current bad diagnostic behavior:

The parser stayed inside `METHOD first` and parsed `METHOD second` as a statement, producing
expression and semicolon diagnostics before reaching `END_METHOD`.

Expected improved behavior:

Report the missing `END_METHOD` at `METHOD second` and parse `METHOD second` as a separate member.

Recovery boundary:

The next member declaration keyword, such as `METHOD`, `PROPERTY_GET`, or `PROPERTY_SET`.

## Missing `END_PROPERTY` before next member declaration

Status: fixed, tested

Minimal reproducer:

```iecst
FUNCTION_BLOCK FbA
    PROPERTY_GET first: DINT
        first := 1;

    METHOD second
    END_METHOD
END_FUNCTION_BLOCK
```

Current bad diagnostic behavior:

The parser stayed inside the property body, consumed `METHOD second` as a statement, and cascaded
through `END_METHOD` to the enclosing `END_FUNCTION_BLOCK`.

Expected improved behavior:

Report the missing `END_PROPERTY` at `METHOD second` and parse `METHOD second` as a separate
member.

Recovery boundary:

The next member declaration keyword, such as `METHOD`, `PROPERTY_GET`, or `PROPERTY_SET`.

## Missing `END_ACTION` before next action declaration

Status: fixed, tested

Minimal reproducer:

```iecst
PROGRAM Main
END_PROGRAM

ACTIONS Main
    ACTION first
        x := 1;

    ACTION second
        x := 2;
    END_ACTION
END_ACTIONS
```

Current bad diagnostic behavior:

The parser stayed inside `ACTION first` and parsed `ACTION second` as a statement, producing
expression and semicolon diagnostics before it reached the later `END_ACTION`.

Expected improved behavior:

Report the missing `END_ACTION` at `ACTION second` and parse `ACTION second` independently.

Recovery boundary:

The next `ACTION` declaration inside an `ACTIONS` block, or `END_ACTIONS`.

## Missing `END_ACTIONS` before next top-level declaration

Status: fixed, tested

Minimal reproducer:

```iecst
PROGRAM Main
END_PROGRAM

ACTIONS Main
    ACTION first
        x := 1;
    END_ACTION

FUNCTION other
END_FUNCTION
```

Current bad diagnostic behavior:

The parser stayed inside the `ACTIONS` block, reported that `FUNCTION` was not an `ACTION`,
then consumed `FUNCTION other` while recovering to `END_ACTIONS` and reported again at EOF.

Expected improved behavior:

Report the missing `END_ACTIONS` at `FUNCTION other` and parse `FUNCTION other` as a separate
top-level declaration.

Recovery boundary:

The next top-level declaration keyword.

## Missing `END_METHOD` before `END_INTERFACE`

Status: fixed, tested

Minimal reproducer:

```iecst
INTERFACE IFoo
    METHOD first

END_INTERFACE

FUNCTION main
END_FUNCTION
```

Current bad diagnostic behavior:

The parser stayed inside the interface method body, consumed `END_INTERFACE` and the following
`FUNCTION main`, then reported additional method/interface errors at EOF.

Expected improved behavior:

Report the missing `END_METHOD` at `END_INTERFACE`, close the interface, and parse the following
`FUNCTION` independently.

Recovery boundary:

The enclosing interface end token.

## Missing hardware access in `VAR_CONFIG` entry before type tail

Status: fixed, tested

Minimal reproducer:

```iecst
VAR_CONFIG
    main.x AT : BOOL;
    main.y AT %QW1 : INT;
END_VAR

FUNCTION other
END_FUNCTION
```

Current bad diagnostic behavior:

The parser reported the useful missing hardware access at `:`, then reported a second unexpected
token diagnostic for the `: BOOL` tail while trying to recover to the entry semicolon.

Expected improved behavior:

Report only the missing hardware access for the malformed entry, skip the rest of that entry, and
continue parsing the next `VAR_CONFIG` entry and following top-level declaration.

Recovery boundary:

The current config entry semicolon, the next config entry, `END_VAR`, or the next top-level
declaration.
