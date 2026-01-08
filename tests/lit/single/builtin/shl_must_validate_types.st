// RUN: (%COMPILE %s) | %CHECK %s
// XFAIL: *
// SHL must validate types
FUNCTION main
VAR
END_VAR
SHL('foo',2);
END_FUNCTION

// error[E031]: Invalid expression, types STRING and INT are incompatible in the given context
//   ┌─ /workspaces/rusty/tests/lit/single/builtin/shl_must_validate_types.st:7:5
//   │
// 7 │ SHL('foo',2);
//   │     ^^^^^ Invalid expression, types STRING and INT are incompatible in the given context
