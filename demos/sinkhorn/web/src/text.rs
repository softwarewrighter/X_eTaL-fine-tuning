//! The page's prose, for new eyes (docs/new-eyes.md): a reader who
//! knows neither machine learning nor X_eTaL, and has read lesson D1
//! or the welcome page. `[shown|id]` marks a word that opens its plain
//! explanation; `**words**` is bold. Every number a sentence quotes is
//! put in by the page from the run; none is typed here.

pub const LEDE: &str = "How is a table of positive numbers made into one whose rows and columns each sum to 1? Sinkhorn-Knopp, the projection mHC uses, one round with every number, then twenty rounds watched converging, and the three properties of the result checked by the program.";

pub const QUESTION: [&str; 2] = [
    "The page before this one ended on a rule: [mHC|mhc] replaces the [table|table] that mixes its carried state by one whose rows and columns each sum to 1, a [doubly stochastic|doubly-stochastic] table, which, the paper argues, keeps four [streams|stream] in bounds across any depth (the next page is to measure it). It left a question open: **how is such a table made** from whatever numbers a model has learned, and what can be said for certain about the table that comes out?",
    "The paper (Xie and others, \"mHC: Manifold-Constrained Hyper-Connections\") makes it by [Sinkhorn-Knopp|sinkhorn], a procedure from 1967 that is two divisions repeated, and cuts it at twenty rounds. This page runs those rounds in front of you on three starts that stand in for numbers a model might have learned (the paper's own start, and two others), and checks the three things the paper says of the result on the table you are looking at.",
];

pub const BACKGROUND: [&str; 4] = [
    "A [table|table] here is 4 rows of 4 numbers, the size mHC uses for four streams; its **row sums** are the four totals across, its **column sums** the four totals down. A positive table has every number above 0. The one on this page is the [exponential|exponential] of a table of [logits|logits], the way the paper and its Python re-implementation make theirs: e is a fixed number, about 2.718, and whatever the logits are, e raised to each is positive. The logits are the setting a button picks; the paper's start, 1 on the [diagonal|diagonal] and -1 elsewhere, is the first. A table is also called a matrix.",
    "**The row step** ([row normalization|row-normalization]) divides every number in a row by that row's sum, so that the row then sums to 1; **the column step** does the same down each column. Doing one undoes the other a little: after the columns are fixed the rows are off again, by less than before. A **round** is one of each, rows then columns, and [Sinkhorn-Knopp|sinkhorn] is rounds repeated until the sums are as close to 1 as wanted; the paper calls the result the projection of the start, which here means just that: what the rounds turn it into. On this page a sum counts as 1 when it is within a millionth of it (0.000001). Small numbers are written in powers of ten: 2.5e-4 means 0.00025, and 4.1e-12 is four millionths of a millionth.",
    "Three facts the paper states about a doubly stochastic table (its section 4.1), each checked here on the table the run produced rather than taken on trust: its [largest singular value|spectral-norm] (also called its spectral norm) is at most 1, so a list multiplied by it comes out no longer than it went in, a list's size being the square root of the sum of its squares, as on the first page; two such tables multiplied give another, so a stack of them is one; and it is a weighted mix of [permutation tables|permutation-matrix], tables that only reorder a list, with weights that sum to 1 (Birkhoff, 1946). The checks are made by the Check library of X_eTaL-libraries, which prints ok or FAIL with what it expected and what it got.",
    "Every number on this page is computed by [X_eTaL|xetal], the small language the lessons are written in, in your browser as the page loads; the code is in asides, and nothing on the page needs it.",
];

pub const RATIONALE: &str = "A projection sounds like a thing to take on faith; this one is two divisions, and the whole of it fits on a screen. Running it round by round, with the sums printed after each half, shows what it does and how fast, and shows that twenty rounds is a practical cut and not a law: on one of the three starts here the sums are still a few ten-thousandths off at round twenty, and the checks say so. The three properties are then not asserted but checked, by the program, on the table in front of you, so that where they hold, and where twenty rounds were not enough for them to, is seen and not assumed.";

pub const PURPOSE: [&str; 2] = [
    "It is one 4 x 4 table at a time, three starts to pick from (each a file of settings, picked by a button), twenty rounds stepped through, and eight checks. Nothing is trained, no stream state is mixed, no layer is run: this page is about the table alone.",
    "It can show how the projection works, how quickly, and that the three properties hold of what it makes, to the digits X_eTaL keeps. It cannot show why mHC wants those properties at depth, which the next page is to measure, nor the gradient of the projection, which training through it needs (the fourth page). The track's six pages (a track is a series of pages on one subject): the skip and four streams; this one; the gain by depth, what 48 such tables multiplied do; training at depth; the paper's own mixing, which depends on the input; and the comparison.",
];

/// Part 1: the start, and one round.
pub const START: [&str; 5] = [
    "Pick the start by its button. Each is a 4 x 4 of logits, as a file of settings and not code; the page runs the program again with it and reads everything below from that run.",
    "Here are the logits and their exponential, A, the positive table the rounds begin from, with its row sums and its column sums under it. For the paper's start every sum is {rowA0}: the table is the same in every row and column, which is what makes it an easy start. Every number drawn is X_eTaL's. The colors run from blue for a negative number through white to red for a positive one; only the logits have negatives, so they have a scale of their own, and the positive tables share another.",
    "**The row step.** R is A with every row divided by its sum. Row 1 worked out, to four decimals (the pictures round to two): {workedR}. Each row of R now sums to 1 (the program prints {rowsR1}); its columns sum to {colsR1}.",
    "**The column step.** C is R with every column divided by its sum. Each column of C now sums to 1 ({colsC1}); its rows sum to {rowsC1}, which is how far the column step undid the row step. One round is these two steps; the next round starts from C.",
    "The two steps are one equation each, written as text and run by X_eTaL as written; the aside shows them when Explorer is on. The program also makes the same round a second way, the way the twenty rounds below are made, and the two differ by {off}.",
];

/// Part 2: the rounds.
pub const ROUNDS: [&str; 4] = [
    "The same two steps, {tmax} times over, the paper's count. Press **Next** for the next round, **Back** for the one before, **Start** for the first, or **Play** to watch them in turn. Each round shows the table after its row step and after its column step, with their row sums and column sums under them; the shade is the number's size, on one scale for all the rounds.",
    "What is left for the next round to fix is how far the rows are off after the column step, and the chart is X_eTaL's drawing of that after each round (rows), with how far the columns were off before the column step beside it (columns), on a scale of powers of ten (log10): a point at -6 is a millionth. A distance of exactly 0 cannot be drawn on such a scale and sits at the floor, -16.",
    "**From this start, every row sum is within a millionth of 1 after {rounds} rounds.** After round {tmax} the farthest row sum is {rowOff20} from 1, and the farthest column sum before that round's column step was {colOff20}.",
    "**From this start the sums are not within a millionth after {tmax} rounds**: the farthest row sum is still {rowOff20} from 1. The paper cuts at twenty and gives cost as the reason: each round is two more divisions, every time the table is made. A start like this one, where the fourth row and column barely connect to the rest, closes in slowly, and the checks below report what twenty rounds reached; this page does not run it further.",
];

/// Said at each round; `{}` marks are filled from the run.
pub const ROUND: [&str; 3] = [
    "**Round {t} of {tmax}.** After the row step every row sums to 1 and the columns sum to {colsR}; after the column step every column sums to 1 and the rows sum to {rowsC}. The farthest row sum is {rowOff} from 1.",
    "**Round {t} of {tmax}: the first.** A's rows are divided by their sums, {rowsA}, and then the columns of that by theirs, {colsR}. The farthest row sum after it is {rowOff} from 1.",
    "**Round {t} of {tmax}: the last.** After it the rows sum to {rowsC}, the farthest {rowOff} from 1. This table is H, the one the checks below are made on. Late rounds change the numbers by less than the pictures' two decimals show; the sums under them show the change.",
];

/// Part 3: the three properties, checked.
pub const CHECKS: [&str; 7] = [
    "H is the table after round {tmax}. The program checks eight things about it, each within a millionth, and prints a line for each: two for its sums, two that it stretches nothing, two for the product of two such tables, and two for the mix of reorderings. The lines are quoted here as X_eTaL printed them; in a FAIL line, \"expected true, got 0\" means the condition checked came out false (0 stands for false), not that a sum was 0.",
    "The [largest singular value|spectral-norm] of a table is the most it can lengthen any list it multiplies: A's is {sigmaA}; H's is {sigmaH}. The paper states that for a doubly stochastic table it is at most 1; here it is measured, and comes out 1. The program also tries one list: v, of size {sizeV}, becomes H v, of size {sizeHv}. (The singular value is found by multiplying by the table over and over, written out in the program, since the library that would give it does not have it yet.)",
    "A second positive table, B0, set up in the program from the numbers 2, 3, 4, 5, 1, 2 ..., is made doubly stochastic the same way, into B, and H times B is taken. Its row sums are {rowsHB} and its column sums {colsHB}: the product of these two doubly stochastic tables is one too, which is what the next page is to build on, 48 tables multiplied.",
    "A [permutation table|permutation-matrix] has one 1 in each row and each column and 0 elsewhere; multiplying a list by it only reorders the list. Birkhoff's theorem (1946) says every doubly stochastic table is a weighted mix of permutation tables, the weights positive and summing to 1. The program finds such a mix one table at a time: of the 24 ways to pick one number from each row of H with no two in the same column, it takes the way whose smallest picked number is largest, uses that smallest number as the table's weight, and subtracts it from the numbers picked; then again on what is left, ten tries in all, a try that finds nothing left taking weight 0. For this H, {tables} tables carried weight, the weights {weights}, summing to {weightsSum}; what is left of H after them has largest number {rest}. Shown on this one table, not proved.",
    "**Every check passed**: H is doubly stochastic within a millionth, and the three properties hold of it, as the Check library reports: {summary}.",
    "**Not every check passed**: {summary}. The rows of H are not within a millionth of 1 after {tmax} rounds, and what follows from the sums is off by as much: the weights of the permutation tables sum to {weightsSum}, and what is left of H after them has largest number {rest}. The largest singular value, {sigmaH}, is within a millionth of 1, so that check passed. On this start the three properties are not shown to a millionth; the lopsided start reached a millionth in 11 rounds, and this one was not run past {tmax}.",
    "A second positive table, B0, set up in the program from the numbers 2, 3, 4, 5, 1, 2 ..., is made doubly stochastic the same way, into B, and H times B is taken. Its row sums are {rowsHB} and its column sums {colsHB}: the rows are off by about as much as H's are, since H itself is not doubly stochastic to a millionth from this start, so the product shows nothing here about two such tables.",
];

pub const SUMMARY: [&str; 4] = [
    "**How the table is made.** Two divisions, rows by their sums and then columns by theirs, repeated: each round leaves the rows a little less off than the round before, and on the paper's start one round is already exact, because every row and every column of its exponential sums to the same amount to begin with (pick it above to see it). On the lopsided start the rows were within a millionth of 1 after eleven rounds.",
    "**What twenty rounds is.** The paper's practical cut, not a guarantee: on the nearly split start the rows were still a few ten-thousandths off after twenty, and the checks reported FAIL for what depends on the sums. How close twenty rounds get depends on the start.",
    "**What was checked of the result.** On the two starts where the rounds reached a millionth (the paper's and the lopsided one), all three properties held within a millionth, checked by the program and not asserted: the largest singular value is 1, so the table stretches no list; the product of two such tables is one too; and the table is a weighted mix of permutation tables, found by a search one table at a time. On the nearly split start the sums, and what depends on them, did not reach a millionth in twenty rounds. The paper states the properties (section 4.1) and builds mHC on them.",
    "**What was not shown.** Why mHC needs the properties at depth, which is what 48 such tables multiplied do (the next page). The gradient of the projection, which training needs (the fourth page). Any proof: Birkhoff's theorem is shown on one example, and the singular value is found by repeated multiplication, not derived. And anything about a real model: this is one 4 x 4 at a time.",
];

pub const NEXT: &str = "The next page of this track, the gain by depth, is to multiply the first page's two tables, HC's (the exponential, rows summing to 3.82) and mHC's (its doubly stochastic form), by themselves 12, 24 and 48 times, and measure how large each product gets, in the paper's measure and in the one its Python re-implementation used. It is being built; the catalog lists what is live.";

pub const CODE: [&str; 3] = [
    "This part is for readers who want to see the code. Nothing above depends on it. Lesson 1 says how to read X_eTaL as the page draws it.",
    "The one round as the program writes it: a pipeline of two equations written as text, each run as written. The twenty rounds are the same two divisions in a loop, written out because the function is asked of X_eTaL-ML (FM13) and not yet there; beside it, the Python of the re-implementation this track follows (mHC-poc, `sinkhorn_doubly_stochastic` in `mlx/src/mhc.py`, MIT), line for line.",
    "The whole program, as it runs in your browser each time a start is picked, then what the page asks of it afterward: the value of every stage of the round, and the round drawn as mathematics.",
];

/// The Python (mHC-poc, mlx/src/mhc.py, MIT; its docstring and two
/// comments left out) and the X_eTaL loop, line for line (app::compare
/// lays them out).
pub const PYTHON: [&str; 8] = [
    "def sinkhorn_doubly_stochastic(A: mx.array, iters: int = 20, eps: float = 1e-12) -> mx.array:",
    "    X = A",
    "    for _ in range(iters):",
    "        row_sum = mx.sum(X, axis=1, keepdims=True)",
    "        X = X / mx.maximum(row_sum, eps)",
    "        col_sum = mx.sum(X, axis=0, keepdims=True)",
    "        X = X / mx.maximum(col_sum, eps)",
    "    return X",
];
pub const LOOP: [&str; 8] = [
    "B := tmax '{ M -> ... } p_ower B0",
    "p_ower B0",
    "tmax '{ M -> ... }",
    "'+ r_/_2 M",
    "R := M / ('+ r_/_2 M) '* t_able ones",
    "'+ r_/_1 R",
    "R / ones '* t_able '+ r_/_1 R",
    "B := ...",
];
pub const COMPARE: [&str; 8] = [
    "The function and its arguments: the positive start and the count, the paper's 20. Here the start is B0 (or A, in the loop above that also keeps a record), the count tmax, and the result is bound to a name.",
    "The Python keeps the table in X and begins with A; p_ower begins with the value on its right and hands it to the lambda as M.",
    "A for-loop of iters rounds; p_ower applies the lambda tmax times, each round given the table the round before returned.",
    "The row sums: a sum along axis 1 in the Python, a plus-reduce along axis 2 here (X_eTaL counts axes from 1).",
    "The row step: the table divided by its row sums, laid along each row ('* t_able ones). The Python guards a sum of 0 with eps; a sum of positive numbers cannot be 0, so there is no guard here.",
    "The column sums: axis 0 there, axis 1 here.",
    "The column step: the table divided by its column sums, laid down each column; the lambda's last value is the round's result.",
    "Both hand back the table after the last column step.",
];

/// Every text of the page, for the list of ideas and the tests.
pub fn all() -> Vec<&'static str> {
    let mut out = vec![LEDE, RATIONALE, NEXT];
    for group in [&QUESTION[..], &BACKGROUND, &PURPOSE, &START, &ROUNDS, &ROUND, &CHECKS, &SUMMARY, &CODE, &COMPARE] {
        out.extend_from_slice(group);
    }
    out
}
