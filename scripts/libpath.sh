#!/usr/bin/env bash
# The library search path, XETAL_PATH (docs/plan.md, F2): this
# repository's libs/*/src, then X_eTaL-ML's at the pin, then every
# library of the X_eTaL-libraries commit X_eTaL-ML pins (so Matrix is
# found, though X_eTaL-ML links only those it uses). PREFIX is put
# before each path, which is otherwise relative to the repository's
# root: "../../" from a demo's directory, so baselines and messages do
# not depend on where the repository is checked out; none gives
# absolute paths.
#   scripts/libpath.sh [PREFIX]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
prefix="${1-$root/}"
cd "$root"
{ ls -d libs/*/src work/xetal-ml/libs/*/src work/xetal-libraries/libs/*/src 2>/dev/null || true; } | sed "s#^#$prefix#" | paste -sd: -
