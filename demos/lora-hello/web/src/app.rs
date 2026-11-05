//! The page: the input x and alpha set in number fields; the frozen
//! layer's output, the correction the adapter adds and the adapted
//! output, row over row; the update's rank, the merge and the numbers
//! trained; and the program the page ran.

use web_sys::HtmlInputElement;
use yew::prelude::*;

use lesson::chrome::footer;
use microscope::chrome::{header, panel};
use microscope::source::{block, NONE};

use crate::micro::{program, run, Hello, Settings};

/// A number as the page shows it: six decimals at most, no trailing zeros.
fn show(v: f64) -> String {
    let s = format!("{v:.6}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s == "-0" { "0".into() } else { s.to_string() }
}

/// A row of four numbers under a heading; `changed` marks the cells
/// whose value differs from the row `against` (the frozen output).
fn row(name: &str, v: &[f64], against: Option<&[f64]>, class: &'static str) -> Html {
    let cells = v.iter().enumerate().map(|(i, x)| {
        let changed = against.is_some_and(|a| (a[i] - x).abs() > 1e-12);
        html! { <td class={classes!(changed.then_some("changed"))}>{ show(*x) }</td> }
    });
    html! { <tr class={class}><th scope="row">{name.to_string()}</th>{ for cells }</tr> }
}

/// What the run gave, in two panels.
fn results(h: &Hello, s: Settings) -> Html {
    let moved = h.y.iter().zip(&h.base).filter(|(a, b)| (*a - *b).abs() > 1e-12).count();
    html! { <>
        { panel("What the adapter changes", "y := (W, A, B, s) lo:f_orward x", "The input goes two ways. Through the frozen W, as before the adapter existed. And through the adapter: A squeezes the four numbers of x into one, B spreads that one back over the four outputs, and s scales it. The two are added.", false, html! { <>
            <p>{ format!("A x = {}: the input squeezed to one number. Scaled by s = {} (alpha / r) and spread by B, it is what the adapter adds: {}.", show(h.h), show(h.s), h.correction.iter().map(|v| show(*v)).collect::<Vec<_>>().join(", ")) }</p>
            <table class="vectors">
                { row("x: the input", &s.x, None, "") }
                { row("W x: the frozen layer's output", &h.base, None, "") }
                { row("+ s B (A x): what the adapter adds", &h.correction, None, "") }
                { row("= y: the adapted layer's output", &h.y, Some(&h.base), "sum") }
            </table>
            <p class="muted">{ format!("{moved} of the 4 outputs changed (outlined){}. W itself did not change: no number of it is trained.", if h.adds_up { "; the two outputs differ by exactly the correction, checked by X_eTaL" } else { "" }) }</p>
        </> }) }
        { panel("The update, the merge and the cost", "lo:m_erge (W, A, B, s)", "As one matrix, the adapter's update is s B A, 4 x 4, but every row of it is a multiple of A. Folded into W it gives a single ordinary matrix that computes the same layer: how an adapter is deployed.", false, html! {
            <table class="facts">
                <tr><td>{"the update's rank (lo:r_ank, by Gram-Schmidt)"}</td><td>{ h.rank }</td></tr>
                <tr><td>{"the merged matrix W + s B A gives the same output"}</td><td>{ if h.merged_same { "yes (within 1e-12)" } else { "no" } }</td></tr>
                <tr><td>{"numbers the adapter trains, of the matrix's (lo:p_arams)"}</td><td>{ format!("{} of {}", h.trained.0, h.trained.1) }</td></tr>
            </table>
        }) }
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    let settings = use_state(Settings::default);
    let s = *settings;
    let result = use_memo(s, |s| run(*s));
    // A number field: an empty or unfinished entry keeps the last value.
    let field = |label: String, value: f64, set: Callback<f64>| {
        let oninput = Callback::from(move |e: InputEvent| {
            if let Ok(v) = e.target_unchecked_into::<HtmlInputElement>().value().parse::<f64>() {
                if v.is_finite() { set.emit(v); }
            }
        });
        html! { <label>{label}<input type="number" step="0.5" value={show(value)} {oninput} /></label> }
    };
    let inputs = (0..4).map(|i| {
        let st = settings.clone();
        field(format!("x{i}"), s.x[i], Callback::from(move |v| { let mut n = *st; n.x[i] = v; st.set(n); }))
    });
    let alpha = { let st = settings.clone(); field("alpha".into(), s.alpha, Callback::from(move |v| st.set(Settings { alpha: v, ..*st }))) };
    let reset = { let st = settings.clone(); Callback::from(move |_| st.set(Settings::default())) };
    let body = match result.as_ref() {
        Err(e) => html! { <pre class="err">{ e.clone() }</pre> },
        Ok(h) => results(h, s),
    };
    html! {
        <>
        <header>
            { header("LoRA hello", "What does a small adapter add to a layer whose weights do not change? A frozen 4 x 4 matrix W, adapted by a rank-1 update: the layer is y = W x + s B (A x), computed by X_eTaL in your browser. Change the input or alpha and watch what the adapter adds; W never moves.") }
            <div class="controls">
                { for inputs }
                { alpha }
                <button onclick={reset}>{"Reset"}</button>
            </div>
            <p class="muted">{"x0 to x3 are the four numbers of the input. alpha sets how strongly the adapter's correction counts (s = alpha / r, and the rank r is 1 here); at 0 the layer is the frozen one."}</p>
        </header>
        <main>
            { body }
            <section class="panel code">
                <h2>{"The program"}</h2>
                <p class="note">{"Everything the page runs, in your browser, at each setting: lora-hello.xtl, with x and alpha as you set them. Each line without a name on its left prints one of the values above."}</p>
                { block(&program(s), NONE) }
            </section>
        </main>
        { footer() }
        </>
    }
}
