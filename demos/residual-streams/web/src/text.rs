//! The page's prose, for new eyes (docs/new-eyes.md): a reader who
//! knows neither machine learning nor X_eTaL, and has read lesson 1
//! or the welcome page. `[shown|id]` marks a word that opens its plain
//! explanation; `**words**` is bold. Every number a sentence quotes is
//! put in by the page from the run; none is typed here.

pub const LEDE: &str = "Why does a deep model need a skip connection, and what happens when there are four of them? A list sent through 48 small layers, measured after every one; then one layer of Hyper-Connections, stage by stage, with every number in view.";

pub const QUESTION: [&str; 2] = [
    "The lessons so far changed one [layer|layer] of a model. A real model has dozens of layers in a row, and a question comes before any of them can be taught anything: **how does a list of numbers get through 48 layers at all** without being lost on the way, or blown up out of all proportion?",
    "Almost every deep model answers it the same way, with a skip connection. A 2025 paper from DeepSeek (Xie and others, \"mHC: Manifold-Constrained Hyper-Connections\") is about what happens when that connection is widened into several side by side: [Hyper-Connections|hyper-connections], HC for short, and their constrained form, [mHC|mhc]. This page is the first of the depth track, six pages on that paper: the skip, measured; then one layer of Hyper-Connections taken apart.",
];

pub const BACKGROUND: [&str; 4] = [
    "A [layer|layer] takes a [list|list] of numbers and hands on a new list. On this page a layer's work is done by a small **block** of three steps: a [table|table] of 8 x 16 [weights|weight] turns the 8 numbers into 16 (each of the 16 is a sum of products of the 8 with one column of the table, as lesson 2 showed for 8 x 8), a short list of 16 **bias** numbers is added, and a fixed curve called [GELU|gelu] is applied to each of the 16; then a second table, 16 x 8, with its own 8 bias numbers, turns them back into 8. The weights are not trained on this page: every one is a sine or a cosine of a counter, scaled, a different set for each of the 48 layers, so that the blocks are as good as random and the same every time.",
    "A stack of layers is the layers one after another: the second is given what the first handed on, and so on to the forty-eighth. The question of this page is what the forty-eighth gets. The numbers are small so that every one can be shown: 8 in a list, 16 inside a block; 48 layers because the paper's Python re-implementation this track follows ran 12, 24 and 48.",
    "Two lists are sent through, the second differing from the first by a hundredth in its first number. **The gap** between them is the size of their difference, number by number (the size of a list being the square root of the sum of its squares), divided by the size of the difference sent in. Measured after every layer, it says whether the layers still tell the two inputs apart: a gap of 1.0 means the difference went through as it was; a gap of 0 means the two inputs now give the same output, and the stack has forgotten what it was given. A hundredth is small enough to be a nudge and large enough to show in six decimals.",
    "Every number on this page is computed by [X_eTaL|xetal], the small language the lessons are written in, in your browser as the page loads; the code is in asides, and nothing on the page needs it.",
];

pub const RATIONALE: &str = "Skip connections are usually introduced with a drawing of an arrow going around a box, and the reader is asked to take the arrow's worth on trust. Here the stack is run both ways, with the arrow and without, and measured after every layer, so what the arrow does is a number you can read. The widened kind, Hyper-Connections, is then taken apart one stage at a time on the same small block, so that the mixing of its streams is arithmetic you can check, with one line of it worked out in numbers, and not a diagram.";

pub const PURPOSE: [&str; 2] = [
    "It is 48 tiny layers on a list of 8 numbers, run forward only: nothing is trained, and no loss is measured. You can send the list through with the skip and without, at three weight scales (each a file of settings, picked by a button), and step through one layer of Hyper-Connections with its carried state mixed the unconstrained way (HC) or the constrained way (mHC).",
    "It can show what a skip connection keeps and what mixing four streams does to one list. It cannot show what training does at depth (the track's fourth page), nor anything about a real model: the blocks are made by a formula, and the numbers are not those of any model. The track's six pages: this one; Sinkhorn-Knopp, the projection mHC uses; the gain by depth, what 48 mixings do; training at depth; the paper's own mixing, which depends on the input; and the comparison.",
];

/// Part 1.
pub const STACK: [&str; 4] = [
    "Here are the two lists sent in: {x0}, and the same but for a hundredth in its first number, {x1}.",
    "Each of the 48 layers can be used in two ways. **Replaced**: the layer hands on the block's output and nothing else. **Added to**: the layer hands on the list it was given plus the block's output, which is a [skip connection|residual]: the block only works out what to add, and if it added nothing the list would go through untouched. The same 48 blocks, the same two lists, both ways.",
    "Pick the scale of the blocks' weights by its button: every weight is multiplied by it. Each button is a file of settings, not code; the page runs the stack again with it and reads the result below, from the run.",
    "The chart is X_eTaL's drawing of the gap after each layer, both ways, on a scale of powers of ten: a point at -2 is a gap a hundredth of the one sent in. A gap of exactly 0 cannot be drawn on such a scale and sits at the floor, -12. The table under it quotes the run at five depths.",
];

/// What the run shows, said from its numbers (app::reading fills the marks).
pub const READING: [&str; 4] = [
    "With the blocks **replacing** the list, the gap after layer 1 is {p1} of the gap sent in, after layer 12 it is {p12}, and after layer 48 it is {p48}. With the blocks **adding to** the list, the gap is {s1} after layer 1, {s12} after layer 12 and {s48} after layer 48.",
    "The size of the list itself tells a different story from the gap: replaced, the list after layer 48 has size {psize48}; added to, {ssize48}. A list can keep a size and still carry nothing of its input, which is why the gap is measured and not the size alone.",
    "A gap of 0 means the two lists have become the same to the last digit X_eTaL keeps: the output no longer depends on which input was sent.",
    "At this scale the gap after 48 layers is more than ten times the gap sent in both ways, with the skip and without: a stack can lose its input by inflating a difference as well as by shrinking it. The skip does not guard against that by itself; the track's fourth page is about what does.",
];

/// Part 2.
pub const STREAMS: [&str; 6] = [
    "[Hyper-Connections|hyper-connections] widen the one list a model carries into several [streams|stream], here four, so that what goes from layer to layer is a 4 x 8 table, a stream a row: **the stream state**, X. At the start all four rows hold the list sent in. Each layer then does five things: mixes the streams, averages them into one list for its block, runs the block once, hands the block's output back to the streams in shares, and mixes X once more for the state it carries on, adding the shares to that. (The nine stages below are these five with the block's three steps counted apart.)",
    "The mixing is done by three small [mixing tables|mixing-table], two of them 4 x 4 and one a row of 4; in a trained model they are learned. Here they are what the paper and its Python re-implementation start from: a table of [logits|logits] with 1 on its [diagonal|diagonal] and -1 everywhere else (written 2I - 1, two times the identity table less one), made into weights in two ways. H_pre, which mixes before the block, is the logits put through a [softmax|softmax] row by row, so that each row holds positive numbers summing to 1: {hpre0} of each new stream is its old self and {hpre1} is each of the others. H_post, the shares of the block's output, is the first row of H_pre.",
    "H_res mixes X for the state the layer carries on. With one stream, a plain skip connection hands the list on as it is and adds the block's output; with four, what is handed on is X multiplied by H_res, and the shares are added to that. **HC** uses the [exponential|exponential] of the logits as it is: {e} on the diagonal, {inv} elsewhere, every row summing to {rowsum}. **mHC** first replaces that table by one whose rows and columns each sum to 1, by [Sinkhorn-Knopp|sinkhorn]: twenty rounds of dividing every row by its sum, then every column by its sum, twenty being the paper's count and enough here for the sums to reach 1 in every digit kept. The row sums and column sums of both are quoted under the tables, from the run.",
    "For this start the projection lands on the softmax table: mHC's H_res has the same numbers as H_pre, {hpre0} and {hpre1}, because the logits are the same in every row and every column. The two tables part once the logits are trained and lose that symmetry.",
    "The sums are the point. X is multiplied by H_res at every layer, so across 48 layers it is multiplied by H_res 48 times over. A row summing to 1 makes each new stream a weighted average of the old four, so no stream can grow past the largest of them; a column summing to 1 keeps the total of the four streams, and so their average, where it was. A row summing to {rowsum} makes each new stream {rowsum} times a weighted average, at every layer. The paper calls what the sums keep the [identity mapping|identity-mapping]: with one stream it is the plain skip, which hands on exactly what it was given. What 48 such multiplications do is the track's third page; this one names the property and shows the two tables.",
    "Choose which table the layer below uses, then step through it. Both layers run with the table chosen. The state the second layer is given is the one after the first, where the four streams already differ: the first layer was given four copies of the list sent in, and H_post handed {hpost0} of its block's output to the first stream and {hpost1} to each of the others. (Pictured above, on the right, for the table chosen.)",
];

pub const STEP: [&str; 2] = [
    "The second layer in nine stages: the stream state X; the streams mixed by H_pre, P; their average, m; the block's inner list z and its GELU h; the block's output f; f handed to the streams in H_post's shares, Fs; X mixed by H_res, R; and the new state, R plus Fs. Press **Next** to move on a stage, **Back** for the one before, **Start** for the first, or **Play** to watch them in turn. The circles name what each stage works out; the squares worked out in a stage have a solid outline, the ones it reads a dashed one. Click any square to read its number in full.",
    "Every stage is one equation written as text, which X_eTaL runs as written; the block's two tables and two bias lists are the second layer's own.",
];

/// The nine stages, as the page says them; `{}` marks are filled from the run.
pub const STAGES: [&str; 9] = [
    "**Stage 1 of 9, the stream state.** Four streams of 8 numbers, a stream a row: the state after the first layer. The rows differ, the first most of all, because the first layer handed the first stream {hpost0} of its block's output and each of the others {hpost1}. Both P and R below are made from this X.",
    "**Stage 2 of 9, the streams mixed before the block.** P is H_pre times X: row s of P is the rows of X weighted by row s of H_pre, {hpre0} of stream s itself and {hpre1} of each other stream. Its first number, worked out: {workedP}. Each row of H_pre sums to 1, so a row of P is a weighted average of the four streams.",
    "**Stage 3 of 9, the average over streams.** The block takes one list of 8, not a table of four, so the four rows of P are [averaged|stream-average] column by column into m. On this page that is the only place the streams meet before the block.",
    "**Stage 4 of 9, the block's first table.** z is m times the second layer's first table, 8 by 16, plus that table's bias list: a list of 16, a sum of products in each entry, as in lesson 2.",
    "**Stage 5 of 9, the curve.** h is [GELU|gelu] applied to each of the 16 numbers of z: a positive number comes through about as it was, a negative one is pulled toward zero. This is the one stage that is not a sum of products, and it is what lets a block do more than a single table could.",
    "**Stage 6 of 9, the block's output.** f is h times the second table, 16 by 8, plus its bias list: a list of 8 again. This is what a plain layer would add to the list; with four streams it is handed to each in a share.",
    "**Stage 7 of 9, the shares.** Fs is f handed to each stream: row s of Fs is f times H_post's weight for stream s, {hpost0} for the first and {hpost1} for the others. Every stream gets the same list, scaled by its share.",
    "**Stage 8 of 9, X mixed for the state carried on.** R is H_res times X: the table chosen above times the same X that stage 2 read. Its first number, worked out: {workedR}. The 32 numbers of X sum to {totalX} and those of R to {totalR}: with mHC's table, whose columns sum to 1, the total is kept; with HC's, whose columns sum to {rowsum}, it is {rowsum} times as large.",
    "**Stage 9 of 9, the new state.** X2 is R plus Fs, the mixed state plus the shares of the block's output: the state the third layer will be given. With one stream and H_res the number 1 this would read X plus f, the skip connection of part 1. The same layer by matrix products, outside the pipeline, differs from this by {off}.",
];

pub const SUMMARY: [&str; 4] = [
    "**What the skip keeps.** At the small weight scale, with the blocks replacing the list, the gap between two inputs a hundredth apart was under a millionth of a millionth within a dozen layers and exactly 0 by two dozen: the stack had forgotten its input. With the blocks adding to the list the gap stayed between 1.0 and 1.6 times the gap sent in through all 48 layers. The numbers are quoted above from the run.",
    "**What it does not keep.** At the largest scale (pick it above and the run is quoted) the gap grew past ten times the gap sent in, with the skip and without. A skip keeps the input from being lost; it does not keep the corrections from outgrowing it.",
    "**What four streams add.** A state of four rows, three small tables that mix them, and one block run once per layer on their average. HC and mHC differ in one table, H_res, and in one property of it: whether its rows and columns sum to 1, which is what keeps each stream a weighted average and the streams' total where it was. The paper calls what that keeps the identity mapping; what it does across depth is the next two pages.",
    "**What was not shown.** Anything trained: the mixing tables and the blocks here are fixed by a formula. What a product of 48 mixing tables does, which the third page measures. The paper's own mixing tables, which depend on the layer's input (the fifth page). And anything at the paper's scale: its models have billions of weights and these layers have a few hundred.",
];

pub const NEXT: &str = "The next page of this track, Sinkhorn-Knopp, is about the projection mHC uses: how a table of positive numbers is made to have rows and columns that each sum to 1, step by step, and what such a table can and cannot do to a list. It is being built; the catalog lists what is live.";

pub const CODE: [&str; 3] = [
    "This part is for readers who want to see the code. Nothing above depends on it. Lesson 1 says how to read X_eTaL as the page draws it.",
    "The layer as the program writes it: a pipeline, a stage a line, each an equation written as text except the curve, which is the Activations library's GELU applied as it is. The mixing tables are matrix products; the Sinkhorn projection is a loop of two normalizations, written out here because the function is asked of X_eTaL-ML (FM13) and not yet there.",
    "The whole program, as it runs in your browser each time a scale or a table is picked, then what the page asks of it afterward: the value of every stage of the layer, and the layer drawn as mathematics.",
];

/// Every text of the page, for the list of ideas and the tests.
pub fn all() -> Vec<&'static str> {
    let mut out = vec![LEDE, RATIONALE, NEXT];
    for group in [&QUESTION[..], &BACKGROUND, &PURPOSE, &STACK, &READING, &STREAMS, &STEP, &STAGES, &SUMMARY, &CODE] {
        out.extend_from_slice(group);
    }
    out
}
