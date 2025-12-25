#!/usr/bin/env bash
set -euo pipefail

# Usage: runs in a GitHub Actions environment on PR events
# Expects GITHUB_BASE_REF and GITHUB_HEAD_REF to be set

if [ -z "${GITHUB_BASE_REF:-}" ] || [ -z "${GITHUB_HEAD_REF:-}" ]; then
  echo "This script must be run within a PR GitHub Action where GITHUB_BASE_REF and GITHUB_HEAD_REF are set."
  exit 2
fi

BASE="origin/${GITHUB_BASE_REF}"
HEAD="${GITHUB_HEAD_REF}"

echo "Fetching base branch ${BASE}..."
git fetch origin ${GITHUB_BASE_REF}:${GITHUB_BASE_REF}

commit_list=$(git rev-list --reverse ${BASE}..${HEAD})
if [ -z "$commit_list" ]; then
  echo "No commits in PR range; cannot verify tests-first proof."
  exit 2
fi

first_test_commit=""
for c in $commit_list; do
  # Look for added lines that include #[test] or mod tests or files under tests/
  if git show $c | grep -qE '^\+.*#\[test\]|^\+.*mod tests|^\+.*tests/'; then
    first_test_commit=$c
    break
  fi
done

if [ -z "$first_test_commit" ]; then
  echo "No commit adding tests found in PR branch. Per constitution, include a failing-test commit as proof (add tests first then implement)."
  exit 2
fi

echo "Found test-introducing commit: $first_test_commit"

# Check out that commit and run tests; we expect tests to FAIL (non-zero)
git checkout $first_test_commit
set +e
cargo test --all-features
rc=$?
set -e
if [ $rc -eq 0 ]; then
  echo "Tests passed at the commit that introduced tests — expected a failing test as proof."
  echo "Please include a failing-test commit (tests added and failing), followed by a fix commit."
  exit 2
else
  echo "Confirmed tests failed at test-introducing commit (rc=$rc) — good failing-proof found."
fi

# Now run tests on PR HEAD and expect success
git checkout $HEAD
cargo test --all-features
echo "Tests pass at PR HEAD — tests-first (red→green) requirement satisfied."
exit 0
