//! The page, lesson D2 of the depth track, for new eyes: how a table
//! of positive numbers is made into one whose rows and columns each
//! sum to 1. The question, the background, the rationale and the
//! purpose; part 1, the start and one round with every number; part
//! 2, the twenty rounds stepped through, X_eTaL's chart of the
//! distances, the count of rounds to a millionth; part 3, the eight
//! checks as the Check library reported them, with the tables they
//! are made on; the summary and what is next; the code (Advanced),
//! the Python beside the loop. Presentation only: every number is one
//! X_eTaL printed, and Rust never divides, sums or checks anything.

use yew::prelude::*;

use lesson::arithmetic::{rounded, show};
use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::explain::{table, words};
use lesson::heat::{key, largest, Heat, Mark, Matrix};
use lesson::link::{number, param};
use lesson::mode::{when, Mode, Switch};
use lesson::stepper::Stepper;
use microscope::chrome::picture;
use microscope::run::now;

use crate::micro::{pipeline, presets, program, run, Run, Settings, PROGRAM, S, TMAX};
use crate::text;

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

/// A distance from 1 as the prose quotes it: 0 is said as 0; a small one in powers of ten.
fn far(v: f64) -> String {
    if v == 0.0 {
        "exactly 0".into()
    } else if v < 0.001 {
        format!("{v:.1e}")
    } else {
        cut(v)
    }
}

/// Four sums as the prose quotes them.
fn sums(v: &[f64]) -> String {
    v.iter().map(|x| format!("{x:.6}")).collect::<Vec<_>>().join(", ")
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

/// A picture of a table, numbers written in, at a shared scale.
fn heat(name: &str, caption: String, m: Matrix, scale: f64, marks: Vec<(usize, Mark)>) -> Html {
    html! { <Heat name={name.to_string()} {caption} matrix={m} {scale} shared=true legend=false numbers=true cell=42.0 {marks} hint=false /> }
}

/// A table's row sums and column sums, as the run printed them.
fn sums_table(caption: &str, rows: &[(&str, &[f64], &[f64])]) -> Html {
    let body = rows.iter().map(|(name, r, c)| html! {
        <tr><th scope="row">{ name.to_string() }</th><td>{ sums(r) }</td><td>{ sums(c) }</td></tr>
    });
    html! {
        <div class="scroll">
            <table class="vectors sums">
                <caption>{ caption.to_string() }</caption>
                <thead><tr><th></th><th scope="col">{"the four row sums"}</th><th scope="col">{"the four column sums"}</th></tr></thead>
                <tbody>{ for body }</tbody>
            </table>
        </div>
    }
}

/// Row 1 of A divided by its sum, as the program's numbers say it.
fn worked_row(r: &Run) -> String {
    let r1 = Run::at(&r.rs, 1, S * S);
    let parts: Vec<String> = (0..S).map(|j| format!("{} / {} = {}", show(rounded(r.a[j], 4)), show(rounded(r.rows_a[0], 4)), show(rounded(r1[j], 4)))).collect();
    parts.join("; ")
}

/// Part 1: the start, and one round.
fn start(r: &Run, chosen: usize, onpreset: Callback<usize>, mode: Mode, linker: &Linker) -> Html {
    let s = &r.settings;
    let buttons = presets().into_iter().enumerate().map(|(i, p)| {
        let onclick = { let onpreset = onpreset.clone(); Callback::from(move |_| onpreset.emit(i)) };
        html! { <button type="button" class={classes!("preset", (i == chosen).then_some("on"))} {onclick} aria-pressed={(i == chosen).to_string()}><b>{ p.title.clone() }</b><span>{ p.setup.clone() }</span></button> }
    });
    let (lg, a) = (matrix(&r.logits), matrix(&r.a));
    let (r1, c1) = (matrix(Run::at(&r.rs, 1, S * S)), matrix(Run::at(&r.cs, 1, S * S)));
    let scale = largest(&[&a, &r1, &c1]);
    let signed = largest(&[&lg]);
    let fills = [
        ("rowA0", cut(r.rows_a[0])),
        ("workedR", worked_row(r)),
        ("rowsR1", sums(Run::at(&r.rows_r, 1, S))),
        ("colsR1", sums(Run::at(&r.cols_r, 1, S))),
        ("colsC1", sums(Run::at(&r.cols_c, 1, S))),
        ("rowsC1", sums(Run::at(&r.rows_c, 1, S))),
        ("off", far(r.off)),
    ];
    let written: Vec<(usize, Mark)> = (0..S * S).map(|k| (k, Mark::Written)).collect();
    let row1: Vec<(usize, Mark)> = (0..S).map(|k| (k, Mark::Written)).collect();
    let alike = r.rows_a.iter().chain(r.cols_a.iter()).all(|v| (v - r.rows_a[0]).abs() < 1e-9);
    let shown: Vec<&str> = r.trace.stages.iter().filter(|st| st.label != "A").map(|st| st.text.as_str()).collect();
    html! { <>
        { p(text::START[0]) }
        <div class="presets">{ for buttons }</div>
        <p class="setup"><b>{ s.title.clone() }</b>{ format!(": {}.", s.setup) }</p>
        <p>{ said(if alike { text::START[1] } else { text::START[1].split(" For the paper's start").next().unwrap_or(text::START[1]) }, &fills) }</p>
        <div class="tables">
            { heat("logits", "the logits, a 4 x 4 picked by the button (their own scale)".into(), lg, signed, vec![]) }
        </div>
        { key(signed, false) }
        <div class="tables">
            { heat("A", "A: e raised to each logit, the positive start".into(), a, scale, vec![]) }
        </div>
        { sums_table("What A's rows and columns sum to, from the run", &[("A", &r.rows_a, &r.cols_a)]) }
        <p>{ said(text::START[2], &fills) }</p>
        <div class="tables">
            { heat("R", "R: every row of A divided by its sum (row 1 outlined: the one worked out)".into(), r1, scale, row1) }
        </div>
        <p>{ said(text::START[3], &fills) }</p>
        <div class="tables">
            { heat("C", "C: every column of R divided by its sum, the table after one round".into(), c1, scale, written) }
        </div>
        { key(scale, true) }
        { sums_table("What R's and C's rows and columns sum to, from the run", &[("R, after the row step", Run::at(&r.rows_r, 1, S), Run::at(&r.cols_r, 1, S)), ("C, after the column step", Run::at(&r.rows_c, 1, S), Run::at(&r.cols_c, 1, S))]) }
        <p>{ said(text::START[4], &fills) }</p>
        { when(mode, Mode::Explorer, html! { <>
            <p class="note">{"The two steps in the program:"}</p>
            { for shown.iter().map(|text| linker.block(text)) }
        </> }) }
        { when(mode, Mode::Advanced, html! { <>
            <h3>{"The round as mathematics"}</h3>
            <p class="note">{"The two steps, drawn by X_eTaL-ML's Flow from the same text that is run: a row a step."}</p>
            <div class="tex" data-tex={r.tex.clone()}>{ r.tex.clone() }</div>
        </> }) }
    </> }
}

/// Part 2: the rounds, stepped through.
fn rounds(r: &Run, step: usize, onstep: Callback<usize>) -> Html {
    let t = step + 1;
    let (rt, ct) = (matrix(Run::at(&r.rs, t, S * S)), matrix(Run::at(&r.cs, t, S * S)));
    // One scale for every round: the record of all of them bounds it.
    let all = |v: &[f64]| Matrix { rows: v.len() / S, cols: S, items: v.to_vec() };
    let scale = largest(&[&all(&r.rs), &all(&r.cs)]);
    let fills = [
        ("t", t.to_string()),
        ("tmax", TMAX.to_string()),
        ("rowsA", sums(&r.rows_a)),
        ("colsR", sums(Run::at(&r.cols_r, t, S))),
        ("rowsC", sums(Run::at(&r.rows_c, t, S))),
        ("rowOff", far(r.row_off[t - 1])),
        ("rowOff20", far(r.row_off[TMAX - 1])),
        ("colOff20", far(r.col_off[TMAX - 1])),
        ("rounds", r.rounds.to_string()),
    ];
    let which = if t == 1 { 1 } else if t == TMAX { 2 } else { 0 };
    let written: Vec<(usize, Mark)> = (0..S * S).map(|k| (k, Mark::Written)).collect();
    html! { <>
        <p>{ said(text::ROUNDS[0], &fills) }</p>
        <Stepper len={TMAX} {step} {onstep} unit="Round" />
        <p class="stage-says" role="status" key={format!("says-{step}")}>{ said(text::ROUND[which], &fills) }</p>
        <div class="tables" key={format!("round-{step}")}>
            { heat("after the row step", format!("round {t}: the table after its row step"), rt, scale, vec![]) }
            { heat("after the column step", format!("round {t}: the table after its column step"), ct, scale, written) }
        </div>
        { key(scale, true) }
        { sums_table(&format!("Round {t}: what the rows and columns sum to, from the run"), &[("after the row step", Run::at(&r.rows_r, t, S), Run::at(&r.cols_r, t, S)), ("after the column step", Run::at(&r.rows_c, t, S), Run::at(&r.cols_c, t, S))]) }
        <p>{ said(text::ROUNDS[1], &fills) }</p>
        { picture(&r.chart, "How far the farthest sum is from 1 after each round, on a log scale: the rows after the column step, and the columns before it") }
        <div class="said" role="status">
            <p>{ said(if r.rounds <= TMAX { text::ROUNDS[2] } else { text::ROUNDS[3] }, &fills) }</p>
            <p class="note">{ format!("Said from the run alone: {TMAX} rounds from this start, a sum counted as 1 within a millionth, every figure as X_eTaL printed it.") }</p>
        </div>
    </> }
}

/// Part 3: the three properties, checked.
fn checks(r: &Run) -> Html {
    let n = r.tables();
    let fills = [
        ("tmax", TMAX.to_string()),
        ("sigmaA", cut(r.sigma_a)),
        ("sigmaH", cut(r.sigma_h)),
        ("sizeV", cut(r.size_v)),
        ("sizeHv", cut(r.size_hv)),
        ("rowsHB", sums(&r.rows_hb)),
        ("colsHB", sums(&r.cols_hb)),
        ("tables", n.to_string()),
        ("weights", r.weights[..n].iter().map(|w| show(rounded(*w, 3))).collect::<Vec<_>>().join(", ")),
        ("weightsSum", cut(r.weights_sum)),
        ("rest", far(r.rest)),
        ("summary", r.summary.clone()),
    ];
    let (h, b0, b, hb) = (matrix(&r.h), matrix(&r.b0), matrix(&r.b), matrix(&r.hb));
    let scale = largest(&[&h, &b, &hb]);
    let scale0 = largest(&[&b0]);
    let lines = r.checks.iter().map(|c| html! {
        <li class={classes!("check", if c.ok { "ok" } else { "fail" })}><code>{ c.line.clone() }</code></li>
    });
    let orderings = (0..n).map(|k| {
        let o: Vec<String> = r.orderings[k * S..(k + 1) * S].iter().map(|i| i.to_string()).collect();
        html! { <tr><th scope="row">{ format!("table {}", k + 1) }</th><td>{ show(rounded(r.weights[k], 6)) }</td><td>{ format!("row 1 keeps column {}, row 2 column {}, row 3 column {}, row 4 column {}", o[0], o[1], o[2], o[3]) }</td></tr> }
    });
    html! { <>
        <p>{ said(text::CHECKS[0], &fills) }</p>
        <div class="tables">
            { heat("H", format!("H: the table after round {TMAX}, the one checked"), h, scale, vec![]) }
        </div>
        { sums_table("What H's rows and columns sum to, from the run", &[("H", Run::at(&r.rows_c, TMAX, S), Run::at(&r.cols_c, TMAX, S))]) }
        <h3>{"It stretches nothing"}</h3>
        <p>{ said(text::CHECKS[1], &fills) }</p>
        <div class="scroll">
            <table class="vectors">
                <caption>{"A list before and after H, from the run"}</caption>
                <thead><tr><th></th><th scope="col">{"the four numbers"}</th><th scope="col">{"size"}</th></tr></thead>
                <tbody>
                    <tr><th scope="row">{"v"}</th><td>{ sums(&r.v) }</td><td>{ cut(r.size_v) }</td></tr>
                    <tr><th scope="row">{"H v"}</th><td>{ sums(&r.hv) }</td><td>{ cut(r.size_hv) }</td></tr>
                </tbody>
            </table>
        </div>
        <h3>{"Two make a third"}</h3>
        <p>{ said(if r.checks[4].ok { text::CHECKS[2] } else { text::CHECKS[6] }, &fills) }</p>
        <div class="tables">
            { heat("B0", "B0: a second positive start, set up in the program".into(), b0, scale0, vec![]) }
            { heat("B", format!("B: B0 after {TMAX} rounds"), b, scale, vec![]) }
            { heat("H B", "H times B: the product of the two".into(), hb, scale, (0..S * S).map(|k| (k, Mark::Written)).collect()) }
        </div>
        { key(scale, true) }
        { sums_table("What the product's rows and columns sum to, from the run", &[("H B", &r.rows_hb, &r.cols_hb)]) }
        <h3>{"It is a mix of reorderings"}</h3>
        <p>{ said(text::CHECKS[3], &fills) }</p>
        <div class="scroll">
            <table class="vectors perms">
                <caption>{"The permutation tables found, from the run: each keeps one number of each row, in the column named, and sets the rest to 0"}</caption>
                <thead><tr><th></th><th scope="col">{"weight"}</th><th scope="col">{"the reordering"}</th></tr></thead>
                <tbody>{ for orderings }</tbody>
            </table>
        </div>
        <h3>{"The report"}</h3>
        <ul class="checks">{ for lines }</ul>
        <div class="said" role="status">
            <p>{ said(if r.passed() { text::CHECKS[4] } else { text::CHECKS[5] }, &fills) }</p>
        </div>
    </> }
}

/// The Python beside the X_eTaL loop, line for line.
fn compare(linker: &Linker) -> Html {
    let rows = text::PYTHON.iter().zip(text::LOOP.iter()).zip(text::COMPARE.iter()).map(|((py, xtl), what)| html! {
        <tr><td><code class="py">{ py.to_string() }</code></td><td>{ linker.block(xtl) }</td><td>{ p(what) }</td></tr>
    });
    html! {
        <div class="scroll">
            <table class="compare">
                <caption>{"The Python's sinkhorn_doubly_stochastic (mHC-poc, mlx/src/mhc.py; its docstring and two comments left out) and the loop here, line for line"}</caption>
                <thead><tr><th scope="col">{"the Python"}</th><th scope="col">{"X_eTaL"}</th><th scope="col">{"what the line does"}</th></tr></thead>
                <tbody>{ for rows }</tbody>
            </table>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    // Where the page starts may be named in its address (lesson::link):
    // ?mode=explorer&preset=1&round=5 (the round from 1, the preset from 0).
    let all = presets();
    let mode = use_state(|| Mode::from_link().unwrap_or(Mode::Beginner));
    let chosen = use_state(|| param("preset").and_then(|p| p.parse::<usize>().ok()).filter(|i| *i < 3).unwrap_or(0));
    let step = use_state(|| number("round", 1, TMAX).map_or(0, |n| n - 1));
    let settings: Settings = all[*chosen].clone();
    // The run, and how long X_eTaL took over it in this browser (read by
    // the tooling from the document; shown nowhere, since it is not
    // X_eTaL's number).
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
    let body = match result.as_ref() {
        Err(e) => html! { <pre class="err">{ e.clone() }</pre> },
        Ok(r) => html! { <>
            { part(1, "A positive table, and one round", start(r, *chosen, onpreset, *mode, &linker)) }
            { part(2, "Twenty rounds, watched", rounds(r, *step, onstep)) }
            { part(3, "Three properties of the result, checked", checks(r)) }
        </> },
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("sinkhorn".into())}>
        <header>
            { header("Sinkhorn-Knopp", text::LEDE) }
        </header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                { p(text::QUESTION[0]) }
                { p(text::QUESTION[1]) }
                <p class="hint">{"This is the second page of the depth track; "}<a href="../residual-streams/">{"the first, Residual streams"}</a>{", introduces the streams and the table this page makes. "}<a href="../welcome/">{"The welcome page"}</a>{" shows how the lessons work. A word with a dotted underline opens a plain explanation. The paper: "}<a href="https://arxiv.org/abs/2512.24880">{"arXiv:2512.24880"}</a>{"; the Python re-implementation this track follows: "}<a href="https://github.com/softwarewrighter/mHC-poc">{"mHC-poc"}</a>{"; the procedure: Sinkhorn and Knopp, 1967."}</p>
            </> }) }
            { frame(Frame::Background, "What you need before", html! { <>{ for text::BACKGROUND.iter().map(|t| p(t)) }</> }) }
            { frame(Frame::Rationale, "Why run it round by round, and check rather than state", p(text::RATIONALE)) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ p(text::PURPOSE[0]) }{ p(text::PURPOSE[1]) }</> }) }
            <section class="panel">
                <h2>{"How much to show"}</h2>
                <Switch mode={*mode} onchange={onmode} />
                <p class="note">{"Explorer adds the two steps as the program writes them. Advanced adds the round drawn as mathematics, the Python beside the loop, and the program. Switching keeps the start and the round you are on."}</p>
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
                    <h3>{"The round as a pipeline"}</h3>
                    { p(text::CODE[1]) }
                    { linker.block(pipeline()) }
                    <h3>{"The loop beside the Python"}</h3>
                    { compare(&linker) }
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
