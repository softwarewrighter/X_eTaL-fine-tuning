//! The page's prose, for new eyes (docs/new-eyes.md): a reader who
//! knows neither machine learning nor X_eTaL, and has read lessons 1
//! and 2. `[shown|id]` marks a word that opens its plain explanation;
//! `**words**` is bold. Every number a sentence quotes is put in by the
//! page from the run; none is typed here.

pub const LEDE: &str = "Where do the numbers of an adapter come from? One step of training, told stage by stage with every number quoted from the run; then training with a record you can step back through.";

pub const QUESTION: [&str; 2] = [
    "Lessons 1 and 2 used an [adapter|adapter] whose numbers were made up, and said that in a real adapter they would be trained. This lesson is about that: **where the numbers of A and B come from.**",
    "It also meets the question the lessons began with. If a model can learn something new this way, how does anyone know it learned it, rather than memorized the few examples it was shown? The answer is to keep some examples back and measure on those.",
];

pub const BACKGROUND: [&str; 3] = [
    "From the lessons before: a [layer|layer] takes a [list|list] of 8 numbers, x, and gives back a list of 8, y. Its own [table|table] W is [frozen|frozen]. Beside it, the adapter's two thin tables A and B add a [correction|correction]: y = W x + s B (A x), with s [the strength|strength], which is [alpha|alpha] divided by the rank; alpha is 4 on this page.",
    "[Training|training] is practice with correction. The layer is given an [example|example]: an input and the output wanted for it. It predicts an output; each number of the prediction less the number wanted is **the error**; one number made from the errors, [the loss|loss], says how wrong the prediction was in all. [The gradient|gradient] says which way to move each number of A and B, and how hard, to lower the loss, and **the update** moves them a little that way. Then the next example, and so on.",
    "On this page the examples come from a second layer, W + T, where T is a shift of W that the adapter never sees: the output wanted for an input is W + T times it. The adapter is to learn, from examples alone, to give the outputs that W + T gives. 12 examples are made; the first few are trained on, the last 4 are [held out|held-out]: never trained on, only measured; any between are used by neither.",
];

pub const RATIONALE: &str = "Training is usually shown as a curve going down, and it is easy to nod at a curve. Here one step of it is taken apart into stages, each a sum of products like those of lesson 2, and every number of every stage is in view, so that \"the gradient\" and \"the update\" are things you can check and not words. The curve comes after, and the record behind it is there to step through.";

pub const PURPOSE: [&str; 2] = [
    "It is one layer, 12 examples and an adapter that starts as LoRA starts: B all zeros, so that the correction is zero and the layer begins by giving exactly what W gives. You can step through one step of training, then watch training run and step back through its record, and try three experiments set up as data.",
    "The layer is tiny and the examples are made by a formula, so the page can show how training works and what a held-out loss is for; it cannot show how long or how well a real model trains. The numbers here are not those of any model.",
];

pub const TASK: [&str; 3] = [
    "Here is everything training has. The frozen table W. The shift T, which the outputs wanted are made with and which is otherwise hidden: the adapter never reads it, and training never changes it. You are shown it so that you can set the adapter's table beside it at the end.",
    "The 12 inputs are the columns of one table, each column one example's input; the outputs wanted are the columns of another, each the shifted layer's answer for the input above it. The first ones, on the left, are trained on; the last 4, on the right, are held out.",
    "And the adapter at the start: A has small numbers from a fixed formula of the LoRA library's (sines, scaled down; the same every time, nothing random), B is all zeros. So B times anything is zero, the correction is zero, and before any training the layer gives exactly what W gives.",
];

pub const STEP: [&str; 2] = [
    "One step of plain gradient descent, the simplest kind of step, in six stages: the prediction p, the error g, the error carried back through B, b, the gradients dA and dB, the update A2 and B2, and the frozen table given back, W2. Press **Next** to move on a stage, **Back** for the one before, **Start** for the first, or **Play** to watch them in turn. The circles name what each stage works out, by the letters the stages use below. The squares worked out in a stage have a solid outline. Click any square to read its number in full.",
    "The step is on the examples trained on, all at once: each stage's table has one column per example.",
];

/// The six stages, as the page says them; `{}` marks are filled from the run.
pub const STAGES: [&str; 6] = [
    "**Stage 1 of 6, the prediction.** The layer predicts an output for every example trained on: W times its input, plus the correction. The prediction is called p here, because y is the output wanted. B is all zeros, so the correction is zero and each prediction is W's own output, as it was before there was an adapter. Beside it, the outputs wanted.",
    "**Stage 2 of 6, the error.** Each prediction less the output wanted, number by number: g. The [loss|loss] is half the sum of their squares, over all the examples trained on; divided by how many there are, so that it can be set beside the loss on the 4 held out, it is {before} per example.",
    "**Stage 3 of 6, the gradient at the output.** The [gradient|gradient] of the loss, which way to move each output to lower it, is the error g itself: that is why the loss is defined with the half and the squares. Carried back through B it gives b = B times g, read column by column; B is all zeros, so b is all zeros.",
    "**Stage 4 of 6, the gradients of A and B.** A's gradient, dA, is built from b, which is zero, so dA is zero: on the first step A does not move at all. B's gradient, dB, is built from the error g and the squeeze h; it is not zero. Each is a sum over the examples, the same shape as the table it belongs to.",
    "**Stage 5 of 6, the update.** A and B move against their gradients by [the learning rate|learning-rate], {lr}: A2 = A minus the learning rate times dA (so A2 is A, since dA is zero), B2 = B minus the learning rate times dB. After this one step the loss per example on the examples trained on is {after}, from {before}.",
    "**Stage 6 of 6, W unchanged.** The step gives the layer back: W2 is W, and A2, B2 and s with it. The difference between W2 and W, taken number by number, is {wmoved}. The same step by the LoRA library's own function (lo:s_gd) gives A and B at most {off} from these, and the same W.",
];

pub const TRAIN: [&str; 3] = [
    "Now many steps. They are taken by [Adam|adam], which uses the same gradients as the step above but decides how far to move from running averages of them rather than from the last one alone. Every few steps the [record|record] takes down the loss per example on the examples trained on, the loss on the held-out ones, and A and B as they are then. The chart is X_eTaL's drawing of the two losses, a record a point.",
    "Pick an experiment by its button. Each is a file of settings, not code: the rank, the learning rate, how many examples are trained on. The page trains again from the start with those settings and says what the record shows.",
    "Move the record slider (with the mouse, or the arrow keys once it is focused) to see A, B and the adapter's table s B A as they were at that point, beside the shift T. Moving back shows the record; nothing is trained again.",
];

/// What the record shows, said from its numbers (app::reading fills
/// the marks); a judgment is the reader's, so none of these calls a
/// setting better or worse.
pub const READING: [&str; 4] = [
    "The trained-on loss did not fall steadily: between records {k0} and {k1} (steps {s0} and {s1}) it rose from {a} to {b}, the largest rise of the run.",
    "The trained-on loss had small rises between records, all under a twentieth of where it started.",
    "The trained-on loss fell at every record.",
    "The held-out loss ended higher than it started: it was lowest, {low}, at record {k} (step {s}), and highest, {high}, at record {kh} (step {sh}).",
];

pub const SUMMARY: [&str; 4] = [
    "**Where the numbers come from.** From the examples, a step at a time: predict, measure the error, carry it back to get each table's gradient, move each table a little against its gradient. The frozen table is not in the update, and its difference before and after is exactly zero.",
    "**What the first step showed.** B starts at zero, so A's gradient is zero and only B moves on the first step. From the second step on, both move.",
    "**What the held-out loss is for.** The loss on the examples trained on says how closely they were fit; only the loss on examples never trained on can tell learning from memorizing ([overfitting|overfitting]). The three experiments are set up to differ in this; what each record shows is said under its chart, from its numbers.",
    "**What was not shown.** Why the loss is defined as it is, beyond that its gradient is the error; what happens on a real model, with millions of examples and noise in them; whether these settings are good ones in general; and what training does to the layer's outputs for its old inputs, which no lesson has measured yet. Each experiment here says what its own record shows, no more.",
];

pub const NEXT: &str = "The next lesson, Rank and scaling, trains the same task at every rank from 1 to 8, and with two ways of setting the strength, and measures what each buys. It is being built; the catalog lists what is live.";

pub const CODE: [&str; 3] = [
    "This part is for readers who want to see the code. Nothing above depends on it. Lesson 1 says how to read X_eTaL as the page draws it.",
    "The step as the program writes it: a pipeline, a stage a line, each after the first an equation written as text. The forward stages are lesson 2's, with one column per example; the error, the gradients and the update are three more of the same kind.",
    "The whole program, as it runs in your browser each time an experiment is picked, then what the page asks of it afterward: the value of every stage of the step, and the step drawn as mathematics.",
];

/// Every text of the page, for the list of ideas and the tests.
pub fn all() -> Vec<&'static str> {
    let mut out = vec![LEDE, RATIONALE, NEXT];
    for group in [&QUESTION[..], &BACKGROUND, &PURPOSE, &TASK, &STEP, &STAGES, &TRAIN, &READING, &SUMMARY, &CODE] {
        out.extend_from_slice(group);
    }
    out
}
