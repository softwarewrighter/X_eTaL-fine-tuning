Step foundation (docs/plan.md, Saga 1 row 1; decisions F1, F2, F4, F9).

Build this repository's machinery, adapted from X_eTaL-ML's (read
../X_eTaL-ML/justfile and scripts/ as the model; adapt, with a note of
where each script came from; change nothing outside this repository).

Deliver:
1. The pin (F1): XETAL_ML_COMMIT (a committed ref of ../X_eTaL-ML, its
   HEAD when you start); scripts/xetal-ml.sh clones X_eTaL-ML into the
   gitignored work/xetal-ml/, checks the commit out, refuses a clone
   with local changes, runs that clone's scripts/xetal.sh (which
   fetches and builds the X_eTaL and X_eTaL-libraries it pins) and
   links bin/xetal; XETAL_ML_SOURCE=../X_eTaL-ML for a first clone
   from the local checkout. `just xetal`, `just ml-pin [REF]`, `just
   xetal-version` (all three commits and the binary's report).
2. XETAL_PATH (F2): scripts/xt and `just path`, `just eval`: this
   repo's libs/*/src, then work/xetal-ml/libs/*/src, then the pinned
   X_eTaL-libraries' libs/*/src (so Matrix is found).
3. scripts/py and .python-version (uv only, as X_eTaL-ML's; never the
   system python3).
4. The gate, `just gate` and `just gate --full`: the pinned toolchain
   check, American spelling (with its self-test), ASCII markdown
   (sw-markdown-checker), rebind, literal c_at chains, doc-test, the
   demos' and libraries' reg-rs baselines, the pages built and checked
   in headless Chrome. Also `just new-demo`, `just new-lib`, `just
   run`, `just show`, `just test`, `just test-demo`, `just bless`,
   `just pages`, `just serve` and `just serve-pages` (port 8447),
   `just publish`, `just check-live`, the catalog page from every
   demos/*/demo.toml (with the new `track` and `lesson` fields).
   demos/_template and templates/Library.
5. shared/lesson: an empty Yew crate depending on
   work/xetal-ml/shared/microscope by path, building natively and for
   wasm32-unknown-unknown, with one test that runs an X_eTaL program
   through microscope::run importing a library. Confirm or correct
   asks FM1 and FM2 in docs/xetal-ML-asks.md by trying them (does
   microscope::libs::add make a library of this repo importable? what
   does the footer report?), and record what you found.
6. A first demo, demos/lora-hello (CLI only, track "peft", no lesson):
   a short program on X_eTaL-ML's LoRA library (lo:f_orward,
   lo:m_erge, lo:p_arams on a small layer), with its README and
   baselines, proving the whole chain from the pin to a tested program.
7. README: the Build section true (remove "Not buildable yet"), Demos
   and Status updated. CHANGES.md.

If something X_eTaL-ML's scripts assume cannot hold from a dependent
repository, do not patch the clone: work around it here and file it
in docs/xetal-ML-asks.md (a blocker only if there is no workaround).
