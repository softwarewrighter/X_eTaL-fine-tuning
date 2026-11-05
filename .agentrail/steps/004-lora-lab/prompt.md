Step lora-lab (docs/plan.md, Saga 1 row 3: the research's first
milestone; docs/research.txt sections 3 to 6 and "My recommended first
milestone"; CLAUDE.md rule 8).

Build demos/lora-lab, live: one complete five-minute journey, "How
does a tiny adapter change a frozen neural network?", for a reader who
knows arithmetic and has never trained a network. Track "lora",
lesson 2. It follows lesson 1, demos/lora-hello, which has already
framed the question for a newcomer: build on it, do not repeat it.

The program, demos/lora-lab/lora-lab.xtl (short, with xetal doc
comments): the 8 x 8 layer of X_eTaL-ML's LoRA explorer (read
work/xetal-ml/demos/lora-explorer/ first; the same made-up W, A, B, x
by ripple, with a note of where they came from), the forward pass as a
Flow pipeline of Equations stages, flow:m_ath< for its LaTeX,
flow:s_tages< for the trace, lo:d_elta, lo:m_erge, lo:p_arams, the
frozen layer's output W x beside the adapted y. Runs at the command
line (`just run lora-lab`) with baselines.

The page, in order:
1. Intro: a pretrained model already knows things; fine-tuning changes
   its behavior; the key idea, a small learned correction beside
   frozen weights. Then a prediction question (what happens to the
   output if the adapter changes and W does not?).
2. The 8 x 8 example introduced: x, W, then A and B, each a heat map
   with its legend and inspector, each a glossary term.
3. The forward pass stepped (the stepper over the trace): for each
   stage the inputs read are highlighted, the operation shown, the
   outputs written highlighted, the shape stated, a plain-language
   sentence; next and previous both ways without rerunning.
4. Any output cell clicked: its arithmetic in symbols, then with the
   run's numbers.
5. The equation y = W x + (alpha / r) B A x with selectable symbols:
   selecting one highlights its matrix, its stage, and its place in
   the X_eTaL source (ask FL1's workaround, with its test).
6. The rank slider: the trainable count from lo:p_arams against 64,
   and the narrator's explanation from those numbers (rank 4 trains as
   many as full fine-tuning here, rank 8 twice as many; LoRA saves
   only where r is much smaller than the matrix).
7. One closing comparison: the frozen layer's output beside the
   adapted one's, and the difference, s B A x.
8. The source panel: all the code the page runs.
Beginner mode shows this as a guided sequence; Explorer all at once;
Advanced adds the expanded equations and the macro expansion.

Tests: the model natively (the trace's values against the arithmetic,
the merged layer equals the factored one, the counts), every visible
symbol and control has a glossary term, the browser check (browser.txt
lines that appear only once X_eTaL has run), baselines blessed after
review. Training is NOT in this step (step lora-training).

README (Demos), the demo's README, CHANGES.md, the plan; publish and
check-live.
