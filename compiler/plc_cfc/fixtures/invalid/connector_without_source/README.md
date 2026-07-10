# connector_without_source

A connector/continuation pair whose **connector has no incoming wire**. The continuation `relay` feeds the
`result` sink, so the sink reads the connector's value — but nothing is wired into the connector, so that
"virtual wire" starts from no producer.

```text
   (no source)-->[ Connector "relay" ]

   [ Continuation "relay" ]--id 10-->  result  (0)

   "relay"  the label matching the connector to the continuation
   (0)      evaluation-priority badge shown by the IDE
```

The resolver only records an alias for a continuation when its matching connector carries an incoming wire
(`connector.connection_in`). Here the connector has none, so no alias is created and the continuation's
output (`10`) resolves to nothing. The sink reading it is therefore a dangling connection, reported as
`E081` — see the `connector_without_source_is_reported` test in `validator.rs`.

- `mainProgram.cfc` — the program under test; its connector `relay` has no source.
