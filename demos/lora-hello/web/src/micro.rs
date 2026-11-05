//! The model: the demo's program (lora-hello.xtl) run with the input x
//! and alpha the page sets, and its eight printed lines read back.
//! Nothing here knows about the browser, and nothing here computes the
//! layer: every number is X_eTaL's.

use lesson::run::output;
use microscope::run::{lit, numbers};

/// The command line's program, which the page runs with its settings.
pub const PROGRAM: &str = include_str!("../../lora-hello.xtl");

/// The lines the program prints.
pub const PRINTED: usize = 8;

/// What the page sets: the input's four numbers and alpha (the scale s
/// is alpha / r, and r is 1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub x: [f64; 4],
    pub alpha: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { x: [1.0, 2.0, 3.0, 4.0], alpha: 2.0 }
    }
}

/// The program with the settings put in its data: the two lines that
/// set x and s, nothing else.
pub fn program(s: Settings) -> String {
    let x: Vec<String> = s.x.iter().map(|v| lit(*v)).collect();
    [("x := 1.0 2.0 3.0 4.0\n", format!("x := {}\n", x.join(" "))), ("s := 2.0 lo:s_caling 1\n", format!("s := {} lo:s_caling 1\n", lit(s.alpha)))]
        .iter()
        .fold(PROGRAM.to_string(), |p, (from, to)| p.replacen(from, to, 1))
}

/// What the program printed.
#[derive(Clone, Debug, PartialEq)]
pub struct Hello {
    /// The scale s = alpha / r.
    pub s: f64,
    /// The input squeezed through the adapter, h = A x: one number.
    pub h: f64,
    /// The frozen layer's output, W x.
    pub base: Vec<f64>,
    /// The adapted layer's output, W x + s B (A x).
    pub y: Vec<f64>,
    /// What the adapter adds, s B (A x).
    pub correction: Vec<f64>,
    /// The two outputs differ by exactly the correction.
    pub adds_up: bool,
    /// The rank of the update s B A.
    pub rank: usize,
    /// The merged matrix gives the factored layer's output.
    pub merged_same: bool,
    /// The numbers the adapter trains, and the matrix's.
    pub trained: (usize, usize),
}

/// Run the program at settings `s`.
pub fn run(s: Settings) -> Result<Hello, String> {
    let out = output(&program(s), PRINTED)?;
    let scaled = numbers::<f64>(&out[0], 2)?;
    let trained = numbers::<usize>(&out[7], 2)?;
    Ok(Hello {
        s: scaled[0],
        h: scaled[1],
        base: numbers(&out[1], 4)?,
        y: numbers(&out[2], 4)?,
        correction: numbers(&out[3], 4)?,
        adds_up: out[4].trim() == "1",
        rank: out[5].trim().parse().map_err(|_| format!("not a rank: {}", out[5]))?,
        merged_same: out[6].trim() == "1",
        trained: (trained[0], trained[1]),
    })
}
