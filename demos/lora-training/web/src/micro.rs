//! The model: the demo's program (lora-training.xtl) run at a preset's
//! settings, then asked for the value of every stage of its one step
//! (flow:s_tages<) and for the step drawn as mathematics
//! (flow:m_ath<), and its printed lines and its chart read back.
//! Nothing here trains or computes a loss: every number is X_eTaL's.

use lesson::stepper::{trace, Trace};
use microscope::run::{numbers, output_pictures};

/// The command line's program.
pub const PROGRAM: &str = include_str!("../../lora-training.xtl");

/// The lines the program prints; the page asks for two more (the
/// step's record of stages, and the step drawn as mathematics).
pub const PRINTED: usize = 13;

/// The side of the layer: W is N by N, A is r by N, B is N by r.
pub const N: usize = 8;
/// The examples: M inputs as columns, the last HELD held out.
pub const M: usize = 12;
pub const HELD: usize = 4;

/// The presets, as data (presets/*.toml): the first is the program's own.
pub const PRESETS: [&str; 3] = [
    include_str!("../../presets/first.toml"),
    include_str!("../../presets/too-fast.toml"),
    include_str!("../../presets/overfitting.toml"),
];

/// One experiment's settings: the rank, alpha, the learning rate, how
/// many examples are trained on, and the record (chunks of every steps).
#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    pub title: String,
    pub setup: String,
    pub r: usize,
    pub alpha: f64,
    pub lr: f64,
    pub n: usize,
    pub chunks: usize,
    pub every: usize,
}

impl Default for Settings {
    fn default() -> Self {
        preset(PRESETS[0]).expect("the first preset")
    }
}

/// A preset read from its TOML: `key = value` lines, strings quoted.
pub fn preset(toml: &str) -> Result<Settings, String> {
    let mut s = Settings { title: String::new(), setup: String::new(), r: 0, alpha: 0.0, lr: 0.0, n: 0, chunks: 0, every: 0 };
    for line in toml.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let (key, value) = line.split_once('=').ok_or_else(|| format!("preset: not `key = value`: {line}"))?;
        let (key, value) = (key.trim(), value.trim());
        let number = |v: &str| v.parse::<f64>().map_err(|_| format!("preset: {key} is not a number: {v}"));
        match key {
            "title" => s.title = value.trim_matches('"').to_string(),
            "setup" => s.setup = value.trim_matches('"').to_string(),
            "r" => s.r = number(value)? as usize,
            "alpha" => s.alpha = number(value)?,
            "lr" => s.lr = number(value)?,
            "n" => s.n = number(value)? as usize,
            "chunks" => s.chunks = number(value)? as usize,
            "every" => s.every = number(value)?  as usize,
            other => return Err(format!("preset: unknown key {other}")),
        }
    }
    if s.r == 0 || s.n == 0 || s.n + HELD > M || s.chunks == 0 || s.every == 0 {
        return Err("preset: a setting is missing or out of range".into());
    }
    Ok(s)
}

/// Every preset.
pub fn presets() -> Vec<Settings> {
    PRESETS.iter().map(|p| preset(p).expect("a preset")).collect()
}

/// A number as the program writes it (a float keeps its point).
fn float(v: f64) -> String {
    if v.fract() == 0.0 { format!("{v:.1}") } else { v.to_string() }
}

/// The program with the settings put in its data: six lines change.
pub fn with(s: &Settings) -> String {
    let d = Settings::default();
    PROGRAM
        .replacen(&format!("r := {}\n", d.r), &format!("r := {}\n", s.r), 1)
        .replacen(&format!("alpha := {}\n", float(d.alpha)), &format!("alpha := {}\n", float(s.alpha)), 1)
        .replacen(&format!("lr := {}\n", float(d.lr)), &format!("lr := {}\n", float(s.lr)), 1)
        .replacen(&format!("n := {}\n", d.n), &format!("n := {}\n", s.n), 1)
        .replacen(&format!("chunks := {}\n", d.chunks), &format!("chunks := {}\n", s.chunks), 1)
        .replacen(&format!("every := {}\n", d.every), &format!("every := {}\n", s.every), 1)
}

/// The step's pipeline as the program writes it, from `step := @
/// flow:p_ipe<` to the end of its text.
pub fn pipeline() -> &'static str {
    let from = PROGRAM.find("step := @ flow:p_ipe<").expect("the pipeline");
    let to = from + PROGRAM[from..].find("\"#").expect("its end") + 2;
    &PROGRAM[from..to]
}

/// The program the page runs at settings `s`: the command line's, then
/// the record of the step's every stage, then the step drawn as math.
pub fn program(s: &Settings) -> String {
    let text = &pipeline()[pipeline().find("c#").expect("its text")..];
    format!("{}\n# -- read back by the page: every stage of the step, then the step drawn as mathematics\n@ flow:s_tages< {text}\n@ flow:m_ath< {text}\n", with(s).trim_end())
}

/// What the run gave.
#[derive(Clone, Debug, PartialEq)]
pub struct Training {
    pub settings: Settings,
    /// W and T (N x N), the inputs X and the outputs wanted Y (N x M), by rows.
    pub w: Vec<f64>,
    pub t: Vec<f64>,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    /// The loss per example on the examples trained on, before the
    /// step and after it.
    pub before: f64,
    pub after: f64,
    /// How far the step's A and B are from the library's (lo:s_gd), and
    /// how far the step's W is from W.
    pub off: f64,
    pub w_moved: f64,
    /// The record: the loss per example on the examples trained on and
    /// on the held-out ones, chunks + 1 each (the start first).
    pub loss_t: Vec<f64>,
    pub loss_h: Vec<f64>,
    /// A, B and the correction s B A at each record, flattened in turn.
    pub rec_a: Vec<f64>,
    pub rec_b: Vec<f64>,
    pub rec_d: Vec<f64>,
    /// Every stage of the step: x, h, u, p, g, b, dA, dB, A2, B2, then the
    /// adapter after it, (W2, A3, B3, s2), a record each.
    pub trace: Trace,
    /// The step drawn as mathematics (LaTeX, a row a stage).
    pub tex: String,
    /// The chart of the two losses (an SVG document X_eTaL drew).
    pub chart: String,
}

impl Training {
    /// How many records there are (the start and one per chunk).
    pub fn records(&self) -> usize {
        self.settings.chunks + 1
    }
    /// A (r x N) at record `k`.
    pub fn a_at(&self, k: usize) -> &[f64] {
        let n = self.settings.r * N;
        &self.rec_a[k * n..(k + 1) * n]
    }
    /// B (N x r) at record `k`.
    pub fn b_at(&self, k: usize) -> &[f64] {
        let n = N * self.settings.r;
        &self.rec_b[k * n..(k + 1) * n]
    }
    /// The correction s B A (N x N) at record `k`.
    pub fn d_at(&self, k: usize) -> &[f64] {
        &self.rec_d[k * N * N..(k + 1) * N * N]
    }
    /// The step of training record `k` was taken after.
    pub fn step_of(&self, k: usize) -> usize {
        k * self.settings.every
    }
    /// Columns `from` to `to` of a table of `rows` rows and `cols` columns, by rows.
    pub fn columns(items: &[f64], rows: usize, cols: usize, from: usize, to: usize) -> Vec<f64> {
        (0..rows).flat_map(|i| items[i * cols + from..i * cols + to].to_vec()).collect()
    }
    /// The values of the step's stage named `label`.
    pub fn stage(&self, label: &str) -> &[f64] {
        self.trace.stages.iter().find(|s| s.label == label).map(|s| s.items.as_slice()).unwrap_or(&[])
    }
}

/// Run the program at settings `s`.
pub fn run(s: &Settings) -> Result<Training, String> {
    let (out, pictures) = output_pictures(&program(s), PRINTED + 2)?;
    let (r, k) = (s.r, s.chunks + 1);
    let chart = pictures.into_iter().next().ok_or("the program drew no chart")?;
    Ok(Training {
        settings: s.clone(),
        w: numbers(&out[0], N * N)?,
        t: numbers(&out[1], N * N)?,
        x: numbers(&out[2], N * M)?,
        y: numbers(&out[3], N * M)?,
        before: numbers::<f64>(&out[4], 1)?[0],
        after: numbers::<f64>(&out[5], 1)?[0],
        off: numbers::<f64>(&out[6], 1)?[0],
        w_moved: numbers::<f64>(&out[7], 1)?[0],
        loss_t: numbers(&out[8], k)?,
        loss_h: numbers(&out[9], k)?,
        rec_a: numbers(&out[10], k * r * N)?,
        rec_b: numbers(&out[11], k * N * r)?,
        rec_d: numbers(&out[12], k * N * N)?,
        trace: trace(&out[13], pipeline())?,
        tex: out[14].clone(),
        chart,
    })
}
