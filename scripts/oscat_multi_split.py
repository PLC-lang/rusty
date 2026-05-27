#!/usr/bin/env python3
"""Split a single-file `oscat.st` corpus into one POU per file.

Each `FUNCTION` / `FUNCTION_BLOCK` / `PROGRAM` / `CLASS` / `TYPE`
declaration is emitted to its own `.st` file under
`.baseline/oscat-multi/src/`. Mirrors the "one POU per unit" style of
real PLC projects and gives `cargo xtask metrics`-style measurements a
representative multi-unit corpus.

Run from the repo root after cloning oscat into `.baseline/oscat/`:

    git clone --depth 1 https://github.com/plc-lang/oscat .baseline/oscat
    python3 scripts/oscat_multi_split.py

The output directory is gitignored. Pass `--out` to override.
"""

import argparse
import os
import re
import shutil


END_KINDS = {"FUNCTION_BLOCK", "FUNCTION", "PROGRAM", "CLASS", "TYPE"}
START = re.compile(r"^(FUNCTION_BLOCK|FUNCTION|PROGRAM|CLASS|TYPE)\s+(\S+)")
END = re.compile(r"^END_(FUNCTION_BLOCK|FUNCTION|PROGRAM|CLASS|TYPE|VAR_GLOBAL|VAR_CONFIG)\b")


def sanitize(name: str) -> str:
    return re.sub(r"[^A-Za-z0-9_]", "_", name)


def split(src_path: str, out_dir: str) -> int:
    with open(src_path) as f:
        lines = f.read().split("\n")
    if os.path.isdir(out_dir):
        shutil.rmtree(out_dir)
    os.makedirs(out_dir, exist_ok=True)

    preamble: list[str] = []
    i = 0
    while i < len(lines) and not START.match(lines[i]):
        preamble.append(lines[i])
        i += 1
    if preamble:
        with open(os.path.join(out_dir, "_preamble.st"), "w") as f:
            f.write("\n".join(preamble) + "\n")

    counts: dict[str, int] = {}
    cur_name: str | None = None
    buf: list[str] = []

    while i < len(lines):
        line = lines[i]
        m = START.match(line)
        if m and cur_name is None:
            cur_name = m.group(2)
            buf = [line]
        elif cur_name is not None:
            buf.append(line)
            m_end = END.match(line)
            if m_end and m_end.group(1) in END_KINDS:
                sane = sanitize(cur_name)
                counts[sane] = counts.get(sane, 0) + 1
                suffix = f"_{counts[sane]}" if counts[sane] > 1 else ""
                fname = f"{sane}{suffix}.st"
                with open(os.path.join(out_dir, fname), "w") as f:
                    f.write("\n".join(buf) + "\n")
                cur_name = None
                buf = []
        i += 1

    if buf:
        with open(os.path.join(out_dir, "_tail.st"), "w") as f:
            f.write("\n".join(buf) + "\n")

    return len([f for f in os.listdir(out_dir) if f.endswith(".st")])


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--src", default=".baseline/oscat/oscat.st", help="path to the single-file oscat source"
    )
    parser.add_argument(
        "--out", default=".baseline/oscat-multi/src", help="output directory for per-POU files"
    )
    args = parser.parse_args()
    written = split(args.src, args.out)
    print(f"wrote {written} files to {args.out}")


if __name__ == "__main__":
    main()
