#!/usr/bin/env bash
# Test one demo's built page in a real browser: headless Chrome loads
# pages/<slug>/ (scripts/browser-dom.sh), whose WebAssembly runs the
# demo's .xtl with the pinned X_eTaL; then the rendered page must
# show no X_eTaL error and every line of demos/<slug>/web/browser.txt
# (text the page only shows once X_eTaL has run and printed its
# arrays). Prints one line per check, the same each run, so reg-rs can
# keep it as a baseline (browser-<slug> in the demo's reg/).
# The page must also be framed: a section for each of the question, the
# background, the rationale, the purpose, the summary and what is next
# (lesson::chrome::frame), so no page jumps into its subject.
# And every link from its code into the cross-reference (pages/doc)
# must land on a real entry.
#   scripts/browser-check.sh SLUG
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
slug="${1:?usage: browser-check.sh SLUG}"
want="$root/demos/$slug/web/browser.txt"
[ -f "$want" ] || { echo "browser-check: no $want" >&2; exit 1; }
# Where the cross-reference was built (not checked against the deployed site).
doc=""; [ -n "${XETAL_LIVE:-}" ] || doc="$root/pages/doc"
"$root/scripts/browser-dom.sh" "$slug" | "$root/scripts/py" -c '
import html, re, sys
dom = sys.stdin.read()
want = [l.rstrip("\n") for l in open(sys.argv[1]) if l.strip() and not l.startswith("#")]
main = re.search(r"<main>(.*)</main>", dom, re.S)
text = html.unescape(re.sub(r"<[^>]+>", "", main.group(1))) if main else ""
ok = True
print("page: loaded" if "</html>" in dom else "page: NOT LOADED")
ok &= "</html>" in dom
# Every page is framed (lesson::chrome::Frame): it may not jump into its subject.
FRAMES = ["question", "background", "rationale", "purpose", "summary", "next"]
for f in FRAMES:
    found = f"data-frame=\"{f}\"" in dom
    print(("frame: " if found else "MISSING frame: ") + f)
    ok &= found
errors = [m for m in ("X_eTaL stopped", "class=\"error\"", "panicked") if m in dom]
print("errors: " + (", ".join(errors) if errors else "none"))
ok &= not errors
for w in want:
    found = w in text
    print(("found: " if found else "MISSING: ") + w)
    ok &= found
# Every link from the code of the page into the cross-reference lands on a
# real entry (checked against the built cross-reference; the deployed
# site is the same build, so check-live does not repeat it).
import os
doc = sys.argv[2] if len(sys.argv) > 2 else ""
if doc and os.path.isdir(doc):
    links = sorted(set(re.findall(r"href=\"\.\./doc/([^\"]+)\"", dom)))
    dead = []
    for link in links:
        page, _, anchor = link.partition("#")
        path = os.path.join(doc, page)
        if not os.path.isfile(path) or (anchor and f"id=\"{anchor}\"" not in open(path).read()):
            dead.append(link)
    print(f"cross-reference links: {len(links)}, dead: " + (", ".join(dead) if dead else "none"))
    ok &= not dead
sys.exit(0 if ok else 1)
' "$want" "$doc"
