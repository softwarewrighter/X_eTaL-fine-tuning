# Asks for X_eTaL-ML

What the lessons and experiments here need from
[X_eTaL-ML](https://github.com/softwarewrighter/X_eTaL-ML), pinned by
`XETAL_ML_COMMIT` (`../X_eTaL-ML`, read only from here: its own plan
takes them). This repository does not change X_eTaL-ML: each need is
filed here, the workaround named below is used and shown on the page
meanwhile, and it is removed when the ask lands in the commit pinned.

General needs (mathematics, notation, plotting) are in
[`xetal-libraries-asks.md`](xetal-libraries-asks.md). A need of the
language itself would go in `docs/xetal-asks.md`, which does not exist
because there is none: the order of preference is a library here, an
ask here, an ask to X_eTaL-libraries (a function, or an `.xtlm` macro
where a function cannot do it), and a language change last
(`docs/plan.md`, F3).

Each entry: status, kind (feature, bug, speed), the ask, which demos
or libraries here need it, and the workaround in use. Status: **not
yet taken** (filed here, to be taken to X_eTaL-ML's plan), **planned**
or **in progress** (in its plan), **landed** (in the commit pinned
here, the workaround removed), **declined** (with its reason),
**blocker** (no workaround: the step waits, and the user is told).

Nothing here is a blocker: every ask has a workaround inside this
repository. Asks marked "to confirm" were found by reading X_eTaL-ML
at 6bac452 (2026-11-05) and are checked by the step named.

| # | Status | Kind | Ask | Demos, libraries | Workaround |
| - | ------ | ---- | --- | ---------------- | ---------- |
| FM1 | not yet taken | feature | `shared/microscope` embeds the libraries of its own repository only: its `build.rs` reads `../../libs` and `../../work/libs` beside its manifest, so a page of another repository gets X_eTaL-ML's libraries and the seven of X_eTaL-libraries it links (Check, Plot, Strings, Format, Lists, Equations, Parsing), but not that repository's own (`FineTune`, `PEFT`, `Edit`) nor another general one (Matrix). Asked: more library roots named by the dependent (an environment variable read by `build.rs`, a list of directories), or `microscope::libs::add` documented as the way to add a library's source | every page here that imports a library of this repository (to confirm in step foundation) | `shared/lesson`'s own `build.rs` embeds this repository's `libs/*/src` and the general libraries it names, and puts each in the store with `microscope::libs::add("Name.xtl", source)` before the first run |
| FM2 | not yet taken | feature | `microscope::chrome::footer` names X_eTaL-ML: `REPO` is a constant (`https://github.com/softwarewrighter/X_eTaL-ML`), and `build.rs` takes the build's commit with `git rev-parse` where the crate lies, which from here is the X_eTaL-ML clone's commit, not this repository's. Asked: the footer (and the header's catalog link) taking the repository's name, URL and commit from the page | every page here (to confirm in step foundation) | `shared/lesson` has its own footer: this repository, its commit, and the pinned X_eTaL-ML and X_eTaL commits |
| FM3 | not yet taken | feature | LoRA has one scale, `alpha lo:s_caling r` (alpha / r). Asked: rsLoRA's rank-stabilized scale alpha / sqrt r beside it (a function of the same shape, `lo:s_calingRs`), since an adapter is `(W, A, B, s)` and nothing else changes | rank-and-scaling (Saga 1 step 5) | `pe:s_calingRs` in this repository's PEFT library, offered to LoRA once it has settled |
| FM4 | planned | feature | QLoRA (the NF4 codebook, double quantization, an adapter on a quantized base) and VeRA are X_eTaL-ML's Saga 6 steps quant-nf4, qlora and vera, not built at 6bac452. DoRA (magnitude and direction) is in no plan of X_eTaL-ML: asked, after this repository has built and checked it | qlora, beyond-lora (Saga 3) | Saga 3 starts when those steps have landed at the pin; what is still missing then is written in `pe:` here and said so on the page. DoRA is written here first (`pe:`) |
| FM5 | not yet taken | feature | The matrix heat map (diverging colors about 0, a shared scale across maps side by side, each cell's value) is private to `demos/lora-explorer/web/src/app.rs`, so a second page cannot use it. Offered rather than asked: `shared/lesson`'s heat map (the same colors through `microscope::color`, with a legend, a cell inspector and keyboard focus) can move into `microscope` | every page here | written in `shared/lesson` (Saga 1 step 2), from the explorer's with a note of where it came from |
