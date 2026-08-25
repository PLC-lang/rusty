---
name: create-pr
description: >
  Create a pull request for this repository. Use whenever the user asks to open,
  create, or submit a PR, or when finished work is ready to become one. Assesses
  whether the change should be split into stacked PRs and requires explicit user
  consent before any split.
---

# Create a PR

The AGENTS.md conventions (commit titles, Problem/Solution descriptions,
required pre-PR checks) apply to every PR this skill creates; read them there.

## 1. Assess a stacked split

A split helps review only when all of these hold:

- The change is too large for one reviewer to hold in their head.
- It divides into layers with a natural dependency order
- Each layer compiles and passes the test suite on its own; CI and branch
  protections run on every PR in a stack.

Tightly coupled changes stay as one PR, regardless of size. Skip this
assessment when working from a fork; stacks require branches in this
repository.

## 2. Split helps, `gh` installed

1. Propose the split: branch names in dependency order, one line per layer.
2. Ask the user for consent and wait for a clear answer. Never assume a
   default; silence is not consent. Without an answer, leave the PR uncreated
   and state that you are blocked.
3. On consent, follow [references/gh-stack.md](references/gh-stack.md) to
   build and submit the stack. On decline, create the single PR as usual.

## 3. Split helps, `gh` not installed

Create the single PR as usual. End your message with a bold note that the PR
could be split into stacked PRs if the gh CLI tool were installed but only if
it makes sense to split it up.

## 4. Split does not help

Create the single PR as usual and do not mention stacking.

## References

- [references/gh-stack.md](references/gh-stack.md): stacked PR mechanics
  (non-interactive flags, stack design, commands, troubleshooting). Vendored
  from [github/gh-stack](https://github.com/github/gh-stack), `skills/gh-stack/`
  at commit `4f9188e61196`; refresh by re-fetching from there.
