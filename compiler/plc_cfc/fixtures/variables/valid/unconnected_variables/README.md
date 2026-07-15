What: Two placed-but-unwired variables (`foo`, `bar`) alongside one real
assignment `bar := foo`. The unconnected boxes emit nothing and each raise a
warning; compilation still succeeds and produces the single assignment.

Illustrated:
```
foo          (unconnected — ignored, warns)
bar          (unconnected — ignored, warns)
foo --> bar (0)
```
