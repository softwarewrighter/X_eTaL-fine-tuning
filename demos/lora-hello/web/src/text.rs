//! The page's prose, for a reader with new eyes: one who knows neither
//! machine learning nor X_eTaL, and may not program. `[shown|id]`
//! marks a word that opens its plain explanation
//! (shared/lesson/glossary.tsv); `**words**` is bold. Kept apart from
//! the page so a test can check every mark names an idea the glossary
//! has. Code is described as the page draws it (the arrow, the raised
//! lo), since that is what the reader sees.

pub const LEDE: &str = "How do you teach a model something new without it forgetting what it knows, and without retraining all of it? The idea, on one small piece of a model that you can read in full: leave the model as it is and train a small correction beside it.";

pub const HINT: &str = "New to this? A word with a dotted underline opens a plain explanation when you click it (or press Enter on it). Click again to close it. You need no machine learning and no programming to follow the page; the code is there for those who want it, folded away until opened.";

pub const QUESTION: [&str; 5] = [
    "You have a [language model|model] running on your own machine, and you want to teach it something new: the conventions of your project, a subject it is weak in, a way of answering. Three worries come with that.",
    "**Will it forget?** Teaching a model one thing can damage what it already knew.",
    "**How long will it take?** [Training|training] a whole model is days of computing on hardware most people do not have.",
    "**What will it cost?** That computing is memory and electricity.",
    "This page shows the idea most often used to ease all three, called [LoRA|lora], on one small piece of a model that you can read in full. It shows how the idea works. It does not train anything, so it does not prove the three worries away; the last section says exactly what it does and does not show.",
];

pub const BACKGROUND: [&str; 4] = [
    "A language model is, underneath, a very large collection of numbers called [weights|weight]: several billion in a model that fits on a laptop. They are kept in [tables|table]. The model works by taking its input, itself a list of numbers, and [multiplying it by one table after another|times]; each such step is a [layer|layer]. Everything the model knows is in those numbers.",
    "Teaching it something new is called [fine-tuning|fine-tuning]: the numbers are changed, a little at a time, so that the answers of the model move toward the ones you want. The direct way changes all of them, and that is where the three worries come from.",
    "**Time, memory, electricity.** A number being trained needs more than itself: which way to move it (its [gradient|gradient]) and, with the usual method, two [running averages|averages]. That is about four numbers held and updated for every one trained, billions of them, at every step of training.",
    "**Forgetting.** The new numbers replace the old ones, and what the model knew lived in the old ones. Training hard on one narrow thing can spoil abilities you never meant to touch. This is called [catastrophic forgetting|forgetting].",
];

pub const RATIONALE: [&str; 5] = [
    "LoRA does not change the weights of the model at all: they are [frozen|frozen]. Beside a table of weights, call it W, it puts a small [adapter|adapter] made of two thin tables, A and B, and only those are trained. The layer then gives its old output plus a [correction|correction]:",
    "The letters stand for these: x is the input, a list of numbers; y is the output, the list the layer gives back; W, A and B are tables; s is a single number. Two letters side by side mean \"times\", and here that is [a table times a list|times], not the multiplication of two plain numbers. It is worked through with real numbers just below.",
    "Read the correction from the right. A x: the table A squeezes the input down to a few numbers. B (A x): the table B spreads those few back out to the size of the output. s: [the strength|strength], one number that makes the correction count for more or less; a setting called [alpha|alpha] sets it. The result is added to W x, which is what the layer gave before there was any adapter.",
    "Thin is the point. How many numbers A squeezes the input into is called the [rank|rank]. A correction that has to pass through so narrow a middle cannot say everything, but it needs very few numbers. The bet LoRA makes is that what a model must change to learn one new thing is small enough to fit.",
    "Since W is never changed, nothing it knew is written over. Since only the two thin tables are trained, far fewer numbers are trained. Those are the two reasons the idea is used.",
];

pub const PURPOSE: [&str; 2] = [
    "It is one layer, shrunk from thousands of rows to 4 rows and 4 columns so that every number is on the screen. W is the frozen layer. The adapter has rank 1: A is a single row of 4 numbers and B a single column of 4.",
    "Nothing is trained here. The numbers of the adapter are made up. The aim is to see what an adapter is, where it sits, what it adds to the output of a layer and what it costs, before the lessons on how its numbers are learned. Every number below is worked out in your browser by a short program written in a language called [X_eTaL|xetal], shown at the end of the page. You do not need to read it.",
];

pub const NUMBERS: &str = "Here is the whole layer: the 16 numbers of the frozen table W, and the 8 numbers of the adapter (the one row of A, the one column of B). These are all there is; a real layer is the same thing with thousands of rows.";

pub const CALC: [&str; 9] = [
    "One input goes through the layer, step by step, with every number shown. Change the input or the strength and all of it is worked out again.",
    "x1 to x4 are the four numbers of the input, x. alpha sets [the strength|strength] of the correction: s = alpha / r, and the [rank|rank] r is 1 here, so s is alpha.",
    "**Step 1: what the frozen layer gives.** This is W x, [the table W times the list x|times]. For the first row of W: each of its numbers times the matching number of x, added up.",
    "The other three rows are worked the same way. Together they are what the layer gave before there was any adapter:",
    "**Step 2: the squeeze.** A has one row, so A x is one number: each number of A times the matching number of x, added up.",
    "**Step 3: the strength.** That one number times s.",
    "**Step 4: the spread.** That number times each number of B gives [the correction|correction], one number for each output.",
    "**Step 5: add.** The correction is added to what the frozen layer gave.",
    "W itself did not change, whatever you typed: no number of it is touched by the adapter.",
];

pub const TRIES: [&str; 3] = [
    "Before you change anything, predict: with alpha at 0, what will the last row, y, be? Set alpha to 0 and look.",
    "Change one number of the input. Which rows of the table above change? Does the table W, further up the page, ever change?",
    "Double alpha. What happens to the correction?",
];

pub const TABLE: [&str; 3] = [
    "The correction can also be written out as a table of its own, s B A, with as many rows and columns as W. Here it is. Look along the rows: each one is the row A multiplied by a single number. That is the same [rank|rank] 1 as before, seen from the other side: because the input is squeezed to one number, the whole table has only one row's worth of information in it.",
    "Add that table to W, number by number, and you get one ordinary table that does the work of W and the adapter together. This is called [merging|merge]. It makes a new table: the original W is still there, and an adapter kept apart can still be taken off, or swapped for another.",
    "Why merge? While the adapter is kept apart, the layer does a little extra work on every input (the squeeze and the spread). Once training is over, a merged table does none: the adapted model runs exactly as fast as the original.",
];

pub const COST: [&str; 2] = [
    "How many numbers would be trained? The table W has rows times columns: 4 x 4 = 16. The adapter has rank times (rows + columns): 1 x (4 + 4) = 8. Half of this tiny layer is no saving. The saving comes with size, because the [count|count] of the table grows with rows times columns and that of the adapter only with rows plus columns.",
    "A table of 4096 rows and 4096 columns is one of the tables of [a 7-billion-parameter model|sevenb] (such a model is made of a couple of hundred tables, many of them this size or larger), and 16 is an ordinary rank. There the adapter is 16 x (4096 + 4096) numbers beside the 4096 x 4096 of the table. These are counts, worked out by the program from the sizes. They are not measurements of time or of power.",
];

pub const FACTS: [&str; 4] = [
    "the rank of the correction's table: how many of its rows [say something new|independent]",
    "the merged table gives the same output as W and the adapter together ([to 12 decimal places|tolerance])",
    "numbers the adapter would train, beside the numbers of this table",
    "the same for a table of 4096 rows and 4096 columns, at rank 16",
];

pub const SUMMARY: [&str; 5] = [
    "**Will it forget?** Not shown here: nothing was trained, so nothing could be forgotten. What was shown is why the idea helps: the adapter never touches W, so the original model is kept whole. Take the adapter off (or set alpha to 0) and the layer answers exactly as before, and one model can carry a different adapter for each thing taught. What that does not promise: while an adapter is on, the output changes for every input its A responds to, wanted or not. Studies find that LoRA forgets less than changing every weight does, not that it forgets nothing (Biderman and others, 2024, \"LoRA Learns Less and Forgets Less\"). Measuring it is a later lesson.",
    "**How long, and at what cost?** Not measured here either. What was shown is a count: ",
    "Fewer numbers trained means less memory for [gradients|gradient] and [running averages|averages] and a small file to keep for each thing taught. How much time and electricity that saves has to be measured, and the frozen weights still take part in every step: cheaper, not free.",
    "**What do you give up?** A correction of [rank|rank] r can only say so much. How much rank a task needs, and what too little looks like, is what the next lessons measure rather than assert.",
    "In one sentence: instead of rewriting a model to teach it, you freeze it and train a thin correction beside it, which keeps the original intact and leaves far fewer numbers to train.",
];

pub const NEXT: [&str; 1] = [
    "This page showed what an adapter is. It did not show how A and B get their numbers, how to choose the rank, or how to tell whether the model learned and what it lost. Those are the lessons being built here: the same calculation one step at a time, a training step narrated, rank and strength measured, and LoRA beside its variations.",
];

pub const READING: [&str; 4] = [
    "This part is for readers who want to see the code. Nothing above depends on it.",
    "The numbers on this page are worked out by a short program in [X_eTaL|xetal], a language made for arithmetic on whole lists and tables at once. The page draws the code the way X_eTaL does, which is not quite how it is typed, so here is how to read what you see.",
    "Every name of a [function|function] in the code on this page is a link: it opens the entry for that function in the cross-reference, where its definition and its documentation are.",
    "The whole program, as it runs in your browser each time you change a setting. Its [notes|note] say in words what the lines under them do. A line that gives no name, such as the one that reads just \"y\", prints that value: those printed values are the numbers on this page. Where the program asks a yes-or-no question it prints [1 for yes|yes-no].",
];

/// How to read the code as the page draws it: what is seen, and what it means.
pub const SIGNS: [(&str, &str); 9] = [
    ("x := 1.0 2.0 3.0 4.0", "[The arrow|arrow] gives a value a name. Numbers side by side are a list. So: x is the list 1, 2, 3, 4."),
    ("# a note", "A line that starts with this sign is [a note for the reader|note], not an instruction. A note with a bar after the sign explains the line under it; one with a long rule is a heading."),
    ("r_eshape", "A word with one underlined letter is a [function|function]: a named recipe. (This one arranges numbers into a table.) The signs + and \u{00d7} are functions too."),
    ("2 * 3 + 1", "X_eTaL reads [from the right|right-to-left]: 3 + 1 first, then 2 times that. Round brackets go first, as in arithmetic."),
    ("(W, A, B, s)", "Things in round brackets with commas are handed over [together, as one bundle|tuple]."),
    ("A '+ '* i_nner x", "The three words in the middle mean [multiply the table A by the list x|inner]: pair the numbers up, multiply each pair, add."),
    ("\"lo:\" u_se< \"LoRA\"", "This line opens a [library|library], a box of ready-made functions, called LoRA, and gives it the short name lo. (The < at the end of the word use marks a word that acts while the program is being read, before it runs.)"),
    ("lo:f_orward", "A name with a small raised lo before it comes from that library: this is its function named forward."),
    ("{ a b -> a + b }", "Curly brackets make a function on the spot: before the arrow, the names of what it is given (one on its left, one on its right); after it, what it works out."),
];

/// The words of X_eTaL itself that the program uses, each with what it
/// does in plain words (as typed, so the page can draw and link them).
pub const BUILTINS: [(&str, &str); 10] = [
    ("r_eshape", "arranges the numbers on its right into a table with the rows and columns given on its left: 4 4 reshape makes 4 rows of 4"),
    ("r_ange", "the whole numbers from 1 up to the one given: range 16 is 1, 2, 3 ... 16"),
    ("f_loat", "turns whole numbers into decimal numbers, so they can be multiplied by a decimal"),
    ("s_in", "the sine of each number: used here only to make up 16 varied numbers for W without typing them"),
    ("r_avel", "lays a table out as one list, row after row, so it can be printed on one line"),
    ("i_nner", "with a quoted + and a quoted \u{00d7} before it: multiply a table by a list (or by a table): pair the numbers up, multiply, add"),
    ("c_at", "joins two lists, or two single numbers, into one list"),
    ("t_ake", "the first few items of a list: 4 take gives the first 4"),
    ("r_/", "works a whole list down to one number with the function quoted before it: with max, the largest"),
    ("a_bs", "the size of each number, ignoring its sign"),
];

/// Every text of the page, for the list of ideas and the tests.
pub fn all() -> Vec<&'static str> {
    let mut out = vec![LEDE, HINT, NUMBERS];
    for group in [&QUESTION[..], &BACKGROUND, &RATIONALE, &PURPOSE, &CALC, &TRIES, &TABLE, &COST, &FACTS, &SUMMARY, &NEXT, &READING] {
        out.extend_from_slice(group);
    }
    out.extend(SIGNS.iter().map(|(_, meaning)| *meaning));
    out
}
