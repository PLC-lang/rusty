What: the text declaration (the top editor) declares its variables with an
unknown type. The diagnostic is a plain text location counted within the
declaration text, `declaration_type.cfc:3:22`, not within the XML document.

Illustrated:
```
PROGRAM declaration_type
VAR
    a1, a2, b1, b2 : NOTATYPE;   <-- line 3, column 22
END_VAR
```
