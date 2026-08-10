What: negation bubbles on an inout wire (source side and pin side). The IDE
permits drawing them, and the resolver transpiles them as on any input pin,
`acc := NOT NOT acc1` — but an inout passes by reference and a NOT expression
has no storage behind it, so the later validation stage rejects the call with
E031 ("Expected a reference for parameter acc"). The transpiler-level tests
therefore see no diagnostic; the end-to-end rejection is pinned by the
`tests/lit/cfc/blocks/function_inout_negated` lit test.

Illustrated:
```
             +-------- addInto -------+
a1 ----------| delta          addInto |--------> b1 (1)
acc1 o-----o-| acc                    |
             +------------------------+ (0)
```
