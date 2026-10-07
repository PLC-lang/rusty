What: a sink whose identifier is a negated variable, `-bar`. A sink is an
assignment target, and `-bar` is none, so this is rejected with an
"unsupported CFC expression" error (E083). A source may carry the same text.

Illustrated:
```
foo --> -bar (1)    (negated sink, rejected)
```
