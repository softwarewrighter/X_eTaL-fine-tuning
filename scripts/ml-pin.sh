#!/usr/bin/env bash
# Pin a newer X_eTaL-ML: write the full SHA of a COMMITTED ref of the
# X_eTaL-ML repository into XETAL_ML_COMMIT, then fetch and build it
# (with the X_eTaL and X_eTaL-libraries it pins). Commit
# XETAL_ML_COMMIT on its own, after every baseline passes. Adapted
# from X_eTaL-ML's scripts/xetal-pin.sh.
#   scripts/ml-pin.sh            # HEAD of ../X_eTaL-ML
#   scripts/ml-pin.sh REF        # any ref git rev-parse accepts
#   XETAL_ML_REPO=/path scripts/ml-pin.sh
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
repo="${XETAL_ML_REPO:-$root/../X_eTaL-ML}"
sha="$(git -C "$repo" rev-parse --verify "${1:-HEAD}^{commit}")"
echo "$sha" > "$root/XETAL_ML_COMMIT"
# A local commit not yet on GitHub: fetch it into the clone from the sibling.
clone="$root/work/xetal-ml"
if [ -d "$clone/.git" ] && ! git -C "$clone" cat-file -e "$sha^{commit}" 2>/dev/null; then
    git -C "$clone" fetch --quiet "$repo" "$sha" 2>/dev/null || true
fi
XETAL_ML_SOURCE="${XETAL_ML_SOURCE:-$repo}" "$root/scripts/xetal-ml.sh" >/dev/null
echo "pinned X_eTaL-ML ${sha:0:7}: $(git -C "$repo" log -1 --format=%s "$sha")"
echo "ml-pin: now run everything (just gate --full), then commit XETAL_ML_COMMIT on its own" >&2
