# Outputs

This chapter walks you through the results that a run can produce besides machine code, from the declarations and index entries that the pipeline has already collected.

There are two of them: the C headers of a project, which replace code generation and end the run, and the map of the variables that are bound to hardware addresses, which a normal build or a `--check` run can write beside its artifact. Each subchapter follows one output from the option that asks for it to the file that it writes, with the model and the naming rules in between.
