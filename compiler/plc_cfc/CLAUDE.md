**Think before coding**: State your assumptions explicitly and ask when uncertain. If multiple interpretations exist, present them rather than silently picking one. If a simpler approach exists, say so and push back when warranted. If something is unclear, stop, name what's confusing, and ask.

**Simplicity first**: Write the minimum code that solves the problem — nothing speculative. No features beyond what was asked, no abstractions for single-use code, no unrequested flexibility or configurability, no error handling for impossible scenarios. If you write 200 lines and it could be 50, rewrite it. Ask whether a senior engineer would call it overcomplicated; if so, simplify.

**Surgical changes**: Touch only what you must, and clean up only your own mess. Don't improve adjacent code, comments, or formatting, and don't refactor things that aren't broken. Match the existing style even if you'd do it differently. Remove imports, variables, and functions that your changes made unused, but leave pre-existing dead code alone — mention it instead of deleting it. Every changed line should trace directly to the request.

**Goal-driven execution**: Define success criteria and loop until verified. Turn tasks into verifiable goals ("add validation" → "write tests for invalid inputs, then make them pass"; "fix the bug" → "write a failing test that reproduces it, then make it pass"). For multi-step tasks, state a brief plan with a verification check per step. Strong success criteria let you work independently; weak ones ("make it work") require constant clarification.

**Comment to skim**: Add short comments so a reader can grasp the flow without reading every line. Two patterns:

1. **Step markers** in multi-step functions — one comment per logical step:
   ```ts
   // Read the file content
   const content = await readFile(path, "utf-8");

   // Parse the frontmatter
   const { frontmatter, body } = parseFrontmatter(content);

   // Skip irrelevant skills
   if (frontmatter.fg === undefined) { ... }
   ```

2. **Non-obvious statements** — comment what the line achieves, not what it literally does:
   ```ts
   // Normalize line endings to LF
   const normalized = content.replace(/\r\n/g, "\n").replace(/\r/g, "\n");

   // Find the opening and closing --- delimiters
   if (!normalized.startsWith("---")) return { frontmatter: {}, body: normalized };
   const endIndex = normalized.indexOf("\n---", 3);
   if (endIndex === -1) return { frontmatter: {}, body: normalized };
   ```

   Group related statements under one comment rather than annotating each line. Don't comment obvious one-liners or restate what the code already says.

**No section-divider comments**: Don't use decorative header comments like `// --- Public ---`, `// === Helpers ===`, or `// ########`. Let the code structure and the "public before private" ordering speak for itself. A blank line between functions is enough separation.

**Don't leak specification data into implementation**: When an implementation is derived from a specification or plan (e.g. a PLAN.md file), treat that document as ephemeral and don't reference it in the code or comments. Write as if the developer arrived at the solution directly, without ever having seen the spec. Keep comments at the altitude of normal code comments — explaining what the code does and why, specific enough to be useful but general enough to stay true if the plan changes — rather than restating section numbers, requirement IDs, or phrasing lifted from the spec.