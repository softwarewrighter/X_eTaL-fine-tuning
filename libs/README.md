# Libraries

The libraries of this repository, one directory each (`libs/<Name>/`:
`src/`, `tests/`, `docs/README.md`, `demos/`). None is built yet; the
plan ([`docs/plan.md`](../docs/plan.md)) has four:

| Library | Alias | What |
| ------- | ----- | ---- |
| FineTune | `ft:` | the learning task with its train and held-out split, train, validation and retention losses, the harness every method is compared on, full fine-tuning of a layer |
| PEFT | `pe:` | the LoRA variations X_eTaL-ML's library lacks: rsLoRA's scale, DoRA, and others as needed |
| WeightSpace | `ws:` | learning without back-propagation: seeded perturbations, forward passes only, zeroth-order estimates and evolution strategies |
| Edit | `ed:` | a layer as a key-value memory, rank-one and batch edits in closed form, locality and retention |

Most of the mathematics is not here: it is X_eTaL-ML's (LoRA, Optim,
Flow, NN, Quant) and X_eTaL-libraries' (Equations, Plot, Matrix,
Check), found on the search path at the pinned commits:

```bash
export XETAL_PATH="$(just path)"       # this repository's, then X_eTaL-ML's, then X_eTaL-libraries'
just eval '"lo:" u_se< "LoRA"
2 lo:p_arams 8 8'
```

A library here may not be named like one of those (`just new-lib`
refuses it), since it would hide it.

```bash
just new-lib Name alias: "what it is"   # start one from templates/Library
just test-lib Name                      # its baselines, pinned types, expansions
just doc-test                           # every ## >> example in the doc comments
```
