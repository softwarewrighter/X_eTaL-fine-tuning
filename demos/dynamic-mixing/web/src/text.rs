//! The page's prose, for new eyes (docs/new-eyes.md): a reader who
//! knows neither machine learning nor X_eTaL, and has read lessons D1
//! to D4 or the welcome page. `[shown|id]` marks a word that opens its
//! plain explanation; `**words**` is bold. Every number of the run a
//! sentence quotes is put in by the page from the run.

pub const LEDE: &str = "In the paper, the tables that mix the streams are not fixed: each input makes its own. How, step by step on two inputs, what the gating factor does, and the gradient through all of it, checked.";

pub const QUESTION: [&str; 2] = [
    "Every page of this track so far used [mixing tables|mixing-table] that are the same whatever the input: learned, but fixed once learned, as the Python re-implementation has them. The paper (Xie and others, \"mHC: Manifold-Constrained Hyper-Connections\") does something else: **each layer makes its three tables afresh from the stream state it is given**, so that two different inputs (two stream states a layer could be given) are mixed differently. This is [dynamic mixing|dynamic-mixing]. How is a table made from an input, and does the rule that keeps [mHC's|mhc] H_res [doubly stochastic|doubly-stochastic] survive it?",
    "The original [Hyper-Connections|hyper-connections] were dynamic too (the paper's equation 5, with a tanh, a squashing curve like the sigmoid but between -1 and 1, where mHC has a sigmoid); the Python version of mHC that the earlier pages followed (mHC-poc) simplified that away. This page follows the paper's equations 7 to 9, read at the source.",
];

pub const BACKGROUND: [&str; 4] = [
    "Four [streams|stream] of 8 numbers, a 4 x 8 table, as on the first page. The paper's recipe for one layer's tables, from the stream state X it is given: lay the four rows end to end into one [list|list] of 32; divide it by its root mean square, the square root of the average of its squares ([RMS normalization|rms-normalization]), so that its size no longer matters, only its shape; multiply the list by one learned table, the **projection**, 32 x 24, which gives 24 numbers: 4 for H_pre, 4 for H_post, 16 for H_res; multiply each group by its [gating factor|gating-factor] and add a learned [bias|bias] (the paper's fused form, its equation 14); then squash: H_pre by a [sigmoid|sigmoid] (each weight between 0 and 1), H_post by twice a sigmoid (between 0 and 2), and H_res by the [exponential|exponential] and twenty rounds of [Sinkhorn-Knopp|sinkhorn], the second page's procedure. The factor 2 and the sigmoid are the paper's; it does not say why 2.",
    "The page calls all three tables, though two of them are rows of four. The layer uses them as the paper's equation 3 does. H_pre here is a row of four weights: the **block**, the layer's own small calculation (as on the first page, two learned tables with a [GELU|gelu] bend between them), is given one list of 8, the four streams weighted by H_pre and added. The block's output is handed back to each stream weighted by H_post. And the state carried on is X mixed by H_res, plus the block's output in H_post's weights. (The first page's H_pre was a 4 x 4 table whose rows were averaged; the paper's is this row of four.)",
    "**The gating factor** decides how much the input matters. At 0 the input's part is multiplied away and the tables are the biases alone, the same for every input: the static tables of the earlier pages. The paper starts every gating factor at 0.01, so that a model begins nearly static and learns how much to let the input in. The biases' starting values the paper does not state; this page uses the Python's static start (0 for H_pre and H_post; for H_res, 1 on the diagonal and -1 elsewhere, written 2I - 1, as on the first page), so that at 0 this page's tables are the earlier pages'.",
    "Every number on this page is computed by [X_eTaL|xetal], in your browser as the page loads: two inputs, each sent through one layer, and the gradient of the layer checked. Nothing is trained on this page (the last part says why).",
];

pub const RATIONALE: &str = "The paper's formula fits in four lines and hides most of what happens in them: a list of 32 numbers becomes 24, then three tables, one of them through twenty rounds of division. Sending two inputs through and setting their tables side by side shows what dynamic means in numbers; the slider on the gating factor shows how far the tables move with the input, and where the twenty rounds stop being enough. The gradient is checked because the next thing anyone does with such a layer is train it, and a gradient worked out by hand has to be shown right before it is trusted.";

pub const PURPOSE: [&str; 2] = [
    "It is one layer of four streams, two inputs, a slider over the gating factor, every stage of the layer for the first input, and the gradient of a measure of the layer's output by ten of its numbers, worked out by hand and checked the slow way.",
    "It cannot show what training does with dynamic tables: the fourth page's training run, repeated with these tables, is not on this page, and the last part says why. Nor anything about a real model: two made-up inputs of 32 numbers are not tokens of text.",
];

/// Part 1: two inputs, their tables.
pub const TWO: [&str; 4] = [
    "Move the slider to set the gating factor, the same for all three tables: 0, 0.01 (the paper's start), 0.1, 0.3, 1 or 3. The program runs again at each setting and everything below is read from that run. The pictures round to two decimals; the prose quotes the run's numbers to six.",
    "Here are the two inputs, two 4 x 8 stream states a layer could be given, and the three tables each one makes at this gating factor. At the most, the two inputs' H_pre weights differ by {pre}, their H_post weights by {post} and their H_res numbers by {res}.",
    "At a gating factor of 0 the two inputs make exactly the same tables: the input's part is multiplied by 0, and what is left is the biases, the static tables of the pages before. H_pre's weights are the sigmoid of 0, one half each; H_post's twice that, 1 each; H_res is the second page's table, 0.711 on the diagonal and 0.096 elsewhere. The projection then has no effect at all, so the gradient by its numbers in part 3 is exactly 0, both ways, and their agreement checks nothing.",
    "The first input's H_res has row sums {rows} and column sums {cols}. {sinkhorn}",
];

/// Said of the sums, from the run.
pub const SUMS: [&str; 2] = [
    "For both inputs H_res comes out doubly stochastic to within a millionth (the check in part 3 looks at both): the twenty rounds reached it.",
    "For one of the two inputs, not every sum is within a millionth of 1 (the check in part 3 looks at both). At this gating factor the input moves H_res's logits far apart, and twenty rounds of Sinkhorn-Knopp are not enough to reach a doubly stochastic table from numbers that far apart: the second page found the same for a fixed start like it, so it is the size of the logits, not their coming from the input, that the rounds fall short on. The paper stops at twenty all the same; this page does not run more.",
];

/// Part 2: one input, every stage; `{}` marks filled from the run.
pub const STAGES: [&str; 9] = [
    "**Stage 1 of 9, the stream state.** The first input, four streams of 8 numbers.",
    "**Stage 2 of 9, flattened and normalized.** The four rows laid end to end, 32 numbers, divided by their root mean square (the square root of the mean of their squares), so that it becomes 1 whatever the input's size: {xn0}, {xn1}, ... .",
    "**Stage 3 of 9, projected.** The 32 numbers times the projection, a 32 x 24 table: 24 numbers, four for H_pre, four for H_post, sixteen for H_res. The first is {p0}.",
    "**Stage 4 of 9, H_pre.** Its four projected numbers times the gating factor {alpha}, plus its bias, through a sigmoid: {hpre}. They need not sum to 1.",
    "**Stage 5 of 9, H_post.** The same with twice a sigmoid: {hpost}.",
    "**Stage 6 of 9, H_res.** Its sixteen numbers times the gating factor, plus its bias (1 on the diagonal, -1 elsewhere), laid out as a 4 x 4, through the exponential and twenty rounds of Sinkhorn-Knopp: a table whose rows and columns sum to {rows0}.",
    "**Stage 7 of 9, the block's input.** The four streams weighted by H_pre and added, column by column: {u0}, {u1}, ... .",
    "**Stage 8 of 9, the block's output.** The block: 8 numbers to 16 by a table, the GELU bend, back to 8 by a second table: {f0}, {f1}, ... .",
    "**Stage 9 of 9, the new state.** X mixed by H_res, plus the block's output handed to each stream in H_post's weight.",
];

pub const STEP: &str = "The first input through the layer, a stage at a time. Press **Next** to move on, **Back** for the one before, **Start** for the first, or **Play** to watch them in turn. Every number is the run's.";

/// Part 3: the gradient, checked.
pub const GRADIENT: [&str; 3] = [
    "To train such a layer, a program has to know how a measure of its output changes with each of its numbers: the gradient. Here the measure is a fixed weighted sum of the new state's 32 numbers (a stand-in for the [loss|loss] training would use), and the gradient is worked out by hand backward through every stage: the new state, the block, H_pre's weighted sum, the sigmoids, the twenty Sinkhorn rounds and the exponential, the gating factors and biases, the projection, the normalization, and back to the input. Then the program checks ten of its numbers the slow way, by [central differences|central-differences]: nudging each number up and down by a millionth and measuring twice.",
    "The table quotes both for the ten numbers, from this run; the Check library's report, as X_eTaL printed it: {summary}.",
    "The check covers these ten numbers at this gating factor and this input; it says the gradient worked out by hand is right there, not everywhere. How to read one row: the gradient by H_post's gating factor says how much the measure would change, per unit, if that factor were raised a little; a training step would move the factor against it.",
];

/// Part 4: what is not here.
pub const WAITS: [&str; 2] = [
    "The fourth page trained the plain skip, HC and mHC with fixed tables, on 16 inputs at a time. Repeating that with dynamic tables means three tables for each of the 16 inputs at every layer, so the gradient the fourth page wrote out by hand, already about forty lines, has to be worked through sixteen sets of tables per layer at once, the normalization and the projection added; and each run would cost more, by an amount not measured here. That is past what reads well as a page's program.",
    "So this page stops at one layer, checked. The layer with its gradient is asked of X_eTaL-ML, the library the lessons draw on (a Hyper-Connections layer, static or dynamic, with its way back); when it is there, the training at depth with dynamic tables becomes a page of its own, using it.",
];

pub const SUMMARY: [&str; 4] = [
    "**How a dynamic table is made.** The stream state flattened and normalized, one projection, a gating factor and a bias for each table, then a sigmoid for H_pre, twice a sigmoid for H_post, and Sinkhorn-Knopp of the exponential for H_res, the paper's equations 7 to 9.",
    "**What the gating factor does.** At 0 the tables are static, the same for every input; at the paper's starting 0.01 two different inputs' tables differ in the second or third decimal; larger factors let the input move them further.",
    "**Whether the rule survives.** H_res stayed doubly stochastic within a millionth for both inputs at gating factors up to 0.3; at 1 and 3, for one of these two inputs, twenty rounds of Sinkhorn-Knopp fell short, because the logits were far apart, as the second page found with fixed ones.",
    "**What was not shown.** Training with dynamic tables, which waits on the library; the paper's models and its measured results for its dynamic mixing.",
];

pub const NEXT: &str = "The last page of this track sets the plain skip, HC and mHC side by side, the toy's measurements beside the Python's and the paper's, and the X_eTaL programs beside the Python's functions. It is being built; the catalog lists what is live.";

pub const CODE: [&str; 2] = [
    "This part is for readers who want to see the code. Nothing above depends on it. Lesson 1 says how to read X_eTaL as the page draws it. The layer is one lambda (a function written in place, between braces) mapped over a list of jobs: the two inputs, then the first input's numbers nudged up and down for the check, since a lambda can be applied only where it is written. The layer and its gradient are written out because the library does not have them yet (ask FM13 of X_eTaL-ML).",
    "The whole program, as it runs in your browser at the gating factor chosen.",
];

/// Every text of the page, for the list of ideas and the tests.
pub fn all() -> Vec<&'static str> {
    let mut out = vec![LEDE, RATIONALE, NEXT, STEP];
    for group in [&QUESTION[..], &BACKGROUND, &PURPOSE, &TWO, &SUMS, &STAGES, &GRADIENT, &WAITS, &SUMMARY, &CODE] {
        out.extend_from_slice(group);
    }
    out
}
