# Libraries

There is none here, and none is planned: this repository composes
library functions and does not define its own
([`docs/plan.md`](../docs/plan.md), decision F3). The mathematics of
the lessons is X_eTaL-ML's (LoRA, Optim, Flow, NN, Quant) and
X_eTaL-libraries' (Equations, Plot, Matrix, Check), found on the
search path at the pinned commits:

```bash
export XETAL_PATH="$(just path)"       # X_eTaL-ML's libraries, then X_eTaL-libraries'
just eval '"lo:" u_se< "LoRA"
2 lo:p_arams 8 8'
```

A function a lesson needs and those libraries lack is asked for, with
the signature wanted: of X_eTaL-ML
([`docs/xetal-ML-asks.md`](../docs/xetal-ML-asks.md)), of
X_eTaL-libraries
([`docs/xetal-libraries-asks.md`](../docs/xetal-libraries-asks.md)),
or, for native code, of X_eTaL-extensions
([`docs/xetal-extensions-asks.md`](../docs/xetal-extensions-asks.md)).

The tooling for a library (`just new-lib`, `just test-lib`,
`templates/Library`) is kept for an exception the user asks for.
