//! The page's prose, for new eyes (docs/new-eyes.md): a reader who
//! knows neither machine learning nor X_eTaL, and has read lessons 1
//! to 5. `[shown|id]` marks a word that opens its plain explanation;
//! `**words**` is bold. The counts a sentence quotes are put in by the
//! page from the run; none is typed here. No family is called better
//! than another: the page says what each is, what it guarantees by
//! definition, and what has been measured.

pub const LEDE: &str = "LoRA is one way to change a model. What are the others, and how would you choose? The map: the families of ways to change a layer's weights, sorted by where the change may live and by how it is found; what each one costs, counted by X_eTaL; and what this course has measured so far against the three worries.";

pub const QUESTION: [&str; 2] = [
    "Five lessons have taught one method, [LoRA|lora]: two thin tables beside a [frozen|frozen] [layer|layer], trained by [back-propagation|back-propagation]. A newcomer who asked how to teach a model something new without [forgetting|forgetting], and without too much time and electricity, now knows one answer in detail. **LoRA is one way; what are the others, and how would I choose?**",
    "This page does not choose for you. It lays the ways out as a map, says in plain words what each does to a layer's [weights|weight], shows what each costs when the cost is something that can be counted, and sets each against the three worries, forgetting, time and cost, with what has been measured in this course so far and what the lessons to come will measure.",
];

pub const BACKGROUND: [&str; 3] = [
    "From the lessons before: a layer is a [table|table] W that turns an input list x into an output list W x. [Fine-tuning|fine-tuning] changes what the layer does by changing numbers: W's own, or an [adapter|adapter]'s beside it. [Training|training] by back-propagation finds the [gradient|gradient] of the [loss|loss], which way each number should move, and [Adam|adam] moves it, keeping two [running averages|averages] per number trained. The [rank|rank] r sets how thin LoRA's tables are. The [count|count] of numbers trained follows from the method's definition, so it is analytical; a loss is measured.",
    "Two words this page adds. A **forward pass** is one run of the layer from input to output; back-propagation is the trace back from the error to each weight, which needs the inputs of every step kept until it returns. Some methods use forward passes only. And **a closed-form edit** is a change to W worked out by a formula and applied once, with no training steps at all.",
    "A map needs axes. Any way of changing a layer answers two questions, and the answers can be paired freely: [where may the change live|representation] (in every weight; in two thin tables beside them; in a few numbers scaling tables that are fixed and shared; in one rank-one edit, a change made from a single input list and a single output list, the thinnest adapter there is, rank 1), and [how is the change found|optimizer] (by back-propagation; by [forward passes|forward-pass] alone; by a formula; by an AI proposing and testing). LoRA is one cell of that grid: thin tables, found by back-propagation. Most of the rest of the course fills in other cells. The lessons are built in groups, each group a [saga|saga]: this page names the saga each lesson to come belongs to.",
];

pub const RATIONALE: &str = "A newcomer who has learned one method well tends to see every problem as that method's. The map is drawn now, before the next lessons, so that each lesson to come is met as one cell of a grid already seen, and so that the two questions, where the change lives and how it is found, are kept apart from the start: the same place can be searched by different means, and the same means can search different places. Shared [bases|basis] (tables fixed once and reused, with only a few numbers learned in them), which several later lessons use, are a row of the grid, not a further variation of LoRA.";

pub const PURPOSE: [&str; 2] = [
    "This page is a map and an account book. Its one program counts, for each family whose cost is a count, the numbers trained, the numbers held while training and the numbers stored afterward, on this course's 8 by 8 layer and on one real layer of 4096 by 4096 (a common width in a [7-billion-parameter model|sevenb]); every figure is analytical, and the families whose cost is not a count say so. Nothing on this page is trained.",
    "It is not a comparison of results: no family's loss, forgetting or time is measured here. Where this course has measured something, the map links to the lesson; where it has not, the map says what the definition gives and no more. Methods this course has not yet run are described from their definitions, with the paper that defines each named in the lesson to come; where this page says anything about what has been published, it names the source.",
];

/// One family of the map: its name, what it does in two or three plain
/// sentences, one line of its arithmetic on a layer W, and where it
/// stands in this course.
pub struct Family {
    pub name: &'static str,
    pub said: &'static str,
    pub arithmetic: &'static str,
    pub status: &'static str,
}

pub const FAMILIES: [Family; 6] = [
    Family {
        name: "Change no weight",
        said: "Put the new facts or examples [in the context|context], the text the model reads before it answers, and the weights stay as they are. Nothing is trained and nothing is kept; the next answer starts from scratch, and the context has a size limit. It is the baseline every other family is measured against: what does changing weights buy over not changing them?",
        arithmetic: "y = W x, with the new material inside x",
        status: "The baseline of the comparisons to come (Saga 4's last lesson).",
    },
    Family {
        name: "Change every weight, or chosen ones",
        said: "[Full fine-tuning|full-fine-tuning] moves every number of W by back-propagation and Adam. [Selective fine-tuning|selective] moves a chosen part, some rows, some layers, and leaves the rest. Nothing limits what either can reach; everything moved can change what the layer did before.",
        arithmetic: "W becomes W - lr g: g the gradient of the loss with respect to W (or to its chosen rows), lr the learning rate",
        status: "Measured once here, as the comparison in lesson 5 (full fine-tuning beside LoRA and LoRA-FA, with retention); its own lesson comes in Saga 4.",
    },
    Family {
        name: "Train a small part beside frozen weights",
        said: "LoRA: W frozen, two thin tables A and B trained, their product added to what W gives. Its variations change one thing each: [LoRA-FA|lora-fa] freezes A too; [QLoRA|qlora] stores the frozen W at 4 [bits a number|bits]; [DoRA|dora] splits each column of W into its size, one number, and its direction, the column scaled to size 1, adapts the direction and trains the size apart; [VeRA|vera] freezes both tables, grown from a [seed|seed] and shared across layers, and trains two short lists that scale them (so it keeps LoRA's shape with fixed tables; the grid below puts it in a row of its own).",
        arithmetic: "y = W x + s B (A x); VeRA: y = W x + b * (B (d * (A x))): A and B frozen and shared, b and d the two trained lists, each * scaling the list beside it number by number",
        status: "Lessons 1 to 5 here are LoRA and LoRA-FA. QLoRA, DoRA, VeRA and others to be chosen when that group is planned are Saga 2, each a lesson, then all on one task.",
    },
    Family {
        name: "Learn without back-propagation",
        said: "[Learning without back-propagation|weight-space]: nudge the weights by a random amount, run forward, nudge them the other way, run forward, and step toward whichever loss was lower. Only forward passes: no gradient, no inputs kept for a trace back, and the loss need not be a smooth formula, any score will do (how many tests a program passed, say). It learns from two losses at a time, so by its definition it takes many passes; how many, against back-propagation, is a thing to measure.",
        arithmetic: "g is estimated by ((L(W + e z) - L(W - e z)) / 2 e) z: L the loss, z a random table the size of W, e a small number; many z at once for evolution strategies",
        status: "Saga 4, after the shared-subspace lessons: MeZO and evolution strategies first, then the methods that choose where to nudge.",
    },
    Family {
        name: "Edit in closed form",
        said: "A [closed-form edit|closed-form] changes W by a formula, once: to make the layer give output v for input k, add a rank-one change worked out from k and v. No training steps, no gradient, exact for k. What the same change does to other inputs is the whole question; Saga 5 measures it at toy scale and reads the papers' evaluations.",
        arithmetic: "W becomes W + (v - W k) k' / (k' k), k' being k turned on its side: then the new W times k is v",
        status: "Saga 5 (editing a fact, many edits, a stream of them with drift measured).",
    },
    Family {
        name: "Let an AI do the editing",
        said: "[An AI editing weights|agent-edit]: a language model is given the weights, or a way to probe them, proposes a change, applies it to a copy and tests it, keeping the change only if a check passes. The arithmetic is one of the families above; what is new is who chooses the change. Published so far (as of November 2026): one small demonstration, a blog post from MIT CSAIL in which a language model found a direction in a small model's weights and subtracted it, removing a defect; no paper and no replication yet.",
        arithmetic: "W becomes W - a u (u' W): u a direction (a list) chosen by the AI, u' the same list turned on its side, a a strength; kept if a score improves and a keep-intact check holds",
        status: "Saga 5, after the closed-form edits it would use; what may be called an AI on such a page is settled before it is built.",
    },
];

/// The grid's axes: where the change may live (rows) and how it is found (columns).
pub const LIVES: [&str; 4] = ["in every weight", "in two thin tables beside frozen weights", "in a few numbers scaling fixed, shared tables", "in one rank-one edit"];
pub const FOUND: [&str; 4] = ["by back-propagation", "by forward passes alone", "by a formula", "by an AI proposing and testing"];

/// A cell of the grid: a lesson, live (its slug) or to come (its saga).
pub struct Cell {
    pub what: &'static str,
    pub slug: &'static str,
    pub saga: &'static str,
}

/// The grid, row by row (LIVES) and column by column (FOUND); a cell
/// may hold several lessons, or none.
pub const GRID: [[&[Cell]; 4]; 4] = [
    [
        &[Cell { what: "full fine-tuning beside the adapters, lesson 5", slug: "lora-vs-fa", saga: "" }, Cell { what: "how learning works; full and selective fine-tuning", slug: "", saga: "Saga 4" }, Cell { what: "a model that writes its own training recipe (SEAL), which back-propagation then runs", slug: "", saga: "Saga 5" }],
        &[Cell { what: "two forward passes at a time (MeZO); many nudges at once (evolution strategies); learning from a score; the warning that the pick of many random tries looks like learning", slug: "", saga: "Saga 4" }],
        &[Cell { what: "adding and subtracting whole trained changes (task vectors, merging)", slug: "", saga: "Saga 4" }],
        &[],
    ],
    [
        &[Cell { what: "LoRA, lessons 1 to 4", slug: "lora-training", saga: "" }, Cell { what: "LoRA-FA, lesson 5", slug: "lora-vs-fa", saga: "" }, Cell { what: "QLoRA, DoRA and the variations compared", slug: "", saga: "Saga 2" }],
        &[Cell { what: "the random nudges made thin, as LoRA's tables are (EGGROLL)", slug: "", saga: "Saga 4" }],
        &[],
        &[],
    ],
    [
        &[Cell { what: "VeRA, lesson 9", slug: "", saga: "Saga 2" }, Cell { what: "shared and separate bases; which directions to keep; tasks in sequence", slug: "", saga: "Saga 3" }],
        &[Cell { what: "the search run in a shared reduced space", slug: "", saga: "Saga 4" }],
        &[],
        &[],
    ],
    [
        &[],
        &[],
        &[Cell { what: "editing one fact (ROME); many at once (MEMIT, AlphaEdit); a stream of edits", slug: "", saga: "Saga 5" }],
        &[Cell { what: "a direction found and removed; edits kept only if a check passes; an AI's own run", slug: "", saga: "Saga 5" }],
    ],
];

pub const GRID_NOTE: [&str; 2] = [
    "Each cell: a lesson live here (a link) or to come (its saga). The names in brackets (MeZO, SEAL, ROME and the rest) are the papers' names for methods the lessons to come will explain; here they are only labels. An empty cell is a pairing no lesson is planned for; it is not impossible, only not on the road. Changing no weight is outside the grid: it changes nothing, and is the baseline the grid is measured against. The six families above are not the four rows: a family groups methods by what a newcomer sees them do, a row by where the change lives, so family 3 spans rows 2 and 3 (VeRA keeps LoRA's shape but fixes and shares its tables, so Saga 2 teaches it among the variations and the grid gives it a row of its own) and family 6 spans rows 1 and 4.",
    "The third row is not one more variation of LoRA. On the 8 by 8 layer, with A fixed, every update s B A has rows that are mixes of A's 4 rows: a flat region in the space of lists of 8, a [subspace|subspace], as lesson 5 showed; let A move too and the rows can be any lists at all, so no fixed region holds them. Fixing the tables, as LoRA-FA fixes A and VeRA fixes both, is what makes the region fixed, before training starts. Sharing that region across layers or tasks is a choice with costs of its own (a shared basis can be too narrow, and moving it moves every task learned on it), which Saga 3 measures rather than assumes.",
];

/// The three worries, a column each, for each family: what its
/// definition gives, what this course has measured (with the lesson's
/// slug, "" for none), and what is still to be measured.
pub struct Worry {
    pub gives: &'static str,
    pub measured: &'static str,
    pub slug: &'static str,
    pub to_come: &'static str,
}

pub struct Worries {
    pub forgetting: Worry,
    pub time: Worry,
    pub cost: Worry,
}

pub const WORRIES: [Worries; 6] = [
    Worries {
        forgetting: Worry { gives: "nothing is forgotten: no weight changes", measured: "", slug: "", to_come: "the baseline in Saga 4's comparison" },
        time: Worry { gives: "no training; each answer reads the added material again", measured: "", slug: "", to_come: "" },
        cost: Worry { gives: "not a count: nothing trained, nothing stored; the context's length is the limit", measured: "", slug: "", to_come: "" },
    },
    Worries {
        forgetting: Worry { gives: "no guarantee: every moved number can change old outputs", measured: "the retention loss of full fine-tuning beside LoRA's and LoRA-FA's on one task (lesson 5)", slug: "lora-vs-fa", to_come: "its own lesson, with the old task's loss before and after" },
        time: Worry { gives: "one forward and one backward pass per step; the inputs of every layer kept for the backward pass", measured: "", slug: "", to_come: "steps and passes counted in Saga 4" },
        cost: Worry { gives: "every number of W trained, three more held per number under Adam, W stored", measured: "", slug: "", to_come: "" },
    },
    Worries {
        forgetting: Worry { gives: "no guarantee: the change reaches every input through W x + s B (A x); it is small by construction, not kept away from old inputs", measured: "the retention loss of LoRA and LoRA-FA on one task: every method moved the old outputs (lesson 5)", slug: "lora-vs-fa", to_come: "each variation on one task with retention measured (Saga 2)" },
        time: Worry { gives: "one forward and one backward pass per step, as full fine-tuning; fewer numbers to update", measured: "", slug: "", to_come: "" },
        cost: Worry { gives: "the counts above, by rank: far fewer numbers trained and held on a wide layer", measured: "the losses reached at each rank, beside its count (lesson 4)", slug: "rank-and-scaling", to_come: "bytes stored under QLoRA (Saga 2)" },
    },
    Worries {
        forgetting: Worry { gives: "no guarantee: every weight may move", measured: "", slug: "", to_come: "the old task's loss beside back-propagation's (Saga 4)" },
        time: Worry { gives: "two forward passes per estimate, no backward pass, nothing kept for one; many estimates", measured: "", slug: "", to_come: "forward passes counted against the loss reached, and the estimate's direction against the true gradient" },
        cost: Worry { gives: "the counts above: no gradient and no averages held; the random nudge is regrown from a seed", measured: "", slug: "", to_come: "" },
    },
    Worries {
        forgetting: Worry { gives: "exact for the edited input; no guarantee for any other", measured: "", slug: "", to_come: "what one edit disturbs; whether a stream of edits drifts (Saga 5)" },
        time: Worry { gives: "no training steps: one formula, which may need a matrix solved", measured: "", slug: "", to_come: "" },
        cost: Worry { gives: "the counts above: nothing trained; two lists stored, or W rewritten", measured: "", slug: "", to_come: "" },
    },
    Worries {
        forgetting: Worry { gives: "whatever its check tests for, and no more", measured: "", slug: "", to_come: "edits with and without the check that keeps or discards them (Saga 5)" },
        time: Worry { gives: "not fixed: as many trials and evaluations as the AI makes", measured: "", slug: "", to_come: "trials counted" },
        cost: Worry { gives: "not a count: the edit's own cost is its family's; the AI's is its runs", measured: "", slug: "", to_come: "" },
    },
];

/// The curriculum: each lesson, live (its slug) or to come (its saga).
pub struct Lesson {
    pub number: &'static str,
    pub title: &'static str,
    pub slug: &'static str,
    pub saga: &'static str,
}

pub const CURRICULUM: [Lesson; 19] = [
    Lesson { number: "1", title: "LoRA, hello", slug: "lora-hello", saga: "" },
    Lesson { number: "2", title: "The LoRA lab", slug: "lora-lab", saga: "" },
    Lesson { number: "3", title: "LoRA training", slug: "lora-training", saga: "" },
    Lesson { number: "4", title: "Rank and scaling", slug: "rank-and-scaling", saga: "" },
    Lesson { number: "5", title: "LoRA beside LoRA-FA", slug: "lora-vs-fa", saga: "" },
    Lesson { number: "6", title: "The landscape (this page)", slug: "landscape", saga: "" },
    Lesson { number: "7", title: "QLoRA", slug: "", saga: "Saga 2" },
    Lesson { number: "8", title: "DoRA", slug: "", saga: "Saga 2" },
    Lesson { number: "9", title: "VeRA", slug: "", saga: "Saga 2" },
    Lesson { number: "10", title: "The variations compared", slug: "", saga: "Saga 2" },
    Lesson { number: "11", title: "Adapters in a mixture of experts", slug: "", saga: "Saga 2" },
    Lesson { number: "", title: "What is a subspace? Shared and separate; which directions; tasks in sequence", slug: "", saga: "Saga 3" },
    Lesson { number: "", title: "How learning works; full fine-tuning", slug: "", saga: "Saga 4" },
    Lesson { number: "", title: "Forward passes only; learning from a score; the warning about picking the best of many tries", slug: "", saga: "Saga 4" },
    Lesson { number: "", title: "Every method of its saga compared; the other alternatives", slug: "", saga: "Saga 4" },
    Lesson { number: "", title: "A direction found and removed", slug: "", saga: "Saga 5" },
    Lesson { number: "", title: "Editing a fact; many edits", slug: "", saga: "Saga 5" },
    Lesson { number: "", title: "Edits kept only if a check passes; a model writing its own training recipe", slug: "", saga: "Saga 5" },
    Lesson { number: "", title: "An AI's own run on the toy layer", slug: "", saga: "Saga 5" },
];

pub const ACCOUNT: [&str; 3] = [
    "The counts, computed by X_eTaL for the two layers: this course's 8 by 8 at rank 4, and a real layer of 4096 by 4096 at rank 16. Every figure is analytical, marked A: it follows from the family's definition, and predicts no loss and no runtime. **Trained**: the numbers the method moves. **Held while training**: what is kept beside the weights for the numbers trained, three per number under back-propagation with Adam (the gradient and Adam's two [running averages|averages]), none for a forward-only method (its random nudge is regrown from a [seed|seed], one whole number, and two losses are compared), none for an edit. **Stored afterward**: the change kept apart from W; an adapter [merged|merge] into W is a d by d table again.",
    "Two families are missing from the table because their cost is not a count: changing no weight trains and stores nothing (the limit is the context's length), and an AI editing weights costs its trials, which are not fixed by any definition; the edit it makes has its own family's cost.",
    "What the counts say, in sentences written from the program's output (analytical, like the table), with the small layer's counts worked out in full.",
];

pub const SUMMARY: [&str; 4] = [
    "**The map.** Six families of ways to change a layer, and under them two questions kept apart: where the change may live, and how it is found. LoRA is one cell, thin tables found by back-propagation; the lessons to come fill others, and the shared bases of Saga 3 are a row, not a variation.",
    "**The account.** On a wide layer an adapter trains and holds a small share of the layer's numbers, and the forward-only and closed-form families hold no gradient and no averages; on this course's 8 by 8 layer a rank-4 adapter is as large as the layer. The counts are analytical and predict no loss, no forgetting and no runtime: that is what the lessons measure.",
    "**The worries.** By definition, only changing no weight guarantees nothing is forgotten; every other family's guarantee is partial (a closed-form edit is exact for its one input) or absent. Measured so far: on lesson 5's task every method that changed weights moved the old outputs. Time and cost appear here only as analytical counts, not measurements; passes and steps are measured when Saga 4 runs them.",
    "**What was not shown.** No method's loss or forgetting beyond what lessons 3 to 5 measured; no real model; nothing about the families this course has not yet run beyond their definitions. The map says where each lesson will sit; the lessons say what was found.",
];

pub const NEXT: &str = "This is the last lesson of the first group. The next group, Saga 2, is the variations of LoRA, each a lesson and then all of them on one task at equal ranks and at equal budgets, with retention measured for each; before it, a pass over the parts every lesson page shares. The catalog lists what is live.";

pub const CODE: [&str; 2] = [
    "This part is for readers who want to see the code. Nothing above depends on it. The program defines nothing of its own: it takes LoRA's count from the LoRA library and writes the other families' counts out, as the definitions give them.",
    "The whole program, as it runs in your browser.",
];

/// Every text of the page, for the list of ideas and the tests.
pub fn all() -> Vec<&'static str> {
    let mut out = vec![LEDE, RATIONALE, NEXT];
    for group in [&QUESTION[..], &BACKGROUND, &PURPOSE, &GRID_NOTE, &ACCOUNT, &SUMMARY, &CODE] {
        out.extend_from_slice(group);
    }
    for f in &FAMILIES {
        out.push(f.said);
        out.push(f.status);
    }
    out
}
