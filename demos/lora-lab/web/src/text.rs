//! The page's prose, for new eyes (docs/new-eyes.md): a reader who
//! knows neither machine learning nor X_eTaL, and has read lesson 1.
//! `[shown|id]` marks a word that opens its plain explanation;
//! `**words**` is bold.

pub const LEDE: &str = "How is an adapter's correction actually worked out, number by number? And what does its rank change? One layer, with every number in view.";

pub const QUESTION: [&str; 2] = [
    "The first lesson showed what an [adapter|adapter] adds to a [layer|layer] whose [weights|weight] stay as they are. It left two questions open, and this lesson answers them.",
    "**How exactly is the correction worked out?** And: an adapter has a size, called its [rank|rank]. **What does a larger or smaller rank change, and what does it cost?**",
];

pub const BACKGROUND: [&str; 2] = [
    "From lesson 1: a layer takes a [list|list] of numbers, x, and gives back another list, y. Its own numbers are kept in a [table|table], W, and are [frozen|frozen]. Beside W sits the adapter, two thin tables called A and B. The layer gives what W works out from x, plus the adapter's [correction|correction]. In one line: y = W x + s B (A x), where s is [the strength|strength], a single number: alpha, a number chosen beforehand, divided by the rank. Alpha is 4 on this page.",
    "Two things are new. The layer is bigger: 8 numbers go in and 8 come out, so W has 8 rows (one for each number out) and 8 columns (one for each number in), 64 numbers. And the adapter is no longer the smallest there can be: its rank is 2 to begin with, and you can change it.",
];

pub const RATIONALE: &str = "A line like y = W x + s B (A x) hides almost everything. Read whole, it is easy to accept and hard to picture. Taken one [stage|stage] at a time, with the sum behind each number written out, nothing is left to take on trust: it is multiplying and adding. And once the stages are in view, rank stops being a word. It is how many numbers the middle stage has.";

pub const PURPOSE: [&str; 2] = [
    "It is one layer and one input, worked through in full. You can take the calculation a stage at a time, click any number to see the sum behind it, and change the rank to see what moves and what does not.",
    "Nothing is trained here. The numbers of W, A and B are made up. So this page can show how a correction is worked out and what a rank costs; it cannot show whether a larger rank makes a model better at anything. That needs training, which starts in the next lesson.",
];

pub const LAYER: [&str; 2] = [
    "Here is everything the layer has. The frozen table W. The adapter's two tables: A has as many rows as the rank, and B as many columns. And one input, x, to try them on.",
    "Each square is one number, shown to two decimals: red for positive, blue for negative, pale for close to zero. Click a square to read it in full.",
];

pub const STEPS: [&str; 2] = [
    "Four stages, as on the welcome page. The list x goes in. Stage h squeezes it through A. Stage u spreads that back out through B. Stage y adds u, times the strength, to what W works out from x.",
    "Press **Next** to move on a stage, **Back** to go to the one before, **Start** for the first, or **Play** to watch them in turn. Then click any square of a stage that has been worked out: the sum behind that number is written out underneath. The squares being worked out have a solid outline; the ones they are worked out from have a dashed one.",
];

/// What each stage does, in plain words (the input first).
pub const STAGES: [&str; 4] = [
    "**Stage 1 of 4, x.** The input: a list of 8 numbers. It is given, not worked out. Press Next.",
    "**Stage 2 of 4, h: the squeeze.** This is [A times x|times]. Each row of A gives one number, and A has as many rows as the rank, so the 8 numbers of x become as many numbers as the rank. Click a square of h.",
    "**Stage 3 of 4, u: the spread.** This is B times h. B has 8 rows, so the few numbers of h become 8 again, one for each output. Click a square of u.",
    "**Stage 4 of 4, y: the output.** W times x, which is what the layer gave before there was any adapter, plus the strength s times u; s is alpha divided by the rank. Click a square of y.",
];

pub const SUMS: [&str; 3] = [
    "Click a square of stage h, u or y above (once it has been worked out) to see the sum behind it here.",
    "The numbers in these sums are rounded to 3 decimals so that a sum fits on the page. Click a square in a picture for the number in full.",
    "That is the part of this output the frozen table gives. The adapter's part is added to it:",
];

pub const FORMULA: [&str; 3] = [
    "Each of stages h, u and y is one line of mathematics. Here are the three lines as they are usually written, each with the name of its stage in front. Click any marked letter, or press its button under the lines: the page says what it stands for, and outlines its picture above. Click another to change.",
    "How to read a line. The large sign, a Greek S, means \"add up\". Under and over it is which numbers to add: \"j = 0\" under and \"7\" over mean every position j from 0 to 7, which is all eight, since positions are counted from 0 here. A letter with small letters after it, low down, is one number picked out of a list or a table: the small letters say which. Three letters stand for positions: j runs over the input, from 0 to 7; k over the middle stage, from 0 to q, where q is the rank minus 1; i over the output, from 0 to 7. The pictures and the sums above count from 1 instead, so the first square of h is h(1) here and h with a small 0 in a formula.",
    "Click a letter in a formula.",
];

/// The letters of the formulas: the letter, the picture it has on the
/// page ("" for a single number), and what it stands for.
pub const SYMBOLS: [(&str, &str, &str); 9] = [
    ("x", "x", "The input: the list of 8 numbers that goes into the layer."),
    ("A", "A", "The adapter's first table. It has as many rows as the rank, each row 8 numbers long. It does the squeeze. In a real adapter its numbers would be trained."),
    ("h", "h", "What the squeeze gives: as many numbers as the rank. Everything the adapter passes on about x goes through these numbers."),
    ("B", "B", "The adapter's second table. It has 8 rows, and as many columns as the rank. It does the spread. In a real adapter its numbers would be trained."),
    ("u", "u", "What the spread gives: 8 numbers, one for each output."),
    ("W", "W", "The frozen table of the layer: 8 rows of 8 numbers. Nothing on this page changes it."),
    ("s", "", "The strength: a single number that the spread list is multiplied by before it is added. It is alpha divided by the rank; alpha is 4 on this page."),
    ("y", "y", "The output: the list of 8 numbers the layer gives back."),
    ("q", "", "The last position of h. It is the rank minus 1, because positions are counted from 0: at rank 2 the positions are 0 and 1."),
];

pub const RANK: [&str; 4] = [
    "The rank is how many numbers stage h has. A has that many rows, and B that many columns. Move the slider, from 1 to 8, and the page works the whole layer out again; pick an answer first, and the page says whether it was right.",
    "At rank 2, stage h has 2 numbers. Set the rank to 5: how many numbers will stage h have?",
    "Stage h has as many numbers as the rank.",
    "On a layer this small, an adapter stops saving anything at rank 4, where it has as many numbers as W itself. It saves where the table is large: lesson 1 gave the count for one of 4096 rows and 4096 columns, under 1 in 100 at rank 16.",
];

pub const CHANGED: [&str; 2] = [
    "The whole point, in three rows: what the frozen table gives, what the adapter adds, and their sum, the output. A square of the output is outlined where the correction moved it off the frozen table's number.",
    "W took no part in the change: its 64 numbers are the same at every rank.",
];

pub const SUMMARY: [&str; 4] = [
    "**How the correction is worked out.** Three small sums of products: the squeeze (A times x), the spread (B times that), then times the strength and added to what W gives. You have seen every number of each.",
    "**What the rank is.** The number of numbers in the middle stage, h: the rows of A, the columns of B. Everything the adapter passes on about x goes through h, so the rank is how many numbers that is.",
    "**What it costs.** The adapter has rank times (8 + 8) numbers on this layer: 16 more for every 1 of rank. That passes the 64 of W at rank 4.",
    "**What was not shown.** Whether a larger rank makes a model better at anything. Nothing was trained here: changing the rank changed the correction, but with made-up numbers no correction is more useful than another. What rank buys when an adapter is trained is measured in the lessons after the next.",
];

pub const NEXT: &str = "The next lesson, LoRA training, is about where the numbers of A and B come from: one step of training told stage by stage, then training on examples. It is being built; the catalog lists what is live.";

pub const CODE: [&str; 3] = [
    "This part is for readers who want to see the code. Nothing above depends on it. Lesson 1 says how to read X_eTaL as the page draws it.",
    "The layer as the program writes it: four stages, a line each. The first is the input; each of the others is an equation, written as text between quote marks, with the name of what it works out on its left. The page shows these same four lines as the four stages.",
    "The whole program, as it runs in your browser each time the rank changes, then what the page asks of it afterward: the value of every stage, and each stage's formula.",
];

/// Every text of the page, for the list of ideas and the tests.
pub fn all() -> Vec<&'static str> {
    let mut out = vec![LEDE, RATIONALE, NEXT];
    for group in [&QUESTION[..], &BACKGROUND, &PURPOSE, &LAYER, &STEPS, &STAGES, &SUMS, &FORMULA, &RANK, &CHANGED, &SUMMARY, &CODE] {
        out.extend_from_slice(group);
    }
    out.extend(SYMBOLS.iter().map(|(_, _, what)| *what));
    out
}
