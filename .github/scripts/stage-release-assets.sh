#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 4 ]]; then
  echo "usage: $0 <workflow-name> <version> <download-dir> <staging-dir>" >&2
  exit 1
fi

workflow_name="$1"
version="$2"
download_dir="$3"
staging_dir="$4"

mkdir -p "$staging_dir"

case "$workflow_name" in
  "Build Linux")
    install -m 755 "$download_dir/plc-x86_64/plc" "$staging_dir/plc-linux-x86_64"
    install -m 755 "$download_dir/plc-aarch64/plc" "$staging_dir/plc-linux-aarch64"

    install -m 644 \
      "$download_dir/deb-x86_64/plc-compiler_${version}-1_amd64.deb" \
      "$staging_dir/plc-compiler_${version}-1_amd64.deb"
    install -m 644 \
      "$download_dir/deb-x86_64/plc-stdlib_${version}-1_amd64.deb" \
      "$staging_dir/plc-stdlib_${version}-1_amd64.deb"
    install -m 644 \
      "$download_dir/deb-aarch64/plc-compiler_${version}-1_arm64.deb" \
      "$staging_dir/plc-compiler_${version}-1_arm64.deb"
    install -m 644 \
      "$download_dir/deb-aarch64/plc-stdlib_${version}-1_arm64.deb" \
      "$staging_dir/plc-stdlib_${version}-1_arm64.deb"

    tar -C "$download_dir" -czf "$staging_dir/stdlib-${version}.tar.gz" stdlib
    ;;
  "Build Windows")
    install -m 755 "$download_dir/plc.exe/plc.exe" "$staging_dir/plc.exe"
    install -m 644 "$download_dir/stdlib.dll/iec61131std.dll" "$staging_dir/iec61131std.dll"
    install -m 644 "$download_dir/stdlib.lib/iec61131std.lib" "$staging_dir/iec61131std.lib"
    ;;
  *)
    echo "unsupported workflow: $workflow_name" >&2
    exit 1
    ;;
esac

find "$staging_dir" -maxdepth 1 -type f -printf '%f\n' | sort
