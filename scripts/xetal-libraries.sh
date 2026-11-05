#!/usr/bin/env bash
# Get X_eTaL-libraries at this repository's own pin (docs/plan.md, F1):
# clone it into work/xetal-libraries (gitignored) and check out the
# known-good commit in XETAL_LIBRARIES_COMMIT. Its libraries go on the
# search path (scripts/libpath.sh) and into the pages' store
# (shared/lesson), in place of the copy X_eTaL-ML holds at its own
# libraries pin, so a delivery from X_eTaL-libraries can be taken and
# tested alone. Never edit the clone. Adapted from X_eTaL-ML's
# scripts/xetal-libraries.sh.
#   scripts/xetal-libraries.sh
#   XETAL_LIBRARIES_SOURCE=../X_eTaL-libraries scripts/xetal-libraries.sh   # first clone from a local checkout
#   scripts/xetal-libraries.sh --pin [REF]   # pin a committed ref of ../X_eTaL-libraries (default HEAD)
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source="${XETAL_LIBRARIES_SOURCE:-https://github.com/softwarewrighter/X_eTaL-libraries.git}"
clone="$root/work/xetal-libraries"
if [ "${1:-}" = --pin ]; then
    repo="${XETAL_LIBRARIES_REPO:-$root/../X_eTaL-libraries}"
    sha="$(git -C "$repo" rev-parse --verify "${2:-HEAD}^{commit}")"
    echo "$sha" > "$root/XETAL_LIBRARIES_COMMIT"
    # A local commit not yet on GitHub: fetch it into the clone from the sibling.
    if [ -d "$clone/.git" ] && ! git -C "$clone" cat-file -e "$sha^{commit}" 2>/dev/null; then
        git -C "$clone" fetch --quiet "$repo" "$sha" 2>/dev/null || true
    fi
    source="${XETAL_LIBRARIES_SOURCE:-$repo}"
    echo "pinned X_eTaL-libraries ${sha:0:7}: $(git -C "$repo" log -1 --format=%s "$sha")" >&2
    echo "libs-pin: now run everything (just gate --full), then commit XETAL_LIBRARIES_COMMIT on its own" >&2
fi
commit="$(tr -d '[:space:]' < "$root/XETAL_LIBRARIES_COMMIT")"
if [ ! -d "$clone/.git" ]; then
    mkdir -p "$root/work"
    echo "xetal-libraries: cloning $source into work/xetal-libraries" >&2
    git clone --quiet "$source" "$clone"
fi
if ! git -C "$clone" cat-file -e "$commit^{commit}" 2>/dev/null; then
    git -C "$clone" fetch --quiet origin
fi
if [ "$(git -C "$clone" rev-parse HEAD)" != "$commit" ]; then
    git -C "$clone" checkout --quiet --detach "$commit"
fi
[ -z "$(git -C "$clone" status --porcelain)" ] || { echo "xetal-libraries: work/xetal-libraries has local changes; it must match XETAL_LIBRARIES_COMMIT" >&2; exit 1; }
