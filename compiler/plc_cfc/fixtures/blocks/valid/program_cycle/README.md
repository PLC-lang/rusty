What: Two distinct programs feeding each other in a cycle — `ping.out` --> `pong.in`
and `pong.out` --> `ping.in` — with `pong` (0) evaluated before `ping` (1). Neither
call can precede the other in data-flow terms; priority order breaks the tie and
each reads the other's persisted member. Confirms wire-tracing terminates on a
block output even across a cross-block loop.

Illustrated:
```
   +--> in [ping] out (1) --+
   |                        |
   +-- out [pong] in <------+   (0)
```
