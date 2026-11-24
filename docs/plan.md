# X_eTaL-fine-tuning -- Implementation Plan

Interactive experiments in model adaptation, fine-tuning and direct
weight editing, written in X_eTaL (the eXperimental Extensible Typed
Array Language, developed in `../X_eTaL`). The repository teaches a
beginner how a model learns, why and how it is fine-tuned, how LoRA
and its relatives work, and how weights can be edited directly, with
every number on a page computed by X_eTaL; and it is a laboratory
for comparing those methods fairly.

Sources (archival, not normative): `docs/research.txt`, a design
review of X_eTaL-ML's LoRA explorer for beginners, then the decision
to do that work in a new repository, X_eTaL-fine-tuning, so the
explorer stays a focused demo; and `docs/research2.txt`, the wider
discussion of fine-tuning so far: LoRA, QLoRA and LoRA-FA on
mixture-of-experts models (where adapters attach, what is stored,
what it costs), how to make a coding model follow a team's
engineering standards and prove it with an executable grader, and how
to express the mathematics in X_eTaL (largely the text of X_eTaL-ML's
`docs/fine-tune-moe-research.txt`, which goes on to VeRA).; and
`docs/research3.txt`, a review of weight-space fine-tuning without
back-propagation (zeroth-order optimization and evolution strategies:
MeZO, ES at Scale, DART-ES, AGZO, StructZO, EGGROLL, Dust, and the
Neural Thickets claim with its replication), with a priority order
and an evaluation design. And `docs/research4.txt`, on shared
subspaces: a fixed, reusable basis with small per-task coefficients as
a cross-cutting idea (a representation, not an optimizer) linking the
LoRA variations, continual learning and search without
back-propagation, with five demos proposed. And `docs/research5.txt`, compiled here: what has
been published about an AI editing a model's weights directly (one
recent, unreplicated result and its neighbors), the closed-form
editing it would use and its documented failures, what fits at toy
scale, and three demos recommended in order. And `docs/research6.txt`
(the user, 2026-11-17): a chat transcript on MoE^2-LoRA (Yang and
others, arXiv:2607.21978, July 2026), LoRA for a mixture-of-experts
model as a pool of LoRA experts routed by the base model's own router
logits, set against MoELoRA (a pool and a new router per layer) and
DAS-LoRA (adapters on the experts a domain selects); a summarizer's
account, so every claim in it is read at the paper before it is built
on. This plan turns them, and the user's
direction since, into sagas and steps.

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
   No page jumps into its subject (the user's review of the first
   live page, 2026-11-05): each is framed by the question in a
   newcomer's words ("how do I teach my local LLM new things, without
   catastrophic forgetting or excessive time and electricity?"), the
   background, the rationale and the purpose before the experiment,
   and a summary against the question and what is next after it. The
   browser check requires the six frames; a demo's README has them too.
   And it is written for new eyes (`docs/new-eyes.md`): a reader who
   knows neither machine learning nor X_eTaL and may not program.
   Every idea opens a plain explanation from the word itself; code is
   folded into asides and never needed to follow the page; a library
   function shown comes with its import, its purpose, its definition
   from the pinned source and links to source and cross-reference;
   every function in code is a link; a page is read cold by a
   reviewer given only its text before it is called done.
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
7. **Prove it** (`docs/research2.txt`): a method is compared with
   another on the same task, split and seeds, and judged on what was
   learned (the new task), what was lost (the old one), and what it
   cost (numbers trained, numbers held while training, bytes stored),
   beside a baseline that changes no weight. What a model does at its
   first attempt is kept apart from what a repair loop gets out of it.
   A figure that is an illustration, not a measurement, says so.
   From `docs/research3.txt`: methods are compared at equal compute
   and at equal memory, forward passes are counted, and the examples
   used to train, to select and to test are kept apart (the best of
   many random tries always looks like learning).

## Architecture decisions

| # | Decision | Why |
| - | -------- | --- |
| F1 | **A pin for each repository asked of, taken one at a time** (the user, 2026-11-05; at first one pin, X_eTaL-ML). `XETAL_ML_COMMIT` holds the known-good commit of X_eTaL-ML: `just xetal` clones it into the gitignored `work/xetal-ml/` and runs that clone's own `scripts/xetal.sh`, which builds the X_eTaL commit X_eTaL-ML pins; `bin/xetal` links to that binary. `XETAL_LIBRARIES_COMMIT` holds this repository's own known-good commit of X_eTaL-libraries, cloned into `work/xetal-libraries/`: the search path and the pages' store take the general libraries from it, not from the copy X_eTaL-ML holds at its own libraries pin. `just ml-pin [REF]` and `just libs-pin [REF]` move one pin each; one pin moves at a time, `just gate --full` runs, and it is committed alone. X_eTaL itself has no pin here: it is X_eTaL-ML's. Nothing under `work/` is ever edited. | Changes asked of several repositories arrive separately, not all at once: each is taken and tested alone, so a failure names the change that caused it, and a library fix does not wait for X_eTaL-ML to move its own libraries pin. The price: this repository can run a library version X_eTaL-ML has not itself run; the full gate here is the test of what the lessons use. X_eTaL stays X_eTaL-ML's because the page shell (microscope) is compiled against X_eTaL-ML's copy: a pin of its own would let the command line and the browser run different languages. |
| F2 | **Reuse, do not copy.** `XETAL_PATH` is X_eTaL-ML's `libs/*/src`, then X_eTaL-libraries' at this repository's pin (`work/xetal-libraries/libs/*/src`: every library, so Matrix is reachable though X_eTaL-ML links only some), with this repository's `libs/*/src` first should it ever have one. The pages depend on `work/xetal-ml/shared/microscope` by path, as X_eTaL-ML's own pages do. | The ML mathematics has one home; the research: "reuse the X_eTaL-ML LoRA implementation" and "extend those rather than introduce another framework". |
| F3 | **Compose, do not define; ask; language last** (the user, 2026-11-05). A demo's program calls library functions and built-in words; it defines no `u:` function, and this repository writes no `l:` library export and no `m:` macro, as a rule. A function the libraries lack is asked for: of X_eTaL-ML when it is machine learning (`docs/xetal-ML-asks.md`; that is where ML `.xtl` and `.xtlm` belong), of X_eTaL-libraries when it is general, a function or an `.xtlm` macro (`docs/xetal-libraries-asks.md`), of X_eTaL-extensions when it needs native code (`docs/xetal-extensions-asks.md`), and of X_eTaL itself last (`docs/xetal-asks.md`, created when the first is needed, justified against keeping the language small). Meanwhile the expression is written out in the program where it is used, named in the ask and on the page; if that would make the program unreadable the step waits and the ask is a blocker. Rust is for the pages' presentation only (the WebAssembly host), never for an algorithm. | The user: this is an X_eTaL demo, to be made from the X_eTaL ecosystem; the asks exist so that what is missing is added where it belongs. X_eTaL-ML is therefore on this repository's path. |
| F4 | **Layout as in X_eTaL-ML** (its A3, A4): each demo (a lesson or an experiment) is `demos/<slug>/` with `demo.toml`, `README.md`, `*.xtl`, `reg/` reg-rs baselines, optional `train/` and `web/`; each library is `libs/<Name>/` with `src/`, `tests/` (`types.rgt`), `docs/README.md`, `demos/`. `demo.toml` gains `track` (lora, variants, subspaces, alternatives, edge, applied: the Tracks below) and `lesson` (its place in the curriculum, or none for an experiment). | One structure across the X_eTaL repos; the research's lessons/experiments/evaluations split is carried by the catalog's fields instead of three trees. |
| F5 | **The teaching shell is shared**: `shared/lesson/`, a Yew crate on top of X_eTaL-ML's `microscope`, gives every page the same parts: the glossary and its tooltips (`shared/lesson/glossary.tsv`, one row per term: short definition, why it matters, a worked example, the lesson that teaches it), the four depths (hover: definition; click: short explanation; expand: worked example; learn more: the lesson), a stepper (play, pause, next, previous, reset) over a recorded trace, the mode switch, heat maps with a legend and a cell inspector (matrix, row, column, value, meaning, whether the color scale is shared), a cell's arithmetic expanded with its numbers, the equation with selectable symbols, the change narrator, and prediction questions. Everything reachable by keyboard and touch, never by hover alone. | The research's last point: the same educational components reused across every lesson; written once, tested once. |
| F6 | **A trace, not a re-implementation.** A page's experiment is an X_eTaL program whose pipelines are Flow pipelines; `flow:s_tages<` gives every stage's value (the trace event's inputs, output and shape), the program's source ranges locate its code, and a training loop keeps the states it shows in its `p_ower` tuple. The stepper steps through that recorded trace (so stepping back reruns nothing). | The research's "experiment trace" exists already as Flow's stages; the UI animates real computations rather than redoing them in Rust or JavaScript. |
| F7 | **Deterministic.** Fixed seeds, presets stored as data (`demos/<slug>/presets/*.toml`), the same preset gives the same trace; reg-rs baselines pin the CLI output and each page's trace; property tests for shapes and the merge identity, finite-difference checks for every gradient. | Experiments must be replayable and comparable; the research's engineering criteria. |
| F8 | **Measured or estimated, always said.** A comparison table marks each figure as measured (from the run) or analytical (from a formula), and states what it does not predict (a parameter count is not an accuracy or a runtime memory figure). | The research: "distinguish analytical estimates from measured quantities". |
| F9 | **Same process as the siblings**: `just` is the entry point (recipes call `scripts/*.sh`), adapted from X_eTaL-ML's scripts; the gate; `CHANGES.md` a line per commit; American spellings; ASCII-only markdown (`sw-markdown-checker`); Python only through uv (`scripts/py`, `.venv` from `uv venv --managed-python`); `pages/` built locally and published as the single commit of `gh-pages`; `xetal doc` comments; each name bound once (lang-choices M1); concise X_eTaL, data in files. Port 8212 is this repo's (the user's choice, 2026-11-05; each X_eTaL repo has its own). | One way of working across the X_eTaL repositories. |
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

## The road (the user, 2026-11-05)

Three stages, in this order:

1. **LoRA and its variations**: LoRA itself, then the variations from
   QLoRA to DoRA (rsLoRA, LoRA-FA, QLoRA, DoRA, VeRA, and others
   chosen when that saga is planned), then the idea underneath
   several of them, **shared subspaces** (`docs/research4.txt`: a
   fixed basis reused across tasks, small coefficients per task), which
   leads to learning tasks in sequence without forgetting, and into
   stage 2.
2. **Alternatives**: above all **weight-space methods that use no
   back-propagation** (the user: "another alternative to LoRA we want
   to eventually demo"; `docs/research3.txt`: zeroth-order
   optimization and evolution strategies, from MeZO to StructZO,
   DART-ES and Dust), set beside back-propagation itself (full and
   selective fine-tuning), then the other alternatives briefly.
3. **Bleeding-edge alternatives**: direct edits of a model's weights,
   and **an AI editing a model's weights directly** (the user: "a new
   focus"; `docs/research5.txt`: removing a direction from a layer
   under a check, closed-form edits under a gate, and a policy that
   chooses its own training recipe).

Shared subspaces and stages 2 and 3 are planned to be planned: each
saga opens with a planning step that settles its methods and demos
with the user and writes them down (`docs/subspaces-plan.md`,
`docs/weight-space-plan.md`, `docs/edge-plan.md`)
before anything is built. What is known now is in "Open questions".

## Tracks

A demo's `track` in its `demo.toml` is one of these.

| Track | Methods | The question it teaches |
| ----- | ------- | ----------------------- |
| start | the welcome page | what are these lessons, are they for me, and how do I use one? |
| lora | LoRA: a frozen layer, a low-rank adapter, its training, rank and scale, LoRA-FA | how can a model learn something new while its weights stay as they are? |
| variants | QLoRA, DoRA, VeRA, rsLoRA, LoRA+; adapters in a mixture of experts, and how the adapters of a mixture are chosen per token (MoELoRA, DAS-LoRA, MoE^2-LoRA) | what does each variation change, and what does it buy or cost? |
| subspaces | a fixed shared basis with per-task coefficients; which directions to keep; tasks in sequence (Share, LoDA, SuLoRA, LoRA-XS) | where may a change live, when does sharing that place between tasks help, and when does it hurt? |
| alternatives | weight-space methods without back-propagation (zeroth-order optimization, evolution strategies, activation perturbation), beside back-propagation itself (full and selective fine-tuning); weight arithmetic, other parameter-efficient methods, methods that change no weight | can a model learn from forward passes alone, at what cost, and when is that the better choice? |
| edge | closed-form edits of a layer (ROME, MEMIT, AlphaEdit), an AI editing weights directly | can a model be changed without training at all, and can a model do that to a model? |
| applied | a coding model taught a team's engineering standards, measured by an executable grader; real open-weight models | does any of this make a real model better at a real job, and how is that proved? |
| depth | residual streams and their mixing: Hyper-Connections and mHC's doubly stochastic constraint (Sinkhorn-Knopp), the gain across depth, training at 12, 24, 48 layers | why does a deep model need a skip connection, what happens when the stream is widened, and what keeps the signal bounded at any depth? (a side track, `docs/mhc-plan.md`) |

## The curriculum

Lesson numbers are the order a newcomer follows; the catalog shows
them in it. A lesson not yet settled has no number.

| # | Lesson (demo slug) | Teaches | The experiment | Saga |
| - | ------------------ | ------- | -------------- | ---- |
| | welcome | how the lessons work, for someone new: nothing to know beforehand | the seven things a lesson uses, each tried on one small calculation that is stepped through and animated | 1 (live) |
| 1 | lora-hello | the question (teaching a model without forgetting or great cost), weights, layers, a frozen layer and an adapter | change the input and alpha, watch what the adapter adds; the count on a real layer | 1 (live) |
| 2 | lora-lab | matrix multiplication, low rank, the forward pass | the forward pass stepped through, every cell's arithmetic, the equation's symbols | 1 |
| 3 | lora-training | loss, gradients, updates of A and B, a held-out set | one training step narrated, then training with validation | 1 |
| 4 | rank-and-scaling | capacity, cost, stability; alpha / r against alpha / sqrt r (rsLoRA) | rank and scale swept, the measured effect explained | 1 |
| 5 | lora-vs-fa | a frozen A and a trainable B (LoRA-FA) | LoRA beside LoRA-FA, why FA's update stays in A's row space | 1 |
| 6 | landscape | where LoRA sits among the ways to change a model, and what the next lessons compare | the same small task under each family's accounting: numbers trained, numbers held while training, what is stored | 1 |
| 7 | qlora | precision and memory: a 4-bit base under a full-precision adapter | quantization error, bytes, the merge that is not bit-exact | 2 |
| 8 | dora | a weight as a magnitude and a direction | the two changing apart; against LoRA at the same budget | 2 |
| 9 | vera | shared frozen random projections, trained scaling vectors | the trainable count against LoRA's; what the sharing costs | 2 |
| 10 | variants-compared | which variation when | every variation on the same task, equal ranks and equal budgets | 2 |
| 11 | adapters-in-moe | where adapters attach in a mixture of experts; a frozen router decides who learns | shared, every expert, selected experts; routing before and after | 2 |
| 12 | moe-lora-routing | a pool of small adapters chosen per token, and who chooses: no one (an adapter rides its expert, DAS-LoRA), a new router trained from scratch (MoELoRA), or the base router's own logits with a compressed input (MoE^2-LoRA's routing-conditioned projection) | the three on one tiny mixture: which adapter each token reaches, how far the adapter router agrees with the base router, the new-task loss, retention and the count; the pool shared across two layers against a pool per layer | 2 |
| | subspace-intro | what a subspace is: a few directions, everything made from them, projecting onto them and what is lost | small vectors, and the row space LoRA-FA was held to | 3 |
| | shared-vs-separate | one basis shared by several tasks, small coefficients for each | related and unrelated tasks, each with its own adapter and with a shared basis: what transfers, what interferes | 3 |
| | subspace-selection | which directions matter as much as how many | random, the layer's own, chosen from the data, at equal coefficients | 3 |
| | continual-subspaces | learning tasks in sequence without forgetting | a shared, a separate and an evolving basis; every old task after each new one | 3 |
| | how-learning-works, full-ft | what back-propagation is and what it costs; every weight moving, forgetting measured | one weight, then one layer: the loss, the gradient, what the backward pass keeps; a shifted task and the old task's loss | 4 |
| | forward-only | learning from forward passes alone (MeZO, evolution strategies) | the estimate beside the true gradient as passes grow; passes, memory and loss against back-propagation | 4 |
| | reward-not-loss | learning from an outcome that cannot be differentiated (DART-ES) | a toy grader as the reward; problems of unequal difficulty | 4 |
| | better-directions | where to search: a layer's input span, a structured subspace, low rank, a shared reduced space (AGZO, StructZO, EGGROLL) | each against random directions at equal forward passes; the same search in the full weights and in a shared subspace | 4 |
| | selection-noise | the best of many random tries looks like learning | selected gain against held-out gain | 4 |
| | weight-space-compared | when learning without back-propagation is the better choice | every method at equal compute and at equal memory | 4 |
| | direction-removal | an edit found and checked without training: contrast, project, check | a planted defect removed by subtracting one direction; the same policy on a layer with nothing to fix | 5 |
| | editing-a-fact, many-edits | a layer as a key-to-value memory; edits in closed form, and how they wear out | one association changed exactly; a stream of edits overwriting each other | 5 |
| | gated-edits | verifying an edit before keeping it | candidate edits scored on copies; accept, shrink or reject | 5 |
| | recipe-policy | a model directing its own fine-tuning (not direct editing) | a policy learns which recipe to apply, then forgets and collapses | 5 |

The depth track is a side track (`docs/mhc-plan.md`): six pages a
reader can take after lesson 1, or on their own, numbered D1 to D6 in
their order. How `demo.toml` carries such a number (its `lesson` is
an integer) is settled by the first of them.

| # | Lesson (demo slug) | Teaches | The experiment | Saga |
| - | ------------------ | ------- | -------------- | ---- |
| D1 | residual-streams | why a deep network needs a skip connection; four streams and the matrices that mix them | the signal through 12 to 48 random layers with and without a residual; one HC layer stepped through | mhc |
| D2 | sinkhorn | a matrix whose rows and columns each sum to 1, and how to make one | Sinkhorn-Knopp stepped through on a 4 x 4; its three properties checked | mhc |
| D3 | gain-by-depth | why an unconstrained mix explodes with depth and a doubly stochastic one cannot | products across 12, 24, 48 layers, two measures of each, a slider on the start | mhc |
| D4 | training-at-depth | what the bound does to training | baseline, HC and mHC trained at three depths; loss, gradient norm and gain per step | mhc |
| D5 | dynamic-mixing | the paper's own mixing, which depends on the input | two inputs through one layer, the gating factor as a slider, the gradient checked | mhc |
| D5b | dynamic-training | training with dynamic mixing (waits on FM13) | lesson D4's runs with the dynamic H | mhc |
| D6 | mhc-compared | what the toy showed and what it could not | the three schemes side by side with the paper's and the Python's figures | mhc |

Each lesson is framed (principle 1): the question, the background,
the rationale and the purpose first; a prediction, one small
experiment, a plain-language analysis of the measured result; then a
summary against the question and what is next. Presets reproduce
named experiments.

## The functions the lessons need (asked for, not written here)

This repository composes library functions; it does not define its
own (decision F3). What the lessons need beyond what X_eTaL-ML and
X_eTaL-libraries have at the pin is asked for, each ask with the
signature wanted and the workaround meanwhile.

| Need | Asked of | Ask | What | First needed |
| ---- | -------- | --- | ---- | ------------ |
| the learning task | X_eTaL-ML | FM7 | a task made from a seed (inputs, targets from a hidden shifted layer, optional noise), a train and held-out split, train, validation and retention losses, a run's record; the comparison harness (a method as its start, its step and its counts, every method on the same task and seeds); full and selective fine-tuning of a layer (its gradient and steps) | Saga 1, lora-training |
| rsLoRA's scale | X_eTaL-ML | FM3 | `alpha lo:s_calingRs r`: landed and pinned (bd178de) | Saga 1, rank-and-scaling |
| marked symbols in LaTeX | X_eTaL-libraries | FL1 | `e:t_exSymbols<`: landed and pinned (bf9e200); through a Flow pipeline it needs FM11 | Saga 1, lora-lab |
| a spectrum | X_eTaL-libraries | FL2 | singular values and the best rank-k approximation of a matrix | Saga 1, lora-vs-fa |
| subspace geometry | X_eTaL-libraries | FL3 | projection onto a subspace, an orthonormal basis of some vectors, the angles between two subspaces, a change of basis (with FL2's singular values) | Saga 1, lora-vs-fa (the projection onto A's row space); Saga 3 |
| the variations | X_eTaL-ML | FM4 | QLoRA (4-bit and NF4 quantization, an adapter on a quantized base), VeRA, DoRA, and the others chosen | Saga 2 |
| a mixture of experts | X_eTaL-ML | FM6 | its MoE library and a small trained base model, with the router's logits readable | Saga 2, adapters-in-moe |
| adapters chosen per token | X_eTaL-ML | FM15 | a pool of LoRA experts with a router: from the input alone (MoELoRA), from the base router's logits and a low-rank projection of the input (MoE^2-LoRA's RCP), or gated by the base router (DAS-LoRA); gradients through the router checked; the routing-agreement measure; one pool shared across layers | Saga 2, moe-lora-routing |
| shared-basis adapters | X_eTaL-ML | FM10 | an adapter in a fixed shared basis, `W x + U C V' x` (C full or diagonal), its gradient and steps; bases from a seed, from W's singular directions, from the data; a family of tasks (related, unrelated, in sequence) with transfer and forgetting measured; later a basis that evolves over tasks (Share), shared with task-specific directions (LoDA), a subspace per task (SuLoRA) | Saga 3 |
| learning without back-propagation | X_eTaL-ML | FM8 | a perturbation regenerated from its seed, the two-point estimate (MeZO), a population with antithetic pairs and centered ranks (evolution strategies), difficulty-aware reweighting and replay (DART-ES), perturbations inside a layer's input span or a structured subspace (AGZO, StructZO), low-rank perturbations (EGGROLL); forward passes and numbers held counted | Saga 4 |
| closed-form editing | X_eTaL-ML | FM9 | a layer as a linear associative memory, a rank-one edit in closed form (through Matrix's solve), many edits by least squares, the update projected off preserved keys, locality and retention | Saga 4 |
| marked mathematics in a pipeline | X_eTaL-ML | FM11 | Flow's `m_ath<` drawn through Equations' `e:t_exSymbols<` (FL1) | Saga 1, lora-lab |
| Hyper-Connections and mHC | X_eTaL-ML | FM13 | the stream mixing, Sinkhorn-Knopp with its gradient, the HC and mHC layer steps with gradients checked, both gains (the paper's Amax, the Python's proxy), the dynamic H; signatures written 2026-11-15 | Saga mhc, sinkhorn (workaround: written out) |
| a tiny transformer | X_eTaL-ML | FM14 | micro-gpt as an importable library (a sequence model a page can run), for Dust's lesson; to be specified at Saga 4's planning step | Saga 4, forward-only or later |

## Saga 1 -- lora-lab  [DONE, 2026-11-13]

The research's recommended first milestone: one complete, polished
learning journey, "How does a tiny adapter change a frozen neural
network?", then training, rank and scaling, and LoRA-FA, all on the
8 x 8 layer of the LoRA explorer and the LoRA library of X_eTaL-ML,
and a map of where LoRA sits, which the later sagas fill in. Stage 1
of the road, with Saga 2.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | foundation | DONE: one pin (`XETAL_ML_COMMIT` 6bac452, X_eTaL abdd371, X_eTaL-libraries 6b09aad), `scripts/libpath.sh` (the three library roots), the gate, the catalog with `track` and `lesson`, `shared/lesson` (the library store, FM1, and the footer, FM2, both confirmed by its tests), and demos/lora-hello with a small live page added to prove the publishing chain (the user asked for a live page early); the footer convention settled (compact build info, no labels). As planned: the repository's machinery, adapted from X_eTaL-ML: `XETAL_ML_COMMIT` and `scripts/xetal-ml.sh` (F1), `scripts/xt` and `just path` (F2), `scripts/py` and `.python-version`, `just gate` (pinned toolchain, spelling with self-test, markdown, rebind, `c_at` chains, doc-test, demos and libraries, pages), `just new-demo`, `just new-lib`, `just test`, `just pages`, `just serve`, `just publish`, `just check-live`, the catalog page; `demos/_template`; `shared/lesson` as an empty crate that builds natively and for wasm32 against microscope; a first CLI demo, `lora-hello`, calling `lo:f_orward` and `lo:m_erge` with baselines; the README's build section true |
| 1a | hello-framing | DONE (inserted 2026-11-05, the user's review of the first live page: it jumped into its subject): demos/lora-hello reframed for a newcomer (the question, the background, the idea, what the page is, things to try, the count on a 4096 x 4096 layer at rank 16 computed by X_eTaL, a summary against forgetting, time and cost, where next); `lesson::chrome::frame` and its six kinds, required of every page by the browser check and of every README by `demos.py check` |
| 1b | hello-new-eyes | DONE (inserted 2026-11-05, the user's second review: asides for `lo:f_orward` and `lo:m_erge`, the import shown, links to source and cross-reference; "new eyes": the reader knows neither ML nor X_eTaL; links to hidden plain explanations; every function in code linked): `shared/lesson` gains `eli5` (a glossary of 32 ideas, opened from the word), `explain` (a library function from the pinned library's own source) and `code` (X_eTaL drawn with every function linked into the cross-reference); demos/lora-hello rewritten against `docs/new-eyes.md` and a cold reading: the layer's numbers shown, one calculation worked in five steps, the correction as a table, code folded into asides, claims cut back to what was shown; the browser check verifies every cross-reference link |
| 2 | lesson-shell | DONE: `shared/lesson` gains `mode` (three depths and their switch), `heat` (on X_eTaL-ML's `microscope::heat`: a legend, a shared-scale note, cells marked read or written, an inspector; arrow keys), `stepper` (a pipeline's trace from `flow:s_tages<`, stepped both ways without running anything), `arithmetic` (a sum in symbols and with its numbers), `narrate` (a change in words: comparisons only, no verdict, no arithmetic), `predict` (a question before the experiment); the glossary's deeper levels (a worked example, the lesson that teaches an idea); macro libraries explained and linked as functions are; demos/shell-gallery, a page showing every part on one run (since remade as demos/welcome, row 2a); 20 shell tests. As planned: `shared/lesson` (F5): the glossary (`glossary.tsv`: the research's 17 terms and every symbol of the LoRA layer) and its tooltip at four depths, heat map with legend and cell inspector, the stepper over a recorded trace, the Beginner, Explorer and Advanced switch, the change narrator (sentences from measured before-and-after values), a prediction question with feedback; keyboard and touch reachable; native tests for each part |
| 2a | welcome | DONE (inserted 2026-11-05, the user's review of the gallery: what is it for, why is it not linked, the stepper has no animation, it should greet beginners): demos/welcome, the first card of the catalog (a `start` track) and linked from the catalog's introduction and from LoRA hello: a greeting, what the lessons are and why they are built as they are, then the seven things a lesson uses, each to try on one small calculation; the stepper animated (a stage's numbers appear one after another, a dot travels to it from each stage it reads, later stages stay blank until reached, a bar shows progress), motion off for a reader who asks for less; numbers written in the cells; read cold and revised |
| 3 | lora-lab | DONE (2026-11-06; read cold and revised: the strength and alpha named on the page, the position letters i, k, q explained and the formulas labeled by stage, what a stage reads includes its table, claims cut to what the page does). demos/lora-lab, live: the first milestone. An intro (a pretrained model, a task, the key idea) and a prediction question; the 8 x 8 example; the forward pass `y = W x + s B (A x)` as a Flow pipeline stepped through stage by stage from `flow:s_tages<` (inputs read highlighted, the operation, outputs written, each shape), forward and back without rerunning; any output cell's arithmetic, written out and with its numbers; the equation's symbols selectable, each highlighting its matrix, its stage and its X_eTaL source; the rank slider with the trainable count and its explanation (on 8 x 8, rank 4 trains as many numbers as full fine-tuning, rank 8 twice as many); a closing comparison of the frozen layer's output with the adapted one's; the source panel shows all the code the page runs |
| 4 | lora-training | DONE (2026-11-07; read cold and revised: loss, error, gradient and update said before use, the six stages listed, p named against y, the loss per example defined, the narration names which loss and quotes to 6 decimals, "held out" one name). demos/lora-training, live: Experiment B, learning from examples (the task, the split and the losses: ask FM7): one training step narrated in six stages (forward, loss, gradient at the output, dA and dB, the update, W unchanged), every number from the run; then training with train and validation loss curves, step back through recorded states; presets: first experiment, too large a learning rate, overfitting; the narration of each run's outcome from its measured curves |
| 5 | rank-and-scaling | DONE (2026-11-07; read cold and revised: the shift, the square root, W's 64 and the loss per example said before use; the account names which loss and marks every sentence; the reading of the sizes no longer judges; the account says where training left the held-out loss above its untrained value; the guess's answer given in words; the publish script rebuilds a stale pages/). demos/rank-and-scaling, live: rank and alpha swept on the learning task; trainable numbers, train and validation loss at each, every figure marked measured or analytical (F8); rsLoRA's alpha / sqrt r (ask FM3) beside LoRA's alpha / r, the update's size at each rank; the narrator explains capacity, cost and stability from the measurements, without claiming rank always helps |
| 6 | lora-vs-fa | DONE (2026-11-10; read cold and revised). demos/lora-vs-fa, live: Experiment A, approximating a known matrix, named as such: LoRA and LoRA-FA fitting the rank-4 T from the same start against the lowest loss possible at the rank (Eckart-Young, from T's singular values by power iteration with deflation, FL2's workaround, checked in the program against known singular values) and LoRA-FA's floor, T's projection onto the frozen A's row space (FL3's workaround through Matrix's inverse): LoRA reaches the lowest to rounding at rank 4 and up, LoRA-FA its floor to 13 decimals; each learned update projected onto the row space, LoRA-FA's remainder 0 and LoRA's not (tested). The row space named as the first subspace a reader meets: the glossary gains subspace, basis, projection, singular values, LoRA-FA, full fine-tuning, retention. Experiment B, learning from examples, kept apart: lesson 3's task, LoRA, LoRA-FA and full fine-tuning of W (FM7's workaround, Adam through Optim), with the loss per example trained on, held out and on 4 old examples (W's own outputs before any training), the first retention measure (added 2026-11-07, the user's question). Measured: on this task every method moves the old outputs (rank 4, 8 examples: LoRA 0.93, LoRA-FA 1.25, full fine-tuning 1.14 per old example); the page says so, and that here an adapter was not by itself a guard of old behavior. A task whose new examples lie in a region of their own, where an update could leave the old region alone, is Saga 3's |
| 7 | landscape | DONE (2026-11-10; read cold and revised). demos/landscape, live: lesson 6, the map. The six families (change no weight; change every weight or chosen ones; a small trained part beside frozen weights, LoRA and its variations LoRA-FA, QLoRA, DoRA, VeRA named with what each changes; learning without back-propagation; a closed-form edit; an AI doing the editing), each in two or three plain sentences with one line of its arithmetic on W and where it stands here (lessons 1 to 5; Saga 2 next; Sagas 3 to 5 to be planned), no claim about an unrun method beyond its definition. The grid of the two questions kept apart (`docs/research4.txt`): where the change may live (every weight; two thin tables; a few numbers scaling fixed shared tables; one rank-one edit) against how it is found (back-propagation; forward passes alone; a formula; an AI proposing and testing), every lesson live (a link) or to come (its saga) in its cell, empty cells marked as no lesson planned, shared bases a row and not a variation, the baseline that changes nothing outside the grid. The accounting computed by `landscape.xtl` for 8 x 8 at rank 4 and 4096 x 4096 at rank 16 (LoRA's count from `lo:p_arams`, the layer's bytes from `lo:b_ytes`, the rest written out as the definitions give them): numbers trained, held while training (three per number under Adam; none for a forward-only method or an edit) and stored apart, each marked A, the two families whose cost is not a count named instead; the sentences under the table written from the run. The three worries (forgetting, time, cost) per family: what its definition gives, what is measured here (linked to lessons 4 and 5), what is still to be measured. The curriculum, live or to come. Tests: every count against its formula, the map's live lessons are demos here, no verdict word, nothing unexplained; the glossary gains 15 ideas (in the context, back-propagation, forward pass, selective fine-tuning, QLoRA, DoRA, VeRA, learning without back-propagation, closed-form edit, an AI editing weights, where the change lives, how it is found, bits a number, seed, saga) |
| 8 | lab-polish | DONE (2026-11-13). The acceptance pass of the research, as tests in a real browser: `scripts/browser-walk.py` (`just browser-walk`, run by the browser check, so each page's reg-rs baseline records it; the DevTools protocol of headless Chrome, the standard library only) opens the page at its deepest mode and lists every control (buttons, inputs, links, the keyboard-reachable figures) and fails on one without a name a screen reader can speak; requires every heat map to carry its legend and scale note and every slider to sit in a label that shows its value; in a page with drawn formulas, requires every marked symbol to have a letter button and presses each, which must explain it; presses Tab from the top until the focus returns and requires every control to have been reached; acts by keyboard (arrows on the mode switch, Enter on a word that explains, Enter on Next and Back, an arrow on a heat map) and by touch (a tap on a cell, on a word, on Next); and replays: play, pause, step and return leave every number the same, and each slider moved a notch and back leaves every number the same (the narration, which compares a run with the one before it, and which cell a reader has picked are left out of that comparison; the rerun's numbers are in). The seven pages pass: 50 to 440 controls each, all named and reached; the prose-level checks (every marked word in the glossary, every function explained) stay in each page's native tests. The replay per preset was already lora-training's `every_preset_replays_to_the_same_record`; the other pages' settings are their sliders, now walked. `docs/novice-check.md`: who to try the lessons with, how, the eight questions with what a good answer contains (rank, why W stays frozen, A and B, why a small adapter changes the output, where A and B's numbers come from, what a larger rank buys and costs, whether the adapter protected the old outputs, two other ways to change a model), and how to record a session under `docs/novice/`. The README: a screenshot per lesson, the status; the catalog's order was the curriculum's already. The plan: Saga 1 closed, Saga 2 reviewed (below) |

Acceptance for the saga (from the research): a beginner completes the
first lesson without other documentation; every visible symbol and
control is explained accessibly; the forward pass steps forward and
back; every matrix value shown comes from the real computation.

Where the saga closed (2026-11-13): of the four, three are tested
(every control named and keyboard-reachable, every marked word
explained; the forward pass stepped both ways with no value changed;
every number read from the X_eTaL run, with the native tests tying
the page's numbers to the command line's program and the library).
The first, a beginner completing the first lesson unaided, is not a
test's to pass: `docs/novice-check.md` is how it is checked with
people, and no session has been held yet. What Saga 1 learned that
Saga 2 inherits: a page is a program plus prose, and the program's
printed lines are the page's whole input (`microscope::run::output`);
the cold read by a reviewer finds what the author cannot (28 findings
on the last page); writing the learning task out (FM7) cost about 20
lines a program and was readable, so the ask is not a blocker until
the comparison harness; a narrator that compares two runs without a
verdict needs its words tested (`lesson::narrate::forbidden`); the
retention measure (lesson 5) is the question every later saga comes
back to. The user's reading at the close (2026-11-14): the seven
pages pass every check and still "need a lot of editing/improvements";
the writing is not done. An editing pass over them is planned as its
own saga after the mHC saga, its kind to be specified with the user
from one example of what is wrong (`docs/plan.md`, Saga edit, below).


## Saga 2 -- lora-variants  [PLANNED, after the mHC saga]

The variations, QLoRA to DoRA, each as a lesson, then all of them on
one task. Source: `docs/research2.txt` (QLoRA and LoRA-FA on a
mixture of experts, sections 2 to 5 and "does LoRA-FA change
anything?") and the papers. X_eTaL-ML's Saga 6 plans the mathematics
of some (quant-nf4, qlora, vera, moe-lib): at this saga's start the
pin is moved, what has landed is used, and what has not is
asked for (asks FM4, FM6): a lesson whose mathematics is not there waits for it.

Reviewed at Saga 1's close (2026-11-13) against what was learned:

- Every variation's lesson keeps lesson 5's shape: the two
  experiments apart (a known matrix, then examples), the three losses
  (trained on, held out, old examples), the numbers trained, held and
  stored beside them, every figure marked. Lesson 6's accounting
  table is the template for the counts; its rows for QLoRA, DoRA and
  VeRA become measured where they were analytical.
- The comparison harness (step 2) is where writing the task out
  stops being readable (FM7's workaround was 20 lines per program in
  Saga 1; a harness running five methods on one task would repeat it
  five times). Step 1 decides whether the harness is written out once
  more here, in one program every lesson imports nothing from (a
  demo never reaches into another: rule 5), or waits for FM7; the
  recommendation is to ask X_eTaL-ML for FM7 now, at Saga 2's start,
  and write step 3 (QLoRA) meanwhile, since it needs no harness.
- QLoRA's mathematics (NF4, double quantization) and VeRA's are
  X_eTaL-ML's Saga 6; DoRA's is asked outright (FM4). The pin moves
  in step 1; a lesson whose library has not landed is written with
  its forward pass and gradient written out only if that stays under
  about 20 lines, else it waits and the step is marked blocked.
- The glossary already names QLoRA, DoRA, VeRA, bits a number and
  selective fine-tuning from lesson 6; each lesson deepens its entry
  with a worked example and takes it as `lesson`.
- The browser walk (`scripts/browser-walk.py`) runs on every new
  page from its first build: its baseline is made with `just bless`.
- Step 6 (more-variants) depends on open question 5; steps 8 and 9
  (adapters-in-moe, moe-lora-routing) on FM6 and FM15, and stay last
  so nothing waits on them.
- Step 9 was added on 2026-11-17 from `docs/research6.txt` (the
  user: the plan is to cover MoE^2-LoRA). Its claims are a chat
  summarizer's; the paper (arXiv:2607.21978) is read at source at
  this saga's planning step, which settles what the toy can show: a
  mixture of 4 experts of 8 x 8 with a top-1 or top-2 router, a pool
  of 3 or 4 LoRA experts of rank 2, two layers so that one pool shared
  across layers can be set against a pool per layer, and the routing
  conditioned on the base router's logits beside a router from
  scratch. What the toy cannot show, said on the page: the paper's
  scale (OLMoE, DeepSeek-V2-Lite, Qwen3-30B-A3B), its tasks, its
  figures (quoted as the paper's), and multimodal inputs.

The steps stand as planned; step 1 writes `docs/variants-plan.md`
and may reorder them.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | variants-plan | a planning step: the pin moved and what X_eTaL-ML has landed recorded; the variations beyond QLoRA, DoRA and VeRA are settled (the user, 2026-11-14): rsLoRA (lesson 4 has its scale, FM3) and LoRA+ (a larger rate for B); not AdaLoRA or PiSSA; the one task and the measures every variation is held to; `docs/variants-plan.md`; the asks updated |
| 2 | variant-harness | the comparison harness asked of X_eTaL-ML (ask FM7) (a method as its start, its step, its forward pass and its counts; the same task, split and seeds for all; the run's record), the retention measure (the old task's loss after learning the new one), and `shared/lesson`'s comparison table, each figure marked measured or analytical |
| 3 | qlora | lesson 7: uniform 4-bit quantization first, then the NF4 codebook; the adapter trained on the dequantized base; the quantization error, the bytes of each part, the merge that is not bit-exact |
| 4 | dora | lesson 8: DoRA (Liu and others, 2024; ask FM4): W as a magnitude per column times a direction, the adapter on the direction, its gradients checked by finite differences; magnitude and direction shown changing apart |
| 5 | vera | lesson 9: VeRA (Kopiczko and others, 2024): A and B frozen, random and shared (regenerated from a seed), two small trained vectors; its count against LoRA's; its bases are the same for every layer and task, the first shared subspace by design (Saga 3) |
| 6 | more-variants | LoRA+ (Hayou and others, 2024) as a lesson or a panel, rsLoRA's scale taken from lesson 4; the two on the harness's task beside LoRA |
| 7 | variants-compared | lesson 10: every variation on the same task at equal ranks and at equal budgets: new-task loss, validation, retention, numbers trained, bytes stored, numbers held while training; what the measurements support saying about which to use when |
| 8 | adapters-in-moe | lesson 11, on X_eTaL-ML's MoE library once it has landed (its Saga 6, steps 7 to 9; otherwise this step waits there): adapters on the shared layer, on every expert, on selected experts, the router frozen; which experts get trained at all; an all-expert adapter is not small (the research's 108 MiB a layer); routing before and after |
| 9 | moe-lora-routing | lesson 12, MoE^2-LoRA (Yang and others, arXiv:2607.21978; `docs/research6.txt`, read at the paper first) beside MoELoRA (Luo and others, 2024) and DAS-LoRA (Tang and others, 2026) on the same tiny mixture (ask FM15, or written out if it stays under about 20 lines): a pool of LoRA experts; three ways a token's adapter is chosen (riding its base expert; a router of its own from the input; the base router's logits joined to a low-rank projection of the input, `p = softmax(W_l [z; W_h h])`); which adapters each token reaches and how far the adapter router agrees with the base router (the paper's routing alignment, measured here on the toy); new-task loss, retention, the count and whether the adapters can be merged into the base (DAS-LoRA's can, the routed pools cannot); one pool shared across two layers against a pool per layer; the paper's figures quoted as its own |

## Saga 3 -- shared-subspaces  [PLANNED, to be planned]

The idea underneath several of the variations, taught for itself:
keep a basis fixed and reusable, and let each task learn only small
coefficients in it, `delta W = U C V'` (U and V shared, C the task's
own). Source: `docs/research4.txt`, which argues that this is more
than an optimization: it is a choice of *representation* (where a
change may live), separate from the *optimizer* that finds it
(back-propagation, a forward-only search, a closed-form edit), and so
the bridge from the LoRA variations to continual learning and to
Saga 4's search without back-propagation. Its claims about papers
(Share, LoDA, SuLoRA, LoRA-XS, Fisher-guided LoRA, OPLoRA, GaLore) are
the review's, read from the papers in step 1 before being built on.

Two things it must not teach: that sharing is always better (a shared
basis can be a bottleneck, unrelated tasks interfere, and moving the
basis moves every task learned on it), and that LoRA's updates form a
subspace (the rank-r updates do not, since both factors move; they do
once U and V are fixed: the precise version is an aside).

The idea is seeded earlier, so this saga names what the reader has
already seen: LoRA-FA's update stays in the row space of its frozen A
(Saga 1, lora-vs-fa: the first fixed subspace), and VeRA shares frozen
random bases and trains two vectors (Saga 2: a diagonal C).

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | subspaces-plan | a planning step, with the user: each paper of `docs/research4.txt` read from its source and cited; what each needs at toy scale; the family of tasks the lessons run on (related ones, unrelated ones, in sequence); what is asked of X_eTaL-libraries (FL3) and X_eTaL-ML (FM10) and what has landed; `docs/subspaces-plan.md`; this saga's steps below confirmed or replaced |
| 2 | subspace-intro | the lesson "what is a subspace?" for new eyes: a few directions and everything that can be made from them; a basis; projecting onto it and what is lost; shown on small vectors and on LoRA-FA's row space, already met. No training |
| 3 | shared-vs-separate | "when does sharing help?": two related tasks and two unrelated ones, each learned with its own LoRA adapter and with one shared basis and per-task coefficients (a full C, a diagonal C); what transfers and what interferes, measured; the counts for a real layer computed by X_eTaL (at 4096 x 4096 and rank 16, 131,072 a task for LoRA beside 256 or 16 for coefficients in a shared basis), and the storage as tasks accumulate (for one task sharing costs more) |
| 4 | subspace-selection | "which directions should be kept?": a random basis (as VeRA's), W's own leading singular directions (LoRA-XS), directions chosen from the data (Fisher-guided), and the pretrained model's dominant directions protected (OPLoRA), at equal coefficients: that which directions can matter as much as how many |
| 5 | continual-subspaces | "can new tasks be learned without forgetting the old?": tasks in sequence with a shared fixed basis, a basis per task kept apart (SuLoRA), a shared basis that evolves (Share), and shared with task-specific directions (LoDA); every earlier task's loss after each new one |
| 6 | subspaces-compared | what the measurements support: when to share, when to separate; the representation named apart from the optimizer, which Saga 4 changes while keeping the representation (its better-directions step searches a shared reduced space without back-propagation) |

## Saga 4 -- weight-space  [PLANNED, to be planned]

Learning without back-propagation: the alternative to LoRA the user
named. Source: `docs/research3.txt` (a review dated 2026-11-05), which
settles what the family is and the order to build it in: MeZO and
basic evolution strategies first, then DART-ES, then AGZO and
StructZO, then EGGROLL, then Dust, then hybrids. Its claims about
these papers are the review's, not yet checked here: step 1 reads each
paper before anything is built on it, and a page repeats a paper's
reported result only as reported, beside what this repository itself
measured at toy scale.

What a toy can show that a benchmark table cannot: this repository
has every exact gradient (X_eTaL-ML's `lo:g_rads`, and the full fine-tuning gradient asked for in FM7),
so an estimate made from forward passes alone can be set beside the
true gradient, its direction compared as the number of passes grows,
and the effect of choosing a better search space seen directly.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | weight-space-plan | a planning step, with the user: each paper of `docs/research3.txt` read from its source and cited (MeZO; evolution strategies and ES at Scale; DART-ES; AGZO; StructZO; EGGROLL; KronZO; Dust; Neural Thickets and its replication), what each needs that a toy layer lacks (a sequence of tokens for Dust: its lesson runs on a tiny transformer, X_eTaL-ML's micro-gpt as an importable library, ask FM14 (the user, 2026-11-14); a population on an accelerator for EGGROLL), which get a lesson, which a panel, which a paragraph; `docs/weight-space-plan.md`; this saga's steps below confirmed or replaced |
| 2 | backprop-baseline | what the alternatives are alternatives to: a loss, a gradient by the chain rule, checked by finite differences, what a backward pass must keep (every layer's input) and what the optimizer keeps; then full and selective fine-tuning (ask FM7) of a layer and the old task's loss before and after (forgetting, measured). The lessons how-learning-works and full-ft |
| 3 | forward-only | the functions asked of X_eTaL-ML (ask FM8) and the lesson: a perturbation regenerated from its seed; the two-point estimate (the loss at the weights moved one way and the other, MeZO's step); a population of them (basic evolution strategies, antithetic pairs, centered ranks). Beside back-propagation on the same task: forward passes used, numbers held in memory, the loss reached, and the estimate's direction against the true gradient as passes grow |
| 4 | reward-not-loss | what back-propagation cannot do directly: learning from an outcome that is not differentiable (a count of checks passed, a threshold, a toy grader), by evolution strategies; DART-ES's difficulty-aware reweighting and targeted replay on problems of unequal difficulty. The toy form of the applied goal's reward |
| 5 | better-directions | search geometry: perturbing inside the span of a layer's inputs, where its gradient lies (AGZO), and a structured, conditioned subspace (StructZO), against random full-dimensional directions at equal forward passes; low-rank perturbations for a population (EGGROLL), which is LoRA's idea again; and the search run in a shared reduced space, `theta = theta0 + P z` with only z perturbed (Saga 3's representation under this saga's optimizer): the optimizer and the budget held, only the search space changed |
| 6 | selection-noise | the warning (Neural Thickets and its replication): pick the best of many random perturbations and it looks like learning; held-out examples say how much was noise. Training, selection and final test sets kept apart, shown mattering |
| 7 | dust | perturbing activations, a token a population member, instead of weights: as step 1 decides (it needs a model over a sequence, so a tiny one from X_eTaL-ML, or an explanation without a run) |
| 8 | weight-space-compared | every method of this saga, back-propagation and LoRA on the same task, at equal compute and at equal memory: learned, lost, cost; what the measurements support and what remains the papers' claim |
| 9 | other-alternatives | the alternatives that are neither: back-propagation itself confined to a small subspace of gradients (GaLore), weight arithmetic (task vectors, merging: no training at the merge, but the merged models were trained), other parameter-efficient methods briefly (biases alone, scaling vectors), and the baseline that changes no weight |
## Saga 5 -- bleeding-edge  [PLANNED, to be planned]

Changing a model without training it, and an AI doing the changing:
the user's new focus. Source: `docs/research5.txt`, a report compiled
here on 2026-11-05 (the earlier research files did not cover it). What
it found, in short, and with its own markings of what was and was not
verified:

- Exactly one published item shows an AI editing a language model's
  weights directly: "Sparks of Weight-Space Programming" (MIT CSAIL, a
  blog post of 2026-11-02; no paper, no review, no replication). An
  LLM found a direction in a small model's weights and subtracted it,
  `W' = W - a u (u' W)`, with no gradient. The gain was a defect
  removed (the model stopped looping), not an ability added, and the
  same procedure on a model without the defect gained nothing.
- Everything else is a neighbor: models that write their own training
  recipes and are then fine-tuned conventionally (SEAL), networks
  trained to emit a weight change in one pass (MEND, Text-to-LoRA),
  and closed-form editing (ROME, MEMIT, AlphaEdit), which is exact on
  one matrix and, by the stricter evaluations, unreliable as a way of
  updating what a model knows.
- At toy scale the arithmetic of every one of these edits fits one
  8 x 8 layer exactly. What does not fit is finding where and what to
  edit inside a deep model, and the reasoning of the AI, which is
  what the one direct result claims as new.
- There is no established name for the topic. This plan says
  "agent-directed weight editing" and keeps "Weight-Space Programming"
  for its authors' sense.

The report recommends three demos in this order, which the steps
follow; every page says what was planted, what was assumed, and what
was only read about.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | edge-plan | a planning step, with the user: what may be called "the AI" on a page (a fixed policy written in X_eTaL, honest for search-driven editing and not for the direct result; or a recorded run of a language model reading the 64 numbers and writing the edit, and how such a run is kept reproducible); whether to build now on a days-old, unreplicated source with a dated caveat, or wait for a paper; the headline edit's strength and how its direction was found read from the source's logs; every figure a page will print re-read at its source (`docs/research5.txt` marks which came through a summarizer or an abstract); `docs/edge-plan.md`; this saga's steps below confirmed or replaced |
| 2 | direction-removal | the lesson "contrast, project, check", the one demo of a published instance: a layer with a planted defect (on some inputs it writes along a direction u); a policy takes a few bad and good examples, finds u from the difference of mean outputs with forward passes only, applies `W' = W - a u (u' W)` to a copy, and keeps the edit only if a defect score improves and a keep-intact score holds on inputs it did not use; the strength swept (exact removal at 1, over-correction beyond); the edit held to the two columns that write u most; then the same policy on a layer with no defect, where it achieves nothing. The rails shown are the source's: a disposable copy, refused kinds of edit, a counted budget, a held-out check, keep only if better. It needs no inverse and no library beyond matrix products. The page says what it does not show: finding the layer and the direction in a deep model, and that a fixed policy is not the source's claim |
| 3 | editing-a-fact | a layer as a key-to-value memory and the rank-one closed-form edit (ROME) hitting its target exactly; the same edit with and without the key covariance, and what each disturbs (ask FM9; Matrix's solve). The page states, with citations, what a toy cannot simulate: where the key and value come from in a real model, and that realistic evaluation finds single-edit success far below what was first reported |
| 4 | many-edits | many edits at once by least squares (MEMIT) and its trade-off; the update projected off preserved keys (AlphaEdit), exact when a null space exists and leaking when it does not (asks FL2, FL3); a stream of edits, each exact when made, earlier ones overwritten, drift accumulating |
| 5 | gated-edits | the loop the field proposes for an agent, driven by a fixed policy so it can be tested: several candidate edits applied to copies, scored on the target, on related and on unrelated keys; the best accepted, shrunk or rejected against a budget; the original always kept. The stream of step 4 run with and without the gate: what verification buys and what it costs in evaluations |
| 6 | recipe-policy | the best-known neighbor (SEAL), in a table: sample a recipe, apply it to a copy by a few gradient steps, score it, move the policy toward what was rewarded; then the two regressions its follow-ups report, forgetting once updates are kept and the policy collapsing onto one recipe. The page says this is self-directed fine-tuning, not direct editing, and that the policy is not the model being edited |
| 7 | ai-edits-weights | what step 1 settles: a language model's own run on the toy layer, if one is wanted, beside the fixed policy of step 2 |

Held in reserve (`docs/research5.txt`): a network that rewrites a
gradient into an edit (MEND), whose benefit at this scale is
unmeasured, and a description-to-adapter generator, which needs its
strong controls (an averaged and a shared adapter) to be honest.

Research methods are reimplemented from their papers and cited
(X_eTaL-ML's A8), at toy scale; each page says what the toy omits.

## Saga mhc -- mHC in X_eTaL  [ACTIVE, started 2026-11-15]

A side track (the user, 2026-11-11): DeepSeek's Manifold-Constrained
Hyper-Connections (Xie and others, arXiv:2512.24880) re-implemented in
X_eTaL as lessons with visualizations and inspectable code, from the
user's own Python implementation (`mHC-poc`) and post. The plan,
what the Python did, what the paper adds (its sections 5, 6 and
appendix read at source on 2026-11-15), what fits at toy scale and
what it omits, the timing, the decisions and the asks are in
`docs/mhc-plan.md`. Found while planning, in X_eTaL: the Python's
headline gain-proxy numbers are linear algebra at initialization (HC
10^27.35 across 48 layers, mHC 10^-0.602 = 1/4 at any depth); the
training lessons confirm them rather than discover them. The paper's
own gain (the largest row sum, forward; the largest column sum,
backward) is a different number from the Python's proxy (the largest
entry) and the pages show both, named. The mathematics (stream mixing,
Sinkhorn-Knopp with its gradient, the layer steps, the gains, the
dynamic `H`) is asked of X_eTaL-ML (FM13), as every method's is;
nothing in this saga depends on Sagas 2 to 5 or they on it. Placed
first (the user, 2026-11-14): its first lessons need nothing upstream,
where Saga 2 waits on FM4 and FM7. Decided with the user (2026-11-15):
a tiny MLP block, the Python's incrementing-token task on a vocabulary
of 16, static `H` first and the paper's dynamic `H` after, the
Python's figures quoted as the user's own runs. Timed: 2.4 ms per
layer per training step natively, so the 48-layer training is done
offline by X_eTaL and replayed, with a short live run on the page.
The pin stays at bd178de: nothing asked has landed; it moves when
FM13 does.

| # | Step slug | Delivers |
| - | --------- | -------- |
| 1 | mhc-plan | DONE (2026-11-15): the paper's later sections read at source; the toy timed; the five questions decided with the user; ask FM13 written with its signatures (and FM14 noted for Saga 4); the steps below confirmed; Saga 1 archived, this saga started |
| 2 | residual-streams | DONE (2026-11-17): lesson D1, demos/residual-streams, live. Two lists a hundredth apart through 48 small layers (a block 8 -> 16 -> 8, GELU, tables fixed by a formula) with the blocks replacing the list and adding to it, at three weight scales as presets: the gap between the two after every layer (replaced: 0.09 after one layer, 0 to the last digit by layer 24; added to: 1.0 to 1.6 throughout at scale 0.2; both past 1 at 0.8), X_eTaL's chart, the run quoted at five depths; the stream state as a 4 x 8 table, the three mixing tables from the logits 2I - 1 with HC's and mHC's `H_res` and their row and column sums (3.82 and 1.0, printed); the second layer stepped through in nine stages as a Flow pipeline, HC's or mHC's table chosen by a radio, checked against the matrix products (0); ten glossary entries, four built-in words explained; a side track's lesson number is the number within its track (`lesson = 1`, shown as D1 by the catalog, `track = depth`, catalog order after the lessons); the WebAssembly factor measured at 1.1 (0.60 s against 0.66 s); seven native tests, the browser walk (372 controls); the cold read found 35 things (HC and mHC unexpanded, the gap undefined, logits, softmax, the diagonal and the exponential unexplained, the preset labels stating results, the summary claiming the unquoted, row sums and column sums conflated, a glossary sum wrong), each fixed: four more glossary entries, the list quoted as numbers, a worked line of each mixing, the totals of X and R printed by the program, the presets named by their setting. Found on the way: for the symmetric start 2I - 1 the Sinkhorn projection is the softmax table itself, so mHC's `H_res` equals `H_pre`; the page says so |
| 3 | sinkhorn | DONE (2026-11-17): lesson D2, demos/sinkhorn, live. A positive 4 x 4 from three starts as data; one round as a Flow pipeline of two equations with row 1 worked; twenty rounds stepped through (every table and sum after each half round, X_eTaL's chart of the distances from 1, the count to a millionth: 1 round from the paper's start, 11 from a lopsided one, more than 20 from a nearly split one, where four of eight checks report FAIL: the paper's 20 is a cut, not a guarantee); the three properties checked by the Check library (largest singular value 1 by power iteration written out, FL2; the product of two; a mix of permutation tables by a greedy search over Combinatorics' 24 orderings); the Python's `sinkhorn_doubly_stochastic` beside the loop, line for line; four glossary entries; cold read |
| 4 | gain-by-depth | DONE (2026-11-20): lesson D3, demos/gain-by-depth, live; HC's forward gain s^k checked at every depth (3.82^48 at the start), mHC's 1; the proxy 6.39, 13.37, 27.35 against the Python's trained 7.0, 14.4, 27.3 (within one power of ten); the slider is a shift c of every logit (HC's row sum crosses 1 between -1.5 and -1.25; mHC's table the same at every c); glossary gain, composite mapping, forward and backward gain, the proxy; cold read. Planned as: lesson D3, the gain by depth, analytical: products of `H_res` across depth for HC and mHC, the paper's Amax and the Python's proxy of each, the Python's table beside the toy's, a slider on the starting logits |
| 5 | training-at-depth | DONE (2026-11-22): experiment D4, demos/training-at-depth, live. The backward pass written out in one program (about 40 lines, the Sinkhorn rounds included) and checked against central differences on nine weights (agreement to 1e-3 relative); the training run and the check one lambda mapped over jobs (a bound lambda cannot be applied twice); nine offline runs, 800 steps, by `just train` (new recipe, `train/train.sh`, 8.6 minutes, records and `data/runs.txt` committed), replayed by `replay.xtl`; a 50-step run on the page (12 layers, about 6 s in Chrome) equal to its record's first 50 steps. Measured: all learn at 12 and 24; at 48 the plain skip does not, HC and mHC do; no NaN event; the Python's HC failure not reproduced, named |
| 6 | dynamic-mixing | DONE (2026-11-23): lesson D5, demos/dynamic-mixing, live: the paper's eq. 7 to 9 read at source again; two inputs through one dynamic layer, a slider on the gating factor (0: the static tables; 0.01: apart by 0.010, 0.020, 0.0055; at 1 and 3 twenty Sinkhorn rounds fall short for one input), nine stages, the gradient through the Sinkhorn rounds, sigmoids, projection and RMS normalization checked by central differences on ten numbers. The training rerun split off (rule 3: the batched backward pass is past readable) into step 6a |
| 6a | dynamic-training | BLOCKED on FM13, reserved: lesson D4's training at depth rerun with the dynamic H, using X_eTaL-ML's layer and gradient once landed and pinned |
| 7 | mhc-compared | summary D6: the three schemes at equal depth, width and steps; the paper's and the Python's figures quoted as theirs; the X_eTaL program beside the Python, function for function |

## Saga edit -- the Saga 1 pages rewritten  [PLANNED, after the mHC saga]

The user's reading at Saga 1's close (2026-11-14): the pages "need a
lot of editing/improvements". A saga of its own, planned with the
user from one example of what is wrong on one page (the length, the
pace, the level, the order, how much X_eTaL shows, the words); its
steps are one page each, in the curriculum's order, each read cold
again before it is done. Not planned further until that example is
given.

Found by the depth track's cold reads (2026-11-17 to 2026-11-20), in
the shared shell (`shared/lesson`), so every page has them; left for
this saga, since changing the shell mid-step changes every page:
"How much to show" printed twice beside the mode switch; a section's
number running into its title in the page's text ("1Two tables");
one built-in reading for `r_/`, `r_/_1` and `r_/_2` (rows and
columns not told apart) and for `t_able` with any function; a heat
map's legend rounding a small scale to "0 0 0 0 0", and a signed
scale shown for a table with no negative numbers; mathematics read
twice by a screen reader (KaTeX's text and its markup).

## Later -- applied  [NOT PLANNED YET]

`docs/research2.txt`'s destination: a coding model that follows a
team's engineering standards (pure functions, composition, low
cyclomatic complexity) and produces fewer warnings and failures from
an executable grader, on a real open-weight mixture-of-experts model.
It is outside this repository's toy scale (F10) and needs real
training infrastructure; what belongs here first is its method, at
toy scale: an executable grader as the measure, first-attempt quality
kept apart from quality after a repair loop, supervised fine-tuning
before preference pairs (DPO) before a reward from the grader, the
ablation (base, prompted, fine-tuned, each with and without repairs),
and a method that changes no weight as the baseline. A saga of its
own when its time comes.

## Open questions (for the user)

Seven of the eight put at Saga 1's close were answered on 2026-11-14
and 2026-11-15 (Settled questions, below). One stays open, and it is
not needed before Saga 4 or 5:

1. **Where do experiments on real models live?** `docs/research3.txt`
   recommends validating on a dense model of 135M to 1B parameters,
   then coding models and mixtures of experts, with a Rust
   coding-quality benchmark. This repository is at toy scale by
   decision F10 (X_eTaL in the browser). Do those experiments belong
   in another repository (with X_eTaL-gpu), or is F10 to change?
   Recommended: another repository; F10 stays. To decide when Saga 4
   or 5 reaches it.

## Cross-cutting

- Move a pin (`just ml-pin`, `just libs-pin`) when an ask has landed
  upstream, or at the start of a saga; never in the middle of a step,
  and one pin at a time. The pin file changes in its own commit, after
  `just gate --full`. A delivery in parts is taken in parts.
- When an ask lands, the step that moves the pin removes its
  workaround and marks it landed.
- Every page: the research's tooltip inventory, the color legend and
  whether the scale is shared, plain-language text under every slider,
  explanations derived from measurements.

## Settled questions

- The repository's name and scope (the research, its last part):
  X_eTaL-fine-tuning, broader than LoRA; X_eTaL-ML's LoRA explorer is
  left unchanged.
- One pin, X_eTaL-ML (F1), to start with (the user, 2026-11-05).
- The ML asks file is `docs/xetal-ML-asks.md`; port 8212.
- Shared subspaces are a saga of their own (the user, 2026-11-05).
- A pin for each repository asked of, moved and tested one at a time
  (the user, 2026-11-05): F1. X_eTaL stays X_eTaL-ML's.
- Whether to build the direction-removal demo on a days-old source or
  wait for a paper is decided when Saga 5 is reached (the user,
  2026-11-05): there is much to build before it, and a paper, more
  examples or repositories may exist by then.
- What an AI editing weights means here: `docs/research5.txt`'s three
  demos in its order (Saga 5); the topic is called agent-directed
  weight editing.
- Shared subspaces are covered as a cross-cutting idea, a
  representation and not an optimizer, not as one more variation, and
  never as always better (`docs/research4.txt`, 2026-11-05): Saga 3.
- The weight-space family and its order (`docs/research3.txt`,
  2026-11-05): MeZO and basic evolution strategies, DART-ES, AGZO and
  StructZO, EGGROLL, Dust, hybrids; Saga 4 follows it at toy scale.
- No functions of its own (the user, 2026-11-05): a demo composes
  library functions; what is missing is asked for (F3). The libraries
  once planned here (FineTune, PEFT, WeightSpace, Edit) are asks of
  X_eTaL-ML (FM7, FM3, FM4, FM8, FM9). Rust is for presentation only.
- New eyes (the user, 2026-11-05): `docs/new-eyes.md`.
- The order (the user, 2026-11-05, after `docs/research2.txt`): LoRA
  and its variations first (QLoRA to DoRA), then alternatives (the
  weight-space, no-back-propagation approach among them), then
  bleeding-edge alternatives (an AI editing weights directly). The
  earlier order (basics and full fine-tuning second, variants third)
  is replaced; the basics a newcomer needs are in each page's framing,
  and back-propagation gets its lesson where the alternatives to it
  begin.
- No change outside this repository (the user, 2026-11-05): needs of
  the sibling repositories are written down as asks, not made.
- The mHC saga before Saga 2 (the user, 2026-11-14): its first
  lessons need nothing upstream; Saga 2 waits on FM4 and FM7.
- Trying the lessons with newcomers can wait (the user, 2026-11-14);
  `docs/novice-check.md` stands ready. The pages themselves "need a
  lot of editing/improvements": Saga edit.
- How asks reach X_eTaL-ML (the user, 2026-11-14): a session in
  X_eTaL-ML works from `docs/xetal-ML-asks.md` here, as the first
  delivery did (its own record is its `docs/fine-tuning-asks.md`);
  the user opens that session. ML functions belong in X_eTaL-ML,
  general ones in X_eTaL-libraries (F3), confirmed.
- The variations beyond QLoRA, DoRA and VeRA (the user, 2026-11-14):
  rsLoRA and LoRA+; not AdaLoRA or PiSSA.
- What "the AI" may be on a Saga 5 page (the user, 2026-11-14): a
  fixed procedure in X_eTaL, never called the AI, beside a recorded
  run of a coding agent on the same layer whose edit is committed as
  data and re-applied and checked by X_eTaL; no live model call.
- Dust gets a lesson on a tiny transformer (the user, 2026-11-14):
  X_eTaL-ML's micro-gpt as an importable library, ask FM14.
- The papers (the user, 2026-11-14): each planning step reads them
  from their source on the web and the plan links them; **no paper
  is copied into this repository** ("we should not redistribute
  papers, only link to them").
- The mHC toy (the user, 2026-11-15): a tiny MLP block; the Python's
  incrementing-token task on a vocabulary of 16; static `H` first,
  the paper's dynamic `H` after; the Python's figures quoted as the
  user's own runs, labeled (`docs/mhc-plan.md`, Decided).
