//! The model: the demo's training program (training-at-depth.xtl) run
//! live at a scheme's settings (12 layers, 50 steps), and its replay
//! program (replay.xtl) run on the records the offline trainer wrote
//! (data/SCHEME-L.txt) for a depth; their printed lines, their charts
//! and the Check report read back. Nothing here trains, sums or checks
//! anything: every number is X_eTaL's, the live run's or the offline
//! runs'.

use microscope::run::{numbers, output_pictures, output_with};

/// The training program and the replay program.
pub const PROGRAM: &str = include_str!("../../training-at-depth.xtl");
pub const REPLAY: &str = include_str!("../../replay.xtl");

/// The live run: its depth and steps (the program's own), the
/// gradient entries it checks, and the lines it prints (its chart
/// aside): the record, the NaN count, the written-out and the central
/// gradients, two checks and the summary.
pub const LIVE_L: usize = 12;
pub const LIVE_STEPS: usize = 50;
pub const CHECKED: usize = 9;
pub const PRINTED: usize = 4 + 2 + 1;

/// What a record holds per step, and the offline runs' steps.
pub const FIELDS: usize = 6;
pub const STEPS: usize = 800;
pub const DEPTHS: [usize; 3] = [12, 24, 48];
/// The lines the replay prints (its four charts aside).
pub const REPLAY_PRINTED: usize = 4;

/// The entries the gradient check looks at, in the program's order.
pub const ENTRIES: [&str; CHECKED] = [
    "layer 1, the block's first table",
    "layer 1, the block's second table",
    "layer 2, a logit of H_pre",
    "layer 2, a logit of H_post",
    "layer 2, a logit of H_res",
    "layer 2, the block's first table",
    "layer 12 (the last), the block's second table",
    "the embedding",
    "the unembedding",
];

/// The presets, as data (presets/*.toml): the schemes. The last is the
/// program's own.
pub const PRESETS: [&str; 3] = [
    include_str!("../../presets/baseline.toml"),
    include_str!("../../presets/hc.toml"),
    include_str!("../../presets/mhc.toml"),
];
/// The data files of the offline runs, by scheme slug and depth.
pub const SLUGS: [&str; 3] = ["baseline", "hc", "mhc"];
pub const DATA: [[&str; 3]; 3] = [
    [include_str!("../../data/baseline-12.txt"), include_str!("../../data/baseline-24.txt"), include_str!("../../data/baseline-48.txt")],
    [include_str!("../../data/hc-12.txt"), include_str!("../../data/hc-24.txt"), include_str!("../../data/hc-48.txt")],
    [include_str!("../../data/mhc-12.txt"), include_str!("../../data/mhc-24.txt"), include_str!("../../data/mhc-48.txt")],
];
/// What made them (data/runs.txt).
pub const RUNS: &str = include_str!("../../data/runs.txt");

/// A scheme: its name, what it is, and its two settings.
#[derive(Clone, Debug, PartialEq)]
pub struct Scheme {
    pub title: String,
    pub setup: String,
    pub streams: usize,
    pub sink: String,
}

/// A preset read from its TOML: `key = value` lines, strings quoted.
pub fn preset(toml: &str) -> Result<Scheme, String> {
    let mut s = Scheme { title: String::new(), setup: String::new(), streams: 0, sink: String::new() };
    for line in toml.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let (key, value) = line.split_once('=').ok_or_else(|| format!("preset: not `key = value`: {line}"))?;
        let (key, value) = (key.trim(), value.trim().trim_matches('"').to_string());
        match key {
            "title" => s.title = value,
            "setup" => s.setup = value,
            "streams" => s.streams = value.parse().map_err(|_| format!("preset: streams is not a count: {value}"))?,
            "sink" => s.sink = value,
            other => return Err(format!("preset: unknown key {other}")),
        }
    }
    if s.streams == 0 || s.title.is_empty() || !matches!(s.sink.as_str(), "0.0" | "1.0") {
        return Err("preset: a setting is missing or out of range".into());
    }
    Ok(s)
}

/// Every scheme.
pub fn schemes() -> Vec<Scheme> {
    PRESETS.iter().map(|p| preset(p).expect("a preset")).collect()
}

/// The training program at scheme `s`: two lines change.
pub fn with(s: &Scheme) -> String {
    PROGRAM.replacen("S := 4\n", &format!("S := {}\n", s.streams), 1).replacen("sink := 1.0\n", &format!("sink := {}\n", s.sink), 1)
}

/// The replay program at depth `d`: one line changes.
pub fn replay_with(d: usize) -> String {
    REPLAY.replacen("depth := \"48\"\n", &format!("depth := \"{d}\"\n"), 1)
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

/// A record, a row a step: loss, gradient size, forward gain, backward
/// gain, proxy, NaN or infinite.
#[derive(Clone, Debug, PartialEq)]
pub struct Record(pub Vec<f64>);

impl Record {
    /// Field `k` (0 loss, 1 gradient size, 2 forward gain, 3 backward
    /// gain, 4 proxy, 5 NaN) at step `t` (from 1).
    pub fn at(&self, t: usize, k: usize) -> f64 {
        self.0[(t - 1) * FIELDS + k]
    }
    pub fn steps(&self) -> usize {
        self.0.len() / FIELDS
    }
}

/// The live run.
#[derive(Clone, Debug, PartialEq)]
pub struct Run {
    pub scheme: Scheme,
    pub record: Record,
    pub nans: f64,
    pub written: Vec<f64>,
    pub central: Vec<f64>,
    pub chart: String,
    pub checks: Vec<Check>,
    pub summary: String,
}

/// Run the training program live at scheme `s`.
pub fn run(s: &Scheme) -> Result<Run, String> {
    let (out, pictures) = output_pictures(&with(s), PRINTED)?;
    let chart = pictures.into_iter().next().ok_or("the program drew no chart")?;
    Ok(Run {
        scheme: s.clone(),
        record: Record(numbers(&out[0], LIVE_STEPS * FIELDS)?),
        nans: numbers::<f64>(&out[1], 1)?[0],
        written: numbers(&out[2], CHECKED)?,
        central: numbers(&out[3], CHECKED)?,
        chart,
        checks: (4..6).map(|k| check(&out[k])).collect::<Result<Vec<_>, _>>()?,
        summary: out[6].clone(),
    })
}

/// The replay at one depth.
#[derive(Clone, Debug, PartialEq)]
pub struct Replay {
    pub depth: usize,
    /// Loss, gradient size, forward gain, proxy: one chart each.
    pub charts: Vec<String>,
    /// For the plain skip, HC and mHC: the first step's six fields, the
    /// last step's, the smallest loss and the count of NaN steps.
    pub first: Vec<f64>,
    pub last: Vec<f64>,
    pub least: Vec<f64>,
    pub nans: Vec<f64>,
}

impl Replay {
    /// Field `k` of scheme `s` (0 plain, 1 HC, 2 mHC) at the last step.
    pub fn last(&self, s: usize, k: usize) -> f64 {
        self.last[s * FIELDS + k]
    }
    pub fn first(&self, s: usize, k: usize) -> f64 {
        self.first[s * FIELDS + k]
    }
}

/// The records the offline runs wrote, as the files the replay reads.
pub fn files() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (i, slug) in SLUGS.iter().enumerate() {
        for (j, d) in DEPTHS.iter().enumerate() {
            out.push((format!("data/{slug}-{d}.txt"), DATA[i][j].to_string()));
        }
    }
    out
}

/// Replay the offline runs at depth `d`.
pub fn replay(d: usize) -> Result<Replay, String> {
    let (out, charts) = output_with(&files(), &replay_with(d), REPLAY_PRINTED)?;
    if charts.len() != 4 {
        return Err(format!("the replay drew {} charts, not 4", charts.len()));
    }
    Ok(Replay { depth: d, charts, first: numbers(&out[0], 3 * FIELDS)?, last: numbers(&out[1], 3 * FIELDS)?, least: numbers(&out[2], 3)?, nans: numbers(&out[3], 3)? })
}

/// A recorded run's record, as the trainer wrote it.
pub fn recorded(scheme: usize, depth: usize) -> Result<Record, String> {
    let j = DEPTHS.iter().position(|d| *d == depth).ok_or("no such depth")?;
    Ok(Record(numbers(DATA[scheme][j].trim(), STEPS * FIELDS)?))
}
