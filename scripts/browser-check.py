# Run with scripts/py (the uv venv .venv), never a system python.
"""Test one demo's built page in a real browser (scripts/browser-check.sh).

Headless Chrome loads pages/<slug>/ (scripts/browser-dom.sh), whose
WebAssembly runs the demo's .xtl with the pinned X_eTaL; the rendered
page must then show no X_eTaL error, be framed (a section for each of
the question, the background, the rationale, the purpose, the summary
and what is next: lesson::chrome::frame, so no page jumps into its
subject), show every line of demos/<slug>/web/browser.txt (text the
page only shows once X_eTaL has run), and have every link from its
code into the cross-reference land on a real entry.

browser.txt: a line is text the page's <main> must show; a line
starting "html: " is a piece of the rendered document itself (an
element that only exists once something has happened); "#" starts a
comment. A line "@ ?stage=3&rank=5" starts a section checked on the
page opened at that address (lesson::link), so states a reader reaches
by clicking are checked too, not only the page as first loaded.

Prints one line per check, the same each run, so reg-rs can keep it as
a baseline (browser-<slug> in the demo's reg/).

  scripts/py scripts/browser-check.py SLUG
"""
import html
import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
# Every page is framed (lesson::chrome::Frame): it may not jump into its subject.
FRAMES = ["question", "background", "rationale", "purpose", "summary", "next"]


def sections(text):
    """(query, lines) for each section of a browser.txt: the page as first loaded, then each "@ ?query"."""
    out = [("", [])]
    for line in text.splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        if line.startswith("@ "):
            out.append((line[2:].strip(), []))
        else:
            out[-1][1].append(line.rstrip("\n"))
    return out


def dom_of(slug, query):
    env = dict(os.environ, XETAL_QUERY=query)
    return subprocess.run([str(ROOT / "scripts" / "browser-dom.sh"), slug], capture_output=True, text=True, env=env).stdout


def check(slug, query, want, live):
    dom = dom_of(slug, query)
    main = re.search(r"<main>(.*)</main>", dom, re.S)
    text = html.unescape(re.sub(r"<[^>]+>", "", main.group(1))) if main else ""
    ok = True
    if query:
        print(f"at {query}:")
    print("page: loaded" if "</html>" in dom else "page: NOT LOADED")
    ok &= "</html>" in dom
    for f in FRAMES:
        found = f'data-frame="{f}"' in dom
        print(("frame: " if found else "MISSING frame: ") + f)
        ok &= found
    errors = [m for m in ("X_eTaL stopped", 'class="error"', "panicked") if m in dom]
    print("errors: " + (", ".join(errors) if errors else "none"))
    ok &= not errors
    for w in want:
        found = w[6:] in dom if w.startswith("html: ") else w in text
        print(("found: " if found else "MISSING: ") + w)
        ok &= found
    # Every link from the code of the page into the cross-reference lands on a
    # real entry (checked against the built cross-reference; the deployed
    # site is the same build, so check-live does not repeat it).
    doc = ROOT / "pages" / "doc"
    if not live and doc.is_dir():
        links = sorted(set(re.findall(r'href="\.\./doc/([^"]+)"', dom)))
        dead = []
        for link in links:
            page, _, anchor = link.partition("#")
            path = doc / page
            if not path.is_file() or (anchor and f'id="{anchor}"' not in path.read_text()):
                dead.append(link)
        print(f"cross-reference links: {len(links)}, dead: " + (", ".join(dead) if dead else "none"))
        ok &= not dead
    return ok


def main(slug):
    want = ROOT / "demos" / slug / "web" / "browser.txt"
    if not want.is_file():
        sys.exit(f"browser-check: no {want}")
    live = bool(os.environ.get("XETAL_LIVE"))
    ok = True
    for query, lines in sections(want.read_text()):
        ok &= check(slug, query, lines, live)
    return 0 if ok else 1


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit("usage: scripts/py scripts/browser-check.py SLUG")
    sys.exit(main(sys.argv[1]))
