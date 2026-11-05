# Asks for X_eTaL-libraries

What the lessons and experiments here need from the general libraries
of
[X_eTaL-libraries](https://github.com/softwarewrighter/X_eTaL-libraries),
reached at this repository's own pin (`XETAL_LIBRARIES_COMMIT`,
cloned into `work/xetal-libraries/`; `../X_eTaL-libraries` is read
only from here: its own plan takes them). ML-specific needs are in
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

Nothing here is a blocker: each has a workaround written out in a
program or in the pages' presentation. Found by reading X_eTaL-libraries at its
HEAD on 2026-11-05; each is checked by the step named.

| # | Status | Kind | Ask | Demos, libraries | Workaround |
| - | ------ | ---- | --- | ---------------- | ---------- |
| FL1 | in progress | feature | Equations' LaTeX (`e:t_ex<`, and Flow's `flow:m_ath<` through it) does not mark the symbols it draws: a page cannot tell which glyphs are `A`, `x` or `s`, so a symbol of the equation cannot be selected or highlighted with its matrix. Asked: an option of the macro (no new syntax) that wraps each array reference and each named scalar in a class naming it, as KaTeX's `\htmlClass{sym sym-A}{A_{k,j}}` (X_eTaL-ML's microscope does this for a pipeline's rows, in Rust, after the fact) | lora-lab (Saga 1 step 3: the equation's symbols selectable), every lesson after it | the page wraps the names it knows in the LaTeX it receives, by text, as `microscope::stages` wraps rows; a test checks every symbol of the glossary was found, so a change in the LaTeX's spelling fails loudly rather than leaving a symbol dead |
| FL2 | planned | feature | Matrix has the product, determinant, solve and inverse, but no spectrum. Asked: the eigenvalues and eigenvectors of a symmetric matrix and, from them, the singular values and a truncated SVD (`k mx:t_runcate M`, the best rank-k approximation), by an iteration with a stated tolerance | lora-vs-fa (Saga 1 step 6: the Eckart-Young optimum and the projection onto A's row space), Edit (Saga 4: a key covariance's conditioning) | power iteration with deflation on M' M, written out in the program as X_eTaL-ML's lora-explorer writes it (an unnamed lambda given to `p_ower`), and checked there against a matrix of known singular values |
| FL3 | planned | feature | Subspace geometry in Matrix (`docs/research4.txt`; with FL2): the projection of a vector or a matrix's rows onto the span of some rows, and what is left over (`B mx:p_roject M`, `B mx:r_emainder M`); an orthonormal basis of some vectors (X_eTaL-ML's `lo:r_ank` does a Gram-Schmidt inside itself and keeps only the count); the principal angles between two subspaces; a change of basis | lora-vs-fa (Saga 1: each learned update projected onto the frozen A's row space); subspace-intro and the rest of Saga 3; better-directions (Saga 4) | a projection onto the span of a few rows is `M '+ '* i_nner (o_\ B) '+ '* i_nner (mx:i_nverse B '+ '* i_nner o_\ B) '+ '* i_nner B` with Matrix's inverse (tried at the pin: the rows of M projected onto the rows of B), written out in the program where it is used; angles between subspaces wait for the ask |

## Where each stands (checked 2026-11-05 at X_eTaL-libraries e6bad16)

X_eTaL-libraries took all three ahead of its own sagas. None is in a
pushed commit as code yet, so nothing was re-pinned.

| # | Its plan | State at e6bad16 | Complete against the ask? |
| - | -------- | ---------------- | ------------------------- |
| FL1 | Saga 26, equations-symbols, 3 steps | step 1 (the design) done and pushed; step 2 (the macro) is in its work tree, not committed | yes: a new macro beside `e:t_ex<`, `e:t_exSymbols<`, same arguments, each symbol wrapped as `\htmlClass{sym sym-NAME}{...}`: the result on the left, every array reference with its subscripts, every named scalar; indices are not wrapped (`idx` reserved for later). Two consequences here, below |
| FL2 | Saga 27, matrix-subspaces, steps 3 and 4 | planned, not begun | yes: `mx:e_igenvalues`, `mx:e_igenvectors` (cyclic Jacobi, a symmetric matrix), `mx:s_ingular`, `k mx:t_runcate M`, `mx:s_vd` |
| FL3 | Saga 27, steps 1, 2 and 5 | planned, not begun | yes: `mx:o_rthonormal`, `mx:r_ank`, `B mx:p_roject M`, `B mx:r_emainder M`, `B mx:c_oordinates M`, `A mx:a_ngles B`. Its plan notes that this file's written-out projection is right but inverts `B B'`, losing digits as B's rows near dependence; the library projects through an orthonormal basis |

What the check found for this repository:

- **FL1 does not reach a Flow pipeline by itself.** The lessons draw
  their mathematics with X_eTaL-ML's `flow:m_ath<`, which goes through
  `e:t_ex<`; the marked LaTeX is a separate macro. Until Flow offers
  it (ask FM11 in [`xetal-ML-asks.md`](xetal-ML-asks.md)) a lesson
  either calls `e:t_exSymbols<` on each stage's equation itself or
  keeps wrapping names by text.
- **A page drawing marked LaTeX must set two KaTeX options** (its
  design, for KaTeX 0.18.10): `trust` allowing `\htmlClass`, and
  `strict` ignoring `htmlExtension`; with `strict: 'error'` the marked
  LaTeX is refused even when trusted.
- **The path a landed ask takes to this repository is one re-pin**
  (`docs/plan.md`, F1, since 2026-11-05): `just libs-pin`, then `just
  gate --full`, then the workaround of what landed removed and the
  ask marked landed here. This repository pins X_eTaL-libraries
  itself (`XETAL_LIBRARIES_COMMIT`), so it does not wait for
  X_eTaL-ML to move its own libraries pin.
