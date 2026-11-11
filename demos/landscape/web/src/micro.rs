//! The model: the demo's program (landscape.xtl) run once, its
//! accounting read back: for each family whose cost is a count, the
//! numbers trained, held while training and stored afterward, at this
//! course's 8 x 8 layer and at one real 4096 x 4096 layer; the layer's
//! bytes at 16 and at 4 bits; the adapters' shares. Every figure is
//! analytical (it follows from a definition) and every one is
//! X_eTaL's: nothing here multiplies a count.

use lesson::arithmetic::commas;
use microscope::run::{numbers, output};

/// The command line's program.
pub const PROGRAM: &str = include_str!("../../landscape.xtl");

/// The lines the program prints: trained, held, stored (9 rows x 2
/// layers each), the bytes, the shares.
pub const PRINTED: usize = 5;

/// The two layers the program counts for: the width and the rank.
pub const LAYERS: [(usize, usize); 2] = [(8, 4), (4096, 16)];

/// The rows of the accounting, in the program's order: the family each
/// belongs to (a position in `text::FAMILIES`), its name, and a
/// remark on what its stored count leaves out.
pub struct Row {
    pub family: usize,
    pub name: &'static str,
    pub stored_note: &'static str,
}

pub const ROWS: [Row; 9] = [
    Row { family: 1, name: "full fine-tuning", stored_note: "the moved W itself" },
    Row { family: 1, name: "selective fine-tuning, one row in eight", stored_note: "the moved rows, and which they are" },
    Row { family: 2, name: "LoRA", stored_note: "A and B; merged into W, d x d again" },
    Row { family: 2, name: "LoRA-FA", stored_note: "B, and the seed A regrows from" },
    Row { family: 2, name: "QLoRA", stored_note: "A and B; the base at 4 bits, below" },
    Row { family: 2, name: "DoRA", stored_note: "A, B and a length per column" },
    Row { family: 2, name: "VeRA", stored_note: "two short lists, and the seed the shared tables regrow from" },
    Row { family: 3, name: "learning without back-propagation, every weight", stored_note: "the moved W itself; while training, no gradient and no averages, only a seed" },
    Row { family: 4, name: "a closed-form rank-one edit", stored_note: "two lists of d numbers, the input k and the change to its output; applied to W, d x d again" },
];

/// What the run gave: a count per row and layer.
#[derive(Clone, Debug, PartialEq)]
pub struct Run {
    pub trained: Vec<[usize; 2]>,
    pub held: Vec<[usize; 2]>,
    pub stored: Vec<[usize; 2]>,
    /// The layer's bytes at 16 bits at each width, then at 4 bits.
    pub bytes: [f64; 4],
    /// LoRA's share of the full count at each width, then LoRA-FA's.
    pub shares: [f64; 4],
}

fn pairs(line: &str) -> Result<Vec<[usize; 2]>, String> {
    let v = numbers::<f64>(line, 18)?;
    Ok(v.chunks(2).map(|c| [c[0] as usize, c[1] as usize]).collect())
}

/// Run the program.
pub fn run() -> Result<Run, String> {
    let out = output(PROGRAM, PRINTED)?;
    let four = |i: usize| -> Result<[f64; 4], String> {
        let v = numbers::<f64>(&out[i], 4)?;
        Ok([v[0], v[1], v[2], v[3]])
    };
    Ok(Run { trained: pairs(&out[0])?, held: pairs(&out[1])?, stored: pairs(&out[2])?, bytes: four(3)?, shares: four(4)? })
}

/// A byte count as the page quotes it: bytes, then KB or MB when large.
pub fn bytes(b: f64) -> String {
    if b >= 1_000_000.0 {
        format!("{} bytes ({:.1} MB)", commas(b as usize), b / 1_000_000.0)
    } else if b >= 1000.0 {
        format!("{} bytes ({:.1} KB)", commas(b as usize), b / 1000.0)
    } else {
        format!("{} bytes", commas(b as usize))
    }
}

/// A share as the page quotes it: one number in so many.
pub fn share(s: f64) -> String {
    if s >= 1.0 {
        "as many as the layer itself".to_string()
    } else {
        format!("1 in {}", commas((1.0 / s).round() as usize))
    }
}

impl Run {
    /// The row of a method, by its name.
    pub fn row(&self, name: &str) -> usize {
        ROWS.iter().position(|r| r.name == name).expect("a row of the accounting")
    }

    /// The accounting told from its numbers, a sentence a layer, then
    /// the shares.
    pub fn said(&self) -> Vec<String> {
        let (full, lora, fa, vera, edit) = (self.row("full fine-tuning"), self.row("LoRA"), self.row("LoRA-FA"), self.row("VeRA"), self.row("a closed-form rank-one edit"));
        let mut out = Vec::new();
        for (j, (d, r)) in LAYERS.iter().enumerate() {
            out.push(format!(
                "At {d} by {d} with rank {r} (analytical): full fine-tuning trains {} numbers, and while it trains their gradients and Adam's two averages are held beside them, {} more; LoRA trains {} and holds {}; LoRA-FA {} and {}; VeRA {} and {}. A rank-one edit trains {} and holds {}. Stored apart from W afterward: LoRA {}, LoRA-FA {}, VeRA {}, the edit {}; the moved W of full fine-tuning, or any adapter merged into it, {}.",
                commas(self.trained[full][j]), commas(self.held[full][j]),
                commas(self.trained[lora][j]), commas(self.held[lora][j]),
                commas(self.trained[fa][j]), commas(self.held[fa][j]),
                commas(self.trained[vera][j]), commas(self.held[vera][j]),
                commas(self.trained[edit][j]), commas(self.held[edit][j]),
                commas(self.stored[lora][j]), commas(self.stored[fa][j]), commas(self.stored[vera][j]), commas(self.stored[edit][j]), commas(self.stored[full][j])
            ));
        }
        let (d, r) = LAYERS[0];
        let (sel, dora, mezo) = (self.row("selective fine-tuning, one row in eight"), self.row("DoRA"), self.row("learning without back-propagation, every weight"));
        out.push(format!(
            "Worked in full at {d} by {d}, rank {r} (analytical): the layer holds {d} x {d} = {} numbers, all of them trained by full fine-tuning and by the forward-only method ({}); one row in eight is {} / 8 = {}; LoRA's tables are {d} by {r} and {r} by {d}, {r} x ({d} + {d}) = {}; LoRA-FA trains B alone, {r} x {d} = {}; DoRA adds one size per column, {} + {d} = {}; VeRA's two lists are {d} and {r} long, {d} + {r} = {}; a rank-one edit trains nothing and stores two lists of {d}, 2 x {d} = {}. Held under Adam: 3 x {} = {} for full fine-tuning, 3 x {} = {} for LoRA.",
            commas(self.trained[full][0]), commas(self.trained[mezo][0]), commas(self.trained[full][0]), commas(self.trained[sel][0]), commas(self.trained[lora][0]), commas(self.trained[fa][0]), commas(self.trained[lora][0]), commas(self.trained[dora][0]), commas(self.trained[vera][0]), commas(self.stored[edit][0]), commas(self.trained[full][0]), commas(self.held[full][0]), commas(self.trained[lora][0]), commas(self.held[lora][0])
        ));
        out.push(format!(
            "LoRA's share of the layer's count (analytical): {} at 8 by 8, {} at 4096 by 4096; LoRA-FA's {} and {}. On the small layer of these lessons an adapter at rank 4 is not small; on the real layer it is. The layer itself takes {} at 16 bits a number and {} at 4 bits; at 4096 by 4096, {} and {}.",
            share(self.shares[0]), share(self.shares[1]), share(self.shares[2]), share(self.shares[3]),
            bytes(self.bytes[0]), bytes(self.bytes[2]), bytes(self.bytes[1]), bytes(self.bytes[3])
        ));
        out
    }
}
