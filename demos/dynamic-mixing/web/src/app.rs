//! The page, lesson D5 of the depth track, for new eyes: the paper's
//! own mixing tables, made afresh from each input. The question, the
//! background, the rationale and the purpose; part 1, two inputs and
//! their tables at the gating factor chosen (a slider); part 2, the
//! first input through the layer in nine stages; part 3, the gradient
//! worked out by hand and checked; part 4, what waits on the library;
//! the summary and what is next; the code (Advanced). Presentation
//! only: every number is one X_eTaL printed.

use web_sys::HtmlInputElement;
use yew::prelude::*;

use lesson::arithmetic::{rounded, show};
use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::explain::{table, words};
use lesson::heat::{key, largest, Heat, Matrix};
use lesson::link::number;
use lesson::mode::{when, Mode, Switch};
use lesson::stepper::Stepper;
use microscope::run::now;

use crate::micro::{run, with, Input, Run, ALPHAS, C, ENTRIES, PAPER, PROGRAM, S};
use crate::text;

fn part(n: usize, title: &str, body: Html) -> Html {
    html! {
        <section class="panel part">
            <h2><span class="n">{ n }</span>{ title.to_string() }</h2>
            { body }
        </section>
    }
}

/// A number as the prose quotes it: six decimals, or in powers of ten when very small.
fn cut(v: f64) -> String {
    if v == 0.0 {
        "0".into()
    } else if v.abs() < 1e-4 {
        format!("{v:.1e}")
    } else {
        show(rounded(v, 6))
    }
}

fn list(v: &[f64]) -> String {
    v.iter().map(|x| show(rounded(*x, 4))).collect::<Vec<_>>().join(", ")
}

fn said(text: &str, fills: &[(&str, String)]) -> Html {
    let mut s = text.to_string();
    for (mark, value) in fills {
        s = s.replace(&format!("{{{mark}}}"), value);
    }
    prose(&s)
}

fn matrix(rows: usize, cols: usize, items: &[f64]) -> Matrix {
    Matrix { rows, cols, items: items.to_vec() }
}

fn heat(name: &str, caption: String, m: Matrix, scale: f64) -> Html {
    html! { <Heat name={name.to_string()} {caption} matrix={m} {scale} shared=true legend=false numbers=true cell=40.0 marks={Vec::new()} hint=false /> }
}

fn slider(at: usize, onat: Callback<usize>) -> Html {
    let oninput = Callback::from(move |e: InputEvent| {
        if let Ok(k) = e.target_unchecked_into::<HtmlInputElement>().value().parse::<usize>() {
            onat.emit(k);
        }
    });
    html! { <label class="rank">{"gating factor"}<input type="range" min="0" max={(ALPHAS.len() - 1).to_string()} step="1" value={at.to_string()} {oninput} aria-label="The gating factor of all three tables: 0, 0.01 (the paper's start), 0.1, 0.3, 1 or 3" /><output>{ show(ALPHAS[at]) }</output></label> }
}

/// The three tables an input makes, as pictures.
fn tables(name: &str, i: &Input, scale: f64) -> Html {
    html! { <>
        { heat(&format!("H_pre, {name}"), format!("H_pre for {name}: four weights"), matrix(1, S, &i.h_pre), scale) }
        { heat(&format!("H_post, {name}"), format!("H_post for {name}: four weights"), matrix(1, S, &i.h_post), scale) }
        { heat(&format!("H_res, {name}"), format!("H_res for {name}: 4 x 4"), matrix(S, S, &i.h_res), scale) }
    </> }
}

/// Part 1: two inputs, their tables.
fn two(r: &Run, at: usize, onat: Callback<usize>) -> Html {
    // The program's own check covers both inputs' H_res.
    let ok = r.checks.get(1).is_some_and(|c| c.ok);
    let sums = |v: &[f64]| v.iter().map(|x| format!("{x:.6}")).collect::<Vec<_>>().join(", ");
    let fills = [
        ("pre", cut(r.apart[0])),
        ("post", cut(r.apart[1])),
        ("res", cut(r.apart[2])),
        ("rows", sums(&r.rows)),
        ("cols", sums(&r.cols)),
        ("sinkhorn", if ok { text::SUMS[0].to_string() } else { text::SUMS[1].to_string() }),
    ];
    let (xa, xb) = (matrix(S, C, &r.a.x), matrix(S, C, &r.b.x));
    let xs = largest(&[&xa, &xb]);
    let hs: Vec<Matrix> = [&r.a, &r.b].iter().flat_map(|i| [matrix(1, S, &i.h_pre), matrix(1, S, &i.h_post), matrix(S, S, &i.h_res)]).collect();
    let scale = largest(&hs.iter().collect::<Vec<_>>());
    html! { <>
        { p(text::TWO[0]) }
        { slider(at, onat) }
        <div class="tables">
            { heat("input A", "input A: a stream state, 4 streams of 8".into(), xa, xs) }
            { heat("input B", "input B: another".into(), xb, xs) }
        </div>
        { key(xs, true) }
        <p>{ said(text::TWO[1], &fills) }</p>
        <div class="tables">{ tables("input A", &r.a, scale) }</div>
        <div class="tables">{ tables("input B", &r.b, scale) }</div>
        { key(scale, true) }
        { if r.alpha == 0.0 { p(text::TWO[2]) } else { html! {} } }
        <div class="said" role="status"><p>{ said(text::TWO[3], &fills) }</p></div>
    </> }
}

/// Part 2: the first input, every stage.
fn stages(r: &Run, step: usize, onstep: Callback<usize>) -> Html {
    let a = &r.a;
    let fills = [
        ("xn0", cut(a.xn[0])),
        ("xn1", cut(a.xn[1])),
        ("p0", cut(a.proj[0])),
        ("alpha", show(r.alpha)),
        ("hpre", list(&a.h_pre)),
        ("hpost", list(&a.h_post)),
        ("rows0", list(&r.rows)),
        ("u0", cut(a.u[0])),
        ("u1", cut(a.u[1])),
        ("f0", cut(a.f[0])),
        ("f1", cut(a.f[1])),
    ];
    let m = match step {
        0 => ("X", "the stream state", matrix(S, C, &a.x)),
        1 => ("normalized", "the 32 numbers flattened and normalized", matrix(1, S * C, &a.xn)),
        2 => ("projected", "24 numbers: 4 for H_pre, 4 for H_post, 16 for H_res", matrix(1, 2 * S + S * S, &a.proj)),
        3 => ("H_pre", "H_pre: four weights", matrix(1, S, &a.h_pre)),
        4 => ("H_post", "H_post: four weights", matrix(1, S, &a.h_post)),
        5 => ("H_res", "H_res: 4 x 4", matrix(S, S, &a.h_res)),
        6 => ("block input", "the block's input: 8 numbers", matrix(1, C, &a.u)),
        7 => ("block output", "the block's output: 8 numbers", matrix(1, C, &a.f)),
        _ => ("new state", "the new state: 4 streams of 8", matrix(S, C, &a.x2)),
    };
    let scale = largest(&[&m.2]);
    let cell = if m.2.cols > 16 { 26.0 } else { 40.0 };
    html! { <>
        { p(text::STEP) }
        <Stepper len={9} {step} {onstep} unit="Stage" />
        <p class="stage-says" role="status" key={format!("says-{step}")}>{ said(text::STAGES[step], &fills) }</p>
        <div class="tables" key={format!("stage-{step}")}>
            <Heat name={m.0.to_string()} caption={m.1.to_string()} matrix={m.2} {scale} shared=false legend=true numbers={cell > 30.0} {cell} marks={Vec::new()} hint=false />
        </div>
    </> }
}

/// Part 3: the gradient, checked.
fn gradient(r: &Run) -> Html {
    let fills = [("summary", r.summary.clone())];
    let rows = ENTRIES.iter().enumerate().map(|(i, e)| html! {
        <tr><th scope="row">{ e.to_string() }</th><td>{ format!("{:.9}", r.written[i]) }</td><td>{ format!("{:.9}", r.central[i]) }</td></tr>
    });
    let lines = r.checks.iter().map(|c| html! { <li class={classes!("check", if c.ok { "ok" } else { "fail" })}><code>{ c.line.clone() }</code></li> });
    html! { <>
        { p(text::GRADIENT[0]) }
        <div class="scroll">
            <table class="vectors">
                <caption>{"The gradient by ten numbers of the layer, two ways, from this run (nine decimals)"}</caption>
                <thead><tr><th></th><th scope="col">{"worked out by hand"}</th><th scope="col">{"central differences"}</th></tr></thead>
                <tbody>{ for rows }</tbody>
            </table>
        </div>
        <p>{ said(text::GRADIENT[1], &fills) }</p>
        <ul class="checks">{ for lines }</ul>
        { p(text::GRADIENT[2]) }
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    // ?mode=explorer&alpha=5&stage=6 (the gating factor's place on the slider from 1, the stage from 1).
    let mode = use_state(|| Mode::from_link().unwrap_or(Mode::Beginner));
    let at = use_state(|| number("alpha", 1, ALPHAS.len()).map_or(PAPER, |n| n - 1));
    let step = use_state(|| number("stage", 1, 9).map_or(0, |n| n - 1));
    let result = use_memo(*at, |at| {
        let t0 = now();
        let r = run(ALPHAS[*at]);
        if let Some(doc) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) {
            let _ = doc.set_attribute("data-xetal-ms", &format!("{:.0}", now() - t0));
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
            { part(1, "Two inputs, and the tables each makes", two(r, *at, onat)) }
            { part(2, "One input through the layer, in nine stages", stages(r, *step, onstep)) }
            { part(3, "The gradient, worked out and checked", gradient(r)) }
            { part(4, "What is not on this page, and why", html! { <>{ p(text::WAITS[0]) }{ p(text::WAITS[1]) }</> }) }
        </> },
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("dynamic-mixing".into())}>
        <header>
            { header("Dynamic mixing", text::LEDE) }
        </header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                { p(text::QUESTION[0]) }
                { p(text::QUESTION[1]) }
                <p class="hint">{"This is the fifth page of the depth track, after "}<a href="../residual-streams/">{"Residual streams"}</a>{", "}<a href="../sinkhorn/">{"Sinkhorn-Knopp"}</a>{", "}<a href="../gain-by-depth/">{"The gain by depth"}</a>{" and "}<a href="../training-at-depth/">{"Training at depth"}</a>{". "}<a href="../welcome/">{"The welcome page"}</a>{" shows how the lessons work. A word with a dotted underline opens a plain explanation. The paper: "}<a href="https://arxiv.org/abs/2512.24880">{"arXiv:2512.24880"}</a>{" (equations 3, 5 and 7 to 9)."}</p>
            </> }) }
            { frame(Frame::Background, "What you need before", html! { <>{ for text::BACKGROUND.iter().map(|t| p(t)) }</> }) }
            { frame(Frame::Rationale, "Why send two inputs through, and check the gradient", p(text::RATIONALE)) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ p(text::PURPOSE[0]) }{ p(text::PURPOSE[1]) }</> }) }
            <section class="panel">
                <h2>{"How much to show"}</h2>
                <Switch mode={*mode} onchange={onmode} />
                <p class="note">{"Advanced adds the program. Switching keeps the gating factor and the stage you are on."}</p>
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
                    { linker.block(&with(ALPHAS[*at])) }
                </section>
            }) }
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
