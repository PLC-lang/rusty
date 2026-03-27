# Changelog

All notable changes to this project will be documented in this file.

## [0.3.1](https://github.com/PLC-lang/rusty/releases/tag/v0.3.1) - 2026-03-27

### Fixed

- **ci**: Use content output from git-cliff-action for version detection (#1647) ([#1647](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Updated call lowering to ensure that output variables are correctly assigned and casted when necessary (#1618) ([#1618](https://github.com/PLC-lang/rusty/pull/{{ number }}))
## [0.3.0](https://github.com/PLC-lang/rusty/releases/tag/v0.3.0) - 2026-03-24

### Added

- Validations for polymorphism (#1630) ([#1630](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add interface upcasting support (#1629) ([#1629](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Polymorphic properties (#1619) ([#1619](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Support IEC hardware addresses in struct members (#1614) ([#1614](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Interface-based polymorphism (#1588) ([#1588](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Refactor initialisers (#1552) ([#1552](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Support DW_TAG_enumeration_type and DW_TAG_enumerator (#1548) ([#1548](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Make sure constructors don't get optimised out (#1540) ([#1540](https://github.com/PLC-lang/rusty/pull/{{ number }}))
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
- Add typed enums and allow type-specifier to be suffixed (#1544) ([#1544](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Support named arguments for builtins (#1529) ([#1529](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add semantic typedef wrappers for all pointer types in debug info (#1538) ([#1538](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Polymorphism (Classes and Function Blocks) (#1493) ([#1493](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Support setting the reference to 0
- Make auto-dereferencing pointers distinguishable in debug-info (#1503) ([#1503](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Function Pointers (#1492) ([#1492](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Initializer support for struct member fields (#1478) ([#1478](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add proper debug info for constants using DW_TAG_const_type (#1485) ([#1485](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- `THIS` keyword
- `FB_INIT` user code initialization (#1458) ([#1458](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Support `SUPER` keyword (#1445) ([#1445](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Use the target triple without unknown in tests, build in the action with multiple triples
- **cli**: Introduce `--ast-lowered` flag
- Global Namespace Operator (#1442) ([#1442](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Property support in interfaces (#1436) ([#1436](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Extensions of interfaces (#1425) ([#1425](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Properties (#1396) ([#1396](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add extension support for Function Block (#1402) ([#1402](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Lower aggregate return types  (#1379) ([#1379](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add method to register default participants with pipeline (#1392) ([#1392](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Derive try_from trait impl for inner AstStatement structs, add convenience macro (#1221) ([#1221](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Introduce pipeline participants (#1372) ([#1372](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add method support to init-lowering-stage (#1370) ([#1370](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Introduce interfaces (#1368) ([#1368](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Validate required `INPUT` and `IN_OUT` arguments for methods (#1364) ([#1364](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Introduce `TO_<DATE,TIME>` functions (#1356) ([#1356](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add `TO_<CHAR,STRING>` functions (#1355) ([#1355](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Method parser error, tests(#1358) ([#1358](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Introduce `TO_<BIT>` conversion functions (#1353) ([#1353](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Support referencing local variables in pointer initialization (#1346) ([#1346](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Introduce `TRUNC_<INT>(<REAL>)` functions (#1340) ([#1340](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Introduce `<REAL>_TRUNC_<INT>` functions
- Add additional pointer and type validations (#1341) ([#1341](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Introduce `TO_<NUM>` functions
- Add support for linker scripts (#1332) ([#1332](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Allow  `VAR_EXTERNAL` declarations (#1324) ([#1324](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Validate if template instance are configured (#1320) ([#1320](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Builtins ADR and REF now have a CONSTANT return specifier  (#1326) ([#1326](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Generate calls via the GOT
- Access references to globals through a custom GOT
- Generate a custom GOT array and save/load its layout
- Validate VAR_CONFIG and template variables (#1303) ([#1303](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Initialize configured template variables (#1317) ([#1317](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Validate types in math operations (#1300) ([#1300](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Parse VAR_CONFIG variables (#1299) ([#1299](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add support for hardware references in code (#1293) ([#1293](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **init**: Function support (#1285) ([#1285](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Aliased Hardware Access Variables (#1265) ([#1265](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **init**: Struct support (#1281) ([#1281](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Init functions for address-initialization (#1259) ([#1259](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Aliasing (#1258) ([#1258](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Introduce `REF=` and `REFERENCE_TO` (#1251) ([#1251](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add --ast CLI argument to emit the AST to stdout (#1256) ([#1256](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Validate argument count (#1233) ([#1233](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Validate Array Ranges (#1195) ([#1195](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Show diagnostics in `panic` in `runner::compile` (#1191) ([#1191](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Be more lenient when dealing with integer values in conditions (#1186) ([#1186](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Convert the `E091` error into a warning (#1177) ([#1177](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Exit with error status on compilation failure (#1183) ([#1183](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Validate action-calls without parentheses (#1170) ([#1170](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Validate `if` and `while` statements (#1140) ([#1140](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Registery based diagnostician (#1077) ([#1077](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Validate `for` loops (#1129) ([#1129](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Support void functions (#1103) ([#1103](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **validation**: Convert enum error to warning (#1120) ([#1120](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **parser**: Support optional semicolon at END_STRUCT keyword (#1110) ([#1110](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **validation**: Add further checks for enum validation (#1064) ([#1064](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Embed the plc json into the source (#1079) ([#1079](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **diagnostics**: Refactor the diagnostics to be more consistant (#1063) ([#1063](https://github.com/PLC-lang/rusty/pull/{{ number }}), [#826](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Improved error messages using slices (#1061) ([#1061](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **validation**: Assignment suggestions for `=` operator (#1049) ([#1049](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **builtins**: Symbols as builtins (#1012) ([#1012](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **validation**: Initializers of struct fields (#1032) ([#1032](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **validation**: Struct initializers within arrays (#996) ([#996](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **ast**: Introduce ParenExpression (#995) ([#995](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **plc.json-validation**: Add validation for build description file (#994) ([#994](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Action support in CFC (#981) ([#981](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Support jumps and labels from CFC (#969) ([#969](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **cfc**: Sink/source connections (#956) ([#956](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **cfc**: Conditional Returns (#950) ([#950](https://github.com/PLC-lang/rusty/pull/{{ number }}))

### Fixed

- Use is_signed_int instead of is_sized for SHR builtin (#1641) ([#1641](https://github.com/PLC-lang/rusty/pull/{{ number }}), [#251](https://github.com/PLC-lang/rusty/pull/{{ number }}), [#1](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Literal arrays are memcpy-ed instead of stored, non constant arrays are unrolled (#1633) ([#1633](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Var_temp indices are no longer referenced in call assignment
- Passing signed information to the expression generator to ensure that division uses the correct operation
- Preserving resolution of integer type sign to reduce warning messages in variadic functions
- Preserving resolution of integer type sign to reduce warning messages in variadic functions
- Addressed PR comments
- Added additional test case
- Passing signed information to the expression generator to ensure that division uses correct operation
- Prepare for rust 1.94 by fixing the conflicting assert calls (#1635) ([#1635](https://github.com/PLC-lang/rusty/pull/{{ number }}))
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
- Invalid string escapes now correctly report errors (#1607) ([#1607](https://github.com/PLC-lang/rusty/pull/{{ number }}), [#1593](https://github.com/PLC-lang/rusty/pull/{{ number }}), [#1601](https://github.com/PLC-lang/rusty/pull/{{ number }}), [#1603](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Var_input initial values were not passed correctly (#1595) ([#1595](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Changed 'division by zero' from warning to error
- Ensuring function outputs are correctly set as pointers in the header generator
- Addressed PR comments
- Ensuring function outputs are correctly set as pointers
- Changed 'division by zero' from warning to error
- Remove the generated headers (#1596) ([#1596](https://github.com/PLC-lang/rusty/pull/{{ number }}))
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
- When assigning arrays of structs, make sure they are annotated correctly (#1589) ([#1589](https://github.com/PLC-lang/rusty/pull/{{ number }}))
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
- **debug**: Prevent stack overflow on recursive type debug info (#1580) ([#1580](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Update LLVM version to 21 in Dockerfile (#1579) ([#1579](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Addressed PR comments
- Extended function block body calls (#1545) ([#1545](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Array var-args decay to pointers (#1560) ([#1560](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Reference nested in parenthesis is correctly assigned
- Addressed pr suggestions
- Reference nested in parenthesis is correctly assigned
- Cargo fmt
- Ensuring that output variables cannot be set outside of their scope
- Moved shl and shr library functions to builtin
- Resolve flaky linking test race condition
- Free up disk space for test-linux job
- Free up disk space for runner… (#1573) ([#1573](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Addressed PR comments
- Panic when using equality rather than assignment operator in enum defnition (#1558) ([#1558](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Run each lit test in its own directory to avoid conflicts in tests (#1569) ([#1569](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Added validation to SHR and SHL builtin methods
- Moved shl and shr library functions to builtin and made them compatible with integers
- Alias subrange-type debuginfo with start & end range and backing type (#1551) ([#1551](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Rename parent struct member to `SUPER` in debug-info (#1550) ([#1550](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Relax enum variant validation to warning for compatibility (#1547) ([#1547](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Fetch all object files in a library path (#1541) ([#1541](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Validation of passing enum as var_in_out (#1521) ([#1521](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Fetch virtual table dependencies in resolver (#1535) ([#1535](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- By-ref ARRAY OF ARRAY fn signatures, callsites and GEP stride calculation (#1525) ([#1525](https://github.com/PLC-lang/rusty/pull/{{ number }}), [#1389](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Resolve LREAL literals to the correct type in initializers (#1526) ([#1526](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add missing dependency tracking for subrange type references (#1520) ([#1520](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Recursive type definitions (#1514) ([#1514](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Subrange should be treated as typedefs (#1519) ([#1519](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Make sure type validation for ref= only happens on references (#1517) ([#1517](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add bats test and codegen tests for cases when calling methods from function blocks or extended function blocks and outside
- Unary plus operator in expressions (#1513) ([#1513](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Ignore parameters with default values if they are at the end
- Make sure vla accessors are cast to 32bit
- Build script for macOS development (#1491) ([#1491](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Make strings distinguishable from char arrays in debug info (#1496) ([#1496](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Handle generics with no return type (#1484) ([#1484](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Do not generate debug information for variables outside of compilation unit (#1486) ([#1486](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Disable type-checks for `POINTER TO` variables (#1482) ([#1482](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Method and action IR names are now separated by `__` instead of `_` (#1471) ([#1471](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add target datalayout and triple to IR (#1472) ([#1472](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Debug info alignment (#1468) ([#1468](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Make method and action IR call-names FFI compatible (#1465) ([#1465](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Set the rustc_target variable
- When generating debug info, only consider dependencies (#1450) ([#1450](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Stack-overflow safeguard for `find_local_member` (#1440) ([#1440](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Stackoverflow when parent name contains child-name separated by a _ (#1439) ([#1439](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **parser**: Array access on fn-call result (#1431) ([#1431](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Improve unresolved reference validation in array access (#1432) ([#1432](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Recursive type check no longer fails on duplicate fields
- Add the end keyword as a debug statement
- Don't generate debug info for the initialization logic of a POU
- Various stack overflows in recursive functions when dealing with… (#1424) ([#1424](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Improve location of reported signature mismatch validations (#1420) ([#1420](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Interface default implementation is now an error (#1421) ([#1421](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Docker-CI needs access to the artifacts (#1427) ([#1427](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Don't consider action containers outside programs, functions or function_blocks (#1426) ([#1426](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Aggregate return type in interface validation (#1410) ([#1410](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Update code-block formatting for error-codes in book (#1413) ([#1413](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Mut_visitor no longer clones (#1394) ([#1394](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Parse IMPLEMENTS keyword after EXTENDS (#1391) ([#1391](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add write_got call to new pipeline (#1385) ([#1385](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Make the online change flag global (#1363) ([#1363](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Generate debug metadata for methods (#1384) ([#1384](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Remove alignment from zero-sized types (#1378) ([#1378](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Methods are now callable locally (#1361) ([#1361](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Array assignments with different inner type are now an error (#1357) ([#1357](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- DebugInfo alignment and offset for aggregate types (#1329) ([#1329](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Remove POU context (#1318) ([#1318](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Debug locations of CASE statement (#1315) ([#1315](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Location of variables declared in VAR_CONFIG (#1304) ([#1304](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Generated globals are not duplicates (#1301) ([#1301](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Internal filemarker (#1294) ([#1294](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Declare external pou initializers as external (#1290) ([#1290](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Potential stack-overflow in index look-up (#1273) ([#1273](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Parse REFERENCE TO STRING (#1267) ([#1267](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Match Option value when parsing alias variables (#1266) ([#1266](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- For loops no longer execute once when condition is already met (#1248) ([#1248](https://github.com/PLC-lang/rusty/pull/{{ number }}), [#1207](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Bitaccess for VAR_OUTPUT (#1214) ([#1214](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Aggregate output assignments (#1234) ([#1234](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Do not interpret `&` as ref-operator (#1219) ([#1219](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Enum-variant names no longer clash (#1188) ([#1188](https://github.com/PLC-lang/rusty/pull/{{ number }}), [#1182](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Array sizes in DWARF (#1190) ([#1190](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- False-positive downcast warnings (#1114) ([#1114](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Derive root directory from unit (#1153) ([#1153](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add macOS triplet to linking test (#1149) ([#1149](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Bitwise-and bitaccess booleans with bitmask (#1143) ([#1143](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- String tests no longer segfault when testing with optimization (#1142) ([#1142](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Use memcpy for aggregate types in builtin `MUX` function (#1131) ([#1131](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **validation**: Add duplicate check for locally defined enum variants (#1099) ([#1099](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **index**: Precedence on inline enum assignments (#1098) ([#1098](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **index**: Include enum variants in local variable search (#1092) ([#1092](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Stop compilation at parser errors (#1107) ([#1107](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Linking and Include folders now work again (#1102) ([#1102](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **codegen**: Don't fail if a const value can not be found (#1067) ([#1067](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **linker**: Avoid filename clashes when linking (#1086) ([#1086](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **debugging**: Don't create debug info for external functions/add LLVM-version-appropriate DI-metadata (#1072) ([#1072](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **validation**: Fix false-positive unresolved generic symbol validation for formal parameters (#1066) ([#1066](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **timers**: Add internal flag to track state (#1068) ([#1068](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Only validate Action-VAR_IN_OUT parameters when necessary (#1057) ([#1057](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **validation**: Don't suggest brackets for call statements when dealing with arrays (#1059) ([#1059](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Dont report generic implementations for duplicates (#1054) ([#1054](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **codegen**: Convert pointer to actual type when passing by-value (#1039) ([#1039](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **stdlib**: Endian conversion functions (#1029) ([#1029](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **resolver**: Annotate assignments with a hint when dealing with structs (#1020) ([#1020](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Schema validation now uses binary location (#1011) ([#1011](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **cfc**: Call statements for function blocks (#1006) ([#1006](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Make sure actions called from other actions get resolved (#980) ([#980](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Make sure the plc json path is relative (#972) ([#972](https://github.com/PLC-lang/rusty/pull/{{ number }}), [#971](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Detect invalid enum variants in var blocks. (#904) ([#904](https://github.com/PLC-lang/rusty/pull/{{ number }}))

### Performance

- Use FxHash (#1224) ([#1224](https://github.com/PLC-lang/rusty/pull/{{ number }}))

### Refactored

- Finalize LLVM update
- Introduce `StatementAnnotation::Argument` (#1470) ([#1470](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Debug implementation of `TextLocation` (#1457) ([#1457](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Remove initializer-lowering from `test_utils::annotate_and_lower_with_ids` (#1466) ([#1466](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Rename `units` field to `pous` (#1438) ([#1438](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Removing backing property field (#1418) ([#1418](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Remove "DataType"-prefix from "DataTypeDeclaration" enum-variants (#1398) ([#1398](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Make use of 'From<T> for SourceLocation' (#1377) ([#1377](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Make init-unit-generation a participant (#1374) ([#1374](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **indexer**: Indexing uses SymbolIndexer instead of visitor.rs (#1245) ([#1245](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Parse VAR_CONFIG variables left-hand-side as references (#1316) ([#1316](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Make second argument in ADD / MUL variadic (#1312) ([#1312](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Date and commit hash in --version flag (#1311) ([#1311](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **AST**: Introduce AstVisitor trait (#1231) ([#1231](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Use memcpy for by-val aggregate type input parameters (#1196) ([#1196](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Implement From<&SourceLocation> (#1197) ([#1197](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Enum Representation in Index (#1175) ([#1175](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Deprecate expect() in mod lexer(#196) (#1113) ([#196](https://github.com/PLC-lang/rusty/pull/{{ number }}), [#1113](https://github.com/PLC-lang/rusty/pull/{{ number }}), [#196](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Suppress printing of LLVM IR in test runs (#1150) ([#1150](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Simplify is_valid_bcd (#1145) ([#1145](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Use the buffered validator (#1076) ([#1076](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Introduce `GlobalContext` (#1058) ([#1058](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add Pre-commit + editorconfig + gitattributes (#1034) ([#1034](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **cfc**: Serializer API (#967) ([#967](https://github.com/PLC-lang/rusty/pull/{{ number }}))

### Documentation

- Update macOS instructions (#1602) ([#1602](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Improve Build & Install section (#1609) ([#1609](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Added missing usage documentation
- Update API guidelines to account for recent polymorphism changes (#1533) ([#1533](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Update macOS installation instructions (#1530) ([#1530](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Update build instructions for Ubuntu (#1395) ([#1395](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Markdown typo (#1387) ([#1387](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Fix lowercase typo in examples/action.st
- Add documentation for pointer initializers (#1321) ([#1321](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add guide for creating plc libraries (#1213) ([#1213](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add error codes to the book (#1200) ([#1200](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Add `lld` dependency (#1157) ([#1157](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- **cfc**: Architecture (#1007) ([#1007](https://github.com/PLC-lang/rusty/pull/{{ number }}))

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

- Introduce a metrics workflow (#786) ([#786](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Allow mulitple files in compilation ([#91](https://github.com/PLC-lang/rusty/pull/{{ number }}))

### Fixed

- Allow empty parameter assignment for Output variables (#827) ([#827](https://github.com/PLC-lang/rusty/pull/{{ number }}))
- Casting to `REAL` and `LREAL` no longer yields internal errors (#801) ([#801](https://github.com/PLC-lang/rusty/pull/{{ number }}))

### CLI

- Fix invalid filenames not printing an error (#235) ([#235](https://github.com/PLC-lang/rusty/pull/{{ number }}))

### Std

- :mem:take diagnostics
## [0.1.0](https://github.com/PLC-lang/rusty/releases/tag/v0.1.0) - 2021-04-20
