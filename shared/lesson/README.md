# The lesson shell: what the pages share

Every page of this repository runs an X_eTaL program in the browser
and explains what it computed. The running, the reading back of
arrays and the decorated source are X_eTaL-ML's `microscope`
(`work/xetal-ml/shared/microscope`, at the pinned commit, never
edited); this crate (`lesson`) is what this repository adds on top.

| Module | What it gives a page |
| ------ | -------------------- |
| `chrome` | `SITE` and `header(title, lede)`: microscope's header, told whose page it is (`microscope::chrome::Site`); `footer()`, as the sibling repositories' pages have it: copyright, license, GitHub, the way back to the catalog, the cross-reference, the pinned X_eTaL-ML commit and the X_eTaL commit it pins, and the build, compact and without labels (host, short commit, `yyyymmddThhmm`) |
| `chrome` | `frame(kind, title, body)`: a framing section of a page, of one of six kinds (`Frame`: the question, the background, the rationale, the purpose, the summary, what is next). No page jumps into its subject: `scripts/browser-check.sh` fails one that lacks a kind |
| `chrome` | `aside(id, summary, body)`: something one reader wants explained and another skips, hidden until its summary is opened; `panel(title, code, note, body)` |
| `eli5` | every idea explained simply, where the reader asks: `glossary.tsv` (an id, the term, a few plain sentences); prose marked `[shown\|id]` and `**bold**`, drawn by `prose`, `p`, `li`: a marked word is a button that opens its explanation under it (keyboard and touch too); `index(texts)` lists every idea a page links, each closed until opened; `missing(texts)` finds marks naming no idea, for a page's test |
| `explain` | a library function from the pinned library's own source: `imports(program)`, `calls(program)`, `definition(library, name)` (its doc comment, its definition, its line, the link to that line on GitHub at the pinned commit, its entry in the cross-reference), `function(alias, d, purpose, reading)` (the aside), `table(program)` (every function a program calls, in the library's words). A page never retypes library code |
| `code` | X_eTaL drawn as microscope draws it, with every function a link into the cross-reference: `Linker::of(program)` (its imports), `Linker::within(library, source)`, `.code(src)`, `.block(src)`; `builtins(src)` lists the built-in words a program uses, for a page to explain |

Still to come (the plan's teaching shell): the stepper over a recorded trace, heat maps with a legend
and a cell inspector, a cell's arithmetic written out, the narrator of
changes, prediction questions, the Beginner, Explorer and Advanced
switch.

## Libraries, and whose page it is

Both come from microscope, since X_eTaL-ML took this repository's asks
FM1 and FM2 ([`docs/xetal-ML-asks.md`](../../docs/xetal-ML-asks.md)):

- Libraries: `.cargo/config.toml` names `work/xetal-libraries/libs`
  (X_eTaL-libraries at this repository's own pin) in `MICROSCOPE_LIBS`,
  and microscope embeds those after X_eTaL-ML's own, replacing the
  seven it links at its own libraries pin. This crate embeds none; a
  page runs programs with `microscope::run`. A test here checks what
  was embedded against the pinned clone.
- The header is microscope's `header_of(&SITE, ...)`. The footer is
  this crate's, for the compact build information the user asked for.

## A page

`demos/lora-hello/web/` is the smallest one: `Cargo.toml` depending on
`lesson` and on `microscope` by path, `index.html` with the trunk
links (the binary, the logo, the favicon, microscope's stylesheet),
`src/micro.rs` (the model: the demo's `.xtl` included, the settings
put in its data, run with `microscope::run::output`, the printed
lines read back; tested natively in `tests/`), `src/text.rs` (its prose, marked), `src/app.rs` (the page) and `browser.txt` (text the page
shows only once X_eTaL has run, for the browser check).

The rules every page keeps. A number shown is a number X_eTaL printed:
the Rust is presentation, it lays values out and never computes the
layer. And the page is written for new eyes
([`docs/new-eyes.md`](../../docs/new-eyes.md)): a reader who knows
neither machine learning nor X_eTaL.
