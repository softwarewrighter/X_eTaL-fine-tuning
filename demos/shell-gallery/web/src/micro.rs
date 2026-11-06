//! The model: the demo's program (shell-gallery.xtl) run as it is, then
//! asked for the value of every stage of its pipeline (flow:s_tages<),
//! and its printed lines read back. Nothing here computes the layer:
//! every number is X_eTaL's.

use lesson::stepper::{trace, Trace};
use microscope::run::{numbers, output};

/// The command line's program.
pub const PROGRAM: &str = include_str!("../../shell-gallery.xtl");

/// The lines the program prints, then the page's one more.
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
pub struct Gallery {
    /// The pipeline's output, y.
    pub y: Vec<f64>,
    /// The frozen table W by rows (4 x 4), the row of A, the column of B.
    pub w: Vec<f64>,
    pub a: Vec<f64>,
    pub b: Vec<f64>,
    /// Each number of W's first row times the matching number of x, and
    /// their sum (what the frozen table gives for the first output).
    pub row1: Vec<f64>,
    pub row1_sum: f64,
    /// The numbers an adapter trains at rank 1 and at rank 2, and the table's.
    pub counts: (usize, usize, usize),
    /// Every stage of the pipeline, as its run recorded it.
    pub trace: Trace,
}

/// Run the program.
pub fn run() -> Result<Gallery, String> {
    let out = output(&program(), PRINTED + 1)?;
    let counts = numbers::<usize>(&out[6], 3)?;
    Ok(Gallery {
        y: numbers(&out[0], 4)?,
        w: numbers(&out[1], 16)?,
        a: numbers(&out[2], 4)?,
        b: numbers(&out[3], 4)?,
        row1: numbers(&out[4], 4)?,
        row1_sum: numbers::<f64>(&out[5], 1)?[0],
        counts: (counts[0], counts[1], counts[2]),
        trace: trace(&out[7], pipeline())?,
    })
}
