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
| `eli5` | every idea explained simply, where the reader asks, at the depths they ask for: `glossary.tsv` (an id, the term, a few plain sentences, and for some a worked example and the lesson that teaches it); `Here`, a context naming the page, so an idea does not point to the lesson it is in; prose marked `[shown\|id]` and `**bold**`, drawn by `prose`, `p`, `li`: a marked word is a button that opens its explanation under it (keyboard and touch too); `index(texts)` lists every idea a page links, each closed until opened; `missing(texts)` finds marks naming no idea, for a page's test |
| `explain` | a library function from the pinned library's own source: `imports(program)`, `calls(program)`, `definition(library, name)` (its doc comment, its definition, its line, the link to that line on GitHub at the pinned commit, its entry in the cross-reference), `function(alias, d, purpose, reading)` (the aside), `table(program)` (every function a program calls, in the library's words); `words(program)` the built-in words it uses, each explained from `builtins.tsv` (`unexplained` finds one missing, for a test). A page never retypes library code |
| `code` | X_eTaL drawn as microscope draws it, with every function a link into the cross-reference: `Linker::of(program)` (its imports), `Linker::within(library, source)`, `.code(src)`, `.block(src)`; `builtins(src)` lists the built-in words a program uses, for a page to explain |
| `mode` | three depths over one computation: `Mode` (Beginner, Explorer, Advanced), `when(mode, depth, body)`, and `Switch`, radio buttons. The mode is the page's state, apart from the experiment's: switching it resets nothing |
| `heat` | X_eTaL-ML's `microscope::heat` colors with what a lesson needs: `Heat` (a legend, a note on whether the scale is shared, cells marked as read or written by the step on show, an inspector naming the matrix, row, column and value; `onpick` tells the page which cell was picked, and `chosen` outlines a whole picture; optionally the number written in each cell, and a `pending` map drawn empty until its stage is reached; marked cells come in one after another); one tab stop, arrow keys over the cells, a click or tap to pick; `key(scale, shared)` for one legend under a group of maps |
| `stepper` | a pipeline stepped through from its recorded trace: `trace(line, statement)` reads what `@ flow:s_tages<` printed (each stage's value, shape, text and what it reads); `Stepper` (Start, Back, Play or Pause, Next, a bar showing how far along it is and, while playing, how long until the next stage); `flow(names, step, reads)` draws the stages in a row with a dot traveling to the stage on show from each stage it reads. Stepping moves a position in the record and runs nothing; motion is off under `prefers-reduced-motion` |
| `arithmetic` | a sum written out: `numeric` (each factor, each product, the total, all as X_eTaL printed them), `written_to` (the same to a given number of decimals, for a long sum; `rounded`), `symbolic` (the same in symbols), `show`, `commas`, `list` for numbers; `adds_up` for a page's test |
| `narrate` | a change in words from measured numbers: `moved`, `count`, `held_out`. They compare and never do arithmetic or call a result better; `forbidden` finds a verdict word, for a test |
| `predict` | `Predict`: a question answered before the experiment under it is drawn; the right answer is given by the page from its run |
| `link` | a page opened at a given state: `?mode=explorer&rank=5&stage=4` in its address sets where it starts (`param`, `number(name, low, high)`), so a state can be linked to from another lesson or a README, and so the browser check (`@ ?query` sections in `browser.txt`) looks at more than the page as first loaded |

Every part is in use on the welcome page,
[`demos/welcome`](../../demos/welcome/README.md), where a newcomer
tries each on one small calculation.

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
