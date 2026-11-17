//! The model: the demo's program (residual-streams.xtl) run at a
//! preset's weight scale and with the mixing table chosen (HC or
//! mHC), then asked for the value of every stage of its one layer
//! (flow:s_tages<) and for the layer drawn as mathematics
//! (flow:m_ath<); its printed lines and its chart read back. Nothing
//! here mixes, sums or sizes anything: every number is X_eTaL's.

use lesson::stepper::{trace, Trace};
use microscope::run::{numbers, output_pictures};

/// The command line's program.
pub const PROGRAM: &str = include_str!("../../residual-streams.xtl");

/// The lines the program prints (its chart aside); the page asks for
/// two more (the layer's record of stages, and the layer drawn as
/// mathematics).
pub const PRINTED: usize = 23;

/// The depth, the width of a list, the number of streams.
pub const L: usize = 48;
pub const D: usize = 8;
pub const S: usize = 4;
/// The block's inner width.
pub const F: usize = 16;

/// The presets, as data (presets/*.toml): the first is the program's own.
pub const PRESETS: [&str; 3] = [
    include_str!("../../presets/small.toml"),
    include_str!("../../presets/larger.toml"),
    include_str!("../../presets/too-large.toml"),
];

/// One run's settings: the scale of the block's weights (a preset) and
/// which table mixes the carried state in the layer stepped through
/// (false: HC, the exponential of the logits; true: mHC, its Sinkhorn
/// projection).
#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    pub title: String,
    pub setup: String,
    pub scale: f64,
    pub mhc: bool,
}

impl Default for Settings {
    fn default() -> Self {
        preset(PRESETS[0]).expect("the first preset")
    }
}

/// A preset read from its TOML: `key = value` lines, strings quoted.
pub fn preset(toml: &str) -> Result<Settings, String> {
    let mut s = Settings { title: String::new(), setup: String::new(), scale: 0.0, mhc: true };
    for line in toml.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let (key, value) = line.split_once('=').ok_or_else(|| format!("preset: not `key = value`: {line}"))?;
        let (key, value) = (key.trim(), value.trim());
        match key {
            "title" => s.title = value.trim_matches('"').to_string(),
            "setup" => s.setup = value.trim_matches('"').to_string(),
            "scale" => s.scale = value.parse::<f64>().map_err(|_| format!("preset: scale is not a number: {value}"))?,
            other => return Err(format!("preset: unknown key {other}")),
        }
    }
    if s.scale <= 0.0 || s.title.is_empty() {
        return Err("preset: a setting is missing or out of range".into());
    }
    Ok(s)
}

/// Every preset, with mHC's table.
pub fn presets() -> Vec<Settings> {
    PRESETS.iter().map(|p| preset(p).expect("a preset")).collect()
}

/// The program with the settings put in its data: two lines change.
pub fn with(s: &Settings) -> String {
    let d = Settings::default();
    PROGRAM
        .replacen(&format!("scale := {}\n", d.scale), &format!("scale := {}\n", s.scale), 1)
        .replacen(&format!("mhc := {}\n", d.mhc as u8), &format!("mhc := {}\n", s.mhc as u8), 1)
}

/// The layer's pipeline as the program writes it, from `layer := @
/// flow:p_ipe<` to the end of its text.
pub fn pipeline() -> &'static str {
    let from = PROGRAM.find("layer := @ flow:p_ipe<").expect("the pipeline");
    let to = from + PROGRAM[from..].find("\"#").expect("its end") + 2;
    &PROGRAM[from..to]
}

/// The program the page runs at settings `s`: the command line's, then
/// the record of the layer's every stage, then the layer drawn as math.
pub fn program(s: &Settings) -> String {
    let text = &pipeline()[pipeline().find("c#").expect("its text")..];
    format!("{}\n# -- read back by the page: every stage of the layer, then the layer drawn as mathematics\n@ flow:s_tages< {text}\n@ flow:m_ath< {text}\n", with(s).trim_end())
}

/// What the run gave.
#[derive(Clone, Debug, PartialEq)]
pub struct Run {
    pub settings: Settings,
    /// The list sent in and the one a hundredth away from it (D each).
    pub x0: Vec<f64>,
    pub x1: Vec<f64>,
    /// After each of the L layers, without the skip and with it: the
    /// size of the list and the gap between the two lists as a share
    /// of the gap sent in.
    pub plain_size: Vec<f64>,
    pub plain_gap: Vec<f64>,
    pub skip_size: Vec<f64>,
    pub skip_gap: Vec<f64>,
    /// The chart of the two gaps (an SVG document X_eTaL drew).
    pub chart: String,
    /// The logits 2I - 1 (S x S), H_pre (S x S), H_post (S), H_res for
    /// HC (S x S) and for mHC (S x S).
    pub logits: Vec<f64>,
    pub h_pre: Vec<f64>,
    pub h_post: Vec<f64>,
    pub h_res_hc: Vec<f64>,
    pub h_res_mhc: Vec<f64>,
    /// What H_res's rows and columns sum to, HC then mHC (S each).
    pub rows_hc: Vec<f64>,
    pub cols_hc: Vec<f64>,
    pub rows_mhc: Vec<f64>,
    pub cols_mhc: Vec<f64>,
    /// The stream state at the start (S x D) and after the first layer.
    pub x_start: Vec<f64>,
    pub x_after_1: Vec<f64>,
    /// The four products that make the first number of H_pre X (row 1
    /// of H_pre times column 1 of X) and of H_res X, for the worked lines.
    pub products_pre: Vec<f64>,
    pub products_res: Vec<f64>,
    /// The sum of all the numbers of X and of H_res X.
    pub total_x: f64,
    pub total_r: f64,
    /// The state after the second layer, the pipeline's result (S x D).
    pub x_after_2: Vec<f64>,
    /// How far the pipeline's result is from the same layer by matrix
    /// products (0 to rounding).
    pub off: f64,
    /// Every stage of the layer: X, P, m, z, h, f, Fs, R, X2.
    pub trace: Trace,
    /// The layer drawn as mathematics (LaTeX, a row a stage).
    pub tex: String,
}

impl Run {
    /// The values of the layer's stage named `label`.
    pub fn stage(&self, label: &str) -> &[f64] {
        self.trace.stages.iter().find(|s| s.label == label).map(|s| s.items.as_slice()).unwrap_or(&[])
    }
    /// H_res as the layer used it: HC's or mHC's.
    pub fn h_res(&self) -> &[f64] {
        if self.settings.mhc { &self.h_res_mhc } else { &self.h_res_hc }
    }
}

/// Run the program at settings `s`.
pub fn run(s: &Settings) -> Result<Run, String> {
    let (out, pictures) = output_pictures(&program(s), PRINTED + 2)?;
    let chart = pictures.into_iter().next().ok_or("the program drew no chart")?;
    Ok(Run {
        settings: s.clone(),
        x0: numbers(&out[0], D)?,
        x1: numbers(&out[1], D)?,
        plain_size: numbers(&out[2], L)?,
        plain_gap: numbers(&out[3], L)?,
        skip_size: numbers(&out[4], L)?,
        skip_gap: numbers(&out[5], L)?,
        chart,
        logits: numbers(&out[6], S * S)?,
        h_pre: numbers(&out[7], S * S)?,
        h_post: numbers(&out[8], S)?,
        h_res_hc: numbers(&out[9], S * S)?,
        h_res_mhc: numbers(&out[10], S * S)?,
        rows_hc: numbers(&out[11], S)?,
        cols_hc: numbers(&out[12], S)?,
        rows_mhc: numbers(&out[13], S)?,
        cols_mhc: numbers(&out[14], S)?,
        x_start: numbers(&out[15], S * D)?,
        x_after_1: numbers(&out[16], S * D)?,
        products_pre: numbers(&out[17], S)?,
        products_res: numbers(&out[18], S)?,
        total_x: numbers::<f64>(&out[19], 1)?[0],
        total_r: numbers::<f64>(&out[20], 1)?[0],
        x_after_2: numbers(&out[21], S * D)?,
        off: numbers::<f64>(&out[22], 1)?[0],
        trace: trace(&out[23], pipeline())?,
        tex: out[24].clone(),
    })
}
