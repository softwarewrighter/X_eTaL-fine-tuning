#!/usr/bin/env bash
# Test one demo's built page in a real browser: headless Chrome loads
# pages/<slug>/ (scripts/browser-dom.sh), whose WebAssembly runs the
# demo's .xtl with the pinned X_eTaL; then the rendered page must show
# no X_eTaL error, be framed, show every line of
# demos/<slug>/web/browser.txt (including its sections for the page
# opened at other addresses, "@ ?stage=3"), and have no dead link into
# the cross-reference. The checks are scripts/browser-check.py; this is
# its entry, so recipes and baselines name one command.
#   scripts/browser-check.sh SLUG
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec "$root/scripts/py" "$root/scripts/browser-check.py" "${1:?usage: browser-check.sh SLUG}"
