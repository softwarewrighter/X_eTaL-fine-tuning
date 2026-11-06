//! The page's prose, for new eyes (docs/new-eyes.md). `[shown|id]`
//! marks a word that opens its plain explanation.

pub const LEDE: &str = "The parts every lesson here is built from, each shown working on one small computation. A page for looking the parts over, not a lesson.";

pub const QUESTION: &str = "A lesson has to explain a calculation to someone who has never seen one like it. What does it have to work with? This page lays the pieces out: words that open a plain explanation, a question to answer first, a calculation you can step through forward and back, pictures of the numbers, a sum written out, and a change put into words.";

pub const BACKGROUND: &str = "The calculation used throughout is the one from the first lesson: an input x goes through a small [adapter|adapter] (two thin [tables|table], A and B) and through a [frozen|frozen] table W, and the two results are added. Here it is written as four steps in a row, called stages: the input; the squeeze, A times x; the spread, B times that; and the output.";

pub const RATIONALE: &str = "Every number on this page comes from one run of a short program in [X_eTaL|xetal]. The program also reports the value of every stage, and that record is what the stepper moves through: going back, or playing it again, runs nothing and changes nothing. The page only arranges what the run produced.";

pub const PURPOSE: &str = "It is a gallery for checking that each part works, reads clearly and can be used with a keyboard or by touch. It teaches nothing new and trains nothing. The first lesson is LoRA hello.";

pub const PREDICT: [&str; 2] = [
    "The input x is a list of 4 numbers. It goes through A, which has one row. How many numbers come out of that stage?",
    "A table gives one number for each of its rows, and A has a single row.",
];

/// What each stage does, in plain words (the data first).
pub const STAGES: [&str; 4] = [
    "The input, x: a list of 4 numbers. Nothing is computed yet.",
    "The squeeze, h = A x: the one row of A [times|times] x gives a single number. The 4 numbers of the input have become 1.",
    "The spread, u = B h: that one number times each number of B gives 4 numbers again, one for each output.",
    "The output, y = W x + s u: what the frozen table gives for x, plus the spread list times [the strength|strength] s.",
];

pub const STEPPING: &str = "Press Next, or Play. The stage being worked out is outlined solid; what it reads from is outlined with dashes. Back takes you to the stage before: nothing is run again.";

pub const SUM: &str = "The first number of the output takes the first row of W: each of its numbers times the matching number of x, added up. That is the part of y the frozen table gives; the adapter's part is added to it.";

pub const CHANGE: &str = "When a setting moves, a lesson says what changed, from the numbers of the two runs. It reports and compares; it does not call a result better.";

pub const MODES: &str = "The same page at three depths. Switching changes what is shown, not what was computed: the step you are on stays where it is.";

pub const SUMMARY: &str = "Six parts, all of them only arranging what one run of the program printed: words that open, a question first, a stepper over a recorded trace, heat maps with a legend and an inspector, a sum written out, a change in words. Each works by keyboard and by touch.";

pub const NEXT: &str = "The lessons are built from these. Start with LoRA hello.";

/// Every text of the page, for the list of ideas and the tests.
pub fn all() -> Vec<&'static str> {
    let mut out = vec![LEDE, QUESTION, BACKGROUND, RATIONALE, PURPOSE, STEPPING, SUM, CHANGE, MODES, SUMMARY, NEXT];
    out.extend_from_slice(&PREDICT);
    out.extend_from_slice(&STAGES);
    out
}
