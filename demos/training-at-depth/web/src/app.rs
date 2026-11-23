//! The page, experiment D4 of the depth track, for new eyes: does the
//! bound on the gain matter once a model is trained? The question,
//! the background, the rationale and the purpose; part 1, a short run
//! of the scheme picked computed in the browser, with its gradient
//! check; part 2, the nine offline runs replayed at the depth picked,
//! X_eTaL's charts and the figures; part 3, beside the Python's run
//! and the paper; the summary and what is next; the code (Advanced).
//! Presentation only: every number is one X_eTaL printed, live or in
//! the records, and Rust never trains, sums or checks anything.

use yew::prelude::*;

use lesson::arithmetic::{rounded, show};
use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::explain::{table, words};
use lesson::link::param;
use lesson::mode::{when, Mode, Switch};
use microscope::chrome::picture;
use microscope::run::now;

use crate::micro::{recorded, replay, replay_with, run, schemes, with, Replay, Run, DEPTHS, ENTRIES, LIVE_STEPS, PROGRAM, REPLAY, RUNS};
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

/// A sentence with its `{marks}` filled, then drawn as prose.
fn said(text: &str, fills: &[(&str, String)]) -> Html {
    let mut s = text.to_string();
    for (mark, value) in fills {
        s = s.replace(&format!("{{{mark}}}"), value);
    }
    prose(&s)
}

/// A row of buttons, one per choice.
fn buttons(names: &[(String, String)], chosen: usize, on: Callback<usize>) -> Html {
    let items = names.iter().enumerate().map(|(i, (title, setup))| {
        let onclick = { let on = on.clone(); Callback::from(move |_| on.emit(i)) };
        html! { <button type="button" class={classes!("preset", (i == chosen).then_some("on"))} {onclick} aria-pressed={(i == chosen).to_string()}><b>{ title.clone() }</b><span>{ setup.clone() }</span></button> }
    });
    html! { <div class="presets">{ for items }</div> }
}

/// Part 1: the run in your browser.
fn live(r: &Run, chosen: usize, on: Callback<usize>) -> Html {
    let rec = &r.record;
    let fills = [
        ("steps", LIVE_STEPS.to_string()),
        ("loss1", cut(rec.at(1, 0))),
        ("lossN", cut(rec.at(LIVE_STEPS, 0))),
        ("grad1", cut(rec.at(1, 1))),
        ("gradN", cut(rec.at(LIVE_STEPS, 1))),
        ("gainN", cut(rec.at(LIVE_STEPS, 2))),
        ("nans", show(r.nans)),
        ("summary", r.summary.clone()),
    ];
    let names: Vec<(String, String)> = schemes().into_iter().map(|s| (s.title, s.setup)).collect();
    let rows = ENTRIES.iter().enumerate().map(|(i, e)| html! {
        <tr><th scope="row">{ e.to_string() }</th><td>{ format!("{:.9}", r.written[i]) }</td><td>{ format!("{:.9}", r.central[i]) }</td></tr>
    });
    let lines = r.checks.iter().map(|c| html! { <li class={classes!("check", if c.ok { "ok" } else { "fail" })}><code>{ c.line.clone() }</code></li> });
    html! { <>
        { p(text::LIVE[0]) }
        { buttons(&names, chosen, on) }
        <p class="setup"><b>{ r.scheme.title.clone() }</b>{ format!(" ({}): 12 layers, {LIVE_STEPS} steps, computed here.", r.scheme.setup) }</p>
        { picture(&r.chart, "The loss and the gradient size at each step of the run computed in your browser, in powers of ten") }
        <div class="said" role="status"><p>{ said(text::LIVE[1], &fills) }</p></div>
        <h3>{"The gradient, checked"}</h3>
        <p>{ said(text::LIVE[2], &fills) }</p>
        <div class="scroll">
            <table class="vectors">
                <caption>{"The gradient by nine weights, two ways, from this run (nine decimals)"}</caption>
                <thead><tr><th></th><th scope="col">{"the backward pass"}</th><th scope="col">{"central differences"}</th></tr></thead>
                <tbody>{ for rows }</tbody>
            </table>
        </div>
        <ul class="checks">{ for lines }</ul>
        <p>{ said(text::LIVE[3], &fills) }</p>
    </> }
}

/// Part 2: the recorded runs.
fn recorded_runs(rp: &Replay, live: &Run, scheme: usize, depth: usize, on: Callback<usize>) -> Html {
    let rec = recorded(scheme, 12).ok();
    let fills = [
        ("steps", LIVE_STEPS.to_string()),
        ("recN", rec.as_ref().map_or("?".into(), |r| cut(r.at(LIVE_STEPS, 0)))),
        ("liveN", cut(live.record.at(LIVE_STEPS, 0))),
        ("b", cut(rp.last(0, 0))),
        ("h", cut(rp.last(1, 0))),
        ("m", cut(rp.last(2, 0))),
        ("bg", cut(rp.last(0, 1))),
        ("hgain", cut(rp.last(1, 2))),
        ("mgain", cut(rp.last(2, 2))),
        ("nans", rp.nans.iter().map(|v| show(*v)).collect::<Vec<_>>().join(", ")),
    ];
    let names: Vec<(String, String)> = DEPTHS.iter().map(|d| (format!("{d} layers"), format!("the three runs at {d} layers, 800 steps each"))).collect();
    let titles = ["The loss, each step", "The gradient size, each step", "The forward gain of the composite, each step", "The proxy of the composite, each step"];
    let charts = rp.charts.iter().zip(titles).map(|(c, t)| html! { <div class="chart">{ picture(c, &format!("{t}, the three recorded runs at {} layers", rp.depth)) }</div> });
    let row = |name: &str, s: usize| html! {
        <tr><th scope="row">{ name.to_string() }</th><td>{ cut(rp.first(s, 0)) }</td><td>{ cut(rp.last(s, 0)) }</td><td>{ cut(rp.least[s]) }</td><td>{ cut(rp.last(s, 1)) }</td><td>{ cut(rp.last(s, 2)) }</td><td>{ show(rp.nans[s]) }</td></tr>
    };
    // Whether the plain skip learned at this depth: its last loss against the even guess, as the run says it.
    let stuck = rp.last(0, 0) > 2.0;
    html! { <>
        { buttons(&names, DEPTHS.iter().position(|d| *d == depth).unwrap_or(2), on) }
        <p>{ said(text::RECORDED[0], &fills) }</p>
        <div class="charts">{ for charts }</div>
        <div class="scroll">
            <table class="vectors depths">
                <caption>{ format!("The three recorded runs at {} layers, measured (offline, replayed)", rp.depth) }</caption>
                <thead><tr><th></th><th scope="col">{"loss, step 1"}</th><th scope="col">{"loss, step 800"}</th><th scope="col">{"smallest loss"}</th><th scope="col">{"gradient size, step 800"}</th><th scope="col">{"forward gain, step 800"}</th><th scope="col">{"NaN events"}</th></tr></thead>
                <tbody>{ row("the plain skip", 0) }{ row("HC", 1) }{ row("mHC", 2) }</tbody>
            </table>
        </div>
        <div class="said" role="status">
            <p>{ said(text::RECORDED[1], &fills) }</p>
            <p>{ said(if stuck { text::RECORDED[2] } else { text::RECORDED[3] }, &fills) }</p>
            <p>{ said(text::RECORDED[4], &fills) }</p>
        </div>
    </> }
}

/// Part 3: beside the Python's run and the paper.
fn beside(r48: &Replay) -> Html {
    let row = |name: &str, s: usize, theirs: &str| html! { <tr><th scope="row">{ name.to_string() }</th><td>{ cut(r48.last(s, 0)) }</td><td class="theirs">{ theirs.to_string() }</td></tr> };
    html! { <>
        { p(text::BESIDE[0]) }
        <div class="scroll">
            <table class="vectors">
                <caption>{"Final losses at 48 layers: this page's runs (measured, 800 steps, 16 tokens; guessing evenly gives 2.77) beside the Python's run (its measurement, 800 steps, 256 tokens; guessing evenly gives 5.55)"}</caption>
                <thead><tr><th></th><th scope="col">{"this page"}</th><th scope="col">{"the Python's run"}</th></tr></thead>
                <tbody>{ row("the plain skip", 0, "3.79") }{ row("HC", 1, "5.54") }{ row("mHC", 2, "0.0002") }</tbody>
            </table>
        </div>
        { p(text::BESIDE[1]) }
        { p(text::BESIDE[2]) }
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    // Where the page starts may be named in its address (lesson::link):
    // ?mode=explorer&scheme=hc&depth=48.
    let all = schemes();
    let mode = use_state(|| Mode::from_link().unwrap_or(Mode::Beginner));
    let scheme = use_state(|| param("scheme").and_then(|s| ["baseline", "hc", "mhc"].iter().position(|x| s.eq_ignore_ascii_case(x))).unwrap_or(2));
    let depth = use_state(|| param("depth").and_then(|d| d.parse::<usize>().ok()).filter(|d| DEPTHS.contains(d)).unwrap_or(48));
    let s = all[*scheme].clone();
    let live_run = use_memo(*scheme, |k| {
        let t0 = now();
        let r = run(&schemes()[*k]);
        if let Some(doc) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) {
            let _ = doc.set_attribute("data-xetal-ms", &format!("{:.0}", now() - t0));
        }
        r
    });
    let replayed = use_memo(*depth, |d| replay(*d));
    let replayed48 = use_memo((), |_| replay(48));
    let linker = Linker::of(PROGRAM);
    let replay_linker = Linker::of(REPLAY);
    let onmode = { let mode = mode.clone(); Callback::from(move |m| mode.set(m)) };
    let onscheme = { let scheme = scheme.clone(); Callback::from(move |i| scheme.set(i)) };
    let ondepth = { let depth = depth.clone(); Callback::from(move |i: usize| depth.set(DEPTHS[i])) };
    let body = match (live_run.as_ref(), replayed.as_ref(), replayed48.as_ref()) {
        (Ok(r), Ok(rp), Ok(r48)) => html! { <>
            { part(1, "A short run, computed in your browser", live(r, *scheme, onscheme)) }
            { part(2, "The long runs, recorded and replayed", recorded_runs(rp, r, *scheme, *depth, ondepth)) }
            { part(3, "Beside the Python's run and the paper", beside(r48)) }
        </> },
        (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => html! { <pre class="err">{ e.clone() }</pre> },
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("training-at-depth".into())}>
        <header>
            { header("Training at depth", text::LEDE) }
        </header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                { p(text::QUESTION[0]) }
                { p(text::QUESTION[1]) }
                <p class="hint">{"This is the fourth page of the depth track, after "}<a href="../residual-streams/">{"Residual streams"}</a>{", "}<a href="../sinkhorn/">{"Sinkhorn-Knopp"}</a>{" and "}<a href="../gain-by-depth/">{"The gain by depth"}</a>{". "}<a href="../welcome/">{"The welcome page"}</a>{" shows how the lessons work. A word with a dotted underline opens a plain explanation. The paper: "}<a href="https://arxiv.org/abs/2512.24880">{"arXiv:2512.24880"}</a>{"; the Python re-implementation this track follows, and its measured results: "}<a href="https://github.com/softwarewrighter/mHC-poc">{"mHC-poc"}</a>{"."}</p>
            </> }) }
            { frame(Frame::Background, "What you need before", html! { <>{ for text::BACKGROUND.iter().map(|t| p(t)) }</> }) }
            { frame(Frame::Rationale, "Why train a tiny model, and record the long runs", p(text::RATIONALE)) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ p(text::PURPOSE[0]) }{ p(text::PURPOSE[1]) }</> }) }
            <section class="panel">
                <h2>{"How much to show"}</h2>
                <Switch mode={*mode} onchange={onmode} />
                <p class="note">{"Advanced adds the two programs and the trainer. Switching keeps the kind of connection and the depth you picked."}</p>
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
                    <h3>{"The library functions the programs use"}</h3>
                    { table(PROGRAM) }
                    <h3>{"The words of X_eTaL itself that they use"}</h3>
                    { words(PROGRAM) }
                    <h3>{"The training program"}</h3>
                    { p(text::CODE[1]) }
                    { linker.block(&with(&s)) }
                    <h3>{"The replay, and the trainer"}</h3>
                    { p(text::CODE[2]) }
                    { replay_linker.block(&replay_with(*depth)) }
                    <pre class="trainer">{ include_str!("../../train/train.sh") }</pre>
                    <p class="note">{"What made the records (data/runs.txt):"}</p>
                    <pre class="trainer">{ RUNS }</pre>
                </section>
            }) }
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
