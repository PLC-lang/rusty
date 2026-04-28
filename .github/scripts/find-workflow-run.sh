#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 3 ]]; then
  echo "usage: $0 <repo> <workflow-file> <sha>" >&2
  exit 1
fi

repo="$1"
workflow_file="$2"
sha="$3"

run_id=$(gh api "repos/$repo/actions/workflows/$workflow_file/runs?branch=master&event=push&status=success&head_sha=$sha&per_page=1" --jq '.workflow_runs[0].id // empty')

if [[ -z "$run_id" ]]; then
  echo "no successful $workflow_file push run found on master for $sha" >&2
  exit 1
fi

printf '%s\n' "$run_id"
