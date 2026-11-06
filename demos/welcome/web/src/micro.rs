//! The model: the demo's program (welcome.xtl) run as it is, then
//! asked for the value of every stage of its pipeline (flow:s_tages<),
//! and its printed lines read back. Nothing here computes the
//! calculation: every number is X_eTaL's.

use lesson::stepper::{trace, Trace};
use microscope::run::{numbers, output};

/// The command line's program.
pub const PROGRAM: &str = include_str!("../../welcome.xtl");

/// The lines the program prints; the page asks for one more.
pub const PRINTED: usize = 7;

/// The pipeline as the program writes it, from `y := @ flow:p_ipe<` to
/// the end of its text.
pub fn pipeline() -> &'static str {
    let from = PROGRAM.find("y := @ flow:p_ipe<").expect("the pipeline");
    let to = from + PROGRAM[from..].find("\"\"#").expect("its end") + 3;
    &PROGRAM[from..to]
}

/// The program the page runs: the command line's, then the record of
/// every stage of its pipeline.
pub fn program() -> String {
    let text = &pipeline()[pipeline().find("c#").expect("its text")..];
    format!("{}\n# -- read back by the page: every stage's value\n@ flow:s_tages< {text}\n", PROGRAM.trim_end())
}

/// What the run gave.
#[derive(Clone, Debug, PartialEq)]
pub struct Welcome {
    /// The pipeline's output, y.
    pub y: Vec<f64>,
    /// The table W by rows (4 x 4), the one row of A, the one column of B.
    pub w: Vec<f64>,
    pub a: Vec<f64>,
    pub b: Vec<f64>,
    /// Stage 2 written out: each number of A times the matching number
    /// of x, and their sum.
    pub products: Vec<f64>,
    pub sum: f64,
    /// The first number of the output at a strength of 4 instead of 2.
    pub stronger: f64,
    /// Every stage of the pipeline, as its run recorded it.
    pub trace: Trace,
}

/// Run the program.
pub fn run() -> Result<Welcome, String> {
    let out = output(&program(), PRINTED + 1)?;
    Ok(Welcome {
        y: numbers(&out[0], 4)?,
        w: numbers(&out[1], 16)?,
        a: numbers(&out[2], 4)?,
        b: numbers(&out[3], 4)?,
        products: numbers(&out[4], 4)?,
        sum: numbers::<f64>(&out[5], 1)?[0],
        stronger: numbers::<f64>(&out[6], 1)?[0],
        trace: trace(&out[7], pipeline())?,
    })
}
