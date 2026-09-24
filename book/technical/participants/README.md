# Participants

After these chapters you know what each participant rewrites, and why the stages after it need the rewrite. A participant runs between two pipeline stages, so that the later ones see a simpler program.

The subchapters follow the order in which the driver registers them, because each participant sees the rewrites of the ones before it: graphical charts become statements, the loops and `ELSIF` become one shape each, properties and methods become calls, inheritance and interfaces become embedded members and method tables, initial values become constructor functions, retained variables become globals, generic and aggregate-returning calls become concrete ones, and array literals become element assignments. Every subchapter shows the tree before and after its rewrite, marks the hooks that the participant uses, and explains which other participants it depends on.

The hooks live in `compiler/plc_driver/src/pipelines`, and the transformations in `compiler/plc_lowering` and `src/lowering`.
