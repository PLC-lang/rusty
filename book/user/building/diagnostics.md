# Diagnostics

Every message of the compiler has a code, a severity, and a position. This chapter explains how to read them, how to look them up, and how to change their severity for a project.


## Read a diagnostic

```
error[E037]: Invalid assignment: cannot assign 'STRING' to 'DINT'
  ┌─ main.st:6:5
  │
6 │     x := 'text';
  │     ^^^^^^^^^^^ Invalid assignment: cannot assign 'STRING' to 'DINT'

error: Compilation aborted due to critical errors.
Hint: You can use `plc explain <ErrorCode>` for more information
```

The first line gives the severity, the code, and the problem. The second gives the position: the compiler writes the path of the file in full, and the examples in this book shorten it to the file name. The marker under the source line shows the part of the statement that the message is about.

The compiler collects the diagnostics of a stage before it stops, so one run usually reports more than one problem. An error stops the run before code generation. A warning and an information message do not.


## Look up a code

```bash
plc explain E037
```

The command prints the explanation of the code: usually a description with an example of the mistake and of the correct form, and for a part of the codes a title and nothing more. It works without a project, and the [error code reference](../reference/error-codes.md) has the same text.


## Severity

A diagnostic has one of four severities:

| Severity | Effect |
|---|---|
| `error` | Reported, and the run stops after the stage |
| `warning` | Reported, the run continues |
| `info` | Reported, the run continues |
| `ignore` | Not reported |

`plc config diagnostics` prints the severity of every code as JSON:

```json
{"ignore":["E132","E015"],"warning":["E096","E042", ...],"info":["E092", ...],"error":["E119", ...]}
```


## Change the severity

Write the codes you want to move into a file, with the severity as the key:

```json
{
    "warning": [ "E037" ],
    "ignore":  [ "E023" ]
}
```

```bash
plc --check main.st --error-config severities.json
```

`E037` is now a warning, so the run continues and the exit code stays `0`. A code that you do not name keeps its default severity.

The option is global, so it works with `build` and `check` as well. To see the result of your file, print the merged configuration:

```bash
plc config diagnostics --error-config severities.json
```


## Output format

| Value | Output |
|---|---|
| `rich` | The default. Source snippet, position marker, and color |
| `clang` | One line per diagnostic, in the format of `clang`, for tools that parse it |
| `none` | No messages. A run that fails still reports that it was aborted |

```bash
plc --check main.st --error-format=clang
```

```
main.st:6:5:{6:5-6:16}: error[E037]: Invalid assignment: cannot assign 'STRING' to 'DINT'
error: Compilation aborted due to critical errors
```


## What's next

The [error code reference](../reference/error-codes.md) has a page per code, with an example of the mistake and of the correct form. The [next chapter](debugging.md) is about the information that the compiler puts into the artifact for a debugger.
