//! The model: the demo's program (dynamic-mixing.xtl) run at the
//! gating factor chosen; its printed lines and the Check library's
//! report read back. Nothing here normalizes, projects, mixes or
//! checks anything: every number is X_eTaL's.

use microscope::run::{numbers, printed};

/// The command line's program.
pub const PROGRAM: &str = include_str!("../../dynamic-mixing.xtl");

/// The streams, a stream's width, the entries the gradient check looks at.
pub const S: usize = 4;
pub const C: usize = 8;
pub const CHECKED: usize = 10;
/// The lines the program prints.
pub const PRINTED: usize = 16;

/// The gating factors the slider offers; 0.01 is the paper's start
/// and the program's own.
pub const ALPHAS: [f64; 6] = [0.0, 0.01, 0.1, 0.3, 1.0, 3.0];
pub const PAPER: usize = 1;

/// The entries checked, in the program's order.
pub const ENTRIES: [&str; CHECKED] = [
    "the gating factor of H_pre",
    "the gating factor of H_post",
    "the gating factor of H_res",
    "a number of the projection, in H_pre's columns",
    "a number of the projection, in H_res's columns",
    "the bias of H_pre, first",
    "the bias of H_post, second",
    "the bias of H_res, row 1, column 2",
    "the input, stream 1, first number",
    "the input, stream 3, fifth number",
];

/// The program at gating factor `a`: one line changes.
pub fn with(a: f64) -> String {
    PROGRAM.replacen("alpha := 0.01\n", &format!("alpha := {a:?}\n"), 1)
}

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

/// One input's run of the layer.
#[derive(Clone, Debug, PartialEq)]
pub struct Input {
    /// The stream state (S x C).
    pub x: Vec<f64>,
    /// The flattened state normalized (S C), and its projection (24).
    pub xn: Vec<f64>,
    pub proj: Vec<f64>,
    /// H_pre (S), H_post (S), H_res (S x S), the block's input (C), its
    /// output (C), the new state (S x C).
    pub h_pre: Vec<f64>,
    pub h_post: Vec<f64>,
    pub h_res: Vec<f64>,
    pub u: Vec<f64>,
    pub f: Vec<f64>,
    pub x2: Vec<f64>,
}

fn input(x: Vec<f64>, stages: &[f64], values: &[f64]) -> Input {
    let n = S * C;
    let k = |a: usize, b: usize| values[a..b].to_vec();
    Input {
        x,
        xn: stages[..n].to_vec(),
        proj: stages[n..].to_vec(),
        h_pre: k(0, S),
        h_post: k(S, 2 * S),
        h_res: k(2 * S, 2 * S + S * S),
        u: k(2 * S + S * S, 2 * S + S * S + C),
        f: k(2 * S + S * S + C, 2 * S + S * S + 2 * C),
        x2: k(2 * S + S * S + 2 * C, 2 * S + S * S + 2 * C + S * C),
    }
}

/// What the run gave.
#[derive(Clone, Debug, PartialEq)]
pub struct Run {
    pub alpha: f64,
    pub a: Input,
    pub b: Input,
    /// The largest difference between the two inputs' H_pre, H_post, H_res.
    pub apart: [f64; 3],
    /// The first input's H_res: its row sums and column sums.
    pub rows: Vec<f64>,
    pub cols: Vec<f64>,
    pub written: Vec<f64>,
    pub central: Vec<f64>,
    pub checks: Vec<Check>,
    pub summary: String,
}

/// Run the program at gating factor `a`.
pub fn run(a: f64) -> Result<Run, String> {
    let out = printed(&with(a))?;
    if out.len() != PRINTED {
        return Err(format!("expected {PRINTED} lines of output, got {}", out.len()));
    }
    let n = |i: usize, k: usize| numbers::<f64>(&out[i], k);
    let width = 2 * S + S * S + 2 * C + S * C;
    let stages = S * C + 2 * S + S * S;
    Ok(Run {
        alpha: a,
        a: input(n(0, S * C)?, &n(2, stages)?, &n(3, width)?),
        b: input(n(1, S * C)?, &n(4, stages)?, &n(5, width)?),
        apart: [n(6, 1)?[0], n(7, 1)?[0], n(8, 1)?[0]],
        rows: n(9, S)?,
        cols: n(10, S)?,
        written: n(11, CHECKED)?,
        central: n(12, CHECKED)?,
        checks: (13..15).map(|k| check(&out[k])).collect::<Result<Vec<_>, _>>()?,
        summary: out[15].clone(),
    })
}
