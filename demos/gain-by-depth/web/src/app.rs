//! The page, lesson D3 of the depth track, for new eyes: what one
//! mixing table does when it is applied at every one of 48 layers. The
//! question, the background, the rationale and the purpose; part 1,
//! the two tables and the shift of the logits (a slider); part 2, the
//! composite stepped through layer by layer with its two gains and the
//! proxy; part 3, X_eTaL's chart, the run at the Python's three depths
//! beside the Python's measured run and the paper's quoted figures,
//! the sweep across the slider and the checks; the summary and what is
//! next; the code (Advanced). Presentation only: every number is one
//! X_eTaL printed, and Rust never multiplies, sums or measures anything.

use web_sys::HtmlInputElement;
use yew::prelude::*;

use lesson::arithmetic::{rounded, show};
use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::explain::{table, words};
use lesson::heat::{largest, Heat, Matrix};
use lesson::link::number;
use lesson::mode::{when, Mode, Switch};
use lesson::stepper::Stepper;
use microscope::chrome::picture;
use microscope::run::now;

use crate::micro::{run, with, Run, DEPTHS, L, PAPER, PROGRAM, S, SHIFTS};
use crate::text;

/// A numbered part of the lesson.
fn part(n: usize, title: &str, body: Html) -> Html {
    html! {
        <section class="panel part">
            <h2><span class="n">{ n }</span>{ title.to_string() }</h2>
            { body }
        </section>
    }
}

/// A number as the prose quotes it: six decimals, or in powers of ten
/// when very large or very small.
fn cut(v: f64) -> String {
    let a = v.abs();
    if v == 0.0 {
        "0".into()
    } else if a >= 1e6 || a < 1e-4 {
        format!("{v:.2e}")
    } else {
        show(rounded(v, 6))
    }
}

/// A power of ten as the prose quotes it, to three decimals.
fn ten(v: f64) -> String {
    show(rounded(v, 3))
}

/// A sentence with its `{marks}` filled from the run, then drawn as prose.
fn said(text: &str, fills: &[(&str, String)]) -> Html {
    let mut s = text.to_string();
    for (mark, value) in fills {
        s = s.replace(&format!("{{{mark}}}"), value);
    }
    prose(&s)
}

fn matrix(items: &[f64]) -> Matrix {
    Matrix { rows: S, cols: S, items: items.to_vec() }
}

/// A picture of a table, numbers written in.
fn heat(name: &str, caption: String, m: Matrix, scale: f64) -> Html {
    // Numbers too wide for a cell (HC's composite reaches 10^27) are not written in; a tap reads them.
    let numbers = (0.01..1000.0).contains(&scale);
    html! { <Heat name={name.to_string()} {caption} matrix={m} {scale} shared=false legend=true {numbers} cell=48.0 marks={Vec::new()} hint=false /> }
}

/// The slider over the shifts.
fn slider(at: usize, onat: Callback<usize>) -> Html {
    let oninput = Callback::from(move |e: InputEvent| {
        if let Ok(k) = e.target_unchecked_into::<HtmlInputElement>().value().parse::<usize>() {
            onat.emit(k);
        }
    });
    html! { <label class="rank">{"shift of every logit"}<input type="range" min="0" max={(SHIFTS.len() - 1).to_string()} step="1" value={at.to_string()} {oninput} aria-label="The shift added to every logit, -2 to 1 by a quarter; 0 is the paper's start" /><output>{ show(SHIFTS[at]) }</output></label> }
}

/// Part 1: the two tables and the shift.
fn tables(r: &Run, at: usize, onat: Callback<usize>) -> Html {
    let s = r.hc.rows[0];
    let fills = [
        ("s", cut(s)),
        ("sm", cut(r.mhc.rows[0])),
        ("s48", cut(r.hc.forward[L - 1])),
        ("same", format!("its numbers are {} on the diagonal and {} elsewhere at every setting of the slider", show(rounded(r.mhc.table[0], 6)), show(rounded(r.mhc.table[1], 6)))),
    ];
    let (lg, hc, hm) = (matrix(&r.logits), matrix(&r.hc.table), matrix(&r.mhc.table));
    let rows = |name: &str, v: &[f64]| html! { <tr><th scope="row">{ name.to_string() }</th><td>{ v.iter().map(|x| format!("{x:.6}")).collect::<Vec<_>>().join(", ") }</td></tr> };
    html! { <>
        { p(text::TABLES[0]) }
        { slider(at, onat) }
        <p>{ said(text::TABLES[1], &fills) }</p>
        <div class="tables">
            { heat("logits", format!("the logits: 2I - 1, every one shifted by {}", show(r.shift)), lg.clone(), largest(&[&lg])) }
            { heat("HC", "HC's table: e raised to each logit".into(), hc.clone(), largest(&[&hc])) }
            { heat("mHC", "mHC's table: HC's after twenty rounds of Sinkhorn-Knopp".into(), hm.clone(), largest(&[&hm])) }
        </div>
        <p class="note">{"Each picture has its own scale, its colors under it: blue for a negative number through white to red for a positive one; only the logits have negatives."}</p>
        <div class="scroll">
            <table class="vectors sums">
                <caption>{"The four row sums of each table, from the run"}</caption>
                <tbody>{ rows("HC", &r.hc.rows) }{ rows("mHC", &r.mhc.rows) }</tbody>
            </table>
        </div>
        <p>{ said(text::TABLES[2], &fills) }</p>
        <p>{ said(text::TABLES[3], &fills) }</p>
    </> }
}

/// Part 2: the composite, layer by layer.
fn layers(r: &Run, step: usize, onstep: Callback<usize>) -> Html {
    let k = step + 1;
    let fills = [
        ("k", k.to_string()),
        ("fHC", cut(r.hc.forward[k - 1])),
        ("bHC", cut(r.hc.backward[k - 1])),
        ("qHC", ten(r.hc.proxy[k - 1])),
        ("fHM", cut(r.mhc.forward[k - 1])),
        ("bHM", cut(r.mhc.backward[k - 1])),
        ("qHM", ten(r.mhc.proxy[k - 1])),
        ("s", cut(r.hc.rows[0])),
        ("f2", cut(r.hc.forward[1])),
    ];
    let (hc, hm) = (matrix(r.hc.product(k)), matrix(r.mhc.product(k)));
    html! { <>
        { p(text::LAYERS[0]) }
        <Stepper len={L} {step} {onstep} unit="Layer" />
        <p class="stage-says" role="status" key={format!("says-{step}")}>{ said(if k == 1 { text::LAYER[1] } else { text::LAYER[0] }, &fills) }</p>
        <div class="tables" key={format!("layer-{step}")}>
            { heat("HC to the power k", format!("HC's composite after layer {k}: {k} copies of its table multiplied together{}", if (0.01..1000.0).contains(&largest(&[&hc])) { "" } else if largest(&[&hc]) >= 1000.0 { " (its numbers are too long to write in: tap or click a cell to read one)" } else { " (its numbers are too small to write in: tap or click a cell to read one)" }), hc.clone(), largest(&[&hc])) }
            { heat("mHC to the power k", format!("mHC's composite after layer {k}"), hm.clone(), largest(&[&hm])) }
        </div>
        <p>{ said(text::LAYERS[1], &fills) }</p>
        <p>{ said(text::LAYERS[2], &fills) }</p>
    </> }
}

/// Part 3: the gain at 12, 24 and 48 layers, the sweep and the checks.
fn depth(r: &Run, at: usize) -> Html {
    let q = |c: &[f64], d: usize| ten(c[d - 1]);
    let fills = [
        ("limit", ten(r.mhc.proxy[L - 1])),
        ("hcway", if r.hc.rows[0] > 1.0 { "climb".into() } else { "fall".into() }),
        ("q12", q(&r.hc.proxy, 12)),
        ("q24", q(&r.hc.proxy, 24)),
        ("q48", q(&r.hc.proxy, 48)),
        ("m12", q(&r.mhc.proxy, 12)),
        ("m24", q(&r.mhc.proxy, 24)),
        ("m48", q(&r.mhc.proxy, 48)),
        ("s", cut(r.hc.rows[0])),
        ("s12", cut(r.hc.forward[11])),
        ("s48", cut(r.hc.forward[L - 1])),
        ("fm48", cut(r.mhc.forward[L - 1])),
        ("lowS", cut(r.sweep[2])),
        ("highS", cut(r.sweep[3])),
        ("lowG", cut(r.sweep_gain[2])),
        ("highG", cut(r.sweep_gain[3])),
        ("summary", r.summary.clone()),
        ("g48p", cut(r.sweep_gain[PAPER])),
    ];
    // The Python's run (mHC-poc, docs/results.md), after 800 steps of training, its proxy.
    let python_hc = ["7.0", "14.4", "27.3"];
    let python_mhc = ["-0.5", "-0.6", "-0.6"];
    let rows = DEPTHS.iter().enumerate().map(|(i, d)| html! {
        <tr><th scope="row">{ format!("{d} layers") }</th>
            <td>{ cut(r.hc.forward[d - 1]) }</td><td>{ q(&r.hc.proxy, *d) }</td><td class="theirs">{ python_hc[i] }</td>
            <td>{ cut(r.mhc.forward[d - 1]) }</td><td>{ q(&r.mhc.proxy, *d) }</td><td class="theirs">{ python_mhc[i] }</td></tr>
    });
    let sweep = SHIFTS.iter().enumerate().map(|(i, c)| html! {
        <tr class={classes!((i == at).then_some("on"))}><th scope="row">{ show(*c) }</th><td>{ cut(r.sweep[i]) }</td><td>{ cut(r.sweep_gain[i]) }</td></tr>
    });
    let lines = r.checks.iter().map(|c| html! {
        <li class={classes!("check", if c.ok { "ok" } else { "fail" })}><code>{ c.line.clone() }</code></li>
    });
    let grows = r.hc.rows[0] > 1.0;
    html! { <>
        <p>{ said(text::DEPTH[0], &fills) }</p>
        { picture(&r.chart, "The gain by depth, in powers of ten: HC's and mHC's Amax forward gain and their proxy, layer by layer") }
        <p>{ said(text::DEPTH[1], &fills) }</p>
        <div class="scroll">
            <table class="vectors depths">
                <caption>{"At the Python's three depths: this run, analytical (the forward gain and the proxy), beside the Python's run, measured after training (its proxy)"}</caption>
                <thead><tr><th></th><th scope="col">{"HC, forward gain"}</th><th scope="col">{"HC, proxy"}</th><th scope="col">{"HC, the Python's run"}</th><th scope="col">{"mHC, forward gain"}</th><th scope="col">{"mHC, proxy"}</th><th scope="col">{"mHC, the Python's run"}</th></tr></thead>
                <tbody>{ for rows }</tbody>
            </table>
        </div>
        <p>{ said(if at == PAPER { text::DEPTH[2] } else { text::DEPTH[3] }, &fills) }</p>
        <p>{ said(text::DEPTH[4], &fills) }</p>
        <h3>{"Across the slider"}</h3>
        <div class="said" role="status">
            <p>{ said(if grows { text::SHIFT[0] } else { text::SHIFT[1] }, &fills) }</p>
            <p>{ said(text::SHIFT[2], &fills) }</p>
        </div>
        <div class="scroll">
            <table class="vectors sweep">
                <caption>{"HC's row sum and its forward gain after 48 layers at every setting of the slider, analytical, from the run (the setting chosen outlined)"}</caption>
                <thead><tr><th scope="col">{"shift"}</th><th scope="col">{"row sum"}</th><th scope="col">{"forward gain after 48 layers"}</th></tr></thead>
                <tbody>{ for sweep }</tbody>
            </table>
        </div>
        <h3>{"Checked"}</h3>
        <p>{ said(text::CHECKED, &fills) }</p>
        <ul class="checks">{ for lines }</ul>
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    // Where the page starts may be named in its address (lesson::link):
    // ?mode=explorer&shift=4&layer=12 (the shift's place on the slider
    // from 1, the layer from 1).
    let mode = use_state(|| Mode::from_link().unwrap_or(Mode::Beginner));
    let at = use_state(|| number("shift", 1, SHIFTS.len()).map_or(PAPER, |n| n - 1));
    let step = use_state(|| number("layer", 1, L).map_or(0, |n| n - 1));
    let c = SHIFTS[*at];
    // The run, and how long X_eTaL took over it in this browser (read by
    // the tooling from the document; shown nowhere).
    let result = use_memo(*at, |at| {
        let t0 = now();
        let r = run(SHIFTS[*at]);
        let ms = now() - t0;
        if let Some(doc) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) {
            let _ = doc.set_attribute("data-xetal-ms", &format!("{ms:.0}"));
        }
        r
    });
    let linker = Linker::of(PROGRAM);
    let onmode = { let mode = mode.clone(); Callback::from(move |m| mode.set(m)) };
    let onstep = { let step = step.clone(); Callback::from(move |s| step.set(s)) };
    let onat = { let at = at.clone(); Callback::from(move |i| at.set(i)) };
    let body = match result.as_ref() {
        Err(e) => html! { <pre class="err">{ e.clone() }</pre> },
        Ok(r) => html! { <>
            { part(1, "Two tables, and a shift of the logits", tables(r, *at, onat)) }
            { part(2, "Layer by layer", layers(r, *step, onstep)) }
            { part(3, "The gain at 12, 24 and 48 layers", depth(r, *at)) }
        </> },
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("gain-by-depth".into())}>
        <header>
            { header("The gain by depth", text::LEDE) }
        </header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                { p(text::QUESTION[0]) }
                { p(text::QUESTION[1]) }
                <p class="hint">{"This is the third page of the depth track, after "}<a href="../residual-streams/">{"Residual streams"}</a>{" and "}<a href="../sinkhorn/">{"Sinkhorn-Knopp"}</a>{". "}<a href="../welcome/">{"The welcome page"}</a>{" shows how the lessons work. A word with a dotted underline opens a plain explanation. The paper: "}<a href="https://arxiv.org/abs/2512.24880">{"arXiv:2512.24880"}</a>{"; the Python re-implementation this track follows, and its measured results: "}<a href="https://github.com/softwarewrighter/mHC-poc">{"mHC-poc"}</a>{"."}</p>
            </> }) }
            { frame(Frame::Background, "What you need before", html! { <>{ for text::BACKGROUND.iter().map(|t| p(t)) }</> }) }
            { frame(Frame::Rationale, "Why work it out from the tables alone", p(text::RATIONALE)) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ p(text::PURPOSE[0]) }{ p(text::PURPOSE[1]) }</> }) }
            <section class="panel">
                <h2>{"How much to show"}</h2>
                <Switch mode={*mode} onchange={onmode} />
                <p class="note">{"Advanced adds the program. Switching keeps the shift and the layer you are on."}</p>
            </section>
            { body }
            { frame(Frame::Summary, "What this showed, and what it did not", html! { <ul>{ for text::SUMMARY.iter().map(|t| li(t)) }</ul> }) }
            { frame(Frame::Next, "Where next", html! { <>
                { p(text::NEXT) }
                <p><a class="go" href="../">{"The catalog of lessons"}</a></p>
            </> }) }
            <section class="panel">
                <h2>{"Every idea on this page, explained simply"}</h2>
                <p class="note">{"The same explanations the underlined words open, in one place."}</p>
                { index(&texts) }
            </section>
            { when(*mode, Mode::Advanced, html! {
                <section class="panel code" id="code">
                    <h2>{"The code, for those who want it"}</h2>
                    { p(text::CODE[0]) }
                    <h3>{"The library functions the program uses"}</h3>
                    { table(PROGRAM) }
                    <h3>{"The words of X_eTaL itself that it uses"}</h3>
                    { words(PROGRAM) }
                    <h3>{"The program"}</h3>
                    { p(text::CODE[1]) }
                    { linker.block(&with(c)) }
                </section>
            }) }
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
