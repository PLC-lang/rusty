# Syntax Validations for IEC 61131-3 Structured Text

This document lists all semantic validations that must be performed **after parsing** because the simplified grammar is intentionally permissive. The parser accepts a superset of valid IEC 61131-3 programs; these validations ensure conformance to the standard.

---

## Table of Contents

1. [POU (Program Organization Unit) Validations](#pou-validations)
2. [Variable Block Validations](#variable-block-validations)
3. [Variable Declaration Validations](#variable-declaration-validations)
4. [Method and Property Validations](#method-and-property-validations)
5. [Data Type Validations](#data-type-validations)
6. [Statement Validations](#statement-validations)
7. [Expression Validations](#expression-validations)
8. [Modifier and Access Specifier Validations](#modifier-and-access-specifier-validations)

---

## POU Validations

### V-POU-001: Function must have return type
- **Rule**: A `FUNCTION` declaration must specify a return type after the colon.
- **Invalid**: `FUNCTION foo ... END_FUNCTION`
- **Valid**: `FUNCTION foo : INT ... END_FUNCTION`

### V-POU-002: Program must not have return type
- **Rule**: A `PROGRAM` declaration must not specify a return type.
- **Invalid**: `PROGRAM foo : INT ... END_PROGRAM`
- **Valid**: `PROGRAM foo ... END_PROGRAM`

### V-POU-003: Class must not have body statements
- **Rule**: A `CLASS` must not contain a statement list (body), only variable blocks, methods, and properties.
- **Invalid**: `CLASS foo VAR END_VAR x := 1; END_CLASS`

### V-POU-004: Interface must not have variable blocks
- **Rule**: An `INTERFACE` must not contain variable declaration blocks.
- **Invalid**: `INTERFACE foo VAR x : INT; END_VAR END_INTERFACE`

### V-POU-005: Interface must not have implementations
- **Rule**: Methods and properties in an `INTERFACE` must be prototypes only (no statement bodies).

### V-POU-006: Action must be defined within a POU context
- **Rule**: An `ACTION` declaration is only valid inside a `PROGRAM` or `FUNCTION_BLOCK`, or in an associated file context.

### V-POU-007: Extends clause type compatibility
- **Rule**: 
  - `FUNCTION_BLOCK` can only extend another `FUNCTION_BLOCK`
  - `CLASS` can only extend another `CLASS`
  - `INTERFACE` can only extend other `INTERFACE`s

### V-POU-008: No duplicate POU names
- **Rule**: POU names must be unique within the compilation scope.

---

## Variable Block Validations

### V-VAR-001: VAR_INPUT not allowed in PROGRAM
- **Rule**: `VAR_INPUT` blocks are not allowed in `PROGRAM` declarations (programs receive inputs via global variables or direct variable access).
- **Note**: Some implementations relax this rule.

### V-VAR-002: VAR_OUTPUT not allowed in FUNCTION
- **Rule**: `VAR_OUTPUT` blocks are not allowed in `FUNCTION` declarations (functions return values via the function name).
- **Note**: Some implementations allow this as an extension.

### V-VAR-003: VAR_IN_OUT restrictions
- **Rule**: Variables in `VAR_IN_OUT` blocks must not have initializers.
- **Invalid**: `VAR_IN_OUT x : INT := 0; END_VAR`

### V-VAR-004: VAR_EXTERNAL must reference existing VAR_GLOBAL
- **Rule**: Variables declared in `VAR_EXTERNAL` must correspond to a `VAR_GLOBAL` declaration elsewhere.

### V-VAR-005: VAR_TEMP not allowed in PROGRAM
- **Rule**: `VAR_TEMP` is typically only allowed in `FUNCTION` and `FUNCTION_BLOCK`.

### V-VAR-006: VAR_GLOBAL only at file scope or in PROGRAM
- **Rule**: `VAR_GLOBAL` as a nested block is typically only valid in `PROGRAM`, not in `FUNCTION` or `FUNCTION_BLOCK`.

### V-VAR-007: Incompatible var block modifiers
- **Rule**: Certain modifier combinations are invalid:
  - `CONSTANT` with `VAR_OUTPUT` or `VAR_IN_OUT`
  - `RETAIN` and `NON_RETAIN` together
  - `RETAIN` or `PERSISTENT` with `VAR_TEMP`

### V-VAR-008: CONSTANT requires initializer
- **Rule**: Variables in a `CONSTANT` block must have an initializer.
- **Invalid**: `VAR CONSTANT x : INT; END_VAR`
- **Valid**: `VAR CONSTANT x : INT := 42; END_VAR`

### V-VAR-009: Method/Property var block restrictions
- **Rule**: Methods and properties may only contain `VAR`, `VAR_INPUT`, `VAR_OUTPUT`, `VAR_IN_OUT`, and `VAR_TEMP`. They must not contain `VAR_EXTERNAL` or `VAR_GLOBAL`.

---

## Variable Declaration Validations

### V-DECL-001: Location only in specific contexts
- **Rule**: The `AT` location specifier is only valid:
  - In `VAR_GLOBAL` blocks
  - In `PROGRAM` variable blocks
  - In `STRUCT` fields (for overlapping)
- **Invalid**: `FUNCTION foo VAR x AT %MW0 : INT; END_VAR END_FUNCTION`

### V-DECL-002: Location prefix must match usage
- **Rule**: 
  - `%I` (input) variables are read-only
  - `%Q` (output) variables are write-only from the PLC perspective
  - `%M` (memory) variables are read-write

### V-DECL-003: No duplicate variable names
- **Rule**: Variable names must be unique within their scope.

### V-DECL-004: Initializer type compatibility
- **Rule**: The initializer expression must be compatible with the declared type.

---

## Method and Property Validations

### V-METH-001: ABSTRACT method must not have body
- **Rule**: A method marked `ABSTRACT` must not contain a statement body.
- **Invalid**: `METHOD ABSTRACT foo x := 1; END_METHOD`

### V-METH-002: ABSTRACT method only in ABSTRACT class/FB
- **Rule**: `ABSTRACT` methods can only appear in POUs that are also marked `ABSTRACT`.

### V-METH-003: FINAL and ABSTRACT are mutually exclusive
- **Rule**: A method cannot be both `FINAL` and `ABSTRACT`.

### V-METH-004: OVERRIDE must override existing method
- **Rule**: A method marked `OVERRIDE` must override a method from a parent class/FB or interface.

### V-METH-005: Cannot override FINAL method
- **Rule**: A method marked `FINAL` in a parent cannot be overridden.

### V-METH-006: Property must have at least one accessor
- **Rule**: A property must have at least a `GET` or `SET` accessor (or both).
- **Invalid**: `PROPERTY foo : INT END_PROPERTY`

### V-METH-007: Property GET must not have input parameters
- **Rule**: The `GET` accessor of a property must not declare `VAR_INPUT` or `VAR_IN_OUT`.

### V-METH-008: Property SET implicit value parameter
- **Rule**: The `SET` accessor has an implicit input parameter; additional `VAR_INPUT` declarations may be restricted.

### V-METH-009: Duplicate accessor
- **Rule**: A property must not have multiple `GET` or multiple `SET` accessors.

---

## Data Type Validations

### V-TYPE-001: Subrange bounds must be integer constants
- **Rule**: The bounds in a subrange specification must be constant integer expressions.
- **Invalid**: `TYPE foo : INT(x..y); END_TYPE` (where x, y are variables)

### V-TYPE-002: Subrange lower bound <= upper bound
- **Rule**: The lower bound must be less than or equal to the upper bound.
- **Invalid**: `TYPE foo : INT(10..5); END_TYPE`

### V-TYPE-003: Array dimensions must be valid
- **Rule**: Array range bounds must be constant expressions (except for variable-length arrays with `*`).

### V-TYPE-004: Enum values must be unique
- **Rule**: Enumeration values within a single enum type must be unique.
- **Invalid**: `TYPE foo : (A, B, A); END_TYPE`

### V-TYPE-005: Enum explicit values must be integer constants
- **Rule**: If an enum value has an explicit assignment, it must be an integer constant.

### V-TYPE-006: String size must be positive integer constant
- **Rule**: The size specification in `STRING[n]` must be a positive integer constant.

### V-TYPE-007: No recursive type definitions
- **Rule**: A type cannot directly or indirectly contain itself (except through pointers/references).
- **Invalid**: `TYPE foo : STRUCT x : foo; END_STRUCT; END_TYPE`

### V-TYPE-008: OVERLAP only valid for STRUCT
- **Rule**: The `OVERLAP` keyword is only valid in struct type definitions.

### V-TYPE-009: No duplicate type names
- **Rule**: Type names must be unique within the compilation scope.

---

## Statement Validations

### V-STMT-001: EXIT only inside loop
- **Rule**: The `EXIT` statement is only valid inside `FOR`, `WHILE`, or `REPEAT` loops.

### V-STMT-002: CONTINUE only inside loop
- **Rule**: The `CONTINUE` statement is only valid inside `FOR`, `WHILE`, or `REPEAT` loops.

### V-STMT-003: RETURN value in FUNCTION
- **Rule**: In a `FUNCTION`, the return value is assigned to the function name, not via `RETURN expr`.
- **Note**: `RETURN` in a function simply exits; the function name variable holds the return value.

### V-STMT-004: FOR control variable must be integer type
- **Rule**: The control variable in a `FOR` loop must be of an integer type.

### V-STMT-005: FOR control variable must not be modified in body
- **Rule**: The control variable of a `FOR` loop must not be assigned within the loop body.

### V-STMT-006: CASE expression must be ordinal type
- **Rule**: The expression in a `CASE` statement must be of an ordinal type (integer, enum).

### V-STMT-007: CASE values must match expression type
- **Rule**: All case values/ranges must be compatible with the type of the case expression.

### V-STMT-008: CASE values must be unique
- **Rule**: Case values must not overlap within a single `CASE` statement.

### V-STMT-009: Assignment operator context
- **Rule**: 
  - `:=` is for regular assignment
  - `=>` is only valid for output parameter assignment in function/FB calls
  - `REF=` is for reference assignment

### V-STMT-010: Output assignment target must be variable
- **Rule**: The left side of `=>` assignment must be a variable that can receive a value.

---

## Expression Validations

### V-EXPR-001: Operator type compatibility
- **Rule**: Operands must be compatible with the operator:
  - Arithmetic operators (`+`, `-`, `*`, `/`, `MOD`, `**`) require numeric types
  - Comparison operators require comparable types
  - Boolean operators (`AND`, `OR`, `XOR`, `NOT`) require boolean or bit-string types

### V-EXPR-002: Division by zero prevention
- **Rule**: Division by a constant zero should be flagged (runtime detection for variables).

### V-EXPR-003: Array index must be integer
- **Rule**: Array subscript expressions must evaluate to integer types.

### V-EXPR-004: Array index bounds (if constant)
- **Rule**: If array indices are constants, they must be within the declared bounds.

### V-EXPR-005: Dereference requires pointer type
- **Rule**: The `^` dereference operator is only valid on pointer or reference types.

### V-EXPR-006: Field access requires structured type
- **Rule**: The `.` field access operator requires the left operand to be a struct, FB, or class instance.

### V-EXPR-007: REF() argument must be variable
- **Rule**: The argument to `REF()` must be a variable (l-value), not an expression.

### V-EXPR-008: Function call argument count and types
- **Rule**: Function/FB calls must provide the required arguments with compatible types.

### V-EXPR-009: THIS/SUPER only in class/FB context
- **Rule**: `THIS` and `SUPER` are only valid inside methods or properties of a class or function block.

### V-EXPR-010: SUPER requires parent class
- **Rule**: `SUPER` is only valid when the containing class/FB extends another class/FB.

---

## Modifier and Access Specifier Validations

### V-MOD-001: Only one access specifier
- **Rule**: Only one access specifier (`PUBLIC`, `PRIVATE`, `PROTECTED`, `INTERNAL`) may be specified.
- **Invalid**: `METHOD PUBLIC PRIVATE foo END_METHOD`

### V-MOD-002: ABSTRACT and FINAL mutually exclusive
- **Rule**: A POU or method cannot be both `ABSTRACT` and `FINAL`.

### V-MOD-003: ABSTRACT POU must not be instantiated
- **Rule**: An `ABSTRACT` class or function block cannot be directly instantiated.

### V-MOD-004: Concrete class must implement all abstract methods
- **Rule**: A non-abstract class/FB extending an abstract class/FB must implement all inherited abstract methods.

### V-MOD-005: Interface implementation completeness
- **Rule**: A class/FB that `IMPLEMENTS` an interface must provide implementations for all interface methods and properties.

### V-MOD-006: Access specifier visibility rules
- **Rule**: 
  - `PRIVATE` members are only accessible within the same POU
  - `PROTECTED` members are accessible in the same POU and derived POUs
  - `INTERNAL` members are accessible within the same namespace/project
  - `PUBLIC` members are accessible everywhere

### V-MOD-007: Access specifier not allowed in PROGRAM
- **Rule**: Access specifiers on `PROGRAM` declarations are typically not meaningful.

### V-MOD-008: Modifier order (stylistic)
- **Rule**: While the grammar is permissive, the recommended order is: `access_specifier` before `ABSTRACT`/`FINAL`/`OVERRIDE`.

---

## Additional Notes

### Permissive Parsing Philosophy

The grammar is designed to:
1. **Accept more than is valid** - Parse successfully even for invalid programs
2. **Enable better error recovery** - A permissive grammar can continue parsing after errors
3. **Support IDE features** - Incomplete code during editing should still parse
4. **Centralize validation** - All semantic checks happen in one validation phase

### Validation Severity Levels

Validations should be categorized by severity:
- **Error**: Must be fixed; code cannot compile/run
- **Warning**: Should be fixed; may cause unexpected behavior
- **Info**: Suggestion for improvement; code is technically valid

### Implementation Priority

Suggested implementation order:
1. Type compatibility validations (V-EXPR-001, V-DECL-004)
2. Scope and declaration validations (V-DECL-003, V-TYPE-009, V-POU-008)
3. Context validations (V-STMT-001, V-STMT-002, V-EXPR-009)
4. Modifier validations (V-MOD-001 through V-MOD-006)
5. Advanced OOP validations (V-MOD-004, V-MOD-005, V-METH-002 through V-METH-005)
