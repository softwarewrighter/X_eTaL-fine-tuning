//! The page's prose, for new eyes (docs/new-eyes.md): a reader who
//! knows neither machine learning nor X_eTaL, and has read lessons D1
//! to D3 or the welcome page. `[shown|id]` marks a word that opens its
//! plain explanation; `**words**` is bold. Every number of a run a
//! sentence quotes is put in by the page from the run or the records;
//! the Python's and the paper's are said to be theirs.

pub const LEDE: &str = "Does the bound on the gain matter once a model is trained? The plain skip, HC and mHC trained on one tiny task at 12, 24 and 48 layers: a short run computed in your browser, the long runs computed offline by the same program and replayed.";

pub const QUESTION: [&str; 2] = [
    "The third page showed that [HC's|hyper-connections] mixing table, repeated over 48 layers, multiplies the carried state's [gain|gain] to about 10 to the power 28, while [mHC's|mhc] keeps it at 1. That was arithmetic on the tables alone. The question here is the one a person training a model asks: **does it matter once the model is trained?** Does a deep stack with each kind of connection still learn, and does anything break on the way?",
    "The paper (Xie and others) reports that in its large models HC's training loss jumped and its gradients swung near one point of training while mHC's ran smoothly; the Python re-implementation this track follows reports that at 48 layers its HC model did not learn at all and its mHC model did. This page trains the three kinds of connection on a task small enough to compute here, and reports what happened, including where it differs from both.",
];

pub const BACKGROUND: [&str; 5] = [
    "**Training** is the loop of the earlier lessons: show the model examples, measure how wrong it is (the [loss|loss]), work out which way each weight should move to make it less wrong (the [gradient|gradient]), move every weight a little that way, and repeat. Each repetition is a **step**. The [size of the gradient|gradient-norm] says how hard the step pushes: very large or wildly changing sizes are a sign of trouble.",
    "**The task** keeps the idea of the Python's, predicting the next token, made tiny: 16 tokens (a token is one item of a vocabulary; think of them as the numbers 1 to 16), and for each the model must give the next one (16 is followed by 1). A token is turned into a [list|list] of 8 numbers by an [embedding|embedding], a table of 16 rows learned like any other weights; the list goes through the layers; the streams are averaged and scaled to a size of 1 (as the Python's model does with its own final scaling); and a second learned table, the unembedding, turns the 8 numbers into 16 scores, one per possible next token, which a [softmax|softmax] makes into chances. The loss is the [cross-entropy|cross-entropy]: small when the right token gets a high chance. Guessing evenly among 16 gives a loss of 2.77; a perfect model, 0.",
    "**The three kinds of connection**, each a pair of settings, otherwise the same program, the same start and the same 16 examples at every step: the **plain skip**, one stream, each layer adding its block's output to the list (lesson D1's part 1); **HC**, four streams, the carried state mixed by the exponential of its [logits|logits]; **mHC**, the same mixed by that table made [doubly stochastic|doubly-stochastic] by twenty rounds of [Sinkhorn-Knopp|sinkhorn]. Each layer is lesson D1's: a block 8 -> 16 -> 8 with [GELU|gelu], and the three mixing tables. Every weight is trained, the mixing tables' logits too, with [Adam|adam]. All three start from the same weights; they pass through different tables, so even their first losses differ. For the plain skip the three tables are each the number 1, so its gains are 1 by construction and its mixing logits have a gradient of 0.",
    "The gradient is worked out by hand: the program goes back through every layer, the twenty Sinkhorn rounds included, and works out how the loss changes with every weight. A mistake there would make every result on this page meaningless, so the program also checks it the slow way, by [central differences|central-differences], on nine weights, and prints the result.",
    "Every number on this page, apart from the figures quoted from the paper and the Python, is computed by [X_eTaL|xetal]: the short run in your browser as the page loads; the long runs by the same program on a computer beforehand, an [offline run|offline-run] whose record is shipped with the page and [replayed|replay]. Each chart says which it is from.",
];

pub const RATIONALE: &str = "The bound mHC keeps is arithmetic, and the third page showed it without training. Whether it changes what a model can learn is a measured question, and it needs a model to be trained at several depths with each kind of connection, everything else held the same. The task is tiny so that the whole training, every layer's gradient included, is the program on this page; the long runs are too long for a browser (about four minutes each at 48 layers), so they are run once, recorded, and replayed, and a short run computed here can be set beside the start of its record.";

pub const PURPOSE: [&str; 2] = [
    "It is one tiny task, three kinds of connection and three depths, 800 steps each, from one start: nine recorded runs, and one short run of 50 steps at 12 layers computed in your browser for the kind you pick. Shown for every step: the loss, the gradient size, and the two gain measures of the third page for the product of the layers' H_res tables as training changes them.",
    "It can show whether each kind of connection learns this task at each depth, and whether its training ran into numbers too large to hold. The mixing tables here are fixed per layer and learned, as the Python's are; the paper's depend on each token, which is the next page. It cannot show anything about a real model: there is no sequence and no attention (each token is predicted from itself alone), the model has a few thousand weights where the paper's have billions, and one task and one start are a single observation, not a rule.",
];

/// Part 1: the run in your browser.
pub const LIVE: [&str; 4] = [
    "Pick the kind of connection. The page trains it for 50 steps at 12 layers, here and now, and reads everything below from that run. Each button is a file of settings, not code.",
    "The chart is X_eTaL's drawing of the loss and the gradient size at each step, in powers of ten (a point at -1 is a tenth). After step 1 the loss is {loss1}; after step {steps}, {lossN}. The gradient size went from {grad1} to {gradN}. The product of the layers' H_res tables has forward gain {gainN} at the end (the third page's measure), and {nans} of the {steps} steps had a [NaN event|nan-event].",
    "For nine weights, the program works out the gradient two ways: by the backward pass it uses to train, and by [central differences|central-differences], nudging the weight up and down by a millionth and measuring the loss each time. The table quotes both. They are compared at a point a little away from the start: with four streams, several of these gradients are 0 at the start itself, because the four streams begin identical, and a 0 against a 0 checks nothing. With the plain skip the three mixing rows are 0 at any point, since its tables are the number 1. The check covers these nine weights at this depth, not every weight.",
    "The report of the Check library, as X_eTaL printed it: {summary}.",
];

/// Part 2: the recorded runs.
pub const RECORDED: [&str; 5] = [
    "Pick a depth. The three charts and the table below replay the three recorded runs at that depth, 800 steps each, by a second short program that reads the records and draws them. These runs were computed offline by the program of part 1, with only the depth, the steps and the kind of connection changed; the recorded run of the kind you picked above at 12 layers begins with the steps part 1 computed: its loss after step {steps} is {recN}, and part 1's is {liveN} (the page's tests compare every number of the first {steps} steps, for all three kinds, and find them equal).",
    "At this depth, after 800 steps, the loss is {b} with the plain skip, {h} with HC and {m} with mHC. Guessing evenly would give 2.77.",
    "**The plain skip did not learn at this depth**: its loss after 800 steps, {b}, is the loss of guessing evenly, and and its gradient size at the end is {bg}, very nearly 0. The gradient chart shows the way there: large swings early, then a fall by many powers of ten. HC and mHC learned. This is the opposite of what the gain alone would suggest, since the plain skip's gain is 1 at every depth; this page does not show why it failed.",
    "All three learned at this depth.",
    "HC's composite forward gain was {hgain} after 800 steps, close to what the third page worked out for this depth before any training, and HC learned all the same. mHC's stayed at {mgain}. No step of any run at this depth had a NaN event: the counts are {nans}.",
];

/// Part 3: beside the Python and the paper.
pub const BESIDE: [&str; 3] = [
    "The Python's run measured its final losses at 48 layers, on its own task (256 tokens, sequences of 128, attention in every block, a model 32 times as wide): 3.79 with its plain skip, 5.54 with HC and 0.0002 with mHC. Guessing evenly among 256 tokens gives 5.55, so its HC model did not learn and its plain skip learned a little. The table sets them beside this page's at 48 layers, each marked.",
    "**What the toy reproduces**: at 48 layers mHC learned, as the Python's did, and its gains stayed at 1 through training. **What it does not**: here HC learned at 48 layers, where the Python's HC did not; and here the plain skip did not learn at all, where the Python's learned a little. This page does not show why. The two set-ups differ in ways that could matter, read from the Python's code (linked at the top): its numbers are 32-bit, which hold numbers up to about 10 to the power 38, where X_eTaL's are 64-bit, up to about 10 to the power 308; its blocks have their own skip and their own scaling inside; its task has sequences and needs attention. Which of these, if any, accounts for the differences would take more runs than this page has.",
    "The paper's figures are of another kind again, from models with billions of weights and real text: a jump in HC's loss and gradient near one point of training, and smooth curves for mHC. Nothing of that scale is computed here.",
];

pub const SUMMARY: [&str; 4] = [
    "**The answer, for this task.** At 12 and 24 layers the three kinds of connection all learned (pick a depth above to see each). At 48 layers the plain skip did not learn at all, while HC and mHC did; no step of any run had a NaN event.",
    "**The bound, in training.** mHC's gains stayed at 1 through every run; HC's stayed near what the third page worked out before training, about 10 to the power 7, 14 and 28 at 12, 24 and 48 layers, and on this task that did not stop it learning.",
    "**Beside the Python.** The toy reproduces the Python's mHC learning at 48 layers; it does not reproduce its HC failing, and its plain skip failed outright where the Python's learned a little. The page names differences that could account for this and does not claim which.",
    "**What was not shown.** Anything about attention, sequences or real text; a model of any size; more than one start and one task; the paper's own mixing, which depends on the input (the next page).",
];

pub const NEXT: &str = "The next page of this track, dynamic mixing, makes the mixing tables depend on the token itself, as the paper's do, and trains the same three ways again. It is being built; the catalog lists what is live.";

pub const CODE: [&str; 3] = [
    "This part is for readers who want to see the code. Nothing above depends on it. Lesson 1 says how to read X_eTaL as the page draws it. The training program is long because the backward pass is written out by hand, layer by layer, the Sinkhorn rounds included: the machine-learning library does not yet have a Hyper-Connections layer with its gradient. A lambda (a function written in place, between braces) can be applied only where it is written, so the training run and the gradient check are one lambda applied to a list of jobs.",
    "The training program, as it runs in your browser for the kind of connection picked: the file has S := 4 and sink := 1.0 (mHC), and the page puts the picked kind's two settings in their place, as the trainer below does with the depth and the steps too.",
    "The replay program, as it runs on the records for the depth picked; and the offline trainer that made the records, a short shell script that runs the training program nine times with its settings changed.",
];

/// Every text of the page, for the list of ideas and the tests.
pub fn all() -> Vec<&'static str> {
    let mut out = vec![LEDE, RATIONALE, NEXT];
    for group in [&QUESTION[..], &BACKGROUND, &PURPOSE, &LIVE, &RECORDED, &BESIDE, &SUMMARY, &CODE] {
        out.extend_from_slice(group);
    }
    out
}
