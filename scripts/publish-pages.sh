#!/usr/bin/env bash
# Publish the built site: pages/ (built by just pages, which the gate
# runs) becomes the only commit of the gh-pages branch, which GitHub
# Pages serves. The branch is replaced on every publish (no history),
# so built files never accumulate in git; main never tracks pages/.
# Refuses when the work tree has uncommitted changes, so a published
# site always matches a commit of main; and rebuilds pages/ when it
# was built at another commit (the gate builds it before the commit).
#   scripts/publish-pages.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
git diff --quiet HEAD -- || { echo "publish: commit your changes first (the site must match a commit)" >&2; exit 1; }
head="$(git rev-parse --short HEAD)"
# The site's footer names the commit it was built from (data-sha in the
# catalog). A pages/ built before the last commit, by the gate, would
# publish with the wrong one: rebuild it at HEAD first.
built="$(grep -o 'data-sha="[0-9a-f]*"' pages/index.html 2>/dev/null | head -1 | cut -d'"' -f2 || true)"
if [ "$built" != "$head" ]; then
  echo "publish: pages/ was built at ${built:-no commit}, not $head: rebuilding"
  "$root/scripts/build-pages.sh"
fi
[ -f pages/index.html ] || { echo "publish: no pages/ (just pages first)" >&2; exit 1; }
ml="$(cut -c1-7 XETAL_ML_COMMIT)"
libs="$(cut -c1-7 XETAL_LIBRARIES_COMMIT)"
index="$(mktemp)"; rm -f "$index"
trap 'rm -f "$index"' EXIT
export GIT_INDEX_FILE="$index"
git --work-tree=pages add -A -f .
tree="$(git write-tree)"
unset GIT_INDEX_FILE
commit="$(git commit-tree "$tree" -m "pages: built from main $head with X_eTaL-ML $ml and X_eTaL-libraries $libs")"
git push --quiet --force origin "$commit:refs/heads/gh-pages"
echo "published pages/ (main $head, X_eTaL-ML $ml, X_eTaL-libraries $libs) to gh-pages: https://softwarewrighter.github.io/X_eTaL-fine-tuning/"
