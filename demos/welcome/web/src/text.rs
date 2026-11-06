//! The page's prose: a greeting for someone new, who knows neither
//! machine learning nor programming (docs/new-eyes.md). `[shown|id]`
//! marks a word that opens its plain explanation; `**words**` is bold.
//! Revised after a cold reading: the greeting is short and says what
//! to do; the words to try are words this page uses; every letter is
//! named before it appears; nothing is claimed that the page does not
//! do.

pub const LEDE: &str = "New here? You need no machine learning and no programming. In five minutes this page shows how the lessons work, so that nothing in the first one is a surprise.";

pub const QUESTION: [&str; 2] = [
    "Welcome. These lessons are about one practical question: how do you teach an [AI model|model] something new, such as your own subject or your own way of working, without it forgetting what it already knows and without days of computing?",
    "They are for someone who has never studied the subject and may never have written a program. You need to be able to multiply and add numbers, including negative ones and ones with decimals. That is all.",
];

pub const BACKGROUND: &str = "Each lesson asks a question, shows one small calculation with every number in view, and then says plainly what that showed and what it did not. The numbers are worked out in your browser when you open the page.";

pub const PURPOSE: &str = "This page lets you try the seven things the lessons use, on one small calculation chosen to practice on. You are not expected to understand that calculation: what it is, and why anyone would want it, is the first lesson. Start with part 1, just below.";

pub const WORDS: [&str; 2] = [
    "A word with a dotted underline is explained on the spot. Try one: [list|list], or [table|table], or [stage|stage]. Click it again to close it.",
    "Some have a worked example inside, to open if you want it: try [multiplying a table by a list|times]. On a keyboard, Tab reaches a word and Enter opens it.",
];

pub const GUESS: [&str; 3] = [
    "The lessons often ask you to guess before they show you. Nothing is scored. Guessing first is what makes the answer stay with you. Here is one.",
    "A table works on a list one row at a time, and each row gives one number. In the calculation of part 3, a list of 4 numbers goes through a table that has only one row. How many numbers do you expect to come out?",
    "Each row of a table gives one number, and this table has a single row.",
];

pub const STEPS: [&str; 4] = [
    "Here is the practice calculation, in four [stages|stage], named x, h, u and y. A [list|list] of four numbers, x, goes in. Stage h boils x down to one number. Stage u spreads that one number back out to four. Stage y adds that list, counted twice, to what a larger [table|table] works out from x.",
    "The four circles are the four stages. Press **Next** to move on one stage, **Back** to go to the one before, **Start** to return to the first, or **Play** to watch all four in turn. The stage being worked out is ringed, and a dot travels to it from each stage it uses.",
    "Under the circles, each stage's numbers are drawn as squares, one square for each number, with the number written in it: red for a positive number, blue for a negative one, pale for one close to zero. A stage not reached yet shows question marks. The squares being worked out have a solid outline; the ones they are worked out from have a dashed one.",
    "Going back just shows the earlier stage again. Nothing is worked out twice.",
];

/// What each stage does, in plain words (the input first).
pub const STAGES: [&str; 4] = [
    "**Stage 1 of 4, x.** The input: a list of 4 numbers. Nothing has been worked out yet. Press Next.",
    "**Stage 2 of 4, h.** The 4 numbers of x are boiled down to 1: each is multiplied by the matching number of a small table called A, and the results are added. (Part 5 writes that sum out.)",
    "**Stage 3 of 4, u.** That one number is spread back out to 4, by multiplying it by each number of a second small table, called B.",
    "**Stage 4 of 4, y.** The result. A larger table, called W, [works out its own list of 4 from x|times], and the list u, counted twice, is added to it.",
];

pub const PICTURES: [&str; 2] = [
    "The squares show each number to two decimals. To see exactly which number a square is, click it, or press Tab until a picture is outlined and then use the arrow keys: the line under that picture gives the full number and where it sits.",
    "Try it on the three tables the calculation uses: A, B and W.",
];

pub const SUM: [&str; 2] = [
    "No number in a lesson comes from nowhere. Here is stage h in full: each number of the table A times the matching number of the list x, then the total. A negative number is written in brackets.",
    "That total is the one number of stage h in part 3.",
];

pub const CHANGE: [&str; 2] = [
    "In a lesson you will change settings yourself, and the page will tell you what changed, using the numbers from before and after. It compares. It does not tell you which is better, because that depends on what you want.",
    "Here is one change, made for you. In stage y the list u was counted twice. Counted four times instead:",
];

pub const DEPTHS: &str = "A lesson can be read at three depths. Beginner is where you are. Explorer adds a line in part 3 saying which stages each stage uses. Advanced also adds the formula in part 5 and, at the end of the page, the program that worked everything out. Try switching: the stage you were on in part 3 stays where it was.";

pub const RATIONALE: &str = "The lessons are built this way because it is easy to nod along to an explanation and much harder to say what a calculation will do before you see it. So they have you guess, then watch each stage, then read the numbers yourself, with every unfamiliar word explained where it is used.";

pub const SUMMARY: &str = "You have now tried or seen the seven things: words that open, a guess first, a calculation taken one stage at a time, pictures of the numbers, a sum written out, a change put into words, and three depths. You have not been taught the calculation itself. That is next.";

pub const NEXT: &str = "The first lesson is called LoRA hello. LoRA is the name of the first method these lessons explain, and the calculation you just stepped through is a small piece of it. The lesson says what it is for: how a model can learn something new while its own numbers stay exactly as they are.";

/// Every text of the page, for the list of ideas and the tests.
pub fn all() -> Vec<&'static str> {
    let mut out = vec![LEDE, BACKGROUND, PURPOSE, DEPTHS, RATIONALE, SUMMARY, NEXT];
    for group in [&QUESTION[..], &WORDS, &GUESS, &STEPS, &STAGES, &PICTURES, &SUM, &CHANGE] {
        out.extend_from_slice(group);
    }
    out
}
