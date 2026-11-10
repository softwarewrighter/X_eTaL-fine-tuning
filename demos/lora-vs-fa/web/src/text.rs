//! The page's prose, for new eyes (docs/new-eyes.md): a reader who
//! knows neither machine learning nor X_eTaL, and has read lessons 1
//! to 4. `[shown|id]` marks a word that opens its plain explanation;
//! `**words**` is bold. Every number a sentence quotes is put in by the
//! page from the run; none is typed here.

pub const LEDE: &str = "Freeze half the adapter and train the other half: what does it cost? LoRA beside LoRA-FA in two experiments kept apart, fitting a known shift and learning from examples, with the first measure of what training does to what the layer did before.";

pub const QUESTION: [&str; 2] = [
    "An [adapter|adapter] has two small tables, A and B, and lessons 3 and 4 trained both. A variation called [LoRA-FA|lora-fa] keeps A as it was seeded, [frozen|frozen], and trains B alone: half the numbers to train, half the memory [Adam|adam] needs for them, and a checkpoint that need not store A at all, since a seed regrows it. **What does freezing A cost?** Is there something B alone cannot reach, and if so, what?",
    "And the question behind this whole course, asked at the start: when I teach my model something new, what happens to what it knew? Nothing so far has measured that. **This page measures it**, on the tiny layer. Some old inputs are kept, with the outputs the layer gave them before any training. After the layer has learned the new examples, its loss on those old examples says how far it has moved from what it did. That is measured for LoRA, for LoRA-FA, and for [full fine-tuning|full-fine-tuning] of the layer itself.",
];

pub const BACKGROUND: [&str; 4] = [
    "From the lessons before: the frozen [table|table] W, 8 by 8; the [correction|correction] s B (A x), where A is r by 8 and B is 8 by r for a [rank|rank] r; the [strength|strength] s = alpha / r with [alpha|alpha] 4; B starts at 0 and A is seeded, so both methods start from the same adapter. [Training|training] by [Adam|adam] moves the numbers against the [gradient|gradient] of the [loss|loss], here half the sum of squared errors. LoRA moves A and B (the LoRA library's function lo:a_dam); LoRA-FA moves B alone (its lo:a_damFA), and its A stays exactly as seeded. One more word used throughout: the **size** of a table or a list is the square root of the sum of its squares, one number for how large it is.",
    "Two experiments, named apart as lesson 3 asked. **Experiment A, approximating a known matrix**: the adapter is to come to a known shift T directly. T is 8 by 8 and made as an 8 by 4 table times a 4 by 8 one, so it has rank 4 in the sense of lesson 2: only 4 of its rows are [independent|independent], and an adapter of rank 4 has room to match it exactly. The inputs given to the layer are the 8 columns of the identity table, one 1 and seven 0s each; the layer's output for such an input is one column of its own table, so fitting those 8 is fitting T itself, and the loss is half the squared size of s B A - T, a total over the 8, not per example. Nothing is held out: this is linear algebra, and it has a known answer. **Experiment B, learning from examples**: the task of lesson 3, 12 examples made by the hidden shift W + T, the first n trained on and the last 4 [held out|held-out]. That is machine learning, and it has no known answer.",
    "Two words for Experiment A. A table can be split into rank-1 pieces, largest first, and its [singular values|singular] (together, its spectrum) are the sizes of those pieces; T has four that are not 0, then zeros. The lowest loss any update of rank k can reach against T is set by them: half the sum of the squared singular values past the k-th (the Eckart-Young theorem, 1936), so it is **analytical**. And a [subspace|subspace]: the rows of A, r lists of 8 numbers, together with every mix of them (each scaled and added) make a flat region inside the space of all lists of 8, called their row space; A's rows are a [basis|basis] of it (a seeded A's rows are independent). Every row of s B A is a mix of A's rows, so the update lies in A's row space; when A is frozen, that subspace is fixed before training starts. A [projection|projection] onto it takes a list to the nearest point of the subspace; what is left over is the remainder. A table is projected row by row, and its remainder's size is one number for all its rows together.",
    "One word for Experiment B. The old examples: 4 inputs, not among the 12, with the outputs the layer gave them before any training, W times each; the held-out examples, by contrast, want W + T's outputs, like the ones trained on. The loss on the old examples after training is the [retention|retention] measure: how far the trained layer has moved from what it did, 0 before training for every method, and the larger, the less was kept. When fewer than 8 examples are trained on, the ones between are set aside, used by neither. The three methods are compared on the same examples, from the same W, for the same number of steps.",
];

pub const RATIONALE: &str = "LoRA-FA's appeal is what it saves, and the saving is analytical: half the numbers, half the optimizer's memory. What it gives up cannot be seen in a count; it shows when both methods try for the same thing from the same start. Fitting a known T gives a floor each method can be measured against. Learning from examples shows whether that floor matters when the target is not known. And adding full fine-tuning of W, every number of the layer moved, puts the adapters beside the thing they stand in for, on the measure a newcomer asked about first: what is lost.";

pub const PURPOSE: [&str; 2] = [
    "It is one program with two sliders, the rank (1 to 8) for both experiments and the examples trained on (2 to 8) for Experiment B. Each move runs everything again: two fits, three training runs, the singular values, the projections. Every sentence that quotes a number is written from the run.",
    "The page compares and marks every figure measured or analytical; it does not say which method to use. The layer is 8 by 8 and the shift to learn is made by a formula, so what the experiments show is the mechanism, the row space and the retention measure as they behave here, not what either would be in a model with millions of rows.",
];

pub const PREDICT_A: [&str; 3] = [
    "Before Experiment A runs: both methods start from the same seeded A and B at 0, at the rank set below, and fit T for the same steps. Does LoRA-FA come to the same loss as LoRA? Pick an answer, or Just show me; the page then says what the runs show.",
    "Yes, the same loss, to 3 decimals",
    "{answer}",
];

pub const FIT: [&str; 3] = [
    "The chart is X_eTaL's drawing of the two fits, a record every 10 steps: LoRA, LoRA-FA, the lowest loss possible at this rank (analytical, a flat line) and LoRA-FA's floor (analytical, a flat line). Under it, the three tables: T, what LoRA learned and what LoRA-FA learned, on one color scale.",
    "T's singular values and the lowest loss possible at each rank, analytical. The program works the singular values out itself, by repeating a step until it settles; to show that this gives the right numbers, the same lines are run on a second table whose singular values were chosen beforehand, and the largest error is shown.",
    "What the fits say, from their numbers (the account below is written again at each move).",
];

pub const PREDICT_B: [&str; 3] = [
    "Before Experiment B runs: the same two adapters and full fine-tuning of W learn from the n examples trained on. Afterward, which has moved the layer's outputs for the old examples least? Pick an answer, or Just show me; the page then reads the column.",
    "LoRA",
    "{answer}",
];

pub const LEARN: [&str; 3] = [
    "The table: a method a row; the numbers trained (marked A, analytical), the three losses per example before training and after the steps (marked M, measured), and the size of each method's change to the layer. The old examples' column after training is the retention measure: the larger, the less was kept. The charts are X_eTaL's, one per loss, a record every 10 steps.",
    "Move the examples slider: the three columns say what training on fewer examples did, for each method.",
    "What the runs say, from their numbers (the account below is written again at each move).",
];

pub const SUMMARY: [&str; 4] = [
    "**What freezing A costs, in Experiment A.** LoRA-FA's update cannot leave the frozen A's row space, so the closest it can come to T is T's projection onto that subspace: an analytical floor, which the measured run comes to and does not pass. LoRA, moving A, is not held to that subspace; the account says whether it came to the lowest loss possible for the rank (Eckart-Young) at the rank set, and at rank 4 and up that lowest is 0 on this T. The remainders make the geometry visible: LoRA-FA's is 0 to rounding at every rank; LoRA's is not, at a rank under 8 (at rank 8 the row space is the whole space).",
    "**What it costs in Experiment B** is measured, not derived: the account says, for this rank and these examples, where each method's three losses ended. The frozen subspace is a seeded one, unrelated to the examples, so what it holds of the shift is a matter of chance; a real LoRA-FA may seed A differently or take it from a trained adapter.",
    "**Retention.** On this task every method moved the old outputs: the account says by how much, and which moved them least. Nothing in any of the three methods kept the change away from the old inputs; the examples ask for W + T, and whatever each learned of that, the old inputs met it too. An adapter stores and trains a change cheaply; here it was not, by itself, a guard of old behavior. That is one task's number, not a rule.",
    "**What was not shown.** Any real model, any noise in the examples, other seeds for A, other learning rates; whether LoRA-FA's saving is worth its floor anywhere in particular. A task whose new examples lie in a region of their own, where an update could leave the old region alone: that is the subject of the shared-subspaces lessons to come.",
];

pub const NEXT: &str = "This closes the first group of lessons: an adapter, where its numbers come from, what the rank buys and what a frozen half costs. The next group compares the variations of LoRA on one task, each at the same rank and then at the same cost, with retention measured for each. The row space met here is the first fixed subspace; the lessons on shared subspaces take the idea further, a basis fixed once and reused across tasks. The catalog lists what is live.";

pub const CODE: [&str; 3] = [
    "This part is for readers who want to see the code. Nothing above depends on it. Lesson 1 says how to read X_eTaL as the page draws it.",
    "Three things are written out in the program because no library has them yet, each named in the asks: the spectrum by power iteration with deflation, as X_eTaL-ML's LoRA explorer writes it (ask FL2); the projection onto A's row space through Matrix's inverse (ask FL3); and full fine-tuning of W, Adam on the gradient of the same loss (ask FM7).",
    "The whole program, as it runs in your browser each time a slider moves.",
];

/// Every text of the page, for the list of ideas and the tests.
pub fn all() -> Vec<&'static str> {
    let mut out = vec![LEDE, RATIONALE, NEXT];
    for group in [&QUESTION[..], &BACKGROUND, &PURPOSE, &PREDICT_A, &FIT, &PREDICT_B, &LEARN, &SUMMARY, &CODE] {
        out.extend_from_slice(group);
    }
    out
}
