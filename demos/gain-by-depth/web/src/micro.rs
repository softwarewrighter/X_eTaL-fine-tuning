//! The model: the demo's program (gain-by-depth.xtl) run at the shift
//! of the logits chosen; its printed lines, its chart and the Check
//! library's report read back. Nothing here multiplies, sums or
//! measures anything: every number is X_eTaL's.

use microscope::run::{numbers, output_pictures};

/// The command line's program.
pub const PROGRAM: &str = include_str!("../../gain-by-depth.xtl");

/// The checks the program makes.
pub const CHECKS: usize = 4;

/// The lines the program prints (its chart aside): 16 lines of
/// numbers, then the Check report, a line per check and a summary.
pub const PRINTED: usize = 16 + CHECKS + 1;

/// The size of the tables and the depth.
pub const S: usize = 4;
pub const L: usize = 48;

/// The depths quoted, the Python's three.
pub const DEPTHS: [usize; 3] = [12, 24, 48];

/// The shifts the slider offers: every logit plus c, from -2 to 1 by
/// a quarter. 0 is the paper's and the Python's start, the program's own.
pub const SHIFTS: [f64; 13] = [-2.0, -1.75, -1.5, -1.25, -1.0, -0.75, -0.5, -0.25, 0.0, 0.25, 0.5, 0.75, 1.0];
pub const PAPER: usize = 8;

/// The program at shift `c`: one line changes.
pub fn with(c: f64) -> String {
    PROGRAM.replacen("shift := 0.00\n", &format!("shift := {c:.2}\n"), 1)
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

/// One table's products and their measures.
#[derive(Clone, Debug, PartialEq)]
pub struct Chain {
    /// The table (S x S) and its row sums (S).
    pub table: Vec<f64>,
    pub rows: Vec<f64>,
    /// H^k after each of the L layers (L x S x S).
    pub products: Vec<f64>,
    /// Per layer: the forward gain (the largest absolute row sum), the
    /// backward gain (the largest absolute column sum), and the proxy
    /// (log10 of the largest absolute entry).
    pub forward: Vec<f64>,
    pub backward: Vec<f64>,
    pub proxy: Vec<f64>,
}

impl Chain {
    /// H^k (from k = 1).
    pub fn product(&self, k: usize) -> &[f64] {
        &self.products[(k - 1) * S * S..k * S * S]
    }
}

/// What the run gave.
#[derive(Clone, Debug, PartialEq)]
pub struct Run {
    pub shift: f64,
    /// The logits (S x S).
    pub logits: Vec<f64>,
    /// HC's chain and mHC's.
    pub hc: Chain,
    pub mhc: Chain,
    /// k log10 s for k = 1..L: the analytical line of HC's forward gain.
    pub line: Vec<f64>,
    /// HC's row sum at every shift of the slider, and its forward gain
    /// after L layers there (SHIFTS.len() each).
    pub sweep: Vec<f64>,
    pub sweep_gain: Vec<f64>,
    /// The chart (an SVG document X_eTaL drew).
    pub chart: String,
    pub checks: Vec<Check>,
    pub summary: String,
}

/// Run the program at shift `c`.
pub fn run(c: f64) -> Result<Run, String> {
    let (out, pictures) = output_pictures(&with(c), PRINTED)?;
    let chart = pictures.into_iter().next().ok_or("the program drew no chart")?;
    let n = |i: usize, k: usize| numbers::<f64>(&out[i], k);
    let hc = Chain { table: n(1, S * S)?, rows: n(3, S)?, products: n(5, L * S * S)?, forward: n(7, L)?, backward: n(8, L)?, proxy: n(9, L)? };
    let mhc = Chain { table: n(2, S * S)?, rows: n(4, S)?, products: n(6, L * S * S)?, forward: n(10, L)?, backward: n(11, L)?, proxy: n(12, L)? };
    let checks = (0..CHECKS).map(|k| check(&out[16 + k])).collect::<Result<Vec<_>, _>>()?;
    Ok(Run { shift: c, logits: n(0, S * S)?, hc, mhc, line: n(13, L)?, sweep: n(14, SHIFTS.len())?, sweep_gain: n(15, SHIFTS.len())?, chart, checks, summary: out[16 + CHECKS].clone() })
}
