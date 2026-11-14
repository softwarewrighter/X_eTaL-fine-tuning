#!/usr/bin/env bash
# Walk one demo's built page by keyboard alone and by touch in a real
# browser (scripts/browser-walk.py, through the DevTools protocol of
# headless Chrome): every control named and reached by Tab, every heat
# map with its legend, Enter, arrows and taps acting, and playing,
# pausing, stepping or moving a slider and back changing no number.
# scripts/browser-check.sh runs it after its own checks; this is the
# walk alone.
#   scripts/browser-walk.sh SLUG
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec "$root/scripts/py" "$root/scripts/browser-walk.py" "${1:?usage: browser-walk.sh SLUG}"
