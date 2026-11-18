//! The model: the demo's program (sinkhorn.xtl) run at a preset's
//! start (the logits the positive table is the exponential of), then
//! asked for the value of every stage of its one round (flow:s_tages<)
//! and for the round drawn as mathematics (flow:m_ath<); its printed
//! lines, its chart and the Check library's report read back. Nothing
//! here divides, sums or checks anything: every number is X_eTaL's.

use lesson::stepper::{trace, Trace};
use microscope::run::{numbers, output_pictures};

/// The command line's program.
pub const PROGRAM: &str = include_str!("../../sinkhorn.xtl");

/// The lines the program prints (its chart aside): 29 lines of
/// numbers, then the Check report, a line per check and a summary.
pub const PRINTED: usize = 29 + CHECKS + 1;

/// The checks the program makes.
pub const CHECKS: usize = 8;

/// The size of the tables, the paper's count of rounds, the most
/// permutation tables a 4 x 4 can need.
pub const S: usize = 4;
pub const TMAX: usize = 20;
pub const PERMS: usize = 10;

/// The presets, as data (presets/*.toml): the first is the program's own.
pub const PRESETS: [&str; 3] = [
    include_str!("../../presets/paper.toml"),
    include_str!("../../presets/lopsided.toml"),
    include_str!("../../presets/split.toml"),
];

/// One run's settings: the logits the start is the exponential of, a
/// 4 x 4 as 16 numbers, a row at a time.
#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    pub title: String,
    pub setup: String,
    pub logits: String,
}

impl Default for Settings {
    fn default() -> Self {
        preset(PRESETS[0]).expect("the first preset")
    }
}

/// A preset read from its TOML: `key = value` lines, strings quoted.
pub fn preset(toml: &str) -> Result<Settings, String> {
    let mut s = Settings { title: String::new(), setup: String::new(), logits: String::new() };
    for line in toml.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let (key, value) = line.split_once('=').ok_or_else(|| format!("preset: not `key = value`: {line}"))?;
        let (key, value) = (key.trim(), value.trim().trim_matches('"').to_string());
        match key {
            "title" => s.title = value,
            "setup" => s.setup = value,
            "logits" => s.logits = value,
            other => return Err(format!("preset: unknown key {other}")),
        }
    }
    let count = s.logits.split_whitespace().filter(|w| w.parse::<f64>().is_ok()).count();
    if count != S * S || s.title.is_empty() {
        return Err(format!("preset: {} needs {} logits, has {count}", s.title, S * S));
    }
    Ok(s)
}

/// Every preset.
pub fn presets() -> Vec<Settings> {
    PRESETS.iter().map(|p| preset(p).expect("a preset")).collect()
}

/// The program with the settings put in its data: one line changes.
pub fn with(s: &Settings) -> String {
    let d = Settings::default();
    PROGRAM.replacen(&format!("logits := (S c_at S) r_eshape {}\n", d.logits), &format!("logits := (S c_at S) r_eshape {}\n", s.logits), 1)
}

/// The round's pipeline as the program writes it, from `round := @
/// flow:p_ipe<` to the end of its text.
pub fn pipeline() -> &'static str {
    let from = PROGRAM.find("round := @ flow:p_ipe<").expect("the pipeline");
    let to = from + PROGRAM[from..].find("\"#").expect("its end") + 2;
    &PROGRAM[from..to]
}

/// The program the page runs at settings `s`: the command line's, then
/// the record of the round's stages, then the round drawn as math.
pub fn program(s: &Settings) -> String {
    let text = &pipeline()[pipeline().find("c#").expect("its text")..];
    format!("{}\n# -- read back by the page: every stage of the round, then the round drawn as mathematics\n@ flow:s_tages< {text}\n@ flow:m_ath< {text}\n", with(s).trim_end())
}

/// One check of the Check library's report: its name, whether it
/// passed, and the line as X_eTaL printed it.
#[derive(Clone, Debug, PartialEq)]
pub struct Check {
    pub name: String,
    pub ok: bool,
    pub line: String,
}

/// A report line, `ok: name` or `FAIL: name: expected ..., got ...`.
pub fn check(line: &str) -> Result<Check, String> {
    if let Some(name) = line.strip_prefix("ok: ") {
        Ok(Check { name: name.to_string(), ok: true, line: line.to_string() })
    } else if let Some(rest) = line.strip_prefix("FAIL: ") {
        let name = rest.split(": expected").next().unwrap_or(rest);
        Ok(Check { name: name.to_string(), ok: false, line: line.to_string() })
    } else {
        Err(format!("not a check: {line}"))
    }
}

/// What the run gave.
#[derive(Clone, Debug, PartialEq)]
pub struct Run {
    pub settings: Settings,
    /// The logits (S x S), the start A, their exponential (S x S), and
    /// what A's rows and columns sum to (S each).
    pub logits: Vec<f64>,
    pub a: Vec<f64>,
    pub rows_a: Vec<f64>,
    pub cols_a: Vec<f64>,
    /// The pipeline's one round, C after it (S x S).
    pub round: Vec<f64>,
    /// After each of the TMAX rounds: the table after the row step and
    /// after the column step (TMAX x S x S each), and their row and
    /// column sums (TMAX x S each).
    pub rs: Vec<f64>,
    pub cs: Vec<f64>,
    pub rows_r: Vec<f64>,
    pub cols_r: Vec<f64>,
    pub rows_c: Vec<f64>,
    pub cols_c: Vec<f64>,
    /// Per round: how far the farthest column sum was from 1 before
    /// the column step, and the farthest row sum after it (TMAX each).
    pub col_off: Vec<f64>,
    pub row_off: Vec<f64>,
    /// How many rounds until every row sum is within the tolerance of
    /// 1; TMAX + 1 when the tolerance was not reached.
    pub rounds: usize,
    /// The chart of the two distances (an SVG document X_eTaL drew).
    pub chart: String,
    /// The result H (S x S), and how far the pipeline's round is from
    /// the loop's first (0 to rounding).
    pub h: Vec<f64>,
    pub off: f64,
    /// The largest singular value of A and of H.
    pub sigma_a: f64,
    pub sigma_h: f64,
    /// A list v, H v, and their sizes.
    pub v: Vec<f64>,
    pub hv: Vec<f64>,
    pub size_v: f64,
    pub size_hv: f64,
    /// The second start B0, its projection B, the product H B (S x S
    /// each) and the product's row and column sums.
    pub b0: Vec<f64>,
    pub b: Vec<f64>,
    pub hb: Vec<f64>,
    pub rows_hb: Vec<f64>,
    pub cols_hb: Vec<f64>,
    /// The permutation tables' weights (PERMS), their sum, the
    /// orderings picked (PERMS x S, from 1), and the largest entry of
    /// what is left of H after them.
    pub weights: Vec<f64>,
    pub weights_sum: f64,
    pub orderings: Vec<usize>,
    pub rest: f64,
    /// The checks, as the Check library reported them, and its summary line.
    pub checks: Vec<Check>,
    pub summary: String,
    /// Every stage of the round: A, R, C.
    pub trace: Trace,
    /// The round drawn as mathematics (LaTeX, a row a stage).
    pub tex: String,
}

impl Run {
    /// The values of the round's stage named `label`.
    pub fn stage(&self, label: &str) -> &[f64] {
        self.trace.stages.iter().find(|s| s.label == label).map(|s| s.items.as_slice()).unwrap_or(&[])
    }
    /// Round `t` (from 1) of a per-round record of `n` numbers.
    pub fn at<'a>(record: &'a [f64], t: usize, n: usize) -> &'a [f64] {
        &record[(t - 1) * n..t * n]
    }
    /// How many permutation tables carried weight (the rest are 0).
    pub fn tables(&self) -> usize {
        self.weights.iter().filter(|w| **w > 0.0).count()
    }
    /// Whether every check passed.
    pub fn passed(&self) -> bool {
        self.checks.iter().all(|c| c.ok)
    }
}

/// Run the program at settings `s`.
pub fn run(s: &Settings) -> Result<Run, String> {
    let (out, pictures) = output_pictures(&program(s), PRINTED + 2)?;
    let chart = pictures.into_iter().next().ok_or("the program drew no chart")?;
    let one = |i: usize| -> Result<f64, String> { Ok(numbers::<f64>(&out[i], 1)?[0]) };
    let two = |i: usize| -> Result<(f64, f64), String> {
        let v = numbers::<f64>(&out[i], 2)?;
        Ok((v[0], v[1]))
    };
    let (sigma_a, sigma_h) = two(16)?;
    let (size_v, size_hv) = two(19)?;
    let checks = (0..CHECKS).map(|k| check(&out[29 + k])).collect::<Result<Vec<_>, _>>()?;
    Ok(Run {
        settings: s.clone(),
        logits: numbers(&out[0], S * S)?,
        a: numbers(&out[1], S * S)?,
        rows_a: numbers(&out[2], S)?,
        cols_a: numbers(&out[3], S)?,
        round: numbers(&out[4], S * S)?,
        rs: numbers(&out[5], TMAX * S * S)?,
        cs: numbers(&out[6], TMAX * S * S)?,
        rows_r: numbers(&out[7], TMAX * S)?,
        cols_r: numbers(&out[8], TMAX * S)?,
        rows_c: numbers(&out[9], TMAX * S)?,
        cols_c: numbers(&out[10], TMAX * S)?,
        col_off: numbers(&out[11], TMAX)?,
        row_off: numbers(&out[12], TMAX)?,
        rounds: numbers::<usize>(&out[13], 1)?[0],
        chart,
        h: numbers(&out[14], S * S)?,
        off: one(15)?,
        sigma_a,
        sigma_h,
        v: numbers(&out[17], S)?,
        hv: numbers(&out[18], S)?,
        size_v,
        size_hv,
        b0: numbers(&out[20], S * S)?,
        b: numbers(&out[21], S * S)?,
        hb: numbers(&out[22], S * S)?,
        rows_hb: numbers(&out[23], S)?,
        cols_hb: numbers(&out[24], S)?,
        weights: numbers(&out[25], PERMS)?,
        weights_sum: one(26)?,
        orderings: numbers(&out[27], PERMS * S)?,
        rest: one(28)?,
        checks,
        summary: out[29 + CHECKS].clone(),
        trace: trace(&out[PRINTED], pipeline())?,
        tex: out[PRINTED + 1].clone(),
    })
}
