//! The model: the demo's program (mhc-compared.xtl) run on its data
//! (the nine records of training-at-depth, copied into data/, and the
//! Python re-implementation's figures, quoted in data/python-*.txt);
//! its printed lines, its two charts and the Check report read back.
//! Nothing here multiplies, sums, counts or checks anything: every
//! number is X_eTaL's, from the analytical start or the records, or
//! the Python's as quoted.

use microscope::run::{numbers, output_with};

/// The program.
pub const PROGRAM: &str = include_str!("../../mhc-compared.xtl");

/// The streams, the depths, the figures the program prints per run.
pub const S: usize = 4;
pub const DEPTHS: [usize; 3] = [12, 24, 48];
pub const FIELDS: usize = 10;
/// The lines it prints (its two charts aside): H_res of HC and mHC,
/// the start, the runs' figures, mHC's distance from 1, the Python's
/// losses and proxies, the counts per layer and the totals; six checks
/// and the summary.
pub const CHECKS: usize = 6;
pub const PRINTED: usize = 9 + CHECKS + 1;

/// The schemes, in the program's order.
pub const SCHEMES: [&str; 3] = ["the plain skip", "HC", "mHC"];

/// The data the program reads: the records, and the Python's figures.
pub const FILES: [(&str, &str); 11] = [
    ("data/baseline-12.txt", include_str!("../../data/baseline-12.txt")),
    ("data/baseline-24.txt", include_str!("../../data/baseline-24.txt")),
    ("data/baseline-48.txt", include_str!("../../data/baseline-48.txt")),
    ("data/hc-12.txt", include_str!("../../data/hc-12.txt")),
    ("data/hc-24.txt", include_str!("../../data/hc-24.txt")),
    ("data/hc-48.txt", include_str!("../../data/hc-48.txt")),
    ("data/mhc-12.txt", include_str!("../../data/mhc-12.txt")),
    ("data/mhc-24.txt", include_str!("../../data/mhc-24.txt")),
    ("data/mhc-48.txt", include_str!("../../data/mhc-48.txt")),
    ("data/python-loss.txt", include_str!("../../data/python-loss.txt")),
    ("data/python-proxy.txt", include_str!("../../data/python-proxy.txt")),
];
/// What made the records (data/runs.txt, training-at-depth's).
pub const RUNS: &str = include_str!("../../data/runs.txt");

/// One check of the Check library's report.
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

/// The run.
#[derive(Clone, Debug, PartialEq)]
pub struct Run {
    /// HC's and mHC's H_res at the start, 4 x 4 each.
    pub hc: Vec<f64>,
    pub mhc: Vec<f64>,
    /// At each depth: HC's forward gain and proxy, mHC's forward gain
    /// and proxy, in powers of ten.
    pub start: Vec<f64>,
    /// For each run (the plain skip, HC, mHC; 12, 24, 48 layers), the
    /// ten figures: last loss, smallest loss, last gradient size,
    /// largest gradient size, last forward gain, largest forward gain,
    /// last backward gain (these three in powers of ten), last proxy,
    /// NaN steps, largest distance of a gain from 1.
    pub figures: Vec<f64>,
    /// How far mHC's gains came from 1 at any step.
    pub off: f64,
    /// The Python's final losses and proxies, the same order.
    pub py_loss: Vec<f64>,
    pub py_proxy: Vec<f64>,
    /// Per layer: the block, the plain skip's tables, the static
    /// tables, the dynamic tables; then the embeddings.
    pub per_layer: Vec<f64>,
    /// At 12, 24, 48 layers: the plain skip, HC or mHC, mHC dynamic.
    pub totals: Vec<f64>,
    pub charts: Vec<String>,
    pub checks: Vec<Check>,
    pub summary: String,
}

impl Run {
    /// Figure `k` of scheme `s` (0 plain, 1 HC, 2 mHC) at depth index `d`.
    pub fn fig(&self, s: usize, d: usize, k: usize) -> f64 {
        self.figures[(s * 3 + d) * FIELDS + k]
    }
    /// The Python's final loss and proxy, the same way.
    pub fn py(&self, s: usize, d: usize) -> (f64, f64) {
        (self.py_loss[s * 3 + d], self.py_proxy[s * 3 + d])
    }
    /// The analytical start at depth index `d`: HC's forward gain and
    /// proxy, mHC's forward gain and proxy.
    pub fn at_start(&self, d: usize) -> [f64; 4] {
        let r = &self.start[d * 4..d * 4 + 4];
        [r[0], r[1], r[2], r[3]]
    }
    /// The numbers trained, scheme `s` (0 plain, 1 HC or mHC, 2 mHC
    /// dynamic), depth index `d`.
    pub fn total(&self, s: usize, d: usize) -> f64 {
        self.totals[s * 3 + d]
    }
}

/// The data, as the files the program reads.
pub fn files() -> Vec<(String, String)> {
    FILES.iter().map(|(n, t)| (n.to_string(), t.to_string())).collect()
}

/// Run the program.
pub fn run() -> Result<Run, String> {
    let (out, charts) = output_with(&files(), PROGRAM, PRINTED)?;
    if charts.len() != 2 {
        return Err(format!("the program drew {} charts, not 2", charts.len()));
    }
    Ok(Run {
        hc: numbers(&out[0], S * S)?,
        mhc: numbers(&out[1], S * S)?,
        start: numbers(&out[2], 12)?,
        figures: numbers(&out[3], 9 * FIELDS)?,
        off: numbers::<f64>(&out[4], 1)?[0],
        py_loss: numbers(&out[5], 9)?,
        py_proxy: numbers(&out[6], 9)?,
        per_layer: numbers(&out[7], 5)?,
        totals: numbers(&out[8], 9)?,
        charts,
        checks: (9..9 + CHECKS).map(|k| check(&out[k])).collect::<Result<Vec<_>, _>>()?,
        summary: out[9 + CHECKS].clone(),
    })
}
