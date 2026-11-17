//! The page, lesson D1 of the depth track, for new eyes: why a deep
//! network needs a skip connection, and what four streams do to one.
//! The question, the background, the rationale and the purpose; part
//! 1, a list through 48 layers with the skip and without at three
//! weight scales (X_eTaL's chart, the run quoted at five depths);
//! part 2, four streams and the three mixing tables, HC's and mHC's
//! H_res with their row and column sums; part 3, one layer stepped
//! through in nine stages with every number shown; the summary and
//! what is next; the code (Advanced). Presentation only: every number
//! is one X_eTaL printed, and Rust never mixes, sums or sizes anything.

use yew::prelude::*;

use lesson::arithmetic::{list, rounded, show, written_to};
use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::explain::{table, words};
use lesson::heat::{key, largest, Heat, Mark, Matrix};
use lesson::link::{number, param};
use lesson::mode::{when, Mode, Switch};
use lesson::stepper::{flow, Stepper};
use microscope::chrome::picture;
use microscope::run::now;

use crate::micro::{pipeline, presets, program, run, Run, Settings, D, F, L, PROGRAM, S};
use crate::text;

/// The nine stages of the layer as the page tells them (the pipeline's
/// labels), and which earlier stage each is told from (the picture's dots).
const STAGES: [&str; 9] = ["X", "P", "m", "z", "h", "f", "Fs", "R", "X2"];
const READS: [&[usize]; 9] = [&[], &[0], &[1], &[2], &[3], &[4], &[5], &[0], &[6, 7]];
/// The depths the run is quoted at.
const DEPTHS: [usize; 5] = [1, 6, 12, 24, 48];

/// How many decimals a quoted number shows (as X_eTaL prints them).
const PLACES: usize = 6;

/// A numbered part of the lesson.
fn part(n: usize, title: &str, body: Html) -> Html {
    html! {
        <section class="panel part">
            <h2><span class="n">{ n }</span>{ title.to_string() }</h2>
            { body }
        </section>
    }
}

/// A number as the prose quotes it.
fn cut(v: f64) -> String {
    show(rounded(v, PLACES))
}

/// A gap as the prose quotes it: 0 is said as 0; a small one in powers of ten.
fn gap(v: f64) -> String {
    if v == 0.0 {
        "0".into()
    } else if v < 0.001 {
        format!("{v:.1e}")
    } else {
        show(rounded(v, 4))
    }
}

/// A sentence with its `{marks}` filled from the run, then drawn as prose.
fn said(text: &str, fills: &[(&str, String)]) -> Html {
    let mut s = text.to_string();
    for (mark, value) in fills {
        s = s.replace(&format!("{{{mark}}}"), value);
    }
    prose(&s)
}

/// Column `j` of a stream state (S rows of D), as the run printed it.
fn column(items: &[f64], j: usize) -> Vec<f64> {
    (0..S).filter_map(|s| items.get(s * D + j).copied()).collect()
}

fn matrix(rows: usize, cols: usize, items: &[f64]) -> Matrix {
    Matrix { rows, cols, items: items.to_vec() }
}

/// A picture of a table, numbers written in, at a shared scale.
fn heat(name: &str, caption: String, m: Matrix, scale: f64, marks: Vec<(usize, Mark)>, cell: f64) -> Html {
    html! { <Heat name={name.to_string()} {caption} matrix={m} {scale} shared=true legend=false numbers=true {cell} {marks} hint=false /> }
}

/// Part 1: the list through 48 layers, with the skip and without.
fn stack(r: &Run, chosen: usize, onpreset: Callback<usize>) -> Html {
    let s = &r.settings;
    let buttons = presets().into_iter().enumerate().map(|(i, p)| {
        let onclick = { let onpreset = onpreset.clone(); Callback::from(move |_| onpreset.emit(i)) };
        html! { <button type="button" class={classes!("preset", (i == chosen).then_some("on"))} {onclick} aria-pressed={(i == chosen).to_string()}><b>{ p.title.clone() }</b><span>{ p.setup.clone() }</span></button> }
    });
    let (x0, x1) = (matrix(1, D, &r.x0), matrix(1, D, &r.x1));
    let inputs = largest(&[&x0, &x1]);
    let at = |v: &[f64], k: usize| v[k - 1];
    let fills = [
        ("p1", gap(at(&r.plain_gap, 1))),
        ("p12", gap(at(&r.plain_gap, 12))),
        ("p48", gap(at(&r.plain_gap, 48))),
        ("s1", gap(at(&r.skip_gap, 1))),
        ("s12", gap(at(&r.skip_gap, 12))),
        ("s48", gap(at(&r.skip_gap, 48))),
        ("psize48", cut(at(&r.plain_size, 48))),
        ("ssize48", cut(at(&r.skip_size, 48))),
    ];
    // What else the run says: a gap gone to exactly 0, or gaps grown past 1 both ways.
    let gone = r.plain_gap.iter().any(|v| *v == 0.0) || r.skip_gap.iter().any(|v| *v == 0.0);
    let grown = at(&r.plain_gap, L) > 10.0 && at(&r.skip_gap, L) > 10.0;
    let lists = [("x0", list(&r.x0.iter().map(|v| rounded(*v, 3)).collect::<Vec<_>>())), ("x1", list(&r.x1.iter().map(|v| rounded(*v, 3)).collect::<Vec<_>>()))];
    let rows = DEPTHS.iter().map(|k| html! {
        <tr><th scope="row">{ format!("after layer {k}") }</th><td>{ gap(at(&r.plain_gap, *k)) }</td><td>{ gap(at(&r.skip_gap, *k)) }</td><td>{ cut(at(&r.plain_size, *k)) }</td><td>{ cut(at(&r.skip_size, *k)) }</td></tr>
    });
    html! { <>
        <p>{ said(text::STACK[0], &lists) }</p>
        <div class="tables">
            { heat("x", "the list sent in: 8 numbers".into(), x0, inputs, vec![], 40.0) }
            { heat("x'", "the second list: the same but for a hundredth in its first number".into(), x1, inputs, vec![(0, Mark::Written)], 40.0) }
        </div>
        { key(inputs, true) }
        { p(text::STACK[1]) }
        { p(text::STACK[2]) }
        <div class="presets">{ for buttons }</div>
        <p class="setup"><b>{ s.title.clone() }</b>{ format!(": {}; {L} layers, a block {D} -> {F} -> {D} with GELU in each, two lists sent through, both ways.", s.setup) }</p>
        { p(text::STACK[3]) }
        { picture(&r.chart, "The gap between the two lists after each layer, as a share of the gap sent in, on a log scale: the blocks replacing the list and the blocks adding to it") }
        <div class="scroll">
            <table class="vectors depths">
                <caption>{"The run at five depths, measured: the gap as a share of the gap sent in, and the size of the list"}</caption>
                <thead><tr><th></th><th scope="col">{"gap, replaced"}</th><th scope="col">{"gap, added to"}</th><th scope="col">{"size, replaced"}</th><th scope="col">{"size, added to"}</th></tr></thead>
                <tbody>{ for rows }</tbody>
            </table>
        </div>
        <div class="said" role="status">
            <p>{ said(text::READING[0], &fills) }</p>
            <p>{ said(text::READING[1], &fills) }</p>
            { if gone { p(text::READING[2]) } else { html! {} } }
            { if grown { p(text::READING[3]) } else { html! {} } }
            <p class="note">{ format!("Said from the run alone: {L} layers, the two lists {} apart at the start, every figure as X_eTaL printed it.", show(r.x1[0] - r.x0[0])) }</p>
        </div>
    </> }
}

/// Part 2: four streams and the three mixing tables.
fn streams(r: &Run, onmhc: Callback<bool>) -> Html {
    let mhc = r.settings.mhc;
    let fills = [
        ("hpre0", cut(r.h_pre[0])),
        ("hpre1", cut(r.h_pre[1])),
        ("hpost0", cut(r.h_post[0])),
        ("hpost1", cut(r.h_post[1])),
        ("e", show(rounded(r.h_res_hc[0], 3))),
        ("inv", show(rounded(r.h_res_hc[1], 3))),
        ("rowsum", show(rounded(r.rows_hc[0], 2))),
    ];
    let (lg, pre, post) = (matrix(S, S, &r.logits), matrix(S, S, &r.h_pre), matrix(1, S, &r.h_post));
    let (hc, m) = (matrix(S, S, &r.h_res_hc), matrix(S, S, &r.h_res_mhc));
    let scale = largest(&[&lg, &pre, &post, &hc, &m]);
    let (start, after) = (matrix(S, D, &r.x_start), matrix(S, D, &r.x_after_1));
    let states = largest(&[&start, &after]);
    let sums = |v: &[f64]| v.iter().map(|x| format!("{x:.6}")).collect::<Vec<_>>().join(", ");
    let pick = |which: bool, label: &str, about: &str| {
        let onchange = { let onmhc = onmhc.clone(); Callback::from(move |_| onmhc.emit(which)) };
        html! { <label class="pick"><input type="radio" name="hres" checked={mhc == which} {onchange} />{ format!(" {label}") }<span class="about">{ format!(" {about}") }</span></label> }
    };
    html! { <>
        { p(text::STREAMS[0]) }
        <div class="tables">
            { heat("X at the start", "the stream state at the start: the list sent in, in each of 4 streams (4 rows, 8 columns)".into(), start, states, vec![], 36.0) }
            { heat("X after layer 1", format!("the stream state after the first layer, with {}'s table: the rows differ", if mhc { "mHC" } else { "HC" }), after, states, vec![], 36.0) }
        </div>
        { key(states, true) }
        <p>{ said(text::STREAMS[1], &fills) }</p>
        <div class="tables">
            { heat("logits", "the logits, 2I - 1: what the three tables are made from".into(), lg, scale, vec![], 40.0) }
            { heat("H_pre", "H_pre: the logits put through a softmax, row by row; each row sums to 1".into(), pre, scale, vec![], 40.0) }
            { heat("H_post", "H_post: the shares of the block's output, one per stream; they sum to 1".into(), post, scale, vec![], 40.0) }
        </div>
        <p>{ said(text::STREAMS[2], &fills) }</p>
        <div class="tables">
            { heat("H_res, HC", "H_res for HC: the exponential of the logits, as it is".into(), hc, scale, vec![], 40.0) }
            { heat("H_res, mHC", "H_res for mHC: the same, projected by 20 rounds of Sinkhorn-Knopp".into(), m, scale, vec![], 40.0) }
        </div>
        { key(scale, true) }
        <div class="scroll">
            <table class="vectors sums">
                <caption>{"What H_res's rows and columns sum to, from the run"}</caption>
                <thead><tr><th></th><th scope="col">{"the four row sums"}</th><th scope="col">{"the four column sums"}</th></tr></thead>
                <tbody>
                    <tr><th scope="row">{"HC"}</th><td>{ sums(&r.rows_hc) }</td><td>{ sums(&r.cols_hc) }</td></tr>
                    <tr><th scope="row">{"mHC"}</th><td>{ sums(&r.rows_mhc) }</td><td>{ sums(&r.cols_mhc) }</td></tr>
                </tbody>
            </table>
        </div>
        <p>{ said(text::STREAMS[3], &fills) }</p>
        <p>{ said(text::STREAMS[4], &fills) }</p>
        <p>{ said(text::STREAMS[5], &fills) }</p>
        <fieldset class="hres">
            <legend>{"The table that mixes the carried state in the layer below"}</legend>
            { pick(false, "HC", &format!("the exponential of the logits, rows summing to {}", show(rounded(r.rows_hc[0], 2)))) }
            { pick(true, "mHC", &format!("its Sinkhorn projection, rows and columns summing to {}", show(rounded(r.rows_mhc[0], 2)))) }
        </fieldset>
    </> }
}

/// The pictures of a told stage: the stage's own value, outlined as
/// written, beside what it was made from.
fn pictures(r: &Run, step: usize) -> Html {
    let shape = |label: &str| -> (usize, usize) {
        match label {
            "m" | "f" => (1, D),
            "z" | "h" => (1, F),
            "H_pre" | "H_res" => (S, S),
            "H_post" => (1, S),
            _ => (S, D),
        }
    };
    let caption = |label: &str| -> String {
        match label {
            "X" => "X: the stream state the layer is given (4 streams, 8 numbers each)".into(),
            "P" => "P = H_pre X: the streams mixed before the block".into(),
            "m" => "m: the average of P's four rows, the block's input".into(),
            "z" => "z = m W1 + b1: the block's inner list of 16".into(),
            "h" => "h = GELU(z): the bend applied to each number".into(),
            "f" => "f = h W2 + b2: the block's output, 8 numbers".into(),
            "Fs" => "Fs: f handed to each stream in H_post's share".into(),
            "R" => "R = H_res X: the carried state mixed".into(),
            "X2" => "X2 = R + Fs: the new stream state".into(),
            "H_pre" => "H_pre: the mixing table before the block".into(),
            "H_post" => "H_post: the shares".into(),
            "H_res" => format!("H_res, {}'s", if r.settings.mhc { "mHC" } else { "HC" }),
            other => other.into(),
        }
    };
    let own = STAGES[step];
    // What the stage is made from, shown beside it.
    let from: &[&str] = match own {
        "P" => &["H_pre", "X"],
        "m" => &["P"],
        "z" => &["m"],
        "h" => &["z"],
        "f" => &["h"],
        "Fs" => &["H_post", "f"],
        "R" => &["H_res", "X"],
        "X2" => &["R", "Fs"],
        _ => &[],
    };
    let value = |label: &str| -> Matrix {
        let (rows, cols) = shape(label);
        match label {
            "H_pre" => matrix(rows, cols, &r.h_pre),
            "H_post" => matrix(rows, cols, &r.h_post),
            "H_res" => matrix(rows, cols, r.h_res()),
            _ => matrix(rows, cols, r.stage(label)),
        }
    };
    let mut maps: Vec<(String, Matrix)> = from.iter().map(|l| (l.to_string(), value(l))).collect();
    maps.push((own.to_string(), value(own)));
    let scale = largest(&maps.iter().map(|(_, m)| m).collect::<Vec<_>>());
    let drawn = maps.into_iter().map(|(name, m)| {
        let marks = if name == own { (0..m.items.len()).map(|k| (k, Mark::Written)).collect() } else { (0..m.items.len()).map(|k| (k, Mark::Read)).collect() };
        let cell = if m.cols > D { 30.0 } else { 42.0 };
        html! { <div key={format!("{name}-{step}")}>{ heat(&name, caption(&name), m, scale, marks, cell) }</div> }
    });
    html! { <>
        <div class="tables">{ for drawn }</div>
        { key(scale, true) }
    </> }
}

/// Part 3: one layer, stage by stage.
fn one_layer(r: &Run, step: usize, onstep: Callback<usize>, mode: Mode, linker: &Linker) -> Html {
    let fills = [
        ("hpost0", cut(r.h_post[0])),
        ("hpost1", cut(r.h_post[1])),
        ("hpre0", cut(r.h_pre[0])),
        ("hpre1", cut(r.h_pre[1])),
        ("rowsum", show(rounded(r.rows_hc[0], 2))),
        ("off", if r.off == 0.0 { "exactly 0".to_string() } else { format!("{:.0e}", r.off) }),
        ("workedP", written_to(3, "P(1,1)", &r.h_pre[..S], &column(r.stage("X"), 0), &r.products_pre, r.stage("P").first().copied().unwrap_or(0.0))),
        ("workedR", written_to(3, "R(1,1)", &r.h_res()[..S], &column(r.stage("X"), 0), &r.products_res, r.stage("R").first().copied().unwrap_or(0.0))),
        ("totalX", cut(r.total_x)),
        ("totalR", cut(r.total_r)),
    ];
    let shown: Vec<&str> = r.trace.stages.iter().filter(|st| st.label == STAGES[step]).map(|st| st.text.as_str()).collect();
    let _ = F;
    html! { <>
        { p(text::STEP[0]) }
        { p(text::STEP[1]) }
        <p class="setup">{ format!("The layer's carried state is mixed by {}'s H_res.", if r.settings.mhc { "mHC" } else { "HC" }) }</p>
        <div key={format!("flow-{step}")}>{ flow(&STAGES, step, READS[step]) }</div>
        <Stepper len={9} {step} {onstep} unit="Stage" />
        <p class="stage-says" role="status" key={format!("says-{step}")}>{ said(text::STAGES[step], &fills) }</p>
        { when(mode, Mode::Explorer, html! { <>
            <p class="note">{"This stage in the program:"}</p>
            { for shown.iter().map(|text| linker.block(text)) }
        </> }) }
        <p class="marks-key">{"Outlines:"}<span class="k read"></span>{"read by this stage"}<span class="k written"></span>{"worked out in this stage"}</p>
        { pictures(r, step) }
        { when(mode, Mode::Advanced, html! { <>
            <h3>{"The layer as mathematics"}</h3>
            <p class="note">{"Every stage of the layer, drawn by X_eTaL-ML's Flow from the same text that is run: a row a stage."}</p>
            <div class="tex" data-tex={r.tex.clone()}>{ r.tex.clone() }</div>
        </> }) }
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    // Where the page starts may be named in its address (lesson::link):
    // ?mode=explorer&preset=2&hres=hc&stage=4 (the stage from 1, the preset from 0).
    let all = presets();
    let mode = use_state(|| Mode::from_link().unwrap_or(Mode::Beginner));
    let chosen = use_state(|| param("preset").and_then(|p| p.parse::<usize>().ok()).filter(|i| *i < 3).unwrap_or(0));
    let mhc = use_state(|| param("hres").map_or(true, |h| !h.eq_ignore_ascii_case("hc")));
    let step = use_state(|| number("stage", 1, 9).map_or(0, |n| n - 1));
    let settings = Settings { mhc: *mhc, ..all[*chosen].clone() };
    // The run, and how long X_eTaL took over it in this browser (the
    // WebAssembly factor against the command line, read by the tooling
    // from the document; shown nowhere, since it is not X_eTaL's number).
    let result = use_memo(settings.clone(), |s| {
        let t0 = now();
        let r = run(s);
        let ms = now() - t0;
        if let Some(doc) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) {
            let _ = doc.set_attribute("data-xetal-ms", &format!("{ms:.0}"));
        }
        r
    });
    let linker = Linker::of(PROGRAM);
    let onmode = { let mode = mode.clone(); Callback::from(move |m| mode.set(m)) };
    let onstep = { let step = step.clone(); Callback::from(move |s| step.set(s)) };
    let onpreset = { let chosen = chosen.clone(); Callback::from(move |i| chosen.set(i)) };
    let onmhc = { let mhc = mhc.clone(); Callback::from(move |b| mhc.set(b)) };
    let body = match result.as_ref() {
        Err(e) => html! { <pre class="err">{ e.clone() }</pre> },
        Ok(r) => html! { <>
            { part(1, "A list through 48 layers, with the skip and without", stack(r, *chosen, onpreset)) }
            { part(2, "Four streams, and the tables that mix them", streams(r, onmhc)) }
            { part(3, "One layer of Hyper-Connections, in nine stages", one_layer(r, *step, onstep, *mode, &linker)) }
        </> },
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("residual-streams".into())}>
        <header>
            { header("Residual streams", text::LEDE) }
        </header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                { p(text::QUESTION[0]) }
                { p(text::QUESTION[1]) }
                <p class="hint">{"This is the first page of the depth track, a side track you can take after "}<a href="../lora-hello/">{"lesson 1, LoRA hello"}</a>{"; "}<a href="../welcome/">{"the welcome page"}</a>{" shows how the lessons work. A word with a dotted underline opens a plain explanation. The paper: "}<a href="https://arxiv.org/abs/2512.24880">{"arXiv:2512.24880"}</a>{"; the Python re-implementation this track follows: "}<a href="https://github.com/softwarewrighter/mHC-poc">{"mHC-poc"}</a>{"."}</p>
            </> }) }
            { frame(Frame::Background, "What you need before", html! { <>{ for text::BACKGROUND.iter().map(|t| p(t)) }</> }) }
            { frame(Frame::Rationale, "Why measure the skip, and take a layer apart", p(text::RATIONALE)) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ p(text::PURPOSE[0]) }{ p(text::PURPOSE[1]) }</> }) }
            <section class="panel">
                <h2>{"How much to show"}</h2>
                <Switch mode={*mode} onchange={onmode} />
                <p class="note">{"Explorer adds each stage as the program writes it. Advanced adds the layer drawn as mathematics and the program. Switching keeps the scale, the table and the stage you are on."}</p>
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
                    <h3>{"The layer as a pipeline"}</h3>
                    { p(text::CODE[1]) }
                    { linker.block(pipeline()) }
                    <h3>{"The library functions the program uses"}</h3>
                    { table(PROGRAM) }
                    <h3>{"The words of X_eTaL itself that it uses"}</h3>
                    { words(PROGRAM) }
                    <h3>{"The program"}</h3>
                    { p(text::CODE[2]) }
                    { linker.block(&program(&settings)) }
                </section>
            }) }
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
