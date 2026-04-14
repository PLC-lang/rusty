# Compatibility Profiles

Different PLC environments (IEC 61131-3 standard, CODESYS, etc.) have different semantics for certain language features.
Compatibility profiles let you control how the compiler behaves — affecting runtime behavior, validation, and the type system — so that the compiled output matches the expectations of your target environment.

## The `--profile` flag

```bash
plc --profile <name-or-path> input.st
```

The `--profile` flag accepts either a **built-in profile name** or a **path to a profile file**.
It is a global flag and works with all subcommands (`build`, `check`, `config`, `generate`).

```bash
# Use a built-in profile
plc --profile standard input.st

# Use a custom profile file
plc --profile my-profile.json input.st

# Works with the build subcommand
plc --profile codesys build project.json
```

If no `--profile` is specified, the compiler defaults to `codesys`.

## Built-in profiles

| Profile    | Description |
|------------|-------------|
| `codesys`  | **Default.** Produces output compatible with CODESYS and similar runtimes. |
| `standard` | Follows IEC 61131-3 strict semantics where they differ from CODESYS behavior. |

## Profile file format

A profile is a JSON or TOML file with three sections:

```json
{
  "name": "my-custom-profile",
  "behaviors": {},
  "diagnostics": {
    "ignore": ["E015"],
    "warning": [],
    "error": []
  }
}
```

Or in TOML:

```toml
name = "my-custom-profile"

[behaviors]

[diagnostics]
ignore = ["E015"]
warning = []
error = []
```

### Sections

- **`name`** (optional): A human-readable label for the profile.
- **`behaviors`**: Flags that control compiler semantics. Each flag may affect runtime behavior, validation rules, or the type system. All flags are optional — when omitted, they default to CODESYS-compatible behavior.
- **`diagnostics`**: Severity overrides for diagnostic error codes. This is a map from severity level (`ignore`, `info`, `warning`, `error`) to lists of error codes. Use this to suppress warnings, promote them to errors, or ignore specific diagnostics. See [Error Configuration](./error_configuration.md) for details on error codes.

All sections are optional. A partial profile will use defaults for any missing fields.

### Forward compatibility

Unknown fields in the `behaviors` section are silently skipped. This means profile files created for newer compiler versions will still work with older compilers — unrecognized behavior flags are simply ignored rather than causing an error.

## Exporting and customizing profiles

Use `plc config profile` to export a profile as a starting point for customization:

```bash
# Export the default (codesys) profile as JSON
plc config profile > my-profile.json

# Export the standard profile as TOML
plc config --format=toml profile --profile standard > my-profile.toml
```

Edit the exported file to adjust behaviors or diagnostics, then use it:

```bash
plc --profile my-profile.json input.st
```

## Relationship to `--error-config`

The compiler also accepts an `--error-config` flag, which takes a JSON file containing only diagnostics severity overrides (the same format as the `diagnostics` section above). When `--error-config` is used, the diagnostics overrides are applied on top of the default CODESYS profile.

If you need to control both diagnostics and behavior flags, use `--profile` instead — it combines both in a single file.

If both `--profile` and `--error-config` are specified, `--profile` takes precedence.

## Behavior flags

Behavior flags control specific aspects of how the compiler translates and validates your code. A single flag may affect multiple aspects simultaneously — for example, a flag might change how an expression evaluates at runtime and what warnings are emitted during compilation.

Flags will be added incrementally as the profile system evolves. Each flag is documented below with its default value, which profiles set it, and what effect it has.

<!-- As behavior flags are added, document each one here with:
     - Flag name and type
     - Default value
     - Which profiles set it to what
     - Which compiler phases it affects
     - Example showing the behavioral difference -->
