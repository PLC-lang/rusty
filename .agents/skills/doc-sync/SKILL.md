---
name: doc-sync
description: Check that a change keeps the book and the code in sync, in both directions. Use when reviewing a diff or before opening a pull request.
entmoot:
    mode: fast
---

# Documentation Sync

The book in `book/` and the compiler are two views of the same thing. A change to one is complete only when the other still agrees with it. This is easy to forget, so check it on every pull request.


## What to check

Start at `book/SUMMARY.md`. It is the outline of the whole book and names every page in its part, so it tells you where a subject lives. Follow it to the pages that describe the behavior the change touches, and search the book for the construct, the flag, the diagnostic, or the mechanism by name. A subject often appears twice, once in the user part as behavior and once in the technical part as mechanism, so look in both.

Read those pages in full, then judge every statement that the change touches: responsibilities, hook points and order, the shape of index entries and annotations, the generated code, the diagnostics and their codes, the flags, and the examples.

When the change touches the book itself, check the other direction: every new or changed statement must describe what the compiler does today, and every example must compile as written.


## What to report

- A statement in the book that the change makes wrong. Name the page and the smallest edit that restores agreement.
- A behavior that the change introduces and that the book covers nowhere, where a reader of that part would expect it.

Ignore implementation details that the book does not describe; it explains architecture and behavior, not functions line by line. An edit that the report proposes follows the style of the pages around it.
