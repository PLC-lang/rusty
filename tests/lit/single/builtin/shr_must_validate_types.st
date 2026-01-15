// RUN: (%COMPILE %s) | %CHECK %s
// XFAIL: *
// SHR must validate types
FUNCTION main
VAR
END_VAR
SHR('foo',2);
END_FUNCTION

// error[E031]: Invalid expression, types STRING and INT are incompatible in the given context
//   ┌─ /workspaces/rusty/tests/lit/single/builtin/shr_must_validate_types.st:7:5
//   │
// 7 │ SHR('foo',2);
//   │     ^^^^^ Invalid expression, types STRING and INT are incompatible in the given context
