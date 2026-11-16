Step hello-framing (inserted 2026-11-05 on the user's review of the
first live page: "lacks intro, context, purpose, rationale, and
summary. It sort of jumps into the subject. A newcomer who asked how
do I teach my local LLM new things (without catastrophic forgetting or
excessive time/electricity costs) is instantly lost").

1. demos/lora-hello's page, reframed for that newcomer, in this order:
   the question in their words; the background they need (a model is
   numbers in matrices; a layer multiplies; fine-tuning changes the
   numbers; why changing all of them is costly and can overwrite what
   the model knew); the idea (freeze the weights, train a small
   correction beside them); what this page is and is not (one layer
   shrunk to 4 x 4, an adapter with made-up numbers, mechanics before
   training); the experiment with things to try; the same count on a
   layer of a real local model, computed by X_eTaL; a summary of what
   it showed against the question's three worries (forgetting, time,
   electricity), said honestly (what a count does not measure, what
   the frozen base does and does not guarantee); and where to go next.
   Every number still comes from the program.
2. The rule, so no later page repeats this: CLAUDE.md (the teaching
   rules) and docs/plan.md (the principles) require every page and
   every demo README to open with the question, the context, the
   purpose and the rationale, and to close with a summary and what is
   next; demos/_template's README carries those headings; a check in
   the gate that each live page's browser.txt names its framing
   sections.
3. The demo's README and demo.toml to match; tests, baselines
   (reblessed after review), README, CHANGES.md; publish, check-live.
