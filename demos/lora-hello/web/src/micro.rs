//! The model: the demo's program (lora-hello.xtl) run with the input x
//! and alpha the page sets, and its seventeen printed lines read back.
//! Nothing here knows about the browser, and nothing here computes the
//! layer: every number is X_eTaL's.

use lesson::run::output;
use microscope::run::{lit, numbers};

/// The command line's program, which the page runs with its settings.
pub const PROGRAM: &str = include_str!("../../lora-hello.xtl");

/// The lines the program prints.
pub const PRINTED: usize = 17;

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
    /// The frozen table W, row by row (4 x 4).
    pub w: Vec<f64>,
    /// The one row of A.
    pub a: Vec<f64>,
    /// The one column of B.
    pub b: Vec<f64>,
    /// Each number of A times the matching number of x; their sum is h.
    pub products: Vec<f64>,
    /// The squeezed number times the strength, s h.
    pub scaled: f64,
    /// Each number of the first row of W times the matching number of x;
    /// their sum is the first number of `base`.
    pub w_row: Vec<f64>,
    /// The correction written out as a table, s B A, row by row (4 x 4).
    pub table: Vec<f64>,
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
    /// The same count on a 4096 x 4096 layer at rank 16.
    pub big: (usize, usize),
    /// The first of those as a percentage of the second.
    pub percent: f64,
}

/// Run the program at settings `s`.
pub fn run(s: Settings) -> Result<Hello, String> {
    let out = output(&program(s), PRINTED)?;
    let pair = numbers::<f64>(&out[3], 2)?;
    let trained = numbers::<usize>(&out[14], 2)?;
    let big = numbers::<usize>(&out[15], 2)?;
    Ok(Hello {
        w: numbers(&out[0], 16)?,
        a: numbers(&out[1], 4)?,
        b: numbers(&out[2], 4)?,
        s: pair[0],
        h: pair[1],
        products: numbers(&out[4], 4)?,
        scaled: numbers::<f64>(&out[5], 1)?[0],
        w_row: numbers(&out[6], 4)?,
        base: numbers(&out[7], 4)?,
        y: numbers(&out[8], 4)?,
        correction: numbers(&out[9], 4)?,
        adds_up: out[10].trim() == "1",
        table: numbers(&out[11], 16)?,
        rank: out[12].trim().parse().map_err(|_| format!("not a rank: {}", out[12]))?,
        merged_same: out[13].trim() == "1",
        trained: (trained[0], trained[1]),
        big: (big[0], big[1]),
        percent: out[16].trim().parse().map_err(|_| format!("not a percentage: {}", out[16]))?,
    })
}

/// The program's line that starts with `start`, as the program has it
/// (so the page shows the program's own lines, not copies), or "".
pub fn line(start: &str) -> &'static str {
    PROGRAM.lines().find(|l| l.starts_with(start)).unwrap_or("")
}
