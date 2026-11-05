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

- `plan` the road reset after `docs/research2.txt` and the user's order: LoRA and its variations first (QLoRA to DoRA), then alternatives (the weight-space, no-back-propagation approach among them), then bleeding-edge alternatives (an AI editing weights directly, approach to be decided). docs/plan.md: the road, new tracks (lora, variants, alternatives, edge, applied), the curriculum renumbered (LoRA hello is lesson 1), principle 7 (prove it: the same task and seeds, learned, lost and cost, a baseline that changes no weight), a WeightSpace library, Saga 1 gains a step, landscape (the map of where LoRA sits), Saga 2 is lora-variants, Saga 3 alternatives and Saga 4 bleeding-edge each open with a planning step, the applied goal (a coder that follows engineering standards, proved by a grader) noted for later, open questions for the user. `demo.toml`'s tracks renamed in the tooling and the catalog; README's road; asks FM4 updated, FM6 (the MoE library) added
- `chore` saga step hello-framing closed
- `demo` LoRA hello framed for a newcomer (step hello-framing; the user's review: it "lacks intro, context, purpose, rationale, and summary"): the page opens with the question (how do I teach my local model something new without it forgetting, and without the time and electricity?), the background (weights, layers, fine-tuning, why changing every number is costly and can overwrite what was known), the idea (freeze, train a thin correction) with its terms, and what the page is and is not; then things to try, the experiment, the count on a 4096 x 4096 layer at rank 16 computed by X_eTaL (131,072 of 16,777,216, 0.78%); then a summary against the three worries, honest about what a count and a frozen base do not show, and where next. `lesson::chrome::frame` and its six kinds, required of every page by the browser check and of every demo README by `demos.py check` (the template has them); the rule in CLAUDE.md and the plan. The program prints two more lines; CLI and browser baselines reblessed after review (additions only); a new screenshot
- `chore` saga step foundation closed
- `build` the foundation (step foundation): one pin, `XETAL_ML_COMMIT` (X_eTaL-ML 6bac452, which pins X_eTaL abdd371 and X_eTaL-libraries 6b09aad), cloned and built by `scripts/xetal-ml.sh` through X_eTaL-ML's own script; `scripts/libpath.sh`, the search path over this repository's libraries, X_eTaL-ML's and every X_eTaL-libraries one; the justfile and the scripts adapted from X_eTaL-ML's (gate with `--full`, affected planner, demos and libraries with reg-rs baselines, templates, pages, catalog, cross-reference, browser check, publish, check-live; port 8212); `demo.toml` gains `track` and `lesson`; a library named like an upstream one is refused; `shared/lesson`, on X_eTaL-ML's microscope: the libraries its store lacks added (Matrix; ask FM1 confirmed) and this repository's footer (FM2 confirmed), compact build info without labels (host, short sha, yyyymmddThhmm) as the sibling live demos, 6 tests; demos/lora-hello: a frozen 4 x 4 layer and a rank-1 adapter on X_eTaL-ML's LoRA library, at the command line and as a live page (the input and alpha editable, the frozen output, what the adapter adds, the adapted output, the rank, the merge, 8 of 16 numbers), 5 page tests, CLI and browser baselines; README's build section true
- `docs` README: X_eTaL-gpu and X_eTaL-fpga (hardwarewrighter) in the table of X_eTaL repositories
- `plan` port 8212 for this repository's pages (the user's choice), one pin and the asks file's name settled
- `plan` the repository set up: `docs/plan.md` from `docs/research.txt` (principles, decisions F1 to F10, five tracks, a twelve-lesson curriculum, sagas 1 to 5); the agentrail saga lora-lab with its seven steps (foundation, lesson-shell, lora-lab, lora-training, rank-and-scaling, lora-vs-fa, lab-polish); `CLAUDE.md` (the agentrail briefing and this repository's rules; `AGENTS.md` a symlink to it); README, `COPYRIGHT`, `LICENSE` (MIT), `.gitignore`, `.python-version`, the logo and title; `docs/xetal-ML-asks.md` (FM1 to FM5) and `docs/xetal-libraries-asks.md` (FL1, FL2), none a blocker, no language ask
- `docs` 1st: an empty README
