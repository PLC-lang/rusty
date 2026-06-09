#!/usr/bin/env bash
# Local simulation of the release automation: runs the workflows' actual
# `run:` blocks (extracted by extract-workflow-steps.py) against a scratch
# repo and asserts the version/branch/tag decisions for each release flow.
#
# Requires: git-cliff, cargo-edit (cargo set-version), python3 + pyyaml.
#
# Deliberately not `set -e`: assertion failures are counted and reported,
# the script exits non-zero if any assertion failed.
set -uo pipefail

for tool in git-cliff python3; do
  command -v "$tool" >/dev/null || { echo "missing tool: $tool" >&2; exit 1; }
done
cargo set-version --help >/dev/null 2>&1 || { echo "missing tool: cargo-edit (cargo set-version)" >&2; exit 1; }
python3 -c "import yaml" 2>/dev/null || { echo "missing python module: yaml (pyyaml)" >&2; exit 1; }

root_dir=$(git rev-parse --show-toplevel)
work_dir=$(mktemp -d)
cleanup() {
  rm -rf "$work_dir"
}
trap cleanup EXIT

steps_dir=$work_dir/steps
repo_dir=$work_dir/repo
step_log=$work_dir/step.log
pass=0 fail=0

python3 "$root_dir/.github/scripts/extract-workflow-steps.py" \
  "$root_dir/.github/workflows" "$steps_dir"

note()  { printf '\n\033[1m=== %s ===\033[0m\n' "$*"; }
assert_eq() { # label actual expected
  if [ "$2" = "$3" ]; then echo "  PASS: $1 = '$2'"; pass=$((pass+1));
  else echo "  FAIL: $1 = '$2' (expected '$3')"; fail=$((fail+1)); fi
}
assert_rc() { # label rc expected-rc
  if [ "$2" -eq "$3" ]; then echo "  PASS: $1 rc=$2"; pass=$((pass+1));
  else echo "  FAIL: $1 rc=$2 (expected $3)"; fail=$((fail+1)); fi
}

run_step() { # script-name (env must be pre-exported); outputs go to $GITHUB_OUTPUT
  GITHUB_OUTPUT=$(mktemp); export GITHUB_OUTPUT
  bash "$steps_dir/$1" >"$step_log" 2>&1
  STEP_RC=$?
}
out() { grep "^$1=" "$GITHUB_OUTPUT" | tail -1 | cut -d= -f2-; }

cliff_bumped() { git-cliff --bumped-version 2>/dev/null | tail -1; }

# ---- release-pr.yml: derive + check (sets HEAD_BRANCH BASE_BRANCH CURRENT NEXT SKIP) ----
release_pr_check() { # arg1 = GITHUB_REF_NAME
  export GITHUB_REF_NAME=$1
  run_step release-pr__derive-branch-names.sh
  BASE_BRANCH=$(out base); HEAD_BRANCH=$(out head)
  export STEPS_CLIFF_VERSION_CONTENT=$(cliff_bumped)
  run_step release-pr__check-if-bump-needed.sh
  CURRENT=$(out current); NEXT=$(out next); SKIP=$(out skip)
  CLAMP_WARNING=$(grep -c '^::warning::' "$step_log" || true)
}

# ---- simulate merging the release PR (what create-pull-request + merge queue produce) ----
merge_release_pr() { # arg1 = version
  cargo set-version --workspace "$1" >/dev/null 2>&1
  cargo generate-lockfile --offline >/dev/null 2>&1
  git-cliff --tag "v$1" -o CHANGELOG.md 2>/dev/null
  git add -A >/dev/null
  git commit -qm "chore(release): v$1"
}

# ---- release.yml end to end (sets VERSION TAG PREV DEV_VERSION DEV_BRANCH) ----
release_yml() { # arg1 = base ref (the branch the release PR merged into)
  export PR_BASE_REF=$1 PR_MERGE_SHA=$(git rev-parse HEAD)
  export RUNNER_TEMP=$(mktemp -d)
  run_step release__get-version-from-cargo-toml.sh
  VERSION=$(out version); TAG=$(out tag)
  export STEPS_VERSION_VERSION=$VERSION STEPS_VERSION_TAG=$TAG
  run_step release__determine-previous-release-tag.sh
  PREV=$(out tag)
  export STEPS_PREVIOUS_TAG_TAG=$PREV
  run_step release__prepare-release-notes-workspace.sh
  # git-cliff-action with args + OUTPUT env:
  git-cliff --unreleased --tag "$TAG" --strip header -o "$RUNNER_TEMP/release/raw-notes.md" 2>/dev/null
  run_step release__compose-release-notes.sh
  COMPOSE_RC=$STEP_RC
  NOTES_FILE=$RUNNER_TEMP/release/release-notes.md
  # softprops/action-gh-release with target_commitish: merge_commit_sha
  git tag "$TAG" "$PR_MERGE_SHA"
  run_step release__compute-dev-version.sh
  DEV_VERSION=$(out version); DEV_BRANCH=$(out branch)
  # dev-bump PR (created + merged)
  export STEPS_DEV_VERSION=$DEV_VERSION
  run_step release__bump-to-dev-version.sh
  cargo generate-lockfile --offline >/dev/null 2>&1
  # create-pull-request commits every workspace change, so anything the job
  # left in the workspace beyond the version bump ends up in the dev-bump
  # PR. Fail if the workflow polluted the workspace.
  POLLUTION=$(git status --porcelain | grep -vE 'Cargo\.(toml|lock)$' || true)
  git add -A >/dev/null
  git commit -qm "chore: bump version to $DEV_VERSION"
}

# ---- release-branch-guard.yml on a fresh checkout (rc in GUARD_RC) ----
guard() { # arg1 = base branch, arg2 = PR title, arg3 = PR body
  local dir; dir=$(mktemp -d)
  git clone -q "$repo_dir" "$dir" 2>/dev/null
  ( cd "$dir"
    git checkout -q "$1"
    export GITHUB_BASE_REF=$1 PR_TITLE=$2 PR_BODY=$3
    BASELINE=$(cliff_bumped)
    bash "$steps_dir/release-branch-guard__simulate-the-squash-merge.sh" >/dev/null 2>&1
    WITH_PR=$(cliff_bumped)
    export BASELINE WITH_PR
    bash "$steps_dir/release-branch-guard__reject-minor-major-bumps.sh"
  )
  GUARD_RC=$?
  rm -rf "$dir"
}

# ================= scratch repo =================
mkdir -p "$repo_dir"; cd "$repo_dir"
git init -qb master
git config user.name sim; git config user.email sim@sim
cp "$root_dir/cliff.toml" .
mkdir -p .github/scripts member/src src
cp "$root_dir/.github/scripts/compose-release-notes.sh" .github/scripts/
cat > Cargo.toml <<'EOF'
[package]
name = "sim"
version = "0.5.0"
edition = "2021"

[workspace]
members = ["member"]
EOF
cat > member/Cargo.toml <<'EOF'
[package]
name = "member"
version = "0.5.0"
edition = "2021"
EOF
touch src/lib.rs member/src/lib.rs
cargo generate-lockfile --offline >/dev/null 2>&1
git add -A; git commit -qm "chore: init"
git tag v0.5.0
git commit -q --allow-empty -m "fix: some bugfix on master"
git commit -q --allow-empty -m "feat: some feature on master"

note "S0: fix + feat since the last tag offer a minor release PR on master"
release_pr_check master
assert_eq "head branch" "$HEAD_BRANCH" "release/next"
assert_eq "base branch" "$BASE_BRANCH" "master"
assert_eq "current"     "$CURRENT"     "0.5.0"
assert_eq "next"        "$NEXT"        "0.6.0"
assert_eq "skip"        "$SKIP"        "false"

note "S1: release PR version manually overridden to 1.0.0 and merged"
merge_release_pr 1.0.0
assert_eq "Cargo.toml version" "$(grep -m1 '^version' Cargo.toml | sed 's/.*"\(.*\)".*/\1/')" "1.0.0"

note "S2: release.yml tags v1.0.0 from master, dev-bumps to 1.1.0-dev"
release_yml master
assert_eq "version"     "$VERSION"     "1.0.0"
assert_eq "tag"         "$TAG"         "v1.0.0"
assert_eq "previous"    "$PREV"        "v0.5.0"
assert_rc "compose-release-notes" "$COMPOSE_RC" 0
assert_eq "dev version" "$DEV_VERSION" "1.1.0-dev"
assert_eq "dev branch"  "$DEV_BRANCH"  "post-release/dev-bump"
assert_eq "tag points at release commit" "$(git rev-parse v1.0.0)" "$PR_MERGE_SHA"
assert_eq "no workspace pollution in dev-bump commit" "$POLLUTION" ""
grep -q "feature on master" "$NOTES_FILE" \
  && { echo "  PASS: release notes contain the feature"; pass=$((pass+1)); } \
  || { echo "  FAIL: release notes missing the feature"; fail=$((fail+1)); }

note "S3: fix lands on master at 1.1.0-dev: no release PR, no patch collision"
git commit -q --allow-empty -m "fix: patch after release"
release_pr_check master
assert_eq "cliff computed" "$NEXT" "1.0.1"
assert_eq "skip"           "$SKIP" "true"

note "S4: feat lands on master: 1.1.0 release PR (drop -dev)"
git commit -q --allow-empty -m "feat: new feature"
release_pr_check master
assert_eq "next" "$NEXT" "1.1.0"
assert_eq "skip" "$SKIP" "false"
merge_release_pr 1.1.0
release_yml master
assert_eq "tag"         "$TAG"         "v1.1.0"
assert_eq "previous"    "$PREV"        "v1.0.0"
assert_eq "dev version" "$DEV_VERSION" "1.2.0-dev"

note "S5: cut release/1.0.x at v1.0.0, backport a fix: 1.0.1 release PR"
git checkout -qb release/1.0.x v1.0.0
git commit -q --allow-empty -m "fix: backported fix"
release_pr_check release/1.0.x
assert_eq "head branch" "$HEAD_BRANCH" "release/next-1.0.x"
assert_eq "current"     "$CURRENT"     "1.0.0"
assert_eq "next"        "$NEXT"        "1.0.1"
assert_eq "skip"        "$SKIP"        "false"
assert_eq "no clamp warning" "$CLAMP_WARNING" "0"

note "S6: release v1.0.1 from the maintenance branch"
merge_release_pr 1.0.1
release_yml release/1.0.x
assert_eq "tag"         "$TAG"         "v1.0.1"
assert_eq "previous (must not be master's v1.1.0)" "$PREV" "v1.0.0"
assert_eq "dev version" "$DEV_VERSION" "1.0.2-dev"
assert_eq "dev branch"  "$DEV_BRANCH"  "post-release/dev-bump-1.0.x"

note "S7: guard on PRs targeting release/1.0.x"
guard release/1.0.x "fix: good backport" "plain description"
assert_rc "fix PR passes" "$GUARD_RC" 0
guard release/1.0.x "feat: smuggled feature" ""
assert_rc "feat PR fails" "$GUARD_RC" 1
guard release/1.0.x "fix: sneaky" $'looks fine\n\nBREAKING CHANGE: breaks the ffi boundary'
assert_rc "BREAKING CHANGE body fails" "$GUARD_RC" 1
guard release/1.0.x "chore(release): v1.0.2" "release PR body"
assert_rc "release PR passes" "$GUARD_RC" 0
guard release/1.0.x "chore: bump version to 1.0.2-dev" "dev bump body"
assert_rc "dev-bump PR passes" "$GUARD_RC" 0

note "S8: feat merged onto the maintenance branch anyway: clamped to patch"
git commit -q --allow-empty -m "feat: smuggled past the guard"
release_pr_check release/1.0.x
assert_eq "clamp warning emitted" "$CLAMP_WARNING" "1"
assert_eq "next clamped" "$NEXT" "1.0.2"
assert_eq "skip"         "$SKIP" "false"

note "S9: release-assets head-branch validation"
for case in "master:false" "release/1.0.x:false" "release/next:true" "feature/foo:true"; do
  branch=${case%%:*}; expected=${case##*:}
  export STEPS_SOURCE_MODE=workflow_run STEPS_SOURCE_SOURCE_WORKFLOW_NAME="Build Linux" \
         STEPS_SOURCE_CONCLUSION=success STEPS_SOURCE_EVENT_NAME=push \
         STEPS_SOURCE_HEAD_BRANCH=$branch
  run_step release-assets__validate-source-request.sh
  assert_eq "head_branch=$branch -> skip" "$(out skip)" "$expected"
done

note "S10: idle states produce no release PR"
git checkout -q master
release_pr_check master
assert_eq "master idle at 1.2.0-dev: skip" "$SKIP" "true"
git checkout -qb release/1.1.x v1.1.0
release_pr_check release/1.1.x
assert_eq "fresh-cut release/1.1.x: skip" "$SKIP" "true"

printf '\n\033[1m%d passed, %d failed\033[0m\n' "$pass" "$fail"
exit $((fail > 0))
