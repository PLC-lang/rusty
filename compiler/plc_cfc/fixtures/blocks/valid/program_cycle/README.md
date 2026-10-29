What: Two distinct programs feeding each other in a cycle — `ping.out` --> `pong.in`
and `pong.out` --> `ping.in` — with `pong` (1) evaluated before `ping` (2). Neither
call can precede the other in data-flow terms; priority order breaks the tie and
each reads the other's persisted member. Confirms wire-tracing terminates on a
block output even across a cross-block loop.

Illustrated:
```
            ping (2)         pong (1)
          +----------+     +----------+
   .----> | in   out | --> | in   out | --.
   |      +----------+     +----------+   |
   '--------------------------------------'   [pong.out feeds back into ping.in]
```
