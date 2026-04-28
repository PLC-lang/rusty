#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "usage: $0 <repo> <tag>" >&2
  exit 1
fi

repo="$1"
tag="$2"

root_dir=$(git rev-parse --show-toplevel)
tmp_dir=$(mktemp -d)
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

sha=$(bash "$root_dir/.github/scripts/resolve-tag-sha.sh" "$repo" "$tag")
linux_run=$(bash "$root_dir/.github/scripts/find-workflow-run.sh" "$repo" linux.yml "$sha")
windows_run=$(bash "$root_dir/.github/scripts/find-workflow-run.sh" "$repo" windows.yml "$sha")

version=$(git show "$tag:Cargo.toml" | grep '^version' | head -1 | sed 's/.*"\(.*\)".*/\1/')
if [[ "$version" == *-dev ]]; then
  echo "tag $tag points to dev version $version" >&2
  exit 1
fi

mkdir -p "$tmp_dir/linux" "$tmp_dir/windows" "$tmp_dir/staged"

gh run download "$linux_run" \
  -D "$tmp_dir/linux" \
  -n plc-x86_64 \
  -n plc-aarch64 \
  -n deb-x86_64 \
  -n deb-aarch64 \
  -n stdlib >/dev/null

gh run download "$windows_run" \
  -D "$tmp_dir/windows" \
  -n plc.exe \
  -n stdlib.dll \
  -n stdlib.lib >/dev/null

bash "$root_dir/.github/scripts/stage-release-assets.sh" "Build Linux" "$version" "$tmp_dir/linux" "$tmp_dir/staged" >/dev/null
bash "$root_dir/.github/scripts/stage-release-assets.sh" "Build Windows" "$version" "$tmp_dir/windows" "$tmp_dir/staged" >/dev/null

printf 'repo=%s\n' "$repo"
printf 'tag=%s\n' "$tag"
printf 'sha=%s\n' "$sha"
printf 'version=%s\n' "$version"
printf 'linux_run=%s\n' "$linux_run"
printf 'windows_run=%s\n' "$windows_run"
printf 'staged_dir=%s\n' "$tmp_dir/staged"
printf '%s\n' '--- assets ---'
find "$tmp_dir/staged" -maxdepth 1 -type f -printf '%f\n' | sort
printf '%s\n' '--- stdlib archive sample ---'
tar -tzf "$tmp_dir/staged/stdlib-${version}.tar.gz" | sed -n '1,20p'
