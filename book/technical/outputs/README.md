# Outputs

The compiler can also write C headers and hardware maps. These outputs use declarations and index entries already available after validation.

| Chapter | Output |
|---|---|
| [Header Generator](00-header-generator.md) | The declarations of a project as C headers (`--generate-headers`) |
| [Hardware Map](01-hardware-map.md) | The hardware-bound variables (`%IX0.0`, `%QW2.5`) of a project as a JSON or TOML file (`--hwmap-file`) |

Header generation replaces codegen and ends the run. A requested hardware map can accompany a normal build or a `--check` run. For the layouts and types behind these outputs, continue with [Internals](../internals/README.md).
