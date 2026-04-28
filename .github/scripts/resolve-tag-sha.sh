#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "usage: $0 <repo> <tag>" >&2
  exit 1
fi

repo="$1"
tag="$2"

object_type=$(gh api "repos/$repo/git/ref/tags/$tag" --jq '.object.type')
object_sha=$(gh api "repos/$repo/git/ref/tags/$tag" --jq '.object.sha')

case "$object_type" in
  tag)
    gh api "repos/$repo/git/tags/$object_sha" --jq '.object.sha'
    ;;
  commit)
    printf '%s\n' "$object_sha"
    ;;
  *)
    echo "unsupported tag object type: $object_type" >&2
    exit 1
    ;;
esac
