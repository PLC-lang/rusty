#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 5 ]]; then
  echo "usage: $0 <version> <tag> <previous-tag-or-empty> <raw-changelog-path> <output-path>" >&2
  exit 1
fi

version="$1"
tag="$2"
previous_tag="$3"
raw_changelog_path="$4"
output_path="$5"

{
  printf 'This release publishes `%s`.\n\n' "$tag"

  if [[ -n "$previous_tag" ]]; then
    printf 'Below is the changelog since `%s`.\n\n' "$previous_tag"
  else
    printf 'Below is the changelog for this release.\n\n'
  fi

  cat "$raw_changelog_path"
  printf '\n'
} > "$output_path"
