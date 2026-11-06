//! A matrix as a heat map a newcomer can read: X_eTaL-ML's
//! `microscope::heat` colors (red positive, blue negative, near white
//! about 0), with what it lacked for a lesson: a legend, a note saying
//! whether the scale is shared with the maps beside it, cells marked as
//! read or written by the step being shown, and an inspector that says
//! which matrix, row and column a cell is, its value and what it means.
//! The grid is one tab stop; the arrow keys move over its cells, and a
//! tap or a click picks one. Offered back to X_eTaL-ML (its FM5).

use web_sys::KeyboardEvent;
use yew::prelude::*;

pub use microscope::heat::{column, fill, largest, Matrix};

use crate::arithmetic::show;

/// Cell size, in pixels: larger than microscope's, for a finger.
pub const CELL: f64 = 30.0;

/// Room around the grid, so the outlines of marked cells are not cut off.
const PAD: f64 = 3.0;

/// What a step does to a cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mark {
    /// The step reads it.
    Read,
    /// The step writes it.
    Written,
}

/// What the inspector says of cell `k` (by rows, from 0) of `m`: which
/// it is, counted from 1 as the page's tables are, its value, and how
/// to read its color.
pub fn inspect(name: &str, m: &Matrix, k: usize) -> String {
    let (row, col, v) = (k / m.cols + 1, k % m.cols + 1, m.items[k]);
    let place = match (m.rows, m.cols) {
        (1, 1) => name.to_string(),
        (_, 1) => format!("{name}, number {row}"),
        (1, _) => format!("{name}, number {col}"),
        _ => format!("{name}, row {row}, column {col}"),
    };
    let color = if v > 0.0 { "positive, so red" } else if v < 0.0 { "negative, so blue" } else { "zero, so white" };
    format!("{place}: {} ({color})", show(v))
}

/// The legend's five stops, from the most negative to the most
/// positive value the scale shows: each value with its color.
pub fn legend(scale: f64) -> Vec<(f64, String)> {
    [-1.0, -0.5, 0.0, 0.5, 1.0].iter().map(|t| (t * scale, fill(t * scale, scale))).collect()
}

/// What the note under a legend says about its scale.
pub fn scale_note(shared: bool) -> &'static str {
    if shared {
        "The maps side by side here share one scale, so the same color is the same value in each."
    } else {
        "This map has a scale of its own: its colors cannot be compared with another map's."
    }
}

/// Where the arrow keys take the picked cell of an `rows` x `cols`
/// grid, staying inside it; other keys leave it.
pub fn step(picked: usize, rows: usize, cols: usize, key: &str) -> usize {
    let (r, c) = (picked / cols, picked % cols);
    let (r, c) = match key {
        "ArrowLeft" => (r, c.saturating_sub(1)),
        "ArrowRight" => (r, (c + 1).min(cols - 1)),
        "ArrowUp" => (r.saturating_sub(1), c),
        "ArrowDown" => ((r + 1).min(rows - 1), c),
        "Home" => (r, 0),
        "End" => (r, cols - 1),
        _ => (r, c),
    };
    r * cols + c
}

/// The legend of a scale, drawn: five colors with their values, and the
/// note on whether the scale is shared. A group of maps sharing a scale
/// shows it once, under the group.
pub fn key(scale: f64, shared: bool) -> Html {
    let stops = legend(scale).into_iter().map(|(v, color)| html! { <span class="stop"><span class="swatch" style={format!("background: {color}")}></span>{ show(v) }</span> });
    html! {
        <div class="heat-key">
            <div class="legend" aria-label="Color scale">{ for stops }</div>
            <p class="legend-note">{ scale_note(shared) }</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct HeatProps {
    /// The matrix's name in the text ("W").
    pub name: AttrValue,
    /// A caption: what the matrix is.
    pub caption: AttrValue,
    pub matrix: Matrix,
    /// The size the full red and blue stand for (`largest` of the maps
    /// shown together, or of this one alone).
    pub scale: f64,
    /// Whether that scale is shared with the maps beside this one.
    #[prop_or_default]
    pub shared: bool,
    /// Cells the step being shown reads or writes: (cell, mark).
    #[prop_or_default]
    pub marks: Vec<(usize, Mark)>,
    /// What a cell of this matrix means, said by the inspector.
    #[prop_or_default]
    pub meaning: AttrValue,
    /// Whether to draw the legend (once is enough for maps sharing a scale).
    #[prop_or(true)]
    pub legend: bool,
}

#[function_component(Heat)]
pub fn heat(props: &HeatProps) -> Html {
    let picked = use_state(|| None::<usize>);
    let m = &props.matrix;
    let (w, h) = (m.cols as f64 * CELL, m.rows as f64 * CELL);
    let cells = m.items.iter().enumerate().map(|(k, v)| {
        let (y, x) = ((k / m.cols) as f64 * CELL, (k % m.cols) as f64 * CELL);
        let mark = props.marks.iter().find(|(c, _)| *c == k).map(|(_, mk)| match mk {
            Mark::Read => "read",
            Mark::Written => "written",
        });
        let pick = { let picked = picked.clone(); Callback::from(move |_| picked.set(Some(k))) };
        html! {
            <rect x={x.to_string()} y={y.to_string()} width={CELL.to_string()} height={CELL.to_string()} fill={fill(*v, props.scale)}
                class={classes!("cell", mark, (*picked == Some(k)).then_some("picked"))} onclick={pick}>
                <title>{ inspect(&props.name, m, k) }</title>
            </rect>
        }
    });
    let onkeydown = {
        let (picked, rows, cols) = (picked.clone(), m.rows, m.cols);
        Callback::from(move |e: KeyboardEvent| {
            let key = e.key();
            if ["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown", "Home", "End"].contains(&key.as_str()) {
                e.prevent_default();
                picked.set(Some(step(picked.unwrap_or(0), rows, cols, &key)));
            }
        })
    };
    let onfocus = { let picked = picked.clone(); Callback::from(move |_| if picked.is_none() { picked.set(Some(0)) }) };
    html! {
        <figure class="heat lesson-heat">
            <figcaption>{ props.caption.clone() }</figcaption>
            <svg width={(w + 2.0 * PAD).to_string()} height={(h + 2.0 * PAD).to_string()} viewBox={format!("{} {} {} {}", -PAD, -PAD, w + 2.0 * PAD, h + 2.0 * PAD)} role="img" tabindex="0"
                aria-label={format!("{}: a table of {} rows and {} columns. Arrow keys move over its cells.", props.caption, m.rows, m.cols)} {onkeydown} {onfocus}>
                { for cells }
                <rect x="0" y="0" width={w.to_string()} height={h.to_string()} class="frame" />
            </svg>
            <p class="inspector" role="status">{
                match *picked {
                    Some(k) => html! { <>{ inspect(&props.name, m, k) }{ if props.meaning.is_empty() { html! {} } else { html! { <>{". "}{ props.meaning.clone() }</> } } }</> },
                    None => html! { <span class="hint">{"Click a cell, or use the arrow keys, to read it."}</span> },
                }
            }</p>
            if props.legend {
                { key(props.scale, props.shared) }
            }
        </figure>
    }
}
