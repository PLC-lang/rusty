# Changelog

All notable changes to this project will be documented in this file.

## [0.5.0](https://github.com/PLC-lang/rusty/releases/tag/v0.5.0) - 2026-04-27

### Added

- Migrate property syntax to IEC 61131-3 2025 edition (#1688)

### Fixed

- Allow mixing of implicit and explicit arguments (#1698)
- Preserve lowering diagnostics through validation (#1699)
- Support REFERENCE_TO type declaration and add validation (#1682)
- **validation**: Prevent array-bound panics and diagnose non-constant bounds (#1683)
- Add a validation for non-integer array bounds (#1679)
- Always upcast unsized variadics to 32 bit (#1695)
- Added lowering mechanism to support functions with the reference to * return type (#1644)
- Enum initialized with typed literal is now annotated with correct type (#1689)
- **validation**: Support REFERENCE TO array-of-pointer assignment semantics (#1684)
- Remove the false positive on variadics (#1685)
- **validation**: Handle implicit constant array defaults correctly (#1687)
## [0.4.0](https://github.com/PLC-lang/rusty/releases/tag/v0.4.0) - 2026-04-20

### Added

- Warn if an array gets too big (#1665)
- Reject direct interface calls (#1666)
- Loop desugaring (#1655)
- Validation for trailing access on property setter (#1661)
- **validation**: Warn on array initialization with fewer elements than expected (#1645)
- Improve the relocation strategies with -fpic and -fno-pic instead of --pic and --no-pic which were not doing what's expected (#1638)

### Fixed

- Regression ctors were causing functions not to be debuggable (#1681)
- Treat GET and SET as contextual property keywords (#1668)
- Add a debug line on REF= statements (#1680)
- Updated call lowering to include direct access in call assignment when lowering output assignment (#1657)
- Fixed real parsing for 'prettily' formatted numbers (#1672)
- TP timer no longer panics in specific cases (#1667)
- Ensuring conditional statements are evaluated correctly (#1669)
- Also consider references in array validation (#1660)
- Added validation to block constant reference or pointer assignment (#1654)
- Ignore noisy E015 POINTER TO warning (#1658)
- Resolve itable member type dependencies for Value annotations (#1650)
- Added header generation support for ref_to arrays (#1652)
- Added support for the internal void type in generated headers (#1646)
- **ci**: Use content output from git-cliff-action for version detection (#1647)
- Updated call lowering to ensure that output variables are correctly assigned and casted when necessary (#1618)

### Refactored

- Hoist allocas on function entry (#1656)
## [0.3.0](https://github.com/PLC-lang/rusty/releases/tag/v0.3.0) - 2026-03-24

### Added

- Validations for polymorphism (#1630)
- Add interface upcasting support (#1629)
- Polymorphic properties (#1619)
- Support IEC hardware addresses in struct members (#1614)
- Interface-based polymorphism (#1588)
- Refactor initialisers (#1552)
- Support DW_TAG_enumeration_type and DW_TAG_enumerator (#1548)
- Make sure constructors don't get optimised out (#1540)
- Added a header generator for C and supporting CLI commands to invoke the generation of these headers
- Addressed pr comments and reduced number of snapshots
- Added support for function pointers
- Changes post review of m100-plc-libs
- Added additional CLI to support a top level command for header generation
- Added end-to-end tests and added some missing components
- Documentation and minor refactors
- Adding documentation for the various new features
- Finalised refactor and test cases
- Some directory adjustments
- Formatting fixes
- Completed refactor
- Refactored managers to be traits removing the oop pattern
- Updated tests to include participants
- Included new test cases for header generation
- Added new test cases and framework for re-using snapshots
- Added function blocks and programs to the header generator
- Added several complex types and refactored to allow for easier additions of further languages
- Added primitive types and functions that use primitive types to the header generator
- Added CLI options for generating headers
- Add typed enums and allow type-specifier to be suffixed (#1544)
- Support named arguments for builtins (#1529)
- Add semantic typedef wrappers for all pointer types in debug info (#1538)
- Polymorphism (Classes and Function Blocks) (#1493)
- Support setting the reference to 0
- Make auto-dereferencing pointers distinguishable in debug-info (#1503)
- Function Pointers (#1492)
- Initializer support for struct member fields (#1478)
- Add proper debug info for constants using DW_TAG_const_type (#1485)
- `THIS` keyword
- `FB_INIT` user code initialization (#1458)
- Support `SUPER` keyword (#1445)
- Use the target triple without unknown in tests, build in the action with multiple triples
- **cli**: Introduce `--ast-lowered` flag
- Global Namespace Operator (#1442)
- Property support in interfaces (#1436)
- Extensions of interfaces (#1425)
- Properties (#1396)
- Add extension support for Function Block (#1402)
- Lower aggregate return types  (#1379)
- Add method to register default participants with pipeline (#1392)
- Derive try_from trait impl for inner AstStatement structs, add convenience macro (#1221)
- Introduce pipeline participants (#1372)
- Add method support to init-lowering-stage (#1370)
- Introduce interfaces (#1368)
- Validate required `INPUT` and `IN_OUT` arguments for methods (#1364)
- Introduce `TO_<DATE,TIME>` functions (#1356)
- Add `TO_<CHAR,STRING>` functions (#1355)
- Method parser error, tests(#1358)
- Introduce `TO_<BIT>` conversion functions (#1353)
- Support referencing local variables in pointer initialization (#1346)
- Introduce `TRUNC_<INT>(<REAL>)` functions (#1340)
- Introduce `<REAL>_TRUNC_<INT>` functions
- Add additional pointer and type validations (#1341)
- Introduce `TO_<NUM>` functions
- Add support for linker scripts (#1332)
- Allow  `VAR_EXTERNAL` declarations (#1324)
- Validate if template instance are configured (#1320)
- Builtins ADR and REF now have a CONSTANT return specifier  (#1326)
- Generate calls via the GOT
- Access references to globals through a custom GOT
- Generate a custom GOT array and save/load its layout
- Validate VAR_CONFIG and template variables (#1303)
- Initialize configured template variables (#1317)
- Validate types in math operations (#1300)
- Parse VAR_CONFIG variables (#1299)
- Add support for hardware references in code (#1293)
- **init**: Function support (#1285)
- Aliased Hardware Access Variables (#1265)
- **init**: Struct support (#1281)
- Init functions for address-initialization (#1259)
- Aliasing (#1258)
- Introduce `REF=` and `REFERENCE_TO` (#1251)
- Add --ast CLI argument to emit the AST to stdout (#1256)
- Validate argument count (#1233)
- Validate Array Ranges (#1195)
- Show diagnostics in `panic` in `runner::compile` (#1191)
- Be more lenient when dealing with integer values in conditions (#1186)
- Convert the `E091` error into a warning (#1177)
- Exit with error status on compilation failure (#1183)
- Validate action-calls without parentheses (#1170)
- Validate `if` and `while` statements (#1140)
- Registery based diagnostician (#1077)
- Validate `for` loops (#1129)
- Support void functions (#1103)
- **validation**: Convert enum error to warning (#1120)
- **parser**: Support optional semicolon at END_STRUCT keyword (#1110)
- **validation**: Add further checks for enum validation (#1064)
- Embed the plc json into the source (#1079)
- **diagnostics**: Refactor the diagnostics to be more consistant (#1063)
- Improved error messages using slices (#1061)
- **validation**: Assignment suggestions for `=` operator (#1049)
- **builtins**: Symbols as builtins (#1012)
- **validation**: Initializers of struct fields (#1032)
- **validation**: Struct initializers within arrays (#996)
- **ast**: Introduce ParenExpression (#995)
- **plc.json-validation**: Add validation for build description file (#994)
- Action support in CFC (#981)
- Support jumps and labels from CFC (#969)
- **cfc**: Sink/source connections (#956)
- **cfc**: Conditional Returns (#950)

### Fixed

- Use is_signed_int instead of is_sized for SHR builtin (#1641)
- Literal arrays are memcpy-ed instead of stored, non constant arrays are unrolled (#1633)
- Var_temp indices are no longer referenced in call assignment
- Passing signed information to the expression generator to ensure that division uses the correct operation
- Preserving resolution of integer type sign to reduce warning messages in variadic functions
- Preserving resolution of integer type sign to reduce warning messages in variadic functions
- Addressed PR comments
- Added additional test case
- Passing signed information to the expression generator to ensure that division uses correct operation
- Prepare for rust 1.94 by fixing the conflicting assert calls (#1635)
- Addressed PR comments
- 'else if' is now lowered into an else > if > else
- Formatting
- Addressed PR comments
- Corrected errors created in merge
- 'else if' is now lowered into an else > if > else
- Validation error added for call assignment to void
- Addressed PR comments
- Validation error added for call assignment to void
- Snapshots post merge
- Ensuring that optional parameters are evaluated correctly
- Ensuring that optional parameters are evaluated correctly
- Var_temp indices are no longer referenced in call assignment
- Added failing test
- Ensuring ADR assignment on REFERENCE TO variables raises an error
- Ensuring output assignment is validated correctly
- Ensuring output assignment is validated correctly
- Formatting
- Addressed PR comment
- Invalid string escapes now correctly report errors (#1607)
- Var_input initial values were not passed correctly (#1595)
- Changed 'division by zero' from warning to error
- Ensuring function outputs are correctly set as pointers in the header generator
- Addressed PR comments
- Ensuring function outputs are correctly set as pointers
- Changed 'division by zero' from warning to error
- Remove the generated headers (#1596)
- Removed erroneous declaration movement
- Ensuring ADR assignment on REFERENCE TO variables raises an error
- Ensuring that zero division is validated
- Added support for multi dimensional array syntax
- Changed enum implementation in C to support bigger int types
- Addressed further PR comments
- Resolving alias dependency order
- Addressed PR comments and verified enum values
- Added variadic lit tests for header generator
- Addresed further PR comments
- Resolved further PR comments
- Addressed some of the PR comments
- Cargo fmt
- Addressed comments from the walkthrough
- Cargo fmt
- Merged latest master
- Cargo fmt
- Ensuring global variables are declared for function pointers
- Cargo fmt
- Accepted snapshot changes post merge
- Added option to specify header ouput directories on the top level command
- Accepting snapshot changes after the master merge
- Skipping compile on some documentation
- Cargo fmt
- Cargo fmt
- Small formatting change to reduce complexity
- When assigning arrays of structs, make sure they are annotated correctly (#1589)
- Addressed PR comments
- Ensuring that zero division is validated
- Added test for proving the failure
- Removed duplicate symbol validation for enum variants
- Ensuring that builtins honour named parameters out of order
- Removed magic string
- Ensuring that builtins honour named parameters out of order
- Removed duplicate symbol validation for enum variants
- Ensuring var validation doesn't block against enum symbols
- Same variable length array size in different files no longer causes an ambiguous data type error
- Same variable length array size in different files no longer causes an ambiguous data type error
- Ensuring that output variables cannot be set outside of their scope
- Typos and formatting
- **debug**: Prevent stack overflow on recursive type debug info (#1580)
- Update LLVM version to 21 in Dockerfile (#1579)
- Addressed PR comments
- Extended function block body calls (#1545)
- Array var-args decay to pointers (#1560)
- Reference nested in parenthesis is correctly assigned
- Addressed pr suggestions
- Reference nested in parenthesis is correctly assigned
- Cargo fmt
- Ensuring that output variables cannot be set outside of their scope
- Moved shl and shr library functions to builtin
- Resolve flaky linking test race condition
- Free up disk space for test-linux job
- Free up disk space for runner… (#1573)
- Addressed PR comments
- Panic when using equality rather than assignment operator in enum defnition (#1558)
- Run each lit test in its own directory to avoid conflicts in tests (#1569)
- Added validation to SHR and SHL builtin methods
- Moved shl and shr library functions to builtin and made them compatible with integers
- Alias subrange-type debuginfo with start & end range and backing type (#1551)
- Rename parent struct member to `SUPER` in debug-info (#1550)
- Relax enum variant validation to warning for compatibility (#1547)
- Fetch all object files in a library path (#1541)
- Validation of passing enum as var_in_out (#1521)
- Fetch virtual table dependencies in resolver (#1535)
- By-ref ARRAY OF ARRAY fn signatures, callsites and GEP stride calculation (#1525)
- Resolve LREAL literals to the correct type in initializers (#1526)
- Add missing dependency tracking for subrange type references (#1520)
- Recursive type definitions (#1514)
- Subrange should be treated as typedefs (#1519)
- Make sure type validation for ref= only happens on references (#1517)
- Add bats test and codegen tests for cases when calling methods from function blocks or extended function blocks and outside
- Unary plus operator in expressions (#1513)
- Ignore parameters with default values if they are at the end
- Make sure vla accessors are cast to 32bit
- Build script for macOS development (#1491)
- Make strings distinguishable from char arrays in debug info (#1496)
- Handle generics with no return type (#1484)
- Do not generate debug information for variables outside of compilation unit (#1486)
- Disable type-checks for `POINTER TO` variables (#1482)
- Method and action IR names are now separated by `__` instead of `_` (#1471)
- Add target datalayout and triple to IR (#1472)
- Debug info alignment (#1468)
- Make method and action IR call-names FFI compatible (#1465)
- Set the rustc_target variable
- When generating debug info, only consider dependencies (#1450)
- Stack-overflow safeguard for `find_local_member` (#1440)
- Stackoverflow when parent name contains child-name separated by a _ (#1439)
- **parser**: Array access on fn-call result (#1431)
- Improve unresolved reference validation in array access (#1432)
- Recursive type check no longer fails on duplicate fields
- Add the end keyword as a debug statement
- Don't generate debug info for the initialization logic of a POU
- Various stack overflows in recursive functions when dealing with… (#1424)
- Improve location of reported signature mismatch validations (#1420)
- Interface default implementation is now an error (#1421)
- Docker-CI needs access to the artifacts (#1427)
- Don't consider action containers outside programs, functions or function_blocks (#1426)
- Aggregate return type in interface validation (#1410)
- Update code-block formatting for error-codes in book (#1413)
- Mut_visitor no longer clones (#1394)
- Parse IMPLEMENTS keyword after EXTENDS (#1391)
- Add write_got call to new pipeline (#1385)
- Make the online change flag global (#1363)
- Generate debug metadata for methods (#1384)
- Remove alignment from zero-sized types (#1378)
- Methods are now callable locally (#1361)
- Array assignments with different inner type are now an error (#1357)
- DebugInfo alignment and offset for aggregate types (#1329)
- Remove POU context (#1318)
- Debug locations of CASE statement (#1315)
- Location of variables declared in VAR_CONFIG (#1304)
- Generated globals are not duplicates (#1301)
- Internal filemarker (#1294)
- Declare external pou initializers as external (#1290)
- Potential stack-overflow in index look-up (#1273)
- Parse REFERENCE TO STRING (#1267)
- Match Option value when parsing alias variables (#1266)
- For loops no longer execute once when condition is already met (#1248)
- Bitaccess for VAR_OUTPUT (#1214)
- Aggregate output assignments (#1234)
- Do not interpret `&` as ref-operator (#1219)
- Enum-variant names no longer clash (#1188)
- Array sizes in DWARF (#1190)
- False-positive downcast warnings (#1114)
- Derive root directory from unit (#1153)
- Add macOS triplet to linking test (#1149)
- Bitwise-and bitaccess booleans with bitmask (#1143)
- String tests no longer segfault when testing with optimization (#1142)
- Use memcpy for aggregate types in builtin `MUX` function (#1131)
- **validation**: Add duplicate check for locally defined enum variants (#1099)
- **index**: Precedence on inline enum assignments (#1098)
- **index**: Include enum variants in local variable search (#1092)
- Stop compilation at parser errors (#1107)
- Linking and Include folders now work again (#1102)
- **codegen**: Don't fail if a const value can not be found (#1067)
- **linker**: Avoid filename clashes when linking (#1086)
- **debugging**: Don't create debug info for external functions/add LLVM-version-appropriate DI-metadata (#1072)
- **validation**: Fix false-positive unresolved generic symbol validation for formal parameters (#1066)
- **timers**: Add internal flag to track state (#1068)
- Only validate Action-VAR_IN_OUT parameters when necessary (#1057)
- **validation**: Don't suggest brackets for call statements when dealing with arrays (#1059)
- Dont report generic implementations for duplicates (#1054)
- **codegen**: Convert pointer to actual type when passing by-value (#1039)
- **stdlib**: Endian conversion functions (#1029)
- **resolver**: Annotate assignments with a hint when dealing with structs (#1020)
- Schema validation now uses binary location (#1011)
- **cfc**: Call statements for function blocks (#1006)
- Make sure actions called from other actions get resolved (#980)
- Make sure the plc json path is relative (#972)
- Detect invalid enum variants in var blocks. (#904)

### Performance

- Use FxHash (#1224)

### Refactored

- Finalize LLVM update
- Introduce `StatementAnnotation::Argument` (#1470)
- Debug implementation of `TextLocation` (#1457)
- Remove initializer-lowering from `test_utils::annotate_and_lower_with_ids` (#1466)
- Rename `units` field to `pous` (#1438)
- Removing backing property field (#1418)
- Remove "DataType"-prefix from "DataTypeDeclaration" enum-variants (#1398)
- Make use of 'From<T> for SourceLocation' (#1377)
- Make init-unit-generation a participant (#1374)
- **indexer**: Indexing uses SymbolIndexer instead of visitor.rs (#1245)
- Parse VAR_CONFIG variables left-hand-side as references (#1316)
- Make second argument in ADD / MUL variadic (#1312)
- Date and commit hash in --version flag (#1311)
- **AST**: Introduce AstVisitor trait (#1231)
- Use memcpy for by-val aggregate type input parameters (#1196)
- Implement From<&SourceLocation> (#1197)
- Enum Representation in Index (#1175)
- Deprecate expect() in mod lexer(#196) (#1113)
- Suppress printing of LLVM IR in test runs (#1150)
- Simplify is_valid_bcd (#1145)
- Use the buffered validator (#1076)
- Introduce `GlobalContext` (#1058)
- Add Pre-commit + editorconfig + gitattributes (#1034)
- **cfc**: Serializer API (#967)

### Documentation

- Update macOS instructions (#1602)
- Improve Build & Install section (#1609)
- Added missing usage documentation
- Update API guidelines to account for recent polymorphism changes (#1533)
- Update macOS installation instructions (#1530)
- Update build instructions for Ubuntu (#1395)
- Markdown typo (#1387)
- Fix lowercase typo in examples/action.st
- Add documentation for pointer initializers (#1321)
- Add guide for creating plc libraries (#1213)
- Add error codes to the book (#1200)
- Add `lld` dependency (#1157)
- **cfc**: Architecture (#1007)

### Cli

- Make --got-layout-file override the default GOT layout file location

### Codegen

- Mangle type aliases properly
- Handle more complex types in mangle_type
- Mangle struct and enum types properly
- Propagate errors better when creating section names
- Add mangling for global variables

### Rusty

- Update test snapshots
- Add --online-change CLI flag
- Work around race conditions when compiling multiple modules
- Update insta testsuite for section mangling.
- Accept snapshots with new section names
- Update testsuite for new global variable section names

### Section-mangler

- Add prefix constant, derive Debug and PartialEq
- Add base for handling arrays

### Section_mangler

- Add test for parsing qualified name in section name
- Add parsing of mangled functions
- Add .name() method
- Derive Clone on SectionMangler
- Add decoding of enum types
- Add struct decoding
- Add base for decoding mangled names
- Encode structs and enums
- Change API for `SectionMangler::with_return_type`

### Variable_generator

- Use qualified name when mangling variable sections

### Wip

- More tests for in/out methods
- Test for input/output vars in extended methods
- Feat: functions support optional params
## [0.2.0](https://github.com/PLC-lang/rusty/releases/tag/v0.2.0) - 2023-06-23

### Added

- Introduce a metrics workflow (#786)
- Allow mulitple files in compilation

### Fixed

- Allow empty parameter assignment for Output variables (#827)
- Casting to `REAL` and `LREAL` no longer yields internal errors (#801)

### CLI

- Fix invalid filenames not printing an error (#235)

### Std

- :mem:take diagnostics
## [0.1.0](https://github.com/PLC-lang/rusty/releases/tag/v0.1.0) - 2021-04-20
