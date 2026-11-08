//! The page's prose, for new eyes (docs/new-eyes.md): a reader who
//! knows neither machine learning nor X_eTaL, and has read lessons 1
//! to 3. `[shown|id]` marks a word that opens its plain explanation;
//! `**words**` is bold. Every number a sentence quotes is put in by the
//! page from the run; none is typed here.

pub const LEDE: &str = "What does the rank buy, and what does it cost? The rank swept from 1 to 8 on the task of lesson 3, under two rules for the strength, every figure marked measured or analytical.";

pub const QUESTION: [&str; 2] = [
    "Lesson 2 showed what the [rank|rank] is (how many numbers the middle stage has) and what it costs (16 numbers for every 1 of rank, on this layer). Lesson 3 trained one adapter at one rank. **What does a larger rank buy, when the adapter is trained?** And a smaller question that comes with it: the [strength|strength] s is set from the rank, and there are two rules for doing that. **Does the rule matter?**",
    "Behind both is a newcomer's question: when I set up an adapter for my own model, what rank do I pick, and how would I know? This page cannot answer for a real model; it can show what the measurements look like, and what a count of numbers does and does not tell.",
];

pub const BACKGROUND: [&str; 3] = [
    "From lesson 3: the frozen [table|table] W has 8 rows and 8 columns, 64 numbers. The [examples|example] come from a second table T added to W, the hidden shift: the output wanted for an input is W + T times it, and the adapter never sees T. T has rank 4, so an adapter of rank 4 has room to match it exactly. Of 12 examples, 8 are trained on and 4 [held out|held-out]. The [loss|loss] per example on a set (the loss divided by how many examples the set has, 8 or 4) says how closely that set is fit: the lower, the closer. Here the same task is trained 8 times, at rank 1 to 8, each for the same number of training steps by [Adam|adam], from the same start: B at 0, A seeded by the rank.",
    "The strength. The [correction|correction] is s times B (A x), and s is set from the rank and a number chosen beforehand, [alpha|alpha]. LoRA's rule is s = alpha / r. A later paper (rsLoRA, rank-stabilized LoRA, Kalajdzievski 2023) proposed s = alpha / sqrt r instead, so that the strength shrinks more slowly as the rank grows: sqrt r is the square root of r, the number that times itself gives r, so sqrt 4 is 2 and sqrt 8 is about 2.83. Each rule is one function of the LoRA library, lo:s_caling and lo:s_calingRs. The sweep is run once under each.",
    "Two kinds of figure. A number that follows from the setup alone, like how many numbers an adapter of rank 5 has, is **analytical**: it would be the same whatever the examples were. A number that comes out of training, like the loss after 100 steps, is **measured**: it depends on the examples, the steps and the start. The page marks every figure one or the other.",
];

pub const RATIONALE: &str = "A rank has to be picked before training, and nothing in the number itself says what it will buy. Sweeping it on one task, with the cost beside the result and the result on examples held out, makes the trade visible: what a count predicts and what it does not. The two rules for the strength beside each other show, on the same task, what the rule changes and what it does not.";

pub const PURPOSE: [&str; 2] = [
    "It is one task, 16 training runs, and two sliders: alpha (1, 2, 4, 8 or 16) and the training steps each rank gets (10, 30, 100 or 300). Each move of a slider runs the whole sweep again, and the page's account of the table is written again from the new numbers. A third slider picks a rank to read about.",
    "The task is tiny and made by a formula, and the hidden shift has rank 4, so the sweep can show how rank, strength, cost and the two losses relate on it; it cannot say what rank suits a real model, where the shift to learn has no known rank. No sentence on this page ranks the ranks or the rules: the account compares numbers, and the choice is yours.",
];

pub const PREDICT: [&str; 3] = [
    "Before the table: the numbers trained rise with every step up in rank, 16 at a time. Under s = alpha / r, after the training steps set below, does the loss on the held-out examples fall from each rank to the next, too? Pick an answer, or Just show me; the page then says what the table shows.",
    "Yes, from each rank to the next",
    "{answer}",
];

pub const TABLE: [&str; 3] = [
    "The sweep. A row per rank. The numbers trained are analytical (16 times the rank on this layer: 8 for a row of A and 8 for a column of B). The strength under each rule is analytical (alpha / r, alpha / sqrt r). The two losses after the steps, and the size of the adapter's table s B A, are measured. The size is the square root of the sum of the table's squares, one number for how large a table is. The table s B A times an input gives the correction, so its size is how much correcting the adapter does; T itself, which it is to match, has a size of its own, shown under the table.",
    "Move alpha or the steps and the sweep runs again. The losses before any training, the same at every rank (B starts at 0), are shown for comparison.",
    "The charts are X_eTaL's drawings of the table's columns: the two losses under each rule, by rank, and the sizes.",
];

pub const READ: [&str; 2] = [
    "Pick a rank. The page says what changed from the rank before it, from the two rows' numbers.",
    "And the account of the whole table, from its numbers; it is written again whenever a slider moves.",
];

pub const SUMMARY: [&str; 4] = [
    "**What the rank buys.** More numbers trained, 16 per 1 of rank (analytical), and a closer fit of the examples trained on: on this task they are fit from a rank near the hidden shift's rank, 4. What it buys on examples held out is measured, and the account above says, for this alpha and these steps, at which rank that loss was lowest and whether it fell from each rank to the next.",
    "**What a count does not predict.** The numbers trained rise with every rank; the held-out loss need not, and it can even end above where it was before training. The account says whether either happened here.",
    "**What the rule for s changes.** Analytically, the strength: alpha / r shrinks it as 1 / r and alpha / sqrt r as 1 / sqrt r, so at rank 8 the two differ by a factor of sqrt 8, about 2.83. Measured, what that did to the losses and to the size of s B A after the steps set is in the account, and changes with the steps slider: that is the comparison to make.",
    "**What was not shown.** Any real model: there the shift to learn has no known rank, the examples have noise, and the held-out loss is the only guide. Other alphas and step counts than the sliders offer. Whether a rank or a rule is the right one for anything: the page compares, and does not choose. What a newcomer can take from it: the held-out loss is the figure to watch, and trying several ranks and reading it, as this page does on a tiny task, is how it is read.",
];

pub const NEXT: &str = "The next lesson, LoRA beside LoRA-FA, keeps A frozen and trains B alone, and shows why that update stays in one subspace; and it measures what no lesson has yet: what training does to the layer's outputs for its old inputs. It is being built; the catalog lists what is live.";

pub const CODE: [&str; 3] = [
    "This part is for readers who want to see the code. Nothing above depends on it. Lesson 1 says how to read X_eTaL as the page draws it.",
    "The sweep as the program writes it: a loop over the rank, each turn training the task twice, once under each rule, and adding a row to the table.",
    "The whole program, as it runs in your browser each time a slider moves.",
];

/// Every text of the page, for the list of ideas and the tests.
pub fn all() -> Vec<&'static str> {
    let mut out = vec![LEDE, RATIONALE, NEXT];
    for group in [&QUESTION[..], &BACKGROUND, &PURPOSE, &PREDICT, &TABLE, &READ, &SUMMARY, &CODE] {
        out.extend_from_slice(group);
    }
    out
}
