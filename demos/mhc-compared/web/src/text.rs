//! The page's prose, for new eyes (docs/new-eyes.md): a reader who
//! knows neither machine learning nor X_eTaL, and has read lessons D1
//! to D5 or the welcome page. `[shown|id]` marks a word that opens its
//! plain explanation; `**words**` is bold. Every number of the run a
//! sentence quotes is put in by the page from the run; the Python's
//! and the paper's figures are quoted as theirs, with where they come
//! from.

pub const LEDE: &str = "The plain skip, HC and mHC side by side at the same depth, width and steps: the arithmetic at the start, the nine training runs of the fourth page, the Python re-implementation's figures and the paper's, each marked as measured here, worked out, or quoted.";

pub const QUESTION: [&str; 2] = [
    "Five pages have taken apart one idea from a paper (Xie and others, \"mHC: Manifold-Constrained Hyper-Connections\"): give a deep model four [streams|stream] instead of one, let each layer mix them with a [mixing table|mixing-table], and keep that table [doubly stochastic|doubly-stochastic] so that the mixing cannot grow the signal. This page asks the closing question: **set side by side, what did the three ways of connecting layers do, and how much of it was arithmetic, how much training, and how much is only reported by others?**",
    "The three: the **plain skip**, one stream, each layer adding its output to what it was given; [HC|hyper-connections], four streams mixed by a table of positive numbers; [mHC|mhc], the same table made doubly stochastic by twenty rounds of [Sinkhorn-Knopp|sinkhorn]. Each layer of HC and mHC has three tables, as on the first page: [H_res|h-res] mixes the streams into each other and carries them on; H_pre weighs the streams into the input of the layer's own small calculation (the **block**); H_post hands the block's output back to each stream.",
];

pub const BACKGROUND: [&str; 4] = [
    "**The gain.** When the same mixing table is used layer after layer, the tables multiply. The [gain|gain] says how much their product can enlarge what passes through: the paper's [forward gain|forward-gain] is the largest row sum of the product, its [backward gain|backward-gain] the largest column sum (for the [gradient|gradient] on its way back), and the Python's [proxy|gain-proxy] is the largest number in the product. The third page worked these out for the starting tables; the fourth trained small models and recorded them at every step. Many figures here are written as powers of ten: a 3 means a thousand (10 x 10 x 10), 0 means 1, and a negative power means less than 1 (-0.6 is about a quarter). A number written 3.18e-7 means 3.18 divided by ten seven times, 0.000000318; log10 on a chart's axis means the power of ten.",
    "**The runs.** The fourth page trained the three on one tiny task (16 tokens, each to be followed by the next) at 12, 24 and 48 layers, 800 steps each, from one start, every weight learned with [Adam|adam]: nine runs, made once on a computer and recorded (an [offline run|offline-run]). This page reads those nine records again and reduces each to a few figures: the [loss|loss] at the end, the [gradient size|gradient-norm], the gains, the [NaN events|nan-event].",
    "**Three kinds of figure**, marked wherever one appears. **Measured here**: computed by [X_eTaL|xetal] from this track's own runs. **Worked out**: arithmetic on the tables alone, exact, no training involved. **Quoted**: a figure another source reports, copied with where it comes from: the Python re-implementation this track followed (mHC-poc, a small model with [attention|attention] trained on a computer), and the paper itself (models with billions of [weights|weight]). Quoted figures are not recomputed here; they are set beside the toy's, not checked by it.",
    "Every number on this page that is not quoted is computed by X_eTaL in your browser as the page loads: the starting tables and their products, and the reading of the nine records. Nothing is trained on this page.",
];

pub const RATIONALE: &str = "Each earlier page answered one part of the question with its own numbers. Side by side, at equal depth, equal width and equal steps, they can be read together, and the kinds of figure kept apart: what the arithmetic says holds at the start for any model whose layers start from these tables; what the toy measured holds for the toy; what the Python and the paper report holds for their set-ups, which differ from the toy and from each other in ways the page lists.";

pub const PURPOSE: [&str; 2] = [
    "It is the arithmetic of the starting tables, the nine recorded runs read again and laid out by depth, the count of numbers each way of connecting trains, the Python's and the paper's figures quoted beside, and a list of what the toy showed and what it did not. A choice of depth sets which runs the main table shows.",
    "It cannot show anything new about large models: the paper's figures are its own, from models and text far past anything computed here, and the Python's are from its own run. Training with the paper's input-dependent tables (the fifth page) is not among the runs: it waits on the machine-learning library, as that page says.",
];

/// Part 1: at the start, worked out.
pub const START: [&str; 3] = [
    "Every layer of HC and mHC starts from the same mixing [logits|logits]: 1 on the [diagonal|diagonal], -1 elsewhere. HC's table is their [exponential|exponential]: {hc_d} on the diagonal and {hc_o} elsewhere, each row summing to more than 1. mHC's is that table after twenty rounds of Sinkhorn-Knopp: {m_d} and {m_o}, each row and column summing to 1. The plain skip has no mixing to speak of: with one stream its three tables are each the single number 1, so its stream passes on unchanged, a gain of 1.",
    "Multiplied over the layers, the two tables behave as the third page showed. The table below gives both measures of the product at each depth, in powers of ten. These are worked out, not measured: they hold for any model whose layers start from these tables, the Python's and this toy's alike, before any training.",
    "HC's forward gain at 48 layers is 10 to the power {hc48}: its row sum, multiplied 48 times. mHC's is 1: the run prints its power of ten as {m48}, which is 0 but for rounding in the last digits of a computer's arithmetic; its proxy, {mp48}, is the power of ten of one quarter: products of this table approach the table whose every number is a quarter, the four streams averaged (not every doubly stochastic table does: one that only swaps streams keeps swapping).",
];

/// Part 2: after training.
pub const RUNS: [&str; 5] = [
    "Choose a depth with the three buttons below; the table and the sentences under it change. The table sets the three runs at that depth side by side, every figure read by X_eTaL from the records (measured here). Gains and proxies are in powers of ten. Guessing evenly among the 16 tokens gives a loss of 2.77.",
    "At {depth} layers the loss after 800 steps is {b} with the plain skip, {h} with HC and {m} with mHC.",
    "**The plain skip did not learn at this depth**: it ended at the loss of guessing evenly, its gradient size at the end {bg}, very nearly 0. The fourth page shows its way there.",
    "HC's forward gain after training is 10 to the power {hg}, beside the 10 to the power {hs} worked out for the start, and HC learned all the same. mHC's composite (the product of all its layers' H_res) stayed close to 1 at every step of every depth: the program checks that it stays within a hundred-thousandth, and the largest distance seen was {off}. Twenty Sinkhorn rounds come close to a doubly stochastic table, not exactly to it, so the gain is not exactly 1.",
    "The largest gradient size of any step at this depth was {bmax} for the plain skip, {hmax} for HC and {mmax} for mHC: a very large size means the weights are pushed hard in one step, a size near 0 that they barely move (the fourth page shows the sizes step by step). No step of the nine runs had a NaN event.",
];

/// Part 3: beside the Python's run.
pub const PYTHON: [&str; 4] = [
    "The Python re-implementation (mHC-poc, written by the author of these pages, linked at the top) trained the same three ways at the same three depths for the same 800 steps, on its own task: 256 tokens, sequences of 128, attention in every block, a model 32 times as wide, and numbers held in 32 bits (about seven significant digits; X_eTaL holds them in 64 bits, about sixteen). Its results page reports the final losses and the proxies below; they are quoted, to the digits it gives.",
    "Losses on different tasks are not directly comparable, so the first chart divides each by its own loss of guessing evenly (2.77 here, 5.55 for 256 tokens) and shows the power of ten: 0 is guessing evenly, -3 a thousandth of it. The second chart shows the proxy: worked out at the start, measured here after training, and the Python's after training.",
    "**Where they agree.** The proxies: HC's grows by about seven powers of ten for every 12 layers, in both, close to the start's arithmetic; mHC's stays near the power of ten of one quarter, in both. At 48 layers mHC learned, in both.",
    "**Where they differ.** At 48 layers the Python's HC ended at {py_h48}, which is about its loss of guessing evenly, while this page's HC learned; and the Python's plain skip learned a little at every depth ({py_b}), while this page's learned at 12 and 24 layers and not at all at 48. The fourth page names the differences between the two set-ups that could account for this (32-bit numbers, the Python's blocks with their own skip and scaling inside, its task with attention) and does not claim which; neither does this page.",
];

/// Part 4: beside the paper.
pub const PAPER: [&str; 4] = [
    "The paper's own experiments are of another size: [mixture-of-experts|mixture-of-experts] language models of 3, 9 and 27 billion weights, trained on real text, with four streams and the input-dependent tables of the fifth page. The table quotes its 27-billion-weight results (its table 4) on five [benchmarks|benchmark], the baseline being its model without Hyper-Connections. Higher is a larger share of the benchmark's questions answered right.",
    "On four of the five mHC's score is the highest of the three; on MATH, HC's is. The paper also reports mHC's final training loss as 0.021 under the baseline's; HC's loss jumping, with its gradient size, near step 12,000 of training (its figure 2), where mHC's did not jump; HC's composite gain reaching peaks of about 3000 and mHC's a largest value of about 1.6 (its figures 3 and 7, the forward and backward gains of this track, averaged over the tokens of a sequence); and mHC taking 6.7% more training time than the baseline at four streams.",
    "**What the toy can say about these.** Not much, and the page keeps to it. The paper's tables also come out of twenty Sinkhorn rounds, and the fifth page showed twenty rounds falling short of exactly 1 when the logits are far apart: that is one way a gain above 1 can arise, not shown here to be the paper's. Beside the toy's 10 to the power 28 at 48 layers, the paper's 3000 comes from models of at most 30 layers (its table 5) whose HC tables are input-dependent; which of these differences accounts for the gap, the toy cannot say. The benchmark scores, the loss gap and the time are of a kind the toy has nothing to set beside: no text, no benchmark, no memory traffic.",
    "These figures are the paper's, copied from its text and tables (read at the source in November 2026, linked at the top); nothing here reproduces them.",
];

/// The paper's table 4, at 27B: benchmark, baseline, HC, mHC.
pub const PAPER_TABLE: [(&str, &str, &str, &str); 5] = [
    ("BBH", "43.8", "48.9", "51.0"),
    ("DROP", "47.0", "51.6", "53.9"),
    ("GSM8K", "46.7", "53.2", "53.8"),
    ("MMLU", "59.0", "63.0", "63.4"),
    ("MATH", "22.0", "26.4", "26.0"),
];

/// Part 5: the numbers trained.
pub const COUNTS: [&str; 3] = [
    "What each way of connecting costs in numbers to learn, worked out by the program from the toy's sizes. Per layer, the block (8 -> 16 -> 8, its two tables with their biases) has {block} numbers. The plain skip adds {plain} more, counted though they do not change (its three tables are single numbers that each come out as 1, so their gradient is 0); HC and mHC add {static} (H_pre 4 x 4, H_post 4, H_res 4 x 4); the paper's input-dependent tables of the fifth page would add {dynamic} (the projection 32 x 24, its 24 biases, three gating factors). The token tables in and out add {emb}.",
    "So at this width the static mixing tables add about an eighth to each layer, and the input-dependent ones nearly three times the block. That is a fact about a block of 8 numbers: reasoned from the shapes, not counted: the projection grows with the width of a stream, the block of a real model with the square of it, so at widths of thousands the proportion would be far smaller. The paper does not report its count; the time it reports, 6.7%, is the cost it gives.",
    "HC and mHC train exactly the same numbers: the Sinkhorn rounds add arithmetic, not weights.",
];

/// Part 6: showed, and not.
pub const SHOWED: [&str; 4] = [
    "**The bound is arithmetic.** A product of exactly doubly stochastic tables has every row sum and column sum 1, at any depth (worked out, part 1); twenty Sinkhorn rounds come within a rounding error of that from the starting table, and within millionths after training; HC's starting table, multiplied over the depth, has a gain that grows by a fixed factor with each layer.",
    "**Training at depth followed it, on the toy task.** Through 800 steps of training, mHC's gains stayed within a hundred-thousandth of 1 at every step of every depth, and HC's stayed near the start's arithmetic (measured here).",
    "**A large gain did not stop the toy learning.** HC learned at every depth, with a gain of 10 to the power 28 at 48 layers (the Python's HC did not learn at 48 layers); the plain skip, gain 1, failed at 48 layers (measured here). On this task the gain alone did not tell which would learn.",
    "**The gradients written out were checked.** The fourth and fifth pages checked the gradients they wrote out by hand against [central differences|central-differences] (the slow way: nudging one number up and down and measuring twice), at nine and ten numbers each; the records here come from the fourth page's program, the one checked.",
];

pub const NOT_SHOWN: [&str; 5] = [
    "**Attention, sequences and text.** The toy predicts each token from itself alone; the Python's and the paper's models read sequences.",
    "**Scale.** A few thousand weights here; the Python's model is 32 times as wide; the paper's have billions, and mixture-of-experts layers.",
    "**Training with input-dependent tables.** Shown for one layer on the fifth page; the training waits on the library.",
    "**Memory traffic and the paper's infrastructure** (its section 4.3: how four streams are moved in and out of a chip's memory, results computed again instead of stored, the order work is done in): the 6.7% is about these, and nothing here measures time on such hardware.",
    "**More than one start, one task and one set of settings.** Every figure measured here is one run; another start could differ.",
];

pub const SUMMARY: [&str; 4] = [
    "**What the arithmetic says.** an exactly doubly stochastic mixing keeps the gain of any depth at 1, and twenty Sinkhorn rounds come close to it; HC's starting mixing multiplies it by its row sum at every layer (worked out).",
    "**What the toy measured.** Training kept both as the arithmetic said; all three learned at 12 and 24 layers; at 48 the plain skip did not and HC and mHC did, with no NaN event in any run.",
    "**What the others report.** The Python's run agrees on the gains and on mHC learning at 48 layers, and differs on HC and the plain skip; the paper reports mHC ahead on four of five benchmarks at 27 billion weights, its largest reported gain about 1.6 against HC's peaks of about 3000, for 6.7% more time (quoted).",
    "**What remains open.** Why the toy's HC learned where the Python's did not, and what the input-dependent tables do in training: neither is answered here.",
];

pub const NEXT: &str = "This is the last page of the depth track. Its one waiting part, training with the paper's input-dependent tables, becomes a page when the machine-learning library has the Hyper-Connections layer with its gradient. The catalog lists every lesson that is live.";

pub const CODE: [&str; 4] = [
    "This part is for readers who want to see the code. Nothing above depends on it. Lesson 1 says how to read X_eTaL as the page draws it. The program has three parts: the starting tables and their products, the nine records read and reduced, and the Python's figures read and charted. It defines no function: the mixing, the Sinkhorn rounds and the measures are written out where they are used, as on the other pages (ask FM13 of X_eTaL-ML would make them library functions).",
    "The X_eTaL beside the Python, function for function. The Python's functions are mHC-poc's (MLX version, linked). The X_eTaL lines marked \"this page\" are in the program below; those marked \"the fourth page\" are quoted from its training program, linked at the commit they were taken from.",
    "The whole program, as it runs in your browser.",
    "The data it reads: the nine records, copied from the fourth page's offline runs (what made them is below), and the Python's figures, two small files quoted from its results page.",
];

/// The Python's functions beside the X_eTaL: the Python function, what
/// it does, the X_eTaL, and where that X_eTaL is ("this page" or "the
/// fourth page").
pub const BESIDE: [(&str, &str, &str, &str); 12] = [
    ("sinkhorn_doubly_stochastic(A, iters=20)", "twenty rounds of a row normalization then a column normalization", "tmax '{ M -> R := M / ('+ r_/_2 M) '* t_able ones; R / ones '* t_able '+ r_/_1 R } p_ower A", "this page"),
    ("StreamMix.matrix(), mode \"softplus\" (HC's H_res)", "the exponential of the logits", "A := e_xp I2", "this page"),
    ("StreamMix(...).logits = 2 * eye - 1", "the starting logits", "I2 := (2.0 * f_loat (r_ange S) '= t_able r_ange S) - 1.0", "this page"),
    ("gain_proxy_from_Hres(Hres_list)", "the product over the layers, then the log10 of its largest number", "L '{ P -> A '+ '* i_nner P } p_ower f_loat mx:i_dentity S", "this page"),
    ("(not in the Python)", "the paper's forward gain: the largest row sum of the product", "'m_ax r_/ a_bs '+ r_/_2 Pc", "this page"),
    ("apply_stream_mix(H, x)", "mix the streams by a table", "Hres '+ '* i_nner X", "the fourth page"),
    ("mx.mean(apply_stream_mix(Hpre, x), axis=2)", "H_pre's mix, then the streams averaged: the block's input", "m := (B c_at D) r_eshape ((1 c_at S) r_eshape ('+ r_/_1 Hpre) / f_loat S) '+ '* i_nner X", "the fourth page"),
    ("TransformerBlock (attention, then an MLP)", "the block; here a small MLP alone", "f := r_avel (act:g_elu m nn:d_ense W1) nn:d_ense W2", "the fourth page"),
    ("cross_entropy_logits(logits, targets)", "the loss", "loss := Y nn:c_rossEntropy pr", "the fourth page"),
    ("grad_global_norm(grads)", "the gradient size", "s_qrt '+ r_/ g * g", "the fourth page"),
    ("mx.value_and_grad(...)", "the gradient, found automatically in the Python; here written out layer by layer, about forty lines", "(_, G0, gl) := L '{ (l, G, gl) -> ... } p_ower ...", "the fourth page"),
    ("optim.AdamW(learning_rate=lr)", "one step of Adam (the Python's with weight decay, set to 0 in its runs)", "(th2, m2, v2) := (rate, f_loat 1 + (t_ally rec) d_iv 6) op:a_dam (th, m1, v1, g)", "the fourth page"),
];

/// Every text of the page, for the list of ideas and the tests.
pub fn all() -> Vec<&'static str> {
    let mut out = vec![LEDE, RATIONALE, NEXT];
    for group in [&QUESTION[..], &BACKGROUND, &PURPOSE, &START, &RUNS, &PYTHON, &PAPER, &COUNTS, &SHOWED, &NOT_SHOWN, &SUMMARY, &CODE] {
        out.extend_from_slice(group);
    }
    out
}
