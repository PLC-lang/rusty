# Validation

After the resolver, the compiler knows what every declaration is and what every expression means, but nothing has checked whether the program is correct. In

```iecst
PROGRAM main
    VAR
        sintVar : SINT;
        dintVar : DINT;
    END_VAR

    sintVar := dintVar;
    sintVar := unknown;
END_PROGRAM
```

the first assignment stores a `DINT` into a `SINT` and silently loses the upper bytes, and the second assigns a name that is declared nowhere. The index and the resolver recorded everything needed to see both problems, but neither judges. Validation does: it walks the project, asks the index and the annotation map for the facts, and turns every violated rule into a diagnostic, a warning for the downcast and an error for the unknown name.

```mermaid
flowchart LR
    parse[Parse] --> index[Index] --> annotate[Annotate] --> validate[Validate] --> codegen[Codegen] --> link[Link]
    style validate fill:#bfdbfe,stroke:#000,stroke-width:1px,stroke-dasharray:4 3,color:#0f172a
```

The stage receives the annotated project: the units after all lowering, the global index, and the annotation map. It produces no new data. Its only output is the list of diagnostics, which it hands to the diagnostician as it goes. When the walk is over and any diagnostic reached error severity, the run stops with "Compilation aborted due to critical errors"; otherwise the project goes on to codegen unchanged. `plc --check` runs the pipeline up to here and exits.


## Two sources of facts

Every check needs facts from one of two places. The index answers questions about declarations: does a type of this name exist, is this variable constant, which parameters does this function block have, which class does this POU extend. The annotation map answers questions about expressions: which declaration does this reference point to, what type does this expression have, what type should it become. A check can need both: "is this private member accessed from outside" needs the annotation of the reference (which member it is) and the index (which POU is being validated and what it extends).

The validator carries both, together with the name of the POU it is currently in, in a small context that is passed down the walk:

```rust
pub struct ValidationContext<'s, T: AnnotationMap> {
    /// What every expression is and what it should become
    annotations: &'s T,

    /// What every name declares
    index: &'s Index,

    /// The POU whose declarations or body are being validated
    qualifier: Option<&'s str>,

    // ... other omitted fields
}
```

Every failed check produces a diagnostic. Trimmed to what matters here, it is a message, an error code, and where in the source it applies:

```rust
pub struct Diagnostic {
    /// The description of the problem, as shown to the user
    message: String,

    /// The code that identifies the rule, such as E037
    error_code: &'static str,

    /// Where the problem is
    primary_location: SourceLocation,

    /// Other places that take part in it, such as the second declaration of a duplicate name
    secondary_locations: Option<Vec<SourceLocation>>,

    // ... other omitted fields
}
```

The validator only collects these; what a code means for the build is decided later, see Severity and reporting below.


## Global validation

Some rules concern the project as a whole and cannot be checked file by file. For

```iecst
(* a.st *)
FUNCTION scale : DINT
END_FUNCTION

(* b.st *)
FUNCTION scale : DINT
END_FUNCTION
```

neither file is wrong on its own; the conflict only exists in the merged index. Global validation therefore runs once, before any unit is walked, and reads only the index. It checks that:

- **Names are unique** within their group: callables, types, and global variables. Every declaration of a duplicate is reported, with the others as secondary locations. Built-in names such as `ADD` count too.
- **Data structures are finite.** A struct that contains itself by value, an alias chain that loops, or interfaces that extend each other are reported.
- **Template variables** (`AT %I*`) are configured exactly once in a `VAR_CONFIG` block.
- **Overflowing constants**, such as `TOO_BIG : SINT := 300`, get the reason the index stored reported as a warning.


## Per-unit validation

Everything else is checked unit by unit, in the order the units were parsed. Inside a unit the walk follows the shape of the compilation unit: the POU declarations first, then the user types, the `VAR_CONFIG` blocks, the global variable blocks, the implementations, and finally the interfaces. Declarations and bodies are validated separately, as they are stored.

A POU declaration is checked for rules of its kind (a program cannot have a return type, a class cannot have inputs or outputs), for the interfaces it implements and the class it extends (do they exist, do the method signatures match, is every abstract method implemented), and then block by block for its variables: does the declared type exist, is the initializer assignable to the variable, are array bounds constant and in the right order, is a constant variable an instance of a function block, does a variable shadow one of the parent class. The initializer of a variable is an expression, so it is walked with the same statement visitor as a body.

An implementation is walked statement by statement. The visitor is recursive and mirrors the tree: it visits the children of a node first, then applies the checks for the node itself. A reference is checked for resolution, visibility, and pointer access. An assignment compares the type of the value with the hint the resolver attached to it. A call is matched against the parameters of the callee from the index: argument count, direction of `:=` and `=>`, by-reference arguments, and required `VAR_IN_OUT` arguments. Control statements check their conditions and walk their bodies. For

```iecst
FUNCTION_BLOCK Buffer
    VAR_INPUT
        limit : INT;
    END_VAR
    VAR_IN_OUT
        target : DINT;
    END_VAR
    VAR
        count : DINT;
    END_VAR
END_FUNCTION_BLOCK

ACTIONS Buffer
    ACTION reset
        count := 0;
    END_ACTION
END_ACTIONS

PROGRAM main
    VAR CONSTANT
        MAX : DINT := 10;
    END_VAR
    VAR
        bufferInstance : Buffer;
        i : DINT;
        text : STRING;
    END_VAR

    MAX := 11;
    text := i;
    bufferInstance(limit := 5);
    bufferInstance.count := 3;
    bufferInstance.reset;
END_PROGRAM
```

the body of `main` produces one diagnostic per statement, each from a different combination of the two sources:

```
    MAX := 11;
    ^^^                           E036  annotation: main.MAX is a constant variable
    text := i;
    ^^^^^^^^^                     E037  annotation: i is DINT, its hint is STRING; the index says the types are not compatible
    bufferInstance(limit := 5);
    ^^^^^^^^^^^^^^^^^^^^^^^^^^    E030  index: Buffer has the VAR_IN_OUT target, and no argument was matched to it
    bufferInstance.count := 3;
                   ^^^^^          E049  annotation: Buffer.count is a local variable; index: main is neither Buffer nor a child of it
    bufferInstance.reset;
                   ^^^^^          E095  annotation: the reference resolves to the action Buffer.reset, but the statement is not a call
```

Some nodes are skipped on purpose. Declarations with external or include linkage belong to another compilation and are not checked. Nodes with an internal location were created by a lowering participant, not by the user, and are trusted. Built-in functions such as `ADR`, `SIZEOF`, or `MUX` declare their parameters with placeholder types, so each brings its own validation function instead of going through the general call check.


## Severity and reporting

Every diagnostic carries an error code, and the code decides how serious it is. A registry maps each code to a default severity: `E048` (unresolved reference) is an error, `E067` (implicit downcast) is a warning, `E060` (a hint to use direct access) is informational, and a few codes are ignored by default. `plc explain E067` prints the description behind a code. The defaults can be overridden per project with a JSON file passed as `--error-config`:

```json
{ "error": ["E067"], "ignore": ["E048"] }
```

With this file the downcast in the introduction aborts the compilation and the unknown name is not even printed.

Reporting happens in batches. The validator hands its diagnostics to the diagnostician after global validation and again after every unit. The diagnostician assesses each diagnostic's severity, resolves its locations to file and line, renders it in the selected format (`--error-format`), and returns the highest severity it saw. The stage keeps the maximum over all batches; if it is error, the run aborts after the last unit, so that one run reports every problem in the project and not only the first.


## Validation in participants

Not every check lives in the validator. Some participants rewrite the tree so far that the original construct is no longer recognizable: a property becomes a pair of methods and a call to one of them, an interface-typed variable becomes an internal pointer struct, a generic call becomes a call to a concrete implementation. The validator runs after all lowering, so by then the rule that applied to the original construct can no longer be checked. Those rules are checked by the participant itself, before it rewrites the tree. The participant keeps its diagnostics until the last `post_annotate` hook has run; the stage then handles them before global validation, so they appear at the top of the output.

> **Developer Note**
>
> Validation inside participants is a consequence of the participant model described in the [Driver](00-driver.md) chapter. Because the tree is lowered in place and there is no separate representation of the source program, a check that needs the source form has to run inside the participant that destroys it. A later hook exists that would allow lowering after validation, but moving the participants there breaks other validations that rely on the lowered form.


## Where it lives

| What | Where |
|---|---|
| Validator | `src/validation.rs`, `src/validation/` |
| Diagnostics | `compiler/plc_diagnostics` |


## What's next

The program is now known to be correct as far as the compiler can tell, and it is in its final lowered form. The tree still contains no machine-level decisions: `sintVar := dintVar` is an assignment with a hint `SINT`, `scale(bar, 3)` is a call with arguments matched to parameters, and `text` is a `STRING` with a size but no memory. Turning each of these into LLVM instructions, deciding how a function block instance is laid out in memory, and emitting the string literals as global constants is the job of the next stage, [Codegen](05-codegen.md).
