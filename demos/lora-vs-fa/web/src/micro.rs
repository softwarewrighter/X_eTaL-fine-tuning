//! The model: the demo's program (lora-vs-fa.xtl) run at the rank and
//! the examples the page sets, its two experiments read back (T, its
//! spectrum, the two fits and what each learned, the remainders off
//! the frozen row space, the counts; the three methods' records on the
//! learning task), and what they say, in sentences built from the
//! numbers (lesson::narrate: compared, never judged). Nothing here
//! trains, projects or computes a loss: every number is X_eTaL's.

use lesson::arithmetic::{commas, show};
use lesson::narrate::moved;
use microscope::run::{numbers, output_pictures};

/// The command line's program.
pub const PROGRAM: &str = include_str!("../../lora-vs-fa.xtl");

/// The lines the program prints: T; the singular values, the best at
/// each rank, the check; the two fits, the two updates, the
/// remainders, the floor beside the best, the counts; the three
/// records, the sizes of the three changes.
pub const PRINTED: usize = 15;

/// The ranks the page offers, and the examples trained on.
pub const RANKS: usize = 8;
pub const EXAMPLES: (usize, usize) = (2, 8);

/// The record: chunks of every steps (the program's).
pub const CHUNKS: usize = 30;
pub const EVERY: usize = 10;

/// What the page sets.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub rank: usize,
    pub n: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { rank: 4, n: 8 }
    }
}

/// The program with the settings put in its data: two lines change.
pub fn with(s: Settings) -> String {
    let d = Settings::default();
    PROGRAM
        .replacen(&format!("k := {}\n", d.rank), &format!("k := {}\n", s.rank), 1)
        .replacen(&format!("n := {}\n", d.n), &format!("n := {}\n", s.n), 1)
}

/// One method's fit of T (Experiment A).
#[derive(Clone, Debug, PartialEq)]
pub struct Fit {
    /// The loss after each chunk, the start first (measured).
    pub losses: Vec<f64>,
    /// The update s B A it learned, 8 x 8 by rows (measured).
    pub update: Vec<f64>,
    /// The size of the update's remainder off the frozen A's row space
    /// (measured).
    pub remainder: f64,
    /// The numbers it trains, and the numbers Adam holds for them
    /// (analytical).
    pub trained: usize,
    pub held_by_adam: usize,
}

/// One method's record on the learning task (Experiment B): per
/// example, trained on, held out, old; a row per record, the start
/// first (measured).
#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub rows: Vec<[f64; 3]>,
    /// The size of its change to the layer after the steps (measured).
    pub change: f64,
}

impl Record {
    pub fn first(&self) -> [f64; 3] {
        self.rows[0]
    }
    pub fn last(&self) -> [f64; 3] {
        self.rows[self.rows.len() - 1]
    }
}

/// What the run gave.
#[derive(Clone, Debug, PartialEq)]
pub struct Run {
    pub settings: Settings,
    /// T, 8 x 8 by rows.
    pub t: Vec<f64>,
    /// T's singular values, and the lowest loss possible at each rank 1 .. 8
    /// (analytical, from the spectrum).
    pub singular: Vec<f64>,
    pub lowest: Vec<f64>,
    /// The spectrum code's check against known singular values.
    pub check: f64,
    pub lora: Fit,
    pub fa: Fit,
    /// The size of T's own remainder off the frozen A's row space.
    pub t_remainder: f64,
    /// LoRA-FA's floor (analytical), and the lowest loss possible at the rank set.
    pub floor: f64,
    pub lowest_here: f64,
    /// The learning task's records: LoRA, LoRA-FA, full fine-tuning.
    pub learn_lora: Record,
    pub learn_fa: Record,
    pub learn_full: Record,
    /// The four charts (SVG documents X_eTaL drew): the fit, then the
    /// three losses of the learning task.
    pub charts: Vec<String>,
}

/// Run the program at settings `s`.
pub fn run(s: Settings) -> Result<Run, String> {
    let (out, charts) = output_pictures(&with(s), PRINTED)?;
    let line = |i: usize, n: usize| numbers::<f64>(&out[i], n);
    let t = line(0, 64)?;
    let singular = line(1, 8)?;
    let lowest = line(2, 8)?;
    let check = line(3, 1)?[0];
    let (fit_l, fit_f) = (line(4, CHUNKS + 1)?, line(5, CHUNKS + 1)?);
    let (d1, d2) = (line(6, 64)?, line(7, 64)?);
    let rem = line(8, 3)?;
    let floors = line(9, 2)?;
    let counts = line(10, 4)?;
    let record = |i: usize, change: f64| -> Result<Record, String> {
        let v = line(i, 3 * (CHUNKS + 1))?;
        Ok(Record { rows: v.chunks(3).map(|c| [c[0], c[1], c[2]]).collect(), change })
    };
    let sizes = line(14, 3)?;
    if charts.len() != 4 {
        return Err(format!("the program drew {} charts, not 4", charts.len()));
    }
    let count = |v: f64| v as usize;
    Ok(Run {
        settings: s,
        t,
        singular,
        lowest,
        check,
        lora: Fit { losses: fit_l, update: d1, remainder: rem[0], trained: count(counts[0]), held_by_adam: count(counts[2]) },
        fa: Fit { losses: fit_f, update: d2, remainder: rem[1], trained: count(counts[1]), held_by_adam: count(counts[3]) },
        t_remainder: rem[2],
        floor: floors[0],
        lowest_here: floors[1],
        learn_lora: record(11, sizes[0])?,
        learn_fa: record(12, sizes[1])?,
        learn_full: record(13, sizes[2])?,
        charts,
    })
}

/// A number to 3 decimals, as the page quotes it.
pub fn r3(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

pub fn q(v: f64) -> String {
    show(r3(v))
}

/// How a measured value moved, quoted to 3 decimals.
fn went(what: &str, a: f64, b: f64) -> String {
    moved(what, r3(a), r3(b))
}

/// Whether two numbers are the same to 3 decimals, as the page shows them.
pub fn same(a: f64, b: f64) -> bool {
    r3(a) == r3(b)
}

/// The three methods of Experiment B, as the page names them.
pub const METHODS: [&str; 3] = ["LoRA", "LoRA-FA", "full fine-tuning"];

impl Run {
    pub fn steps(&self) -> usize {
        CHUNKS * EVERY
    }

    pub fn records(&self) -> [&Record; 3] {
        [&self.learn_lora, &self.learn_fa, &self.learn_full]
    }

    /// The last loss of each fit.
    pub fn end(&self, fa: bool) -> f64 {
        let f = if fa { &self.fa } else { &self.lora };
        f.losses[f.losses.len() - 1]
    }

    /// The answer to Experiment A's guess, from the two fits' last
    /// losses: whether LoRA-FA came to the same loss as LoRA (to 3 decimals).
    pub fn fit_same(&self) -> bool {
        same(self.end(true), self.end(false))
    }

    pub fn fit_answer(&self) -> String {
        let (l, f) = (self.end(false), self.end(true));
        if self.fit_same() {
            format!("Read from the two runs: after {} steps both came to {} (measured).", self.steps(), q(l))
        } else {
            format!("Read from the two runs: after {} steps LoRA's loss is {} and LoRA-FA's {} (measured); LoRA-FA's floor, the loss of T's projection onto the frozen A's row space, is {} (analytical).", self.steps(), q(l), q(f), q(self.floor))
        }
    }

    /// Experiment A told from its numbers.
    pub fn fit_said(&self) -> Vec<String> {
        let k = self.settings.rank;
        let start = self.lora.losses[0];
        let at_best = |v: f64| if same(v, self.lowest_here) { "at that lowest, to 3 decimals" } else { "above it" };
        let at_floor = if same(self.end(true), self.floor) { "at its floor, to 3 decimals" } else { "above its floor" };
        vec![
            format!("At rank {k}, fitting T for {} steps. {} (measured). The lowest loss any update of rank {k} can reach is {} (analytical, from T's singular values); LoRA's end is {}.", self.steps(), went("LoRA's loss", start, self.end(false)), q(self.lowest_here), at_best(self.end(false))),
            format!("{} (measured). Its update s B A has its rows in the frozen A's row space, so the closest it can come to T is T's projection onto that subspace, at a loss of {} (analytical); its end is {}.", went("LoRA-FA's loss", start, self.end(true)), q(self.floor), at_floor),
            format!("The remainders off the row space, the size of what a projection onto it leaves out: LoRA-FA's update {}, LoRA's update {}, T itself {} (measured). {}", q(self.fa.remainder), q(self.lora.remainder), q(self.t_remainder), if r3(self.fa.remainder) == 0.0 { "LoRA-FA's is 0 to 3 decimals: its update did not leave the subspace." } else { "LoRA-FA's remainder was expected to be 0: something is off in this run." }),
            format!("Cost (analytical): LoRA trains {} numbers and Adam holds {} for them; LoRA-FA trains {} and Adam holds {}.", commas(self.lora.trained), commas(self.lora.held_by_adam), commas(self.fa.trained), commas(self.fa.held_by_adam)),
        ]
    }

    /// The method whose old-examples loss after the steps is lowest,
    /// and the one whose is highest (positions in METHODS).
    pub fn old_extremes(&self) -> (usize, usize) {
        let old: Vec<f64> = self.records().iter().map(|r| r.last()[2]).collect();
        let lowest = (0..3).fold(0, |a, b| if old[b] < old[a] { b } else { a });
        let highest = (0..3).fold(0, |a, b| if old[b] > old[a] { b } else { a });
        (lowest, highest)
    }

    /// The answer to Experiment B's guess, from the old-examples column.
    pub fn old_answer(&self) -> String {
        let (lo, hi) = self.old_extremes();
        let old: Vec<String> = self.records().iter().map(|r| q(r.last()[2])).collect();
        format!("Read from the old examples' column after {} steps: LoRA {}, LoRA-FA {}, full fine-tuning {} (measured, per example; 0 before any training). The lowest is {}'s, the highest {}'s.", self.steps(), old[0], old[1], old[2], METHODS[lo], METHODS[hi])
    }

    /// Experiment B told from its numbers: each method's three losses,
    /// then the comparison.
    pub fn learn_said(&self) -> Vec<String> {
        let n = self.settings.n;
        let steps = self.steps();
        let mut out: Vec<String> = self
            .records()
            .iter()
            .zip(METHODS)
            .map(|(r, name)| {
                let (a, b) = (r.first(), r.last());
                let held = if b[1] > a[1] { "The held-out loss ended above where it started: this fit the examples trained on at a cost on the ones held out." } else { "The held-out loss ended below where it started." };
                format!("{name}, after {steps} steps on {n} examples (measured, per example): {}; {}; {}. {held} The size of its change to the layer is {}.", went("the loss on the examples trained on", a[0], b[0]), went("the loss on the examples held out", a[1], b[1]), went("the loss on the old examples", a[2], b[2]), q(r.change))
            })
            .collect();
        let (lo, hi) = self.old_extremes();
        let old: Vec<f64> = self.records().iter().map(|r| r.last()[2]).collect();
        out.push(format!(
            "The old examples: every method moved their outputs (measured; the examples ask the layer for W + T, and the old outputs were W's). The loss there after the steps, the retention measure, is lowest for {} ({}) and highest for {} ({}). The layer with LoRA's adapter merged in, W + s B A, changed the old outputs as the moved W of full fine-tuning did: here the adapter was not, by itself, a guard of what the layer did before.",
            METHODS[lo], q(old[lo]), METHODS[hi], q(old[hi])
        ));
        out.push(format!(
            "Cost (analytical): LoRA trains {} numbers, LoRA-FA {}, full fine-tuning all 64 of W; Adam holds two more per number trained: {}, {} and 128.",
            commas(self.lora.trained), commas(self.fa.trained), commas(self.lora.held_by_adam), commas(self.fa.held_by_adam)
        ));
        out
    }
}
