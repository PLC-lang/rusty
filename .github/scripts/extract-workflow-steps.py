#!/usr/bin/env python3
"""Extract `run:` blocks from workflow YAMLs into executable step scripts.

Used by test-release-flows.sh so the simulation executes the exact shell
code the workflows run, instead of a hand-maintained copy that can drift.

GitHub expression substitution:
  ${{ steps.<id>.outputs.<key> }}  ->  ${STEPS_<ID>_<KEY>}   (env var, upper, - -> _)
  selected github.* / inputs.*     ->  fixed env var names
Anything unmapped becomes __UNMAPPED__<expr>__ so its use fails visibly.
"""
import re
import sys
from pathlib import Path

import yaml

WORKFLOWS = Path(sys.argv[1])
OUT = Path(sys.argv[2])
OUT.mkdir(parents=True, exist_ok=True)

FIXED = {
    "github.event.pull_request.base.ref": "${PR_BASE_REF}",
    "github.event.pull_request.merge_commit_sha": "${PR_MERGE_SHA}",
    "github.repository": "${REPOSITORY}",
    "github.event_name": "${EVENT_NAME}",
    "inputs.tag": "${INPUT_TAG}",
    "runner.temp": "${RUNNER_TEMP}",
}


def subst(text):
    def repl(m):
        expr = m.group(1).strip()
        sm = re.fullmatch(r"steps\.([\w-]+)\.outputs\.([\w-]+)", expr)
        if sm:
            sid = sm.group(1).replace("-", "_").upper()
            key = sm.group(2).replace("-", "_").upper()
            return "${STEPS_%s_%s}" % (sid, key)
        if expr in FIXED:
            return FIXED[expr]
        return "__UNMAPPED__%s__" % re.sub(r"\W", "_", expr)

    return re.sub(r"\$\{\{([^}]*)\}\}", repl, text)


def slug(name):
    return re.sub(r"[^a-z0-9]+", "-", name.lower()).strip("-")


for wf in sorted(WORKFLOWS.glob("*.yml")):
    doc = yaml.safe_load(wf.read_text())
    for job in (doc.get("jobs") or {}).values():
        for step in job.get("steps") or []:
            if "run" not in step or "name" not in step:
                continue
            path = OUT / ("%s__%s.sh" % (wf.stem, slug(step["name"])))
            path.write_text("#!/usr/bin/env bash\n" + subst(step["run"]))
            path.chmod(0o755)
