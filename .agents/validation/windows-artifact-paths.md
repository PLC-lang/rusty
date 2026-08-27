# Validation: build artifact paths on Windows

Status: implemented and tested on Linux, needs validation on Windows.

## What was wrong

`plc` named every build artifact after the path of its source file and created that
path inside the build directory:

    <build>/<target>/<target>/<path of the source>/<file>.st.o

The path of the source was relative to the project when the source lives inside it,
and the full path (without the root) otherwise. A project that compiles sources from
outside its own directory, an installed library for example, therefore got a build
directory that mirrors the absolute path of those sources. Two details made it worse:

- The directory of the target was added twice, once by the driver and once by
  `GeneratedModule::persist`.
- On Windows, `fs::canonicalize` returns a verbatim path (`\\?\C:\...`) while the
  project root does not. The comparison that selects the relative name never matched,
  so the full path was mirrored even for sources inside the project.

The result goes past the 260 characters that the Windows file system allows without
long path support. File operations on the workspace (a copy in the file explorer, for
example) then fail, and the error names a directory of the compiler.

The scenario in `artifact_paths_stay_within_the_windows_path_limit` produced a 439
character artifact path before this change.

## What changed

- Artifacts get a flat name of the form `<file name>-<digest>.<extension>`, for
  example `main.st-116b35a8828869ac.o`, and all of them land directly in
  `<build>/<target>/`. See `compiler/plc_driver/src/artifacts.rs`.
- The digest is a SipHash-1-3 of the whole key with a fixed zero key, so the name is
  the same on every run, process and platform. It is what keeps two units with the
  same file name in different directories apart, now that they share one directory.
  The file name is kept for readability and is cut to 32 characters.
- The key of a unit is its path relative to the project, or its full path when it
  lives outside of the project. The project root is canonicalized before the
  comparison, so the relative form is also reached on Windows.
- The directory of the target is added by the driver only
  (`pipelines::target_compile_dir`). `GeneratedModule::persist` writes to the
  directory it is given.

The compiler now adds at most 70 characters to the build directory: the name of the
target, one artifact name and two separators.

Unchanged: the final artifact of a link still lands in `<build>/<target>/<output>`,
and the header generator still writes next to the source or into the requested
directory.

## The tests and how to run them

    cargo test --lib artifact_names
    cargo test integration::build_artifacts -- --test-threads=1

Windows CI runs `cargo test --lib` and `cargo test integration`, so both suites are
part of the Windows job already.

`compiler/plc_driver/src/tests/artifact_names.rs` covers the naming itself. It follows
the mirroring policy of `debug_paths.rs`: platform dependent scenarios live in a
`linux` and a `windows` module, because `Path::file_name` splits on the separators of
the host. Every scenario on one side wants a mirror with the same name on the other,
or a comment that says why it fits one platform only.

`tests/integration/build_artifacts.rs` compiles a project with two external sources
that share a file name and asserts:

- every artifact is a direct child of `<build>/<target>` (flat),
- the characters an artifact adds to the build directory stay under a constant, so the
  path of the artifact does not grow with the path of the source,
- a second build reuses the names of the first,
- with a 150 character build directory and a 200 character source tree, no artifact
  path reaches 260 characters.

The first three were confirmed red before the change went in (the flat assertion, the
constant and the 260 character limit all failed). The fourth guards the new scheme
itself: a digest that is not stable would leave the artifacts of every run behind.

## Open points for the Windows validation

1. **Verbatim prefix.** Build a project with a `plc.json` and compare the artifact
   names to a Linux build of the same project. They must be equal: the digest is taken
   over the separator normalized relative path. Names that differ mean the relative
   form was not reached and the absolute path was used as the key.
2. **Case insensitivity.** Reference the same source twice with different casing
   (`src\main.st` and `SRC\Main.st`). Expect one artifact. `fs::canonicalize` should
   normalize the casing; confirm that it does.
3. **UNC and mapped drives.** Sources under `\\server\share\lib\util.st` and
   `Z:\lib\util.st`. The readable part of the name must be `util.st` and the build must
   succeed.
4. **Reserved device names.** A source called `con.st` or `nul.st`. The artifact is
   `con.st-<digest>.o`, whose base name is not a reserved device name, so it should be
   legal. Confirm on a real file system.
5. **Non-ASCII source names.** A source called `prüf.st` in a workspace with a
   non-UTF-8 code page. The artifact name is plain ASCII by construction; confirm the
   build works.
6. **A build directory that is already too long.** More than 260 characters without
   long path support. Expect a clear diagnostic, not a panic.
7. **Stale artifacts.** A build directory written by an older `plc` keeps the mirrored
   tree, and nothing removes it. Whether `plc` should clean its build directory is an
   open question and not part of this change.

## Tests worth adding

- Mirrors in the `windows` module of `artifact_names.rs` for the scenarios above (UNC
  path, mapped drive, reserved device name, casing).
- An end-to-end test for point 6: a build directory close to the limit must produce a
  diagnostic that names the path, not a panic or a silent truncation.
- An end-to-end test through the `build` subcommand, where the project root comes from
  the `plc.json` instead of the current directory, asserting the same flat layout.
