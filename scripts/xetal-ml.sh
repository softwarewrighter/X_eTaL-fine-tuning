#!/usr/bin/env bash
# Get the toolchain through this repository's pins (docs/plan.md, F1:
# X_eTaL-ML here, X_eTaL-libraries by scripts/xetal-libraries.sh, X_eTaL
# through X_eTaL-ML): clone X_eTaL-ML into work/xetal-ml (gitignored), check out the
# known-good commit in XETAL_ML_COMMIT, and run that clone's own
# scripts/xetal.sh, which fetches and builds the X_eTaL and
# X_eTaL-libraries commits X_eTaL-ML pins (work/xetal-ml/work/). Then
# symlink bin/xetal to that binary and print its path. Safe to run at
# any time: with nothing to do it only confirms the build. Adapted
# from X_eTaL-ML's scripts/xetal.sh.
#   scripts/xetal-ml.sh                               # quiet; prints bin/xetal's path
#   XETAL_ML_SOURCE=../X_eTaL-ML scripts/xetal-ml.sh  # first clone from a local checkout
#   XETAL_SOURCE and XETAL_LIBRARIES_SOURCE pass through to the clone's script.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
commit="$(tr -d '[:space:]' < "$root/XETAL_ML_COMMIT")"
source="${XETAL_ML_SOURCE:-https://github.com/softwarewrighter/X_eTaL-ML.git}"
clone="$root/work/xetal-ml"

if [ ! -d "$clone/.git" ]; then
    mkdir -p "$root/work"
    echo "xetal-ml: cloning $source into work/xetal-ml" >&2
    git clone --quiet "$source" "$clone"
fi
if ! git -C "$clone" cat-file -e "$commit^{commit}" 2>/dev/null; then
    git -C "$clone" fetch --quiet origin
fi
if [ "$(git -C "$clone" rev-parse HEAD)" != "$commit" ]; then
    git -C "$clone" checkout --quiet --detach "$commit"
fi
# work/xetal-ml is a pristine checkout of the known-good commit: never edit it.
[ -z "$(git -C "$clone" status --porcelain)" ] || { echo "xetal-ml: work/xetal-ml has local changes; it must match XETAL_ML_COMMIT" >&2; exit 1; }

# X_eTaL and the libraries X_eTaL-ML pins, built by its own script
# (into work/xetal-ml/target/xetal).
"$clone/scripts/xetal.sh" >/dev/null

# X_eTaL-libraries at this repository's own pin (work/xetal-libraries).
"$root/scripts/xetal-libraries.sh"

mkdir -p "$root/bin"
ln -sfn "../work/xetal-ml/target/xetal/release/xetal" "$root/bin/xetal"
echo "$root/bin/xetal"
