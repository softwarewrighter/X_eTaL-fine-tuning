# X_eTaL-fine-tuning -- Implementation Plan

Interactive experiments in model adaptation, fine-tuning and direct
weight editing, written in X_eTaL (the eXperimental Extensible Typed
Array Language, developed in `../X_eTaL`). The repository teaches a
beginner how a model learns, why and how it is fine-tuned, how LoRA
and its relatives work, and how weights can be edited directly, with
every number on a page computed by X_eTaL; and it is a laboratory
for comparing those methods fairly.

Source: `docs/research.txt` (archival, not normative): a design review
of X_eTaL-ML's LoRA explorer for beginners, then the decision to do
that work in a new repository, X_eTaL-fine-tuning, so the explorer
stays a focused demo. This plan turns it into sagas and steps.

Development is driven by agentrail sagas (one active saga in
`.agentrail/`, finished sagas archived to `.agentrail-archive/`), as
in the sibling repositories. Every step ends with the gate (`just
gate`), docs updated (`README.md`, `CHANGES.md`, this plan, the asks
files, the demo's or library's pages), a sane `.gitignore`, a detailed
commit to `main` (with the `.agentrail/` changes), `agentrail
complete`, and a push.

## Where this repository sits

| Repo | Answers |
| ---- | ------- |
| X_eTaL | what the language is |
| X_eTaL-libraries | language-level extensibility (`.xtl`, `.xtlm`): Equations, Plot, Matrix, Check |
| X_eTaL-ML | why array languages fit machine learning: the ML libraries (LoRA, Optim, Flow, NN, Quant) and focused demos, the LoRA explorer among them |
| **X_eTaL-fine-tuning** | **how a model is adapted, taught step by step and measured**: lessons and experiments built on X_eTaL-ML |

The LoRA explorer in `../X_eTaL-ML/demos/lora-explorer/` is the
starting point and stays as it is. This repository does not copy it:
it imports the same libraries and builds a guided, beginner-first
laboratory beside it.

## Guiding principles

1. **Narrative over more mathematics.** The research's main finding:
   the explorer has the depth; what is missing is the story that ties
   what a learner sees to what the arithmetic does. Every page opens
   with a question, runs one experiment, explains what changed in
   plain words, and keeps the full mathematics a click away.
2. **Progressive depth, never less depth.** Three modes over one
   computation: Beginner (guided, plain language, one experiment at a
   time), Explorer (every control, short explanations), Advanced
   (equations, gradients, the X_eTaL source and macro expansions).
   Switching mode never resets or reruns the experiment.
3. **Every number is real.** Values, shapes, highlighted cells, loss
   figures and the sentences that quote them come from the X_eTaL run
   (Flow's stage trace, printed arrays); the browser code only lays
   them out and animates them. Animation never changes a result.
4. **Explain changes, and do not oversell.** When a control moves, the
   page says what changed and by how much, from the measured results:
   "rank 2 to 4: trainable numbers 32 to 64, as many as full
   fine-tuning on this 8 x 8 matrix; training loss 0.18 to 0.02;
   validation loss up, possibly overfitting". Never "higher rank is
   better".
5. **Approximating a matrix is not learning from examples.** The two
   experiments are kept apart and named: fitting a known update
   against the truncated SVD (linear algebra, capacity) and training
   on examples with a held-out set (machine learning, generalization).
6. **Teach X_eTaL too.** The equation, the numbers, and the X_eTaL
   expression that computed them are linked: select a symbol and its
   matrix, its pipeline stage and its source are highlighted.

## Architecture decisions

| # | Decision | Why |
| - | -------- | --- |
| F1 | **One pin: X_eTaL-ML.** `XETAL_ML_COMMIT` holds the known-good commit of X_eTaL-ML; `just xetal` clones it into the gitignored `work/xetal-ml/` and runs that clone's own `scripts/xetal.sh`, which fetches and builds the X_eTaL and X_eTaL-libraries commits X_eTaL-ML pins. `bin/xetal` links to that binary. `just ml-pin [REF]` moves the pin to a committed ref of `../X_eTaL-ML`. Nothing under `work/` is ever edited. | X_eTaL-ML's pins are already tested together; taking them transitively means this repo can never combine an X_eTaL with libraries or ML code it was not tested against. The sibling vendoring rule (`../X_eTaL/docs/vendoring.md`) applied one level up. |
| F2 | **Reuse, do not copy.** `XETAL_PATH` is this repo's `libs/*/src`, then X_eTaL-ML's `libs/*/src`, then the X_eTaL-libraries it pins (`work/xetal-ml/work/xetal-libraries/libs/*/src`, so Matrix is reachable though X_eTaL-ML links only some). The pages depend on `work/xetal-ml/shared/microscope` by path, as X_eTaL-ML's own pages do. A function X_eTaL-ML lacks is written here in a library of this repo, named in `docs/xetal-ML-asks.md`, and offered upstream once it has settled. | The ML mathematics has one home; the research: "reuse the X_eTaL-ML LoRA implementation" and "extend those rather than introduce another framework". |
| F3 | **Language last.** Order of preference for anything missing: a library in this repo, an ask to X_eTaL-ML (`docs/xetal-ML-asks.md`), an ask to X_eTaL-libraries, preferring an `.xtlm` macro to new syntax (`docs/xetal-libraries-asks.md`), and only then an ask to X_eTaL itself (`docs/xetal-asks.md`, created when the first one is needed, each justified against keeping the language small). Every ask names its workaround, which is used and shown on the page meanwhile. | The user: keep the language small; libraries and macros before language changes. |
| F4 | **Layout as in X_eTaL-ML** (its A3, A4): each demo (a lesson or an experiment) is `demos/<slug>/` with `demo.toml`, `README.md`, `*.xtl`, `reg/` reg-rs baselines, optional `train/` and `web/`; each library is `libs/<Name>/` with `src/`, `tests/` (`types.rgt`), `docs/README.md`, `demos/`. `demo.toml` gains `track` (basics, full, peft, editing, agents) and `lesson` (its place in the curriculum, or none for an experiment). | One structure across the X_eTaL repos; the research's lessons/experiments/evaluations split is carried by the catalog's fields instead of three trees. |
| F5 | **The teaching shell is shared**: `shared/lesson/`, a Yew crate on top of X_eTaL-ML's `microscope`, gives every page the same parts: the glossary and its tooltips (`shared/lesson/glossary.tsv`, one row per term: short definition, why it matters, a worked example, the lesson that teaches it), the four depths (hover: definition; click: short explanation; expand: worked example; learn more: the lesson), a stepper (play, pause, next, previous, reset) over a recorded trace, the mode switch, heat maps with a legend and a cell inspector (matrix, row, column, value, meaning, whether the color scale is shared), a cell's arithmetic expanded with its numbers, the equation with selectable symbols, the change narrator, and prediction questions. Everything reachable by keyboard and touch, never by hover alone. | The research's last point: the same educational components reused across every lesson; written once, tested once. |
| F6 | **A trace, not a re-implementation.** A page's experiment is an X_eTaL program whose pipelines are Flow pipelines; `flow:s_tages<` gives every stage's value (the trace event's inputs, output and shape), the program's source ranges locate its code, and a training loop keeps the states it shows in its `p_ower` tuple. The stepper steps through that recorded trace (so stepping back reruns nothing). | The research's "experiment trace" exists already as Flow's stages; the UI animates real computations rather than redoing them in Rust or JavaScript. |
| F7 | **Deterministic.** Fixed seeds, presets stored as data (`demos/<slug>/presets/*.toml`), the same preset gives the same trace; reg-rs baselines pin the CLI output and each page's trace; property tests for shapes and the merge identity, finite-difference checks for every gradient. | Experiments must be replayable and comparable; the research's engineering criteria. |
| F8 | **Measured or estimated, always said.** A comparison table marks each figure as measured (from the run) or analytical (from a formula), and states what it does not predict (a parameter count is not an accuracy or a runtime memory figure). | The research: "distinguish analytical estimates from measured quantities". |
| F9 | **Same process as the siblings**: `just` is the entry point (recipes call `scripts/*.sh`), adapted from X_eTaL-ML's scripts; the gate; `CHANGES.md` a line per commit; American spellings; ASCII-only markdown (`sw-markdown-checker`); Python only through uv (`scripts/py`, `.venv` from `uv venv --managed-python`); `pages/` built locally and published as the single commit of `gh-pages`; `xetal doc` comments; each name bound once (lang-choices M1); concise X_eTaL, data in files. Port 8447 is this repo's (X_eTaL-ML has 8435; 8413, 8459 and 8473 are taken). | One way of working across the X_eTaL repositories. |
| F10 | **Small first, real models later.** Toy layers (8 x 8) and tiny networks trained in X_eTaL first; small open-weight models only through a later saga, with any native or GPU code in `../X_eTaL-extensions` or `../X_eTaL-gpu`, never here. | The research: "keep the initial experiments small"; an agent choosing an intervention is not an agent knowing which weights to edit. |

## Layout

```
demos/<slug>/            a lesson or an experiment (F4)
  demo.toml README.md *.xtl reg/ presets/ train/ web/
demos/_template/         what just new-demo copies
libs/<Name>/             a library of this repo (F2, F4)
shared/lesson/           the teaching shell, on microscope (F5)
shared/lesson/glossary.tsv  every term the pages explain
scripts/                 the logic behind the just recipes
XETAL_ML_COMMIT          the X_eTaL-ML commit this repo is known to work with
work/xetal-ml/           its clone, with its own work/xetal and libraries (gitignored)
pages/                   the built site (not tracked; just publish)
```

## Tracks

| Track | Techniques | The question it teaches |
| ----- | ---------- | ----------------------- |
| basics | inputs, weights, predictions, loss, gradients, optimizers | how does a model learn? |
| full | full and selective fine-tuning, layer freezing | how does changing weights improve behavior, and what does it cost? |
| peft | LoRA, LoRA-FA, rsLoRA, DoRA, VeRA, QLoRA | how can a model adapt while training few numbers? |
| editing | rank-one editing (ROME), many edits at once (MEMIT), edits kept off preserved knowledge (AlphaEdit) | can one fact be changed without training, and what else breaks? |
| agents | an agent that diagnoses, proposes an intervention, applies it to a copy, evaluates, accepts or rolls back | can an agent improve a model through controlled experiments? |

No-weight-change methods (in-context learning, retrieval) are kept as
comparison baselines, not as fine-tuning.

## The curriculum

| # | Lesson (demo slug) | Teaches | The experiment | Saga |
| - | ------------------ | ------- | -------------- | ---- |
| 1 | what-is-ml | inputs, outputs, weights, predictions | move one weight, watch the prediction | 2 |
| 2 | how-learning-works | loss, gradient, learning rate, optimizer | train a single weight; a rate too large oscillates | 2 |
| 3 | what-is-fine-tuning | pretrained against task-adapted; full fine-tuning | adapt a trained layer to a shifted task, every weight moving; before and after | 2 |
| 4 | lora-lab (intro) | frozen weights and an adapter; why LoRA | the five-minute journey: how does a tiny adapter change a frozen layer? | 1 |
| 5 | lora-lab (forward pass) | matrix multiplication, low rank | the forward pass stepped through, every cell's arithmetic | 1 |
| 6 | lora-training | gradients, loss, updates of A and B | one training step narrated, then training with a held-out set | 1 |
| 7 | rank-and-scaling | capacity, cost, stability; alpha / r against alpha / sqrt r (rsLoRA) | rank and scale swept, the measured effect explained | 1 |
| 8 | lora-vs-fa | a frozen A and a trainable B | LoRA beside LoRA-FA, why FA's update stays in A's row space | 1 |
| 9 | qlora | precision and memory | a 4-bit base under a full-precision adapter | 3 |
| 10 | beyond-lora | DoRA, VeRA, their trade-offs | every method on the same task at equal budgets | 3 |
| 11 | editing-a-fact | a layer as a key-value memory; a rank-one edit | change one association, measure what else moved | 4 |
| 12 | agent-lab | an agent's experiment loop, rollback, provenance | an agent proposing and testing interventions on a toy model | 5 |

Each lesson: a prediction question first, one small experiment, a
plain-language analysis of the measured result, then the mathematics
on request. Presets reproduce named experiments ("first LoRA
experiment", "too little rank", "too large a learning rate",
"overfitting", "LoRA against LoRA-FA").

## The libraries (planned)

Aliases chosen not to collide with X_eTaL's, X_eTaL-libraries' or
X_eTaL-ML's (`lo:` `op:` `flow:` `nn:` `qz:` and the rest).

| Library | Alias | What | Saga |
| ------- | ----- | ---- | ---- |
| FineTune | `ft:` | full fine-tuning of a linear layer (dW, an SGD and an Adam step through Optim), a train and held-out split from a seed, the learning-from-examples task (inputs, targets, a shifted target), train, validation and retention losses, a run's record | 1 (split, task, losses), 2 (full fine-tuning) |
| PEFT | `pe:` | what X_eTaL-ML's LoRA lacks: rsLoRA's scale alpha / sqrt r, DoRA (magnitude and direction: m times the column-normalized W + s B A, its gradients), and VeRA or QLoRA only if X_eTaL-ML has not landed them (its Saga 6) | 1 (rsLoRA), 3 |
| Edit | `ed:` | a layer as a linear associative memory, a rank-one edit in closed form (ROME's update with a key covariance, solved with X_eTaL-libraries' Matrix), many edits by least squares (MEMIT), the update projected off preserved keys (AlphaEdit), locality and retention measures | 4 |

## Saga 1 -- lora-lab  [ACTIVE]

The research's recommended first milestone: one complete, polished
learning journey, "How does a tiny adapter change a frozen neural
network?", then training, rank and scaling, and LoRA-FA, all on the
8 x 8 layer of the LoRA explorer and the LoRA library of X_eTaL-ML.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | foundation | the repository's machinery, adapted from X_eTaL-ML: `XETAL_ML_COMMIT` and `scripts/xetal-ml.sh` (F1), `scripts/xt` and `just path` (F2), `scripts/py` and `.python-version`, `just gate` (pinned toolchain, spelling with self-test, markdown, rebind, `c_at` chains, doc-test, demos and libraries, pages), `just new-demo`, `just new-lib`, `just test`, `just pages`, `just serve`, `just publish`, `just check-live`, the catalog page; `demos/_template`; `shared/lesson` as an empty crate that builds natively and for wasm32 against microscope; a first CLI demo, `lora-hello`, calling `lo:f_orward` and `lo:m_erge` with baselines; the README's build section true |
| 2 | lesson-shell | `shared/lesson` (F5): the glossary (`glossary.tsv`: the research's 17 terms and every symbol of the LoRA layer) and its tooltip at four depths, heat map with legend and cell inspector, the stepper over a recorded trace, the Beginner, Explorer and Advanced switch, the change narrator (sentences from measured before-and-after values), a prediction question with feedback; keyboard and touch reachable; native tests for each part |
| 3 | lora-lab | demos/lora-lab, live: the first milestone. An intro (a pretrained model, a task, the key idea) and a prediction question; the 8 x 8 example; the forward pass `y = W x + s B (A x)` as a Flow pipeline stepped through stage by stage from `flow:s_tages<` (inputs read highlighted, the operation, outputs written, each shape), forward and back without rerunning; any output cell's arithmetic, written out and with its numbers; the equation's symbols selectable, each highlighting its matrix, its stage and its X_eTaL source; the rank slider with the trainable count and its explanation (on 8 x 8, rank 4 trains as many numbers as full fine-tuning, rank 8 twice as many); a closing comparison of the frozen layer's output with the adapted one's; the source panel shows all the code the page runs |
| 4 | lora-training | demos/lora-training, live: Experiment B, learning from examples (FineTune's task, split and losses): one training step narrated in six stages (forward, loss, gradient at the output, dA and dB, the update, W unchanged), every number from the run; then training with train and validation loss curves, step back through recorded states; presets: first experiment, too large a learning rate, overfitting; the narration of each run's outcome from its measured curves |
| 5 | rank-and-scaling | demos/rank-and-scaling, live: rank and alpha swept on the learning task; trainable numbers, train and validation loss at each, every figure marked measured or analytical (F8); `pe:` rsLoRA's alpha / sqrt r beside LoRA's alpha / r, the update's size at each rank; the narrator explains capacity, cost and stability from the measurements, without claiming rank always helps |
| 6 | lora-vs-fa | demos/lora-vs-fa, live: Experiment A, approximating a known matrix, named as such: LoRA and LoRA-FA fitting a known update against the truncated SVD's optimum (Eckart-Young); why LoRA-FA's update stays in the frozen A's row space, shown by projecting each learned update onto it; then the same two on the learning task of step 4, so the two experiments are compared and kept apart |
| 7 | lab-polish | the acceptance pass of the research: every visible symbol and control has an accessible explanation (a test lists them against the glossary), keyboard-only and touch walkthroughs of each lesson in the browser tests, a deterministic replay test per preset, a checklist for trying the lessons with novices (what rank means, why W stays frozen, what A and B do, why a small adapter changes the output) in `docs/novice-check.md`; the README's demos table and screenshots |

Acceptance for the saga (from the research): a beginner completes the
first lesson without other documentation; every visible symbol and
control is explained accessibly; the forward pass steps forward and
back; every matrix value shown comes from the real computation.

## Saga 2 -- basics-and-full  [PLANNED]

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | what-is-ml | lesson 1: one weight, one prediction; the vocabulary of weights and parameters |
| 2 | how-learning-works | lesson 2: a single weight trained, the loss curve, a learning rate too large; gradient by the chain rule, checked by finite differences |
| 3 | full-ft | FineTune's full fine-tuning (dW, steps); lesson 3: a trained layer adapted to a shifted task, every weight moving, retention of the old task measured |
| 4 | full-vs-lora | full fine-tuning, layer freezing and LoRA on the same task and budget: trainable numbers, optimizer state, train, validation and retention losses |

## Saga 3 -- peft-variants  [PLANNED]

Depends on X_eTaL-ML's Saga 6 (quant-nf4, qlora, vera): if those have
landed at the pin, this saga uses them; if not, it waits for them or
builds the missing piece in `pe:` and says so (`docs/xetal-ML-asks.md`).

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | dora | `pe:` DoRA: the weight as a magnitude and a direction, the adapter on the direction, gradients checked; a view of magnitude and direction changing apart |
| 2 | qlora | lesson 9: a 4-bit base (NF4 from X_eTaL-ML's Quant) under a full-precision adapter, against the dequantized computation; the bytes of each part |
| 3 | vera | VeRA: shared frozen random projections, learned scaling vectors; its trainable count |
| 4 | beyond-lora | lesson 10: every method on the same task at equal ranks and equal budgets, a comparison table of measured and analytical figures |

## Saga 4 -- model-editing  [PLANNED]

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | edit-lib | libs/Edit: a layer as a linear associative memory (keys to values), the rank-one closed-form edit with a key covariance (ROME), through Matrix's solve; tests that the edited key gives the new value and that the update has rank one |
| 2 | editing-a-fact | lesson 11: one association changed; what else moved (locality on unrelated keys, retention) |
| 3 | many-edits | MEMIT-style batch edits by least squares; AlphaEdit's projection off preserved keys; sequential edits degrading a model, measured |

Research methods are reimplemented from their papers and cited
(X_eTaL-ML's A8), at toy scale; the page says what the toy omits.

## Saga 5 -- agent-lab  [PLANNED]

An agent loop: diagnose a failure, propose an intervention (a LoRA
update, selective fine-tuning, a targeted edit), apply it to a
disposable copy, evaluate (task, regression, retention, cost), accept,
reject or revise, and record the evidence. First the agent is a
deterministic policy written in X_eTaL, so the loop, the rollback and
the provenance are testable; then a language-model agent driving the
same loop from the command line. Real models and the MLOS work come
after, in a saga of their own (F10).

## Cross-cutting

- Move the X_eTaL-ML pin (`just ml-pin`) at the start of a saga, or
  when an ask has landed upstream; never in the middle of a step.
  `XETAL_ML_COMMIT` changes in its own commit, with every baseline
  re-run.
- When an ask lands, the step that moves the pin removes its
  workaround and marks it landed.
- Every page: the research's tooltip inventory, the color legend and
  whether the scale is shared, plain-language text under every slider,
  explanations derived from measurements.

## Settled questions

- The repository's name and scope (the research, its last part):
  X_eTaL-fine-tuning, broader than LoRA; X_eTaL-ML's LoRA explorer is
  left unchanged.
- No change outside this repository (the user, 2026-11-05): needs of
  the sibling repositories are written down as asks, not made.
