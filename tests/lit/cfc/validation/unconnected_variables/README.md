What: two placed-but-unwired variables (`foo`, `bar`) alongside one real
assignment `bar := foo`. The unconnected boxes emit nothing and each raise an
E084 warning. They have no execution order, so the diagnostic names only the
file. The lit error config raises the warning to an error, which fails the build.

Illustrated:
```
foo          (unconnected, ignored, warns)
bar          (unconnected, ignored, warns)
foo --> bar (0)
```
