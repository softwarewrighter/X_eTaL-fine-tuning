# Changes

Every commit, newest first, grouped by day. Times are Pacific
(UTC-08:00), as committed.

Categories: `feat` new capability, `fix` a bug or wrong behavior,
`refactor` structure without behavior change, `test` tests only,
`build` build and tooling, `demo` a demo (a lesson or an experiment)
or a change to one, `lib` a library or a change to one, `docs`
documentation, `plan` saga planning and reordering, `release`
milestone release, `chore` agentrail bookkeeping (step complete, saga
archive), `pin` a move of the X_eTaL-ML pin.

## 2026-11-05

- `build` the foundation (step foundation): one pin, `XETAL_ML_COMMIT` (X_eTaL-ML 6bac452, which pins X_eTaL abdd371 and X_eTaL-libraries 6b09aad), cloned and built by `scripts/xetal-ml.sh` through X_eTaL-ML's own script; `scripts/libpath.sh`, the search path over this repository's libraries, X_eTaL-ML's and every X_eTaL-libraries one; the justfile and the scripts adapted from X_eTaL-ML's (gate with `--full`, affected planner, demos and libraries with reg-rs baselines, templates, pages, catalog, cross-reference, browser check, publish, check-live; port 8212); `demo.toml` gains `track` and `lesson`; a library named like an upstream one is refused; `shared/lesson`, on X_eTaL-ML's microscope: the libraries its store lacks added (Matrix; ask FM1 confirmed) and this repository's footer (FM2 confirmed), compact build info without labels (host, short sha, yyyymmddThhmm) as the sibling live demos, 6 tests; demos/lora-hello: a frozen 4 x 4 layer and a rank-1 adapter on X_eTaL-ML's LoRA library, at the command line and as a live page (the input and alpha editable, the frozen output, what the adapter adds, the adapted output, the rank, the merge, 8 of 16 numbers), 5 page tests, CLI and browser baselines; README's build section true
- `docs` README: X_eTaL-gpu and X_eTaL-fpga (hardwarewrighter) in the table of X_eTaL repositories
- `plan` port 8212 for this repository's pages (the user's choice), one pin and the asks file's name settled
- `plan` the repository set up: `docs/plan.md` from `docs/research.txt` (principles, decisions F1 to F10, five tracks, a twelve-lesson curriculum, sagas 1 to 5); the agentrail saga lora-lab with its seven steps (foundation, lesson-shell, lora-lab, lora-training, rank-and-scaling, lora-vs-fa, lab-polish); `CLAUDE.md` (the agentrail briefing and this repository's rules; `AGENTS.md` a symlink to it); README, `COPYRIGHT`, `LICENSE` (MIT), `.gitignore`, `.python-version`, the logo and title; `docs/xetal-ML-asks.md` (FM1 to FM5) and `docs/xetal-libraries-asks.md` (FL1, FL2), none a blocker, no language ask
- `docs` 1st: an empty README
