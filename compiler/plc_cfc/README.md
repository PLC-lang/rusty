# plc_cfc

**What this is**: A from-scratch CFC (Continuous Function Chart) frontend for RuSTy. CFC is a graphical language: a `.cfc` file is XML describing blocks and the connections between them rather than textual statements. This crate transpiles that XML into the same AST the ST parser produces, so a PLC project can freely mix `.st` and `.cfc` files.

**The format**: Based on the official IEC 61131-10 XML exchange format. The standard, its XSD schemas and example documents live in `assets/`. Deliberate deviations from the strict standard are documented where they happen, in the code.

**How it fits in**: The entry point is `parse_file`, signature-compatible with the ST parser's `plc::parser::parse_file`, so the driver only needs to dispatch on the source type. Everything downstream of parsing (indexing, validation, codegen) is shared with ST and lives outside this crate.

**The predecessor**: An older CFC implementation exists in `compiler/plc_xml`, built on a custom PLCopen-XML-based format. This crate replaces it. The old code may be consulted for *logic* (what a construct should lower to), but with caution — it was more of a proof of concept with lots of open issues — and no code is derived from it.

**Where to look**: This README is intentionally rough — the code is the source of truth. Start at `src/lib.rs` and follow the pipeline from there.
