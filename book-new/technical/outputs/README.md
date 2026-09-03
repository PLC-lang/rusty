# Outputs

Not every run of the compiler ends in an object file. The chapters in this part describe the outputs that stop the pipeline early and render what an earlier stage knows.

| Chapter | Output |
|---|---|
| [Header Generator](00-header-generator.md) | The declarations of a project as header files for other languages (`--generate-headers`) |
| [Hardware Map](01-hardware-map.md) | The hardware-bound variables (`%IX0.0`, `%QW2.5`) of a project as a JSON or TOML file (`--hwmap-file`) |

Both run after validation and stop the pipeline before codegen; each chapter closes with a "Where it lives" table.
