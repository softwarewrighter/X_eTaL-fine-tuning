# Asks for X_eTaL-libraries

What the lessons and experiments here need from the general libraries
of
[X_eTaL-libraries](https://github.com/softwarewrighter/X_eTaL-libraries),
reached at the commit X_eTaL-ML pins (`work/xetal-ml/`,
`XETAL_LIBRARIES_COMMIT` there; `../X_eTaL-libraries` is read only
from here: its own plan takes them). ML-specific needs are in
[`xetal-ML-asks.md`](xetal-ML-asks.md).

A need is asked for here before the language is: a function in a
library, or an `.xtlm` macro where a function cannot do it. A language
change is the last resort, in `docs/xetal-asks.md`, which does not
exist because none is needed (`docs/plan.md`, F3).

Each entry: status, kind, the ask, which demos or libraries here need
it, and the workaround in use. Status: **not yet taken** (filed here,
to be taken to that repository's plan), **planned** or **in
progress**, **landed** (at the pin, the workaround removed),
**declined** (with its reason), **blocker** (no workaround: the step
waits, and the user is told).

Nothing here is a blocker. Found by reading X_eTaL-libraries at its
HEAD on 2026-11-05; each is checked by the step named.

| # | Status | Kind | Ask | Demos, libraries | Workaround |
| - | ------ | ---- | --- | ---------------- | ---------- |
| FL1 | not yet taken | feature | Equations' LaTeX (`e:t_ex<`, and Flow's `flow:m_ath<` through it) does not mark the symbols it draws: a page cannot tell which glyphs are `A`, `x` or `s`, so a symbol of the equation cannot be selected or highlighted with its matrix. Asked: an option of the macro (no new syntax) that wraps each array reference and each named scalar in a class naming it, as KaTeX's `\htmlClass{sym sym-A}{A_{k,j}}` (X_eTaL-ML's microscope does this for a pipeline's rows, in Rust, after the fact) | lora-lab (Saga 1 step 3: the equation's symbols selectable), every lesson after it | the page wraps the names it knows in the LaTeX it receives, by text, as `microscope::stages` wraps rows; a test checks every symbol of the glossary was found, so a change in the LaTeX's spelling fails loudly rather than leaving a symbol dead |
| FL2 | not yet taken | feature | Matrix has the product, determinant, solve and inverse, but no spectrum. Asked: the eigenvalues and eigenvectors of a symmetric matrix and, from them, the singular values and a truncated SVD (`k mx:t_runcate M`, the best rank-k approximation), by an iteration with a stated tolerance | lora-vs-fa (Saga 1 step 6: the Eckart-Young optimum and the projection onto A's row space), Edit (Saga 4: a key covariance's conditioning) | power iteration with deflation on M' M, as X_eTaL-ML's lora-explorer writes it inline; here it is a function of this repository's FineTune library, checked against a matrix of known singular values |
