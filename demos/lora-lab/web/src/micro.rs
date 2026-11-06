//! The model: the demo's program (lora-lab.xtl) run at the rank the
//! page sets, then asked for the value of every stage of its pipeline
//! (flow:s_tages<) and for each stage's formula with its symbols
//! marked (e:t_exSymbols<), and its printed lines read back. Nothing
//! here computes the layer: every number is X_eTaL's.

use lesson::stepper::{trace, Trace};
use microscope::run::{numbers, output};

/// The command line's program.
pub const PROGRAM: &str = include_str!("../../lora-lab.xtl");

/// The lines the program prints; the page asks for four more (the
/// stages' record, and three formulas).
pub const PRINTED: usize = 12;

/// The side of the layer: W is N by N, A is r by N, B is N by r.
pub const N: usize = 8;

/// What the page sets: the rank, 1 to 8.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub r: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { r: 2 }
    }
}

/// The program with the rank put in its data: one line changes.
pub fn with(s: Settings) -> String {
    PROGRAM.replacen("r := 2\n", &format!("r := {}\n", s.r), 1)
}

/// The pipeline as the program writes it, from `y := @ flow:p_ipe<` to
/// the end of its text.
pub fn pipeline() -> &'static str {
    let from = PROGRAM.find("y := @ flow:p_ipe<").expect("the pipeline");
    let to = from + PROGRAM[from..].find("\"\"#").expect("its end") + 3;
    &PROGRAM[from..to]
}

/// The three equations of the pipeline (stages 2 to 4), each as the
/// call that draws it with its symbols marked: the stage's own text,
/// its label dropped, `e:e_q<` become `e:t_exSymbols<`.
pub fn formulas() -> Vec<String> {
    pipeline()
        .lines()
        .filter_map(|l| l.trim().strip_prefix("|> "))
        .map(|stage| {
            // The last stage's line also carries the end of the pipeline's own text.
            let stage = stage.strip_suffix("\"#").unwrap_or(stage);
            let end = stage.rfind('"').map_or(stage.len(), |i| i + 1);
            stage[..end].replacen("e:e_q<", "e:t_exSymbols<", 1)
        })
        .collect()
}

/// The program the page runs at settings `s`: the command line's, then
/// the record of every stage, then each stage's formula.
pub fn program(s: Settings) -> String {
    let text = &pipeline()[pipeline().find("c#").expect("its text")..];
    format!(
        "{}\n# -- read back by the page: every stage's value, then each stage's formula with its symbols marked\n@ flow:s_tages< {text}\n{}\n",
        with(s).trim_end(),
        formulas().join("\n")
    )
}

/// What the run gave.
#[derive(Clone, Debug, PartialEq)]
pub struct Lab {
    /// The rank it was run at.
    pub r: usize,
    /// The pipeline's output y; what the frozen table gives, W x; the
    /// correction the adapter adds, s B (A x).
    pub y: Vec<f64>,
    pub base: Vec<f64>,
    pub correction: Vec<f64>,
    /// The pipeline's output is the library's (lo:f_orward).
    pub same: bool,
    /// The numbers the adapter trains, and the table's.
    pub trained: (usize, usize),
    /// The strength s, alpha / r.
    pub s: f64,
    /// W by rows (N x N), A (r x N), B (N x r).
    pub w: Vec<f64>,
    pub a: Vec<f64>,
    pub b: Vec<f64>,
    /// Every product: A's numbers times x's (r rows of N), B's times h's
    /// (N rows of r), W's times x's (N rows of N).
    pub ax: Vec<f64>,
    pub bh: Vec<f64>,
    pub wx: Vec<f64>,
    /// Every stage of the pipeline: x, h, u, y.
    pub trace: Trace,
    /// The formulas of h, u and y, as LaTeX with their symbols marked.
    pub tex: Vec<String>,
}

impl Lab {
    /// The factors, the products and the total of the sum behind number
    /// `k` (from 0) of stage `stage` (from 0: x, h, u, y), or None for
    /// the input, which is given and not worked out. For y the sum is
    /// the frozen table's part, W x; the adapter's part is added to it.
    pub fn sum(&self, stage: usize, k: usize) -> Option<(Vec<f64>, Vec<f64>, Vec<f64>, f64)> {
        let s = &self.trace.stages;
        let (x, h) = (&s[0].items, &s[1].items);
        match stage {
            1 => Some((self.a[k * N..(k + 1) * N].to_vec(), x.clone(), self.ax[k * N..(k + 1) * N].to_vec(), h[k])),
            2 => Some((self.b[k * self.r..(k + 1) * self.r].to_vec(), h.clone(), self.bh[k * self.r..(k + 1) * self.r].to_vec(), s[2].items[k])),
            3 => Some((self.w[k * N..(k + 1) * N].to_vec(), x.clone(), self.wx[k * N..(k + 1) * N].to_vec(), self.base[k])),
            _ => None,
        }
    }
}

/// Run the program at settings `s`.
pub fn run(s: Settings) -> Result<Lab, String> {
    let out = output(&program(s), PRINTED + 4)?;
    let trained = numbers::<usize>(&out[4], 2)?;
    let r = s.r;
    Ok(Lab {
        r,
        y: numbers(&out[0], N)?,
        base: numbers(&out[1], N)?,
        correction: numbers(&out[2], N)?,
        same: out[3].trim() == "1",
        trained: (trained[0], trained[1]),
        s: numbers::<f64>(&out[5], 1)?[0],
        w: numbers(&out[6], N * N)?,
        a: numbers(&out[7], r * N)?,
        b: numbers(&out[8], N * r)?,
        ax: numbers(&out[9], r * N)?,
        bh: numbers(&out[10], N * r)?,
        wx: numbers(&out[11], N * N)?,
        trace: trace(&out[12], pipeline())?,
        tex: out[13..16].to_vec(),
    })
}
