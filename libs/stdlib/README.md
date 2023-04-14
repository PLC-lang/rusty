# StandardFunctions
Standard functions for PLC as described by IEC61131

## Structure
The project is managed as a rust project but contains sections implemented directly in ST

### ST
The ST implementation is used for simple functions that are described by the IEC61131-3 standard
It can be compiled using [RuSTy](https://github.com/PLC-lang/rusty)

### Rust
The Rust implementation contains the more complex functions required by the IEC61131-3 standard
These include string or array functions as well as function blocks dealing with Time or other system calls

## Tests
Testing the library is done using rust, the ST parts of the tests are achieved using ruSTy as a library
To test an ST based function, you can use the `use_std` macro included in the tests' `common` module
Rust based ST functions cannot be tested yet, this will be extended later
