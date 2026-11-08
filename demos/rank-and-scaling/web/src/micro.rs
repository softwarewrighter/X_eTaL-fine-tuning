//! The model: the demo's program (rank-and-scaling.xtl) run at the
//! alpha and the steps the page sets, its table (a row per rank), the
//! size of T, the losses before training and its three charts read
//! back; and what the table says, in sentences built from its numbers
//! (lesson::narrate: compared, never judged). Nothing here trains or
//! computes a loss: every number is X_eTaL's.

use lesson::arithmetic::{commas, show};
use lesson::narrate::moved;
use microscope::run::{numbers, output_pictures};

/// The command line's program.
pub const PROGRAM: &str = include_str!("../../rank-and-scaling.xtl");

/// The lines the program prints: the table (8 rows), the size of T,
/// the losses before training.
pub const PRINTED: usize = 10;

/// The ranks swept, and the columns of a row.
pub const RANKS: usize = 8;
pub const COLS: usize = 9;

/// The alphas and the step counts the page offers.
pub const ALPHAS: [f64; 5] = [1.0, 2.0, 4.0, 8.0, 16.0];
pub const STEPS: [usize; 4] = [10, 30, 100, 300];

/// What the page sets.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub alpha: f64,
    pub steps: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { alpha: 4.0, steps: 100 }
    }
}

/// A number as the program writes it (a float keeps its point).
fn float(v: f64) -> String {
    if v.fract() == 0.0 { format!("{v:.1}") } else { v.to_string() }
}

/// The program with the settings put in its data: two lines change.
pub fn with(s: Settings) -> String {
    let d = Settings::default();
    PROGRAM
        .replacen(&format!("alpha := {}\n", float(d.alpha)), &format!("alpha := {}\n", float(s.alpha)), 1)
        .replacen(&format!("steps := {}\n", d.steps), &format!("steps := {}\n", s.steps), 1)
}

/// One rule for the strength, with its row's measurements.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rule {
    /// The strength s (analytical).
    pub s: f64,
    /// The loss per example on the examples trained on and on the
    /// held-out ones after the steps (measured).
    pub train: f64,
    pub held: f64,
    /// The size of s B A after the steps (measured).
    pub size: f64,
}

/// One rank of the sweep.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Row {
    pub rank: usize,
    /// The numbers trained (analytical).
    pub trained: usize,
    /// Under alpha / r, and under alpha / sqrt r.
    pub by_r: Rule,
    pub by_sqrt: Rule,
}

/// What the run gave.
#[derive(Clone, Debug, PartialEq)]
pub struct Sweep {
    pub settings: Settings,
    pub rows: Vec<Row>,
    /// The size of T, the shift the adapter is to come to.
    pub t_size: f64,
    /// The loss per example before any training, trained on and held out.
    pub before: (f64, f64),
    /// The three charts (SVG documents X_eTaL drew): the losses under
    /// alpha / r, under alpha / sqrt r, and the sizes.
    pub charts: Vec<String>,
}

/// Run the program at settings `s`.
pub fn run(s: Settings) -> Result<Sweep, String> {
    let (out, charts) = output_pictures(&with(s), PRINTED)?;
    let mut rows = Vec::with_capacity(RANKS);
    for (k, line) in out[..RANKS].iter().enumerate() {
        let v = numbers::<f64>(line, COLS)?;
        let rule = |at: usize| Rule { s: v[at], train: v[at + 1], held: v[at + 2], size: v[at + 3] };
        rows.push(Row { rank: k + 1, trained: v[0] as usize, by_r: rule(1), by_sqrt: rule(5) });
    }
    let before = numbers::<f64>(&out[RANKS + 1], 2)?;
    if charts.len() != 3 {
        return Err(format!("the program drew {} charts, not 3", charts.len()));
    }
    Ok(Sweep { settings: s, rows, t_size: numbers::<f64>(&out[RANKS], 1)?[0], before: (before[0], before[1]), charts })
}

/// A number to 3 decimals, as the page quotes it.
fn r3(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

fn q(v: f64) -> String {
    show(r3(v))
}

/// How a measured value moved, quoted to 3 decimals.
fn went(what: &str, a: f64, b: f64) -> String {
    moved(what, r3(a), r3(b))
}

impl Sweep {
    /// The row of rank `r` (1 to 8).
    pub fn at(&self, r: usize) -> &Row {
        &self.rows[r - 1]
    }

    /// The rank whose held-out loss is lowest under a rule, and whether
    /// the held-out loss fell at every step up in rank.
    pub fn lowest_held(&self, by_sqrt: bool) -> (usize, bool) {
        let held = |row: &Row| if by_sqrt { row.by_sqrt.held } else { row.by_r.held };
        let best = self.rows.iter().fold(&self.rows[0], |a, b| if held(b) < held(a) { b } else { a });
        let every = self.rows.windows(2).all(|w| held(&w[1]) < held(&w[0]));
        (best.rank, every)
    }

    /// The answer to the guess, from the held-out column under alpha / r.
    pub fn answer(&self) -> String {
        match self.rows.windows(2).find(|w| w[1].by_r.held >= w[0].by_r.held) {
            None => "Read from the held-out column under alpha / r: it fell from each rank to the next.".to_string(),
            Some(w) => format!("Read from the held-out column under alpha / r: it did not fall from each rank to the next; from rank {} to rank {} it went from {} to {}.", w[0].rank, w[1].rank, q(w[0].by_r.held), q(w[1].by_r.held)),
        }
    }

    /// Moving from rank `from` to rank `to`: what changed, from the row's
    /// numbers, under both rules.
    pub fn step_said(&self, from: usize, to: usize) -> Vec<String> {
        let (a, b) = (self.at(from), self.at(to));
        vec![
            format!("From rank {from} to rank {to}, the numbers trained went from {} to {} (analytical: rank times 16 on this layer).", commas(a.trained), commas(b.trained)),
            format!("Under s = alpha / r, {}; {}; {} (measured, after {} steps).", went("the loss on the examples trained on", a.by_r.train, b.by_r.train), went("the loss on the examples held out", a.by_r.held, b.by_r.held), went("the size of s B A", a.by_r.size, b.by_r.size), self.settings.steps),
            format!("Under s = alpha / sqrt r, {}; {}; {} (measured).", went("the loss on the examples trained on", a.by_sqrt.train, b.by_sqrt.train), went("the loss on the examples held out", a.by_sqrt.held, b.by_sqrt.held), went("the size of s B A", a.by_sqrt.size, b.by_sqrt.size)),
            format!("The strength itself (analytical): under alpha / r, {}; under alpha / sqrt r, {}.", went("s", a.by_r.s, b.by_r.s), went("s", a.by_sqrt.s, b.by_sqrt.s)),
        ]
    }

    /// The account of the whole table, from its numbers.
    pub fn account(&self) -> Vec<String> {
        let (first, last) = (&self.rows[0], &self.rows[RANKS - 1]);
        let (best_r, every_r) = self.lowest_held(false);
        let (best_s, every_s) = self.lowest_held(true);
        let steps = self.settings.steps;
        // The first rank whose trained-on loss is under a thousandth, if any.
        let fits = |by_sqrt: bool| self.rows.iter().find(|row| (if by_sqrt { row.by_sqrt.train } else { row.by_r.train }) < 0.0005).map(|row| row.rank);
        // The ranks whose held-out loss after the steps is above the one before any training.
        let worse = |by_sqrt: bool| self.rows.iter().filter(|row| (if by_sqrt { row.by_sqrt.held } else { row.by_r.held }) > self.before.1).map(|row| row.rank.to_string()).collect::<Vec<_>>();
        let mut out = vec![
            format!("With alpha {} and {steps} steps. Cost: the numbers trained go from {} at rank 1 to {} at rank 8 (analytical), past the 64 of W itself at rank 4. Fit: under alpha / r the loss on the examples trained on after {steps} steps goes from {} at rank 1 to {} at rank 8, and the held-out loss from {} to {} (measured; before any training they are {} and {}).", show(self.settings.alpha), commas(first.trained), commas(last.trained), q(first.by_r.train), q(last.by_r.train), q(first.by_r.held), q(last.by_r.held), q(self.before.0), q(self.before.1)),
            match fits(false) {
                Some(k) => format!("The examples trained on are fit to under 0.0005 per example, shown as 0 in the table, from rank {k} on under alpha / r (measured; the shift T has rank 4)."),
                None => format!("No rank fits the examples trained on to under 0.0005 per example within {steps} steps, under alpha / r (measured)."),
            },
            if every_r {
                format!("The held-out loss by rank: under alpha / r it fell from each rank to the next, and is lowest at rank {best_r}, {} (measured); so the numbers trained and the held-out loss moved together here.", q(self.at(best_r).by_r.held))
            } else {
                format!("The held-out loss by rank: under alpha / r it did not fall from each rank to the next; it is lowest at rank {best_r}, {} (measured). The numbers trained rise with every rank (analytical), so a count did not predict the held-out loss on this task.", q(self.at(best_r).by_r.held))
            },
            if every_s {
                format!("Under alpha / sqrt r it fell from each rank to the next and is lowest at rank {best_s}, {} (measured).", q(self.at(best_s).by_sqrt.held))
            } else {
                format!("Under alpha / sqrt r it did not fall from each rank to the next either; it is lowest at rank {best_s}, {} (measured).", q(self.at(best_s).by_sqrt.held))
            },
            format!("The two rules. At rank 1 both give the same strength, {}. By rank 8 alpha / r has shrunk it to {} and alpha / sqrt r to {} (analytical). After {steps} steps at rank 8 the loss on the examples trained on is {} under alpha / r and {} under alpha / sqrt r, and the held-out loss {} and {} (measured).", q(first.by_r.s), q(last.by_r.s), q(last.by_sqrt.s), q(last.by_r.train), q(last.by_sqrt.train), q(last.by_r.held), q(last.by_sqrt.held)),
            {
                let near = |v: f64| (v - self.t_size).abs() < 0.1 * self.t_size;
                let reading = if near(last.by_r.size) && near(last.by_sqrt.size) {
                    format!("At rank 8 both are within a tenth of T's size (within {} of {}), while the held-out losses there are {} and {}.", q(0.1 * self.t_size), q(self.t_size), q(last.by_r.held), q(last.by_sqrt.held))
                } else {
                    format!("At rank 8 the two rules did not end within a tenth of T's size of each other ({} and {} against {}).", q(last.by_r.size), q(last.by_sqrt.size), q(self.t_size))
                };
                format!("The size of s B A after the steps goes from {} at rank 1 to {} at rank 8 under alpha / r, and from {} to {} under alpha / sqrt r (measured); T itself has size {}. {reading}", q(first.by_r.size), q(last.by_r.size), q(first.by_sqrt.size), q(last.by_sqrt.size), q(self.t_size))
            },
        ];
        let (wr, ws) = (worse(false), worse(true));
        out.push(if wr.is_empty() && ws.is_empty() {
            format!("At every rank, under both rules, the held-out loss after the steps is below the {} it was before any training (measured).", q(self.before.1))
        } else {
            format!("Above the {} the held-out loss was before any training (measured): under alpha / r at rank{} {}; under alpha / sqrt r at rank{} {}. There the adapter fit the examples trained on at a cost on the held-out ones.", q(self.before.1), if wr.len() == 1 { "" } else { "s" }, if wr.is_empty() { "none".to_string() } else { wr.join(", ") }, if ws.len() == 1 { "" } else { "s" }, if ws.is_empty() { "none".to_string() } else { ws.join(", ") })
        });
        if let Some(k) = fits(true) {
            out.insert(2, format!("Under alpha / sqrt r they are fit so from rank {k} on (measured)."));
        }
        out
    }
}
