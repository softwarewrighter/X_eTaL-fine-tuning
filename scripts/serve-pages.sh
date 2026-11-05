#!/usr/bin/env bash
# Serve the built pages/ locally as GitHub Pages will, under
# /X_eTaL-fine-tuning/: http://127.0.0.1:PORT/X_eTaL-fine-tuning/ (default 8212).
#   scripts/serve-pages.sh [PORT]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
port="${1:-8212}"
site="$root/target/serve-pages"
mkdir -p "$site"
ln -sfn "$root/pages" "$site/X_eTaL-fine-tuning"
echo "serving http://127.0.0.1:$port/X_eTaL-fine-tuning/ (Ctrl-C stops)"
exec "$root/scripts/py" -m http.server "$port" --bind 127.0.0.1 --directory "$site"
