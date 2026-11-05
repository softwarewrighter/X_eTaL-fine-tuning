# Asks for X_eTaL-extensions

Native code this repository would need, asked of
[X_eTaL-extensions](https://github.com/softwarewrighter/X_eTaL-extensions),
where Rust-based extensions of the language live
(`../X_eTaL-extensions`, read only from here). When something cannot
be written in X_eTaL with the libraries there are, a new extension is
asked for here in preference to a change of the base language (the
user, 2026-11-05); a language change is the last resort, in
`docs/xetal-asks.md`, which does not exist because none is needed.

What is not asked for here: the machine learning itself. Every
algorithm of a lesson is X_eTaL, from the libraries of X_eTaL-ML and
X_eTaL-libraries; functions they lack are asked for in
[`xetal-ML-asks.md`](xetal-ML-asks.md) and
[`xetal-libraries-asks.md`](xetal-libraries-asks.md). Rust in this
repository is the pages' presentation only (the WebAssembly host that
runs X_eTaL in the browser and lays out what it prints), never an
algorithm.

Each entry: status, kind, the ask, which demos need it, and the
workaround in use. Status: **not yet taken**, **planned** or **in
progress**, **landed**, **declined**, **blocker** (no workaround: the
step waits, and the user is told).

There is no ask yet.

| # | Status | Kind | Ask | Demos | Workaround |
| - | ------ | ---- | --- | ----- | ---------- |
