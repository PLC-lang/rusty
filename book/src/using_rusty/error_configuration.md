# Error Configuration

Errors in a `plc` project can be configured by providing a json configuration file.
A diagnostics severity can be changed for example from `warning` to `error` or `info` and vice-versa or `ignore`d completely.
To see a default error configuration use `plc config diagnostics`.
To provide a custom error configuration use `plc --error-config <custom.json>`.
Note that the `--error-config` command can be used with all subcommands such as `build` and `check`.
Running `plc config diagnostics --error-config <custom.json>` will print out the full diagnostics configuration taking the provided overrides into account.

## Error Description

Errors produced by `plc` can be explained using the `plc explain <ErrorCode>` command.
Error codes are usually provided in the diagnostic report.


