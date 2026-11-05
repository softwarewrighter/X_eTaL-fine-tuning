# Run with scripts/py (the uv venv .venv), never a system python.
"""Write pages/index.html, the live catalog of the lessons and experiments, and
pages/recorded/index.html, the demos recorded at the command line.

  scripts/build-catalog.py [OUT]     # default pages/index.html
                                     # (the recorded page beside it, in recorded/)

The recorded page shows each demo's recording.webp (just record),
copied to recorded/<slug>.webp, under how to run the demos yourself.

One card per demo (scripts/demos.py json, catalog order): title,
summary, concepts, status, a link to its live page (pages/<slug>/, when
it has a web app) and to its README. The footer is the sibling
repositories': copyright, license, the repository on GitHub, and the build's provenance,
compact and without labels (host, this repo's short sha, yyyymmddThhmm), plus the pinned X_eTaL-ML commit
(XETAL_ML_COMMIT) and the X_eTaL commit it pins.
"""
import datetime
import html
import json
import shutil
import socket
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REPO = "https://github.com/softwarewrighter/X_eTaL-fine-tuning"
XETAL = "https://github.com/softwarewrighter/X_eTaL"
ML = "https://github.com/softwarewrighter/X_eTaL-ML"
STATUS = {"live": "Live", "draft": "In progress", "deferred": "Waiting on an ask"}
TRACK = {"lora": "LoRA", "variants": "LoRA variations", "alternatives": "Alternatives",
         "edge": "Bleeding edge", "applied": "Applied"}

PAGE = """<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<link rel="icon" href="{up}favicon.ico">
<style>
:root {{ --bg:#fbfaf7; --fg:#1d1d1f; --muted:#5f6368; --card:#ffffff; --line:#e3e0d8;
  --accent:#2457c5; --chip:#eef2fb; --live:#1f7a3a; --draft:#9a6200; --deferred:#8a8a8a;
  --t-builtin:#1c5fd4; --t-user:#2b8a3e; --t-num:#9c6500; --t-sym:#0b7285; }}
@media (prefers-color-scheme: dark) {{ :root:not([data-theme="light"]) {{
  --bg:#141518; --fg:#e8e6e3; --muted:#a0a4ab; --card:#1d1f23; --line:#30333a;
  --accent:#8fb0ff; --chip:#262b36; --live:#5fcf7f; --draft:#e0a84a; --deferred:#8d9097;
  --t-builtin:#8fb0ff; --t-user:#8ce99a; --t-num:#ffd43b; --t-sym:#66d9e8; }} }}
:root[data-theme="dark"] {{ --bg:#141518; --fg:#e8e6e3; --muted:#a0a4ab; --card:#1d1f23;
  --line:#30333a; --accent:#8fb0ff; --chip:#262b36; --live:#5fcf7f; --draft:#e0a84a; --deferred:#8d9097;
  --t-builtin:#8fb0ff; --t-user:#8ce99a; --t-num:#ffd43b; --t-sym:#66d9e8; }}
* {{ box-sizing: border-box; }}
body {{ margin:0; background:var(--bg); color:var(--fg);
  font: 16px/1.5 system-ui, -apple-system, "Segoe UI", sans-serif; }}
main, footer {{ max-width: 980px; margin: 0 auto; padding: 0 16px; }}
header {{ padding: 48px 0 24px; }}
h1 {{ font-size: 2rem; margin: 0 0 8px; letter-spacing: -0.01em; }}
.lede {{ color: var(--muted); max-width: 46rem; margin: 0; }}
.lede a, footer a {{ color: var(--accent); }}
.grid {{ display:grid; grid-template-columns: repeat(auto-fill, minmax(400px, 1fr)); gap:16px; padding: 8px 0 40px; }}
.card {{ background:var(--card); border:1px solid var(--line); border-radius:12px; padding:18px;
  display:flex; flex-direction:column; gap:10px; }}
.card h2 {{ font-size:1.15rem; margin:0; }}
.shot img {{ width:100%; aspect-ratio: 13 / 9; object-fit: cover; object-position: top; border-radius:8px; border:1px solid var(--line); display:block; }}
.card p {{ margin:0; color:var(--muted); }}
.status {{ font-size:.8rem; font-weight:600; }}
.track {{ font-size:.8rem; color:var(--muted); }}
.status.live {{ color:var(--live); }} .status.draft {{ color:var(--draft); }} .status.deferred {{ color:var(--deferred); }}
.chips {{ display:flex; flex-wrap:wrap; gap:6px; }}
.chip {{ background:var(--chip); border-radius:999px; padding:2px 10px; font-size:.8rem; }}
.links {{ margin-top:auto; display:flex; gap:16px; font-weight:600; }}
.links a {{ color:var(--accent); text-decoration:none; }}
.links a:hover {{ text-decoration:underline; }}
.empty {{ color:var(--muted); padding: 24px 0 48px; }}
footer {{ border-top:1px solid var(--line); padding-top:16px; padding-bottom:32px; color:var(--muted); font-size:.85rem; }}
footer .sep {{ margin: 0 8px; }}
.brand {{ display:flex; align-items:center; gap:16px; margin-bottom: 8px; }}
.brand h1 {{ margin: 0; }}
.logo {{ height: 60px; width: auto; border-radius: 8px; }}
.brand h1.ml {{ color: #ff7a00; font-weight: 800; }}
code {{ font-family: ui-monospace, "JuliaMono", Menlo, monospace; }}
pre {{ background:var(--card); border:1px solid var(--line); border-radius:8px; padding:12px 14px;
  overflow-x:auto; font: 14px/1.5 ui-monospace, Menlo, monospace; }}
.rec {{ background:var(--card); border:1px solid var(--line); border-radius:12px; padding:18px; margin: 0 0 24px; }}
.rec h2 {{ margin:0 0 6px; font-size:1.2rem; }} .rec p {{ margin:0 0 12px; color:var(--muted); }}
.rec img {{ width:100%; height:auto; border-radius:8px; display:block; background:#282a36; }}
.howto h2 {{ font-size:1.2rem; margin: 8px 0; }}
.card pre.xtl {{ margin:0; padding:8px 10px; font-size:.8rem; white-space:pre-wrap; overflow-wrap:anywhere; }}
.card p.why {{ font-size:.9rem; }}
.c-builtin {{ color: var(--t-builtin); }} .c-userfunc, .c-libfunc, .c-macro {{ color: var(--t-user); }}
.c-number {{ color: var(--t-num); }} .c-symbol {{ color: var(--t-sym); }} .c-comment {{ color: var(--muted); font-style: italic; }}
/* the catalog's table of contents: a sidebar at the top left, above the cards on a phone */
.with-toc {{ display: grid; grid-template-columns: 200px minmax(0, 1fr); gap: 28px; max-width: 1240px; margin: 0 auto; padding: 0 16px; }}
.with-toc main {{ max-width: none; margin: 0; padding: 0; }}
nav.toc {{ position: sticky; top: 16px; align-self: start; padding-top: 24px; font-size: .92rem; }}
nav.toc h2 {{ font-size: 1rem; margin: 0 0 8px; }}
nav.toc h3 {{ font-size: .8rem; text-transform: uppercase; letter-spacing: .04em; color: var(--muted); margin: 14px 0 4px; }}
nav.toc ul {{ list-style: none; padding: 0; margin: 0; }}
nav.toc li {{ margin: 3px 0; }}
nav.toc a {{ color: var(--accent); text-decoration: none; }}
nav.toc a:hover {{ text-decoration: underline; }}
@media (max-width: 760px) {{ .with-toc {{ grid-template-columns: 1fr; gap: 0; }} nav.toc {{ position: static; padding-top: 12px; }} }}
.start {{ max-width: 46rem; margin: 14px 0 0; padding: 0; list-style: none; color: var(--muted); }}
.start li {{ margin: 6px 0; }} .start b {{ color: var(--fg); }} .start a {{ color: var(--accent); }}
</style>
</head>
<body>
<div class="{layout}">
{toc}
<main>
<header>
{header}
</header>
{body}
</main>
</div>
<footer>
<span>Copyright (c) 2026 Michael A Wright</span><span class="sep">&middot;</span>
<span>MIT License</span><span class="sep">&middot;</span>
<a href="{repo}" target="_blank">GitHub</a><span class="sep">&middot;</span>
<a href="{up}doc/">Cross-reference</a><span class="sep">&middot;</span>
<span>X_eTaL-ML <a href="{ml}/commit/{mlsha}" target="_blank">{mlshort}</a></span><span class="sep">&middot;</span>
<span>X_eTaL <a href="{xetal}/commit/{xsha}" target="_blank">{xshort}</a></span><span class="sep">&middot;</span>
<span class="build" data-sha="{commit}">{host} {commit} {stamp}</span>
</footer>
</body>
</html>
"""


CATALOG_HEADER = """<div class="brand"><img class="logo" src="xetal-logo-orange.png" alt="X_eTaL logo"><h1 class="ml">X_eTaL fine-tuning</h1></div>
<p class="lede">How a trained model is adapted, in <a href="{xetal}">X_eTaL</a>, a typed array
language: fine-tuning, low-rank adapters and direct edits of a model's weights, as lessons you
can step through.</p>
<ul class="start">
<li><b>What you are looking at.</b> Each lesson is a short X_eTaL program running in your browser
(WebAssembly). It opens with a question, runs one small experiment, says what changed in plain
words and measured numbers, and keeps the mathematics and all the code it runs a click away.</li>
<li><b>Who it is for.</b> Someone who knows arithmetic and has never trained a network. Every
symbol and control is explained, by mouse, keyboard or touch; three depths (Beginner, Explorer,
Advanced) show the same computation.</li>
<li><b>Where it comes from.</b> The mathematics is that of
<a href="https://softwarewrighter.github.io/X_eTaL-ML/">X_eTaL-ML</a>'s libraries, starting from its
<a href="https://softwarewrighter.github.io/X_eTaL-ML/lora-explorer/">LoRA explorer</a>. More
X_eTaL: <a href="https://softwarewrighter.github.io/X_eTaL/">the language</a>,
<a href="https://softwarewrighter.github.io/X_eTaL-demos/">visual demos</a>,
<a href="https://softwarewrighter.github.io/X_eTaL-games/">games</a>,
<a href="https://softwarewrighter.github.io/X_eTaL-libraries/">libraries</a>,
<a href="https://softwarewrighter.github.io/X_eTaL-extensions/">extensions</a>,
<a href="https://softwarewrighter.github.io/X_eTaL-gpu/">GPUs</a>,
<a href="https://hardwarewrighter.github.io/X_eTaL-fpga/">FPGAs</a>.</li>
<li><b>Run them yourself.</b> <a href="recorded/">Every lesson and experiment at the command
line</a>, from a clone.</li>
<li><b>Read the code.</b> <a href="doc/">The cross-reference</a> (made by <code>xetal doc</code>):
every program and library here, its documentation and sections, its source drawn as X_eTaL
draws it, every name linked to where it is defined and used.</li>
</ul>"""

RECORDED_HEADER = """<div class="brand"><a href="../"><img class="logo" src="../xetal-logo-orange.png" alt="X_eTaL logo"></a><h1>Run them yourself</h1></div>
<p class="lede">Every one of the <a href="../">lessons and experiments</a> also runs at the command
line, by the X_eTaL-ML and X_eTaL commits <a href="{repo}">the repository</a> pins: each statement
of the program, drawn as X_eTaL renders it, then its result.</p>"""

HOWTO = """<section class="howto">
<h2>From a clone</h2>
<p class="lede">You need Rust (stable), git, <a href="https://github.com/casey/just">just</a> and
<a href="https://docs.astral.sh/uv/">uv</a>. The repository pins the X_eTaL-ML commit it is known
to work with (<a href="{ml}/commit/{mlsha}">{mlshort}</a>), which pins its X_eTaL
(<a href="{xetal}/commit/{xsha}">{xshort}</a>); <code>just xetal</code> fetches and builds exactly
those, so nothing else is installed.</p>
<pre>git clone {repo}
cd X_eTaL-fine-tuning
just xetal               # fetch the pinned X_eTaL-ML and build its X_eTaL (once, a few minutes)
just demos               # the lessons and experiments
just run lora-hello      # a program's results
just show lora-hello     # every statement, then its result
just serve SLUG          # a lesson's page, at http://127.0.0.1:8212/</pre>
</section>"""


def recording(m):
    slug = html.escape(m["slug"])
    links = [f'<a href="{REPO}/blob/main/demos/{slug}/{slug}.xtl">The program</a>',
             f'<a href="{REPO}/tree/main/demos/{slug}#readme">How it works</a>']
    if m.get("web"):
        links.insert(0, f'<a href="../{slug}/">Live page</a>')
    return (f'<article class="rec" id="{slug}">\n<h2>{html.escape(m["title"])}</h2>\n'
            f'<p>{html.escape(m["summary"])}</p>\n'
            f'<img src="{slug}.webp" alt="{html.escape(m["title"])} at the command line: just tour {slug}" loading="lazy">\n'
            f'<div class="links" style="margin-top:12px">{" ".join(links)}</div>\n</article>')


def rendered(line):
    """A line of X_eTaL as the pinned xetal draws it: HTML spans."""
    xetal = subprocess.run([str(ROOT / "scripts" / "xetal-ml.sh")], capture_output=True, text=True, check=True).stdout.strip()
    r = subprocess.run([xetal, "render", "--html", "-e", line], capture_output=True, text=True, check=True)
    return r.stdout.rstrip("\n")


def card(m):
    slug = html.escape(m["slug"])
    links = []
    if m.get("web"):
        links.append(f'<a href="{slug}/">Open the demo</a>')
    if m.get("recording"):
        links.append(f'<a href="recorded/#{slug}">Recorded</a>')
    links.append(f'<a href="{REPO}/tree/main/demos/{slug}#readme">How it works</a>')
    # Its program in the cross-reference (scripts/doc-site.sh: pages/doc).
    links.append(f'<a href="doc/demos-{slug}-{slug}.xtl.html">Read the code</a>')
    chips = "".join(f'<span class="chip">{html.escape(c)}</span>' for c in m["concepts"])
    st = m["status"]
    # Its track, and its place in the curriculum when it is a lesson.
    where = TRACK[m["track"]] + (f' &middot; lesson {m["lesson"]}' if m["lesson"] else " &middot; experiment")
    pic = ""
    alt = html.escape(m["title"])
    if m.get("web") and m.get("picture"):
        pic = f'<a class="shot" href="{slug}/"><img src="{slug}/screenshot.png" alt="{alt}" loading="lazy"></a>\n'
    elif m.get("recording"):
        # No live page: the card shows the demo recorded at the command line.
        pic = f'<a class="shot" href="recorded/#{slug}"><img src="recorded/{slug}.webp" alt="{alt} at the command line" loading="lazy"></a>\n'
    # The demo's key line, drawn by X_eTaL, and why it is one expression.
    one = ""
    if m.get("line"):
        one = f'<pre class="xtl">{rendered(m["line"])}</pre>\n<p class="why">{html.escape(m.get("why", ""))}</p>\n'
    return (f'<article class="card" id="{slug}">\n{pic}'
            f'<span class="status {st}">{STATUS[st]}</span>\n'
            f'<span class="track">{where}</span>\n'
            f'<h2>{html.escape(m["title"])}</h2>\n'
            f'<p>{html.escape(m["summary"])}</p>\n{one}'
            f'<div class="chips">{chips}</div>\n'
            f'<div class="links">{" ".join(links)}</div>\n</article>')


def git(*args):
    r = subprocess.run(["git", "-C", str(ROOT), *args], capture_output=True, text=True)
    return r.stdout.strip() or "unknown"


def toc(demos):
    """The table of contents: every lesson and experiment in catalog
    order (the curriculum's), linking to its card; then the other pages."""
    inside = demos
    items = "".join(f'<li><a href="#{html.escape(m["slug"])}">{html.escape(m["title"])}</a></li>' for m in inside)
    more = ('<li><a href="recorded/">Run them yourself</a></li>'
            '<li><a href="doc/">The cross-reference</a></li>'
            f'<li><a href="{REPO}">The repository</a></li>')
    return (f'<nav class="toc" aria-label="Lessons"><h2>Lessons</h2><ul>{items}</ul>'
            f'<h3>More</h3><ul>{more}</ul></nav>')


def main():
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "pages" / "index.html"
    demos = json.loads(subprocess.run([sys.executable, str(ROOT / "scripts" / "demos.py"), "json"],
                                      capture_output=True, text=True, check=True).stdout)
    mlsha = (ROOT / "XETAL_ML_COMMIT").read_text().strip()
    pinned = ROOT / "work" / "xetal-ml" / "XETAL_COMMIT"
    xsha = pinned.read_text().strip() if pinned.is_file() else "unknown"
    if demos:
        body = '<section class="grid">\n' + "\n".join(card(m) for m in demos) + "\n</section>"
    else:
        body = '<p class="empty">The first lesson is on its way.</p>'
    out.parent.mkdir(parents=True, exist_ok=True)
    common = dict(repo=REPO, xetal=XETAL, ml=ML, mlsha=mlsha, mlshort=mlsha[:7], commit=git("rev-parse", "--short", "HEAD"),
                  xsha=xsha, xshort=xsha[:7], host=socket.gethostname().split(".")[0],
                  stamp=datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M"))
    out.write_text(PAGE.format(body=body, title="X_eTaL fine-tuning", up="", layout="with-toc" if demos else "plain",
                               toc=toc(demos) if demos else "",
                               header=CATALOG_HEADER.format(**common), **common))
    rec = out.parent / "recorded"
    shutil.rmtree(rec, ignore_errors=True)
    rec.mkdir()
    recorded = [m for m in demos if m.get("recording")]
    for m in recorded:
        shutil.copy(ROOT / "demos" / m["slug"] / "recording.webp", rec / f'{m["slug"]}.webp')
    rbody = HOWTO.format(**common) + "\n" + "\n".join(recording(m) for m in recorded)
    if not recorded:
        rbody += '\n<p class="lede">None is shown recorded here; each one runs at the command line as above (<a href="../">the catalog</a>).</p>'
    (rec / "index.html").write_text(PAGE.format(body=rbody, title="X_eTaL fine-tuning: run them yourself", up="../", layout="plain", toc="",
                                                header=RECORDED_HEADER.format(**common), **common))
    print(f"catalog: {out} ({len(demos)} demo(s)), {rec}/ ({len(recorded)} recorded)")


if __name__ == "__main__":
    main()
