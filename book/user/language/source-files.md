# Source Files

Before the language itself, look at the files that hold it: what goes into one, how you write a comment, how the compiler reads a name, and where that name is visible. The last section lists the attributes that change how the compiler treats a declaration.


## What a file contains

A source file is a list of declarations at the top level. There is no wrapper around them and no order requirement:

```iecst
TYPE Level: INT (0..100);
END_TYPE

VAR_GLOBAL
    cycleTime: TIME := T#10ms;
END_VAR

FUNCTION_BLOCK Pump
    (* ... *)
END_FUNCTION_BLOCK

PROGRAM Plant
    (* ... *)
END_PROGRAM
```

A file may hold type declarations (`TYPE`), global variables (`VAR_GLOBAL`), hardware bindings (`VAR_CONFIG`), program organization units (`FUNCTION`, `FUNCTION_BLOCK`, `PROGRAM`, `CLASS`, `INTERFACE`), and the actions of a POU (`ACTIONS`). One file can hold all of them.

There is no import and no module. The compiler reads every file that the command line or the project file names, and everything they declare forms one project. A function in one file calls a function block in another without any declaration between them.

The extension does not group the files, it only selects the reader. `.cfc`, `.fbd`, and `.xml` are read as [graphical sources](cfc.md), `.o`, `.so`, and `.exe` go straight to the linker, and everything else is read as Structured Text. The usual extension is `.st`. A file that only declares interfaces for foreign code is usually named `.pli`, but that is a convention: the compiler reads it as Structured Text like any other file.


## Comments

A comment can go wherever a space can. There are three forms, and the two block forms nest, in themselves and in each other:

```iecst
(* a comment *)

/* another form,
   (* with a nested comment inside it *)
   that ends here */

x := 1;   // to the end of the line
```

Nesting is what lets you comment out a piece of code that already holds a comment.


## Names

A name starts with a letter or an underscore and continues with letters, digits, and underscores. A name that starts with a digit is not accepted.

Names are not case-sensitive. `Motor`, `motor`, and `MOTOR` are the same name, which also means that two declarations that differ only in case collide:

```iecst
PROGRAM Main
END_PROGRAM

FUNCTION main: DINT    (* error[E004]: main: Duplicate symbol. *)
END_FUNCTION
```

A keyword cannot be a name, and because names are not case-sensitive, `type` is the keyword `TYPE`. The names of the [built-in functions](../reference/built-in-functions.md) are taken as well: a function of your own called `Add` collides with the built-in `ADD` and is rejected.

Names that start with two underscores belong to the compiler. It generates names such as `__vtable_Pump` and `__PI_0_0`. It does not reject a name of yours with the same prefix, so a program that declares `__level` is accepted and only collides later, with a message that points somewhere else. Do not start a name with two underscores.


## Where a name is visible

A variable declared in a POU is visible in that POU and in its methods and actions. A variable declared in `VAR_GLOBAL` is visible in the whole project, without a declaration in the POU that uses it.

When a local name and a global name are the same, the local one wins. A leading dot reaches past it to the global:

```iecst
VAR_GLOBAL
    shared: DINT := 1;
END_VAR

FUNCTION main: DINT
    VAR
        shared: DINT := 2;
    END_VAR

    main := shared;    (* 2, the local one *)
    main := .shared;   (* 1, the global one *)
END_FUNCTION
```

There is no block scope. A variable belongs to its POU, not to the `IF` or the `FOR` that uses it.


## Attributes

An attribute in braces changes how the compiler treats a declaration. Three of them matter for everyday code. `{external}` stands before a POU and says that the implementation is elsewhere, which is how you [call C](../interop/calling-c.md). `{ref}` stands after `VAR_INPUT`, the only block that accepts it, and passes the whole block by reference. `{sized}` stands before a variadic type and gives the callee a count and an array.


## What's next

The next chapter declares [variables](variables.md) and gives them their first values.
