# Reference exports

Verbatim CFC files exported straight from the IDE, kept as the **ground truth**
for the on-disk schema. They are not necessarily wired into tests; they exist so
generated fixtures can be checked against a real export and so new element kinds
(blocks, connectors, …) have an authoritative shape to derive from before we
hand-author `valid/` and `invalid/` cases.

Drop new IDE exports here unchanged (keep the original layout/positions). Prefix
or subfolder by the feature they demonstrate, e.g. `reference/block_call/`.
