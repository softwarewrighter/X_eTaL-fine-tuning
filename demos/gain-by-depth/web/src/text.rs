//! The page's prose, for new eyes (docs/new-eyes.md): a reader who
//! knows neither machine learning nor X_eTaL, and has read lessons D1
//! and D2 or the welcome page. `[shown|id]` marks a word that opens its
//! plain explanation; `**words**` is bold. Every number a sentence
//! quotes is put in by the page from the run, or is the paper's or the
//! Python's and said to be theirs; none of the run's is typed here.

pub const LEDE: &str = "What does one mixing table become when it is applied at every one of 48 layers? HC's table and mHC's, multiplied together layer after layer, every product measured two ways, from the tables alone.";

pub const QUESTION: [&str; 2] = [
    "The first page of this track showed that a deep model carries its [streams|stream] from layer to layer through a [mixing table|mixing-table], H_res, written H on this page; what passes from layer to layer, the four streams together, is the **carried state**. The second showed how [mHC|mhc] makes that table [doubly stochastic|doubly-stochastic]. This page asks what the difference is worth: **after 12, 24 or 48 layers, how much can the carried state have been multiplied by**, with [HC's|hyper-connections] table and with mHC's?",
    "The paper (Xie and others, \"mHC: Manifold-Constrained Hyper-Connections\") reports that with HC this [gain|gain] reached about 3000 in its largest model and with mHC about 1.6. The Python re-implementation this track follows reports its own measure at three depths. Both come from training runs. This page works the same question out from the tables alone, at their **start**, the values they have before any training, and sets the trained figures beside it.",
];

pub const BACKGROUND: [&str; 4] = [
    "Applying a table at every layer means multiplying by it once per layer: after two layers the state has been multiplied by H times H, after k layers by k copies of H multiplied together. That one table standing for many layers is the [composite mapping|composite-mapping], written H^k (H to the power k). The **depth** is the number of layers. Here one table is reused at every layer, a simplification; in a real model each layer has its own, and the composite is their product.",
    "The two tables are the first page's. Both start from a 4 x 4 of [logits|logits], 1 on the [diagonal|diagonal] and -1 elsewhere. **HC's** is the [exponential|exponential] of the logits as it is (e, about 2.718, raised to each); **mHC's** is that table made doubly stochastic by twenty rounds of [Sinkhorn-Knopp|sinkhorn], the second page's procedure. A slider adds the same amount to every logit, the **shift**; that multiplies every number of HC's table by the same factor, and so asks how small HC's table has to be before its gain stops growing.",
    "How large is a table? Two measures are used, each named and kept apart. **The paper's**, which it calls the Amax Gain Magnitude (its section 3.1; Amax for the largest absolute value, the size of a number ignoring its sign): the [forward gain|forward-gain], the largest of the table's row sums, which bounds how much any stream can grow on the way through the layers; and the [backward gain|backward-gain], the largest of its column sums, the same on the way back, when training sends its corrections from the last layer toward the first. For the tables here the two are equal, since each table reads the same across as down. **The Python's** [proxy|gain-proxy]: the largest single number in the table, given as its power of ten (log10): 2 means about 100, 0 means about 1, -0.6 means about a quarter. The two are different numbers for the same table, and the page says which one it quotes. Very large and very small numbers are written in powers of ten: 9.71e6 means 9.71 times 10 to the power 6, about ten million, and 2.94e-9 means 2.94 divided by 10 to the power 9 (this e has nothing to do with the e of the exponential).",
    "Every number on this page, apart from the figures quoted from the paper and the Python, is computed by [X_eTaL|xetal], the small language the lessons are written in, in your browser as the page loads; the code is in asides, and nothing on the page needs it. A figure marked **analytical** here follows from the arithmetic of the tables; a figure marked **the Python's run** or **the paper's** is quoted from them, with the link, and was measured after training.",
];

pub const RATIONALE: &str = "The paper's headline is a measured curve from a model with 27 billion weights; the arithmetic underneath it is small enough to do here. Multiplying the two tables together, layer by layer, and measuring each product shows where HC's growth can come from and why, for these tables, mHC's cannot grow, with no training to blur it. The Python's trained figures set beside the untrained ones show how close the two are, and the slider shows that the row sum alone decides whether HC's gain grows or fades: only a row sum of exactly 1 keeps it level.";

pub const PURPOSE: [&str; 2] = [
    "It is two 4 x 4 tables, each multiplied by itself 48 times, every product shown and measured; one slider, the shift of the logits; one chart X_eTaL draws; the measured figures of the Python's run and the paper's quoted beside the arithmetic. Nothing is trained and no list is sent through: this is about the tables alone.",
    "It can show what repeating one mixing table does, exactly, and how the two measures differ. It cannot show what training does to the tables (the next page trains the three schemes at three depths), nor a model whose layers each have their own table, nor anything at the paper's scale.",
];

/// Part 1: the two tables and the shift.
pub const TABLES: [&str; 4] = [
    "Move the slider to add the same amount to every logit; the program runs again and everything below is read from that run. At 0 the logits are the paper's and the Python's start.",
    "Here are the logits and the two tables made from them, each with its four row sums. Every row of HC's table sums to {s}; every row of mHC's to {sm}.",
    "The row sum is the whole story for HC. Each row of HC's table holds the same numbers in another order, so every row sums to the same {s}, and every row of HC's table multiplied by itself k times sums to {s} multiplied by itself k times. After 48 layers that is {s48}.",
    "mHC's table does not move with the shift: {same}. Shifting every logit by the same amount multiplies every number of HC's table by the same factor, and the first row step of Sinkhorn-Knopp divides it out again.",
];

/// Said at each layer of part 2; `{}` marks are filled from the run.
pub const LAYER: [&str; 2] = [
    "**After layer {k}.** HC's composite has forward gain {fHC} and backward gain {bHC}, and proxy {qHC}: its largest number is about 10 to that power. mHC's has forward gain {fHM}, backward gain {bHM}, and proxy {qHM}.",
    "**After layer 1.** The composite is the table itself: HC's row sum is its forward gain, {fHC}; mHC's, {fHM}. The proxy is {qHC} for HC and {qHM} for mHC.",
];

/// Part 2: layer by layer.
pub const LAYERS: [&str; 3] = [
    "Press **Next** for the next layer, **Back** for the one before, **Start** for the first, or **Play** to watch them in turn. Each layer shows HC's composite and mHC's, that many copies of each table multiplied together, with their two gains and the proxy. The pictures have a scale each, since HC's numbers and mHC's can differ by many powers of ten; the numbers in them are rounded, and where they are too long or too small to write in, a tap or a click on a cell reads one.",
    "One step worked out: the forward gain after layer 2 is the largest row sum of H times H. For HC every row of H sums to {s}, so every row of H times H sums to {s} times {s}; the program measured the largest row sum of H times H as {f2}.",
    "mHC's composite does something else: its numbers flatten. Each new product is a weighted average of the old one's rows, so the large diagonal and the small rest even out, toward a quarter in every place: the four streams averaged. Its row and column sums stay at 1 the whole way.",
];

/// Part 3: the gain at 12, 24 and 48 layers.
pub const DEPTH: [&str; 5] = [
    "The chart is X_eTaL's drawing of the forward gain and the proxy for both tables, layer by layer, in powers of ten. HC's two lines {hcway} straight, since multiplying by the same row sum at every layer adds the same amount to its power of ten; mHC's forward gain lies flat at 0, which is a gain of 1, and its proxy settles at {limit}, a largest number of a quarter.",
    "The table quotes the run at the Python's three depths, beside what the Python's run measured at those depths after 800 steps of training (its own measure, the proxy). The run's figures here are **analytical**: from the tables at their start, untrained.",
    "Read across: at the paper's start the untrained proxy for HC is {q12}, {q24} and {q48}; the Python's trained run measured 7.0, 14.4 and 27.3. They are within about one power of ten of each other at every depth, on a scale that runs from 6 to 27: after training, the Python's tables give much the same growth as the untrained ones. This page does not show why; it does not train anything. For mHC the untrained proxy is {m12}, {m24} and {m48}; the Python measured -0.5, -0.6 and -0.6.",
    "At this shift the run is not the start the Python used, so its figures are set beside the Python's for comparison only: the untrained proxy for HC is {q12}, {q24} and {q48}, and the Python's run, from the paper's start, measured 7.0, 14.4 and 27.3.",
    "The paper's figures are of another kind again: its forward gain from a model with 27 billion weights, in training, at 30 layers or fewer. It reports that HC's composite reached about 3000 at its peaks, far below the {g48p} that HC's untrained table at the paper's start gives here after 48 layers, and that mHC's stayed at most about 1.6, not exactly 1. Its setting differs in ways that matter: each layer has its own table, the tables depend on the input, and the paper does not say what its tables start from; its twenty rounds of Sinkhorn-Knopp need not reach 1 exactly (the previous page showed a start where they do not). This page cannot tell which of these accounts for either figure; both are quoted as the paper's (figures 3 and 7).",
];

/// What the slider shows, said from the run.
pub const SHIFT: [&str; 3] = [
    "**At this shift HC's rows sum to {s}, more than 1**, so its forward gain grows by that factor at every layer: {s12} after 12 layers, {s48} after 48. mHC's stays at {fm48}.",
    "**At this shift HC's rows sum to {s}, less than 1**, so its forward gain shrinks by that factor at every layer: {s12} after 12 layers, {s48} after 48. What it carries fades toward 0 instead of growing. mHC's stays at {fm48}.",
    "Only a row sum of exactly 1 keeps HC's gain level, and on this slider it is crossed between two settings: the row sums at -1.5 and -1.25 are {lowS} and {highS}, and their gains after 48 layers {lowG} and {highG}. Nothing holds a table that is learned freely on that edge; Sinkhorn-Knopp makes every row sum 1, whatever table it is given.",
];

pub const CHECKED: &str = "Four checks close the program, made by the Check library and quoted as X_eTaL printed them: that HC's forward gain is its row sum to the power of the depth at every layer, that mHC's forward and backward gains are 1 at every layer, and that mHC's largest number after 48 layers is a quarter (one over S, the number of streams, 4), within a millionth. The result: {summary}.";

pub const SUMMARY: [&str; 5] = [
    "**The answer.** Applied at every layer, a table multiplies the carried state's gain by its row sum at every layer. HC's table at the paper's start has rows summing to 3.82, so after 48 layers its forward gain is 3.82 multiplied by itself 48 times, 8.90e27; mHC's rows and columns sum to 1, so its two gains stay at 1 at every depth.",
    "**The slider.** Shifting the logits multiplies HC's table by one factor: above a row sum of 1 its gain grows layer after layer, below it fades toward 0, and on this slider the edge lies between the shifts -1.5 and -1.25. mHC's table is the same at every shift.",
    "**Two measures, two numbers.** The paper's forward gain and the Python's proxy are different measures of the same tables; for mHC one is 1 and the other -0.602, a largest number of a quarter, and neither is wrong. The page quotes each by its name.",
    "**Beside the trained figures.** At the paper's start the untrained proxy is within about one power of ten of what the Python's trained run measured at 12, 24 and 48 layers; this page does not show why. The paper's 3000 and 1.6, from training at 27 billion weights with a table per layer, are quoted, not reproduced, and this page cannot account for them.",
    "**What was not shown.** A list sent through the layers (the first page did that, with a block in each layer); tables that differ from layer to layer; training, and what it does to HC's gain (the next page); anything at the paper's scale.",
];

pub const NEXT: &str = "The next page of this track, training at depth, trains the plain skip, HC and mHC on one small task at 12, 24 and 48 layers, and measures the loss, the size of the corrections and both gains as training goes. It is being built; the catalog lists what is live.";

pub const CODE: [&str; 2] = [
    "This part is for readers who want to see the code. Nothing above depends on it. Lesson 1 says how to read X_eTaL as the page draws it. The products are a loop of matrix products, one per layer, keeping the product and its three measures after each; Sinkhorn-Knopp and the two measures are written out because the machine-learning library does not have them yet.",
    "The whole program, as it runs in your browser each time the slider moves.",
];

/// Every text of the page, for the list of ideas and the tests.
pub fn all() -> Vec<&'static str> {
    let mut out = vec![LEDE, RATIONALE, NEXT, CHECKED];
    for group in [&QUESTION[..], &BACKGROUND, &PURPOSE, &TABLES, &LAYER, &LAYERS, &DEPTH, &SHIFT, &SUMMARY, &CODE] {
        out.extend_from_slice(group);
    }
    out
}
