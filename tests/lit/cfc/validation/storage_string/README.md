Pins that a storage-mode sink without a numeric engraving fails compilation
naturally: the generated `IF a THEN s := TRUE; END_IF` trips the main
pipeline's assignment validation (E037), located at the sink block. This is
the backstop for shipping PRG-4559 without a CFC-specific type check.
