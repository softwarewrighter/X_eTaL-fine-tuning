//! The page, summary D6 of the depth track, for new eyes: the plain
//! skip, HC and mHC side by side. The question, the background, the
//! rationale and the purpose; part 1, the starting tables and their
//! products, worked out; part 2, the nine recorded runs at the depth
//! picked, measured; part 3, beside the Python's run (X_eTaL's charts
//! and the figures, quoted); part 4, beside the paper (quoted); part 5,
//! the numbers each way trains, worked out; part 6, what the toy
//! showed and what it did not; the summary and what is next; the code
//! (Advanced), with the Python's functions beside the X_eTaL.
//! Presentation only: every number is one X_eTaL printed, or a figure
//! quoted as the Python's or the paper's.

use yew::prelude::*;

use lesson::arithmetic::{rounded, show};
use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::explain::{table, words};
use lesson::heat::{largest, Heat, Matrix};
use lesson::link::param;
use lesson::mode::{when, Mode, Switch};
use microscope::chrome::picture;
use microscope::run::now;

use crate::micro::{run, Run, DEPTHS, FILES, PROGRAM, RUNS, S, SCHEMES};
use crate::text;

/// Where the fourth page's training program was quoted from.
const FOURTH: &str = "https://github.com/softwarewrighter/X_eTaL-fine-tuning/blob/59793c2f302f11b6b5fadbc1cac4df7ee9a2ecdc/demos/training-at-depth/training-at-depth.xtl";
/// The Python's functions (MLX version) and its results page.
const PY_SRC: &str = "https://github.com/softwarewrighter/mHC-poc/tree/cf4504f1a4a0e091a00351bb9770b49fc1575747/mlx/src";
const PY_RESULTS: &str = "https://github.com/softwarewrighter/mHC-poc/blob/cf4504f1a4a0e091a00351bb9770b49fc1575747/docs/results.md";
/// The imports the quoted lines of the fourth page use, for their links.
const FOURTH_IMPORTS: &str = "\"act:\" u_se< \"Activations\"\n\"mx:\" u_se< \"Matrix\"\n\"nn:\" u_se< \"NN\"\n\"op:\" u_se< \"Optim\"\n";

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

/// A power of ten as the tables show it: two decimals.
fn pow(v: f64) -> String {
    let r = rounded(v, 2);
    if r == 0.0 { "0".into() } else { show(r) }
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

/// Part 1: the starting tables and their products, worked out.
fn start(r: &Run) -> Html {
    let fills = [
        ("hc_d", cut(r.hc[0])),
        ("hc_o", cut(r.hc[1])),
        ("m_d", cut(r.mhc[0])),
        ("m_o", cut(r.mhc[1])),
        ("hc48", pow(r.at_start(2)[0])),
        ("m48", cut(r.at_start(2)[2])),
        ("mp48", cut(r.at_start(2)[3])),
    ];
    let (a, b) = (Matrix { rows: S, cols: S, items: r.hc.clone() }, Matrix { rows: S, cols: S, items: r.mhc.clone() });
    let scale = largest(&[&a, &b]);
    let rows = DEPTHS.iter().enumerate().map(|(d, l)| {
        let s = r.at_start(d);
        html! { <tr><th scope="row">{ format!("{l} layers") }</th><td>{ pow(s[0]) }</td><td>{ pow(s[1]) }</td><td>{ pow(s[2]) }</td><td>{ pow(s[3]) }</td></tr> }
    });
    html! { <>
        <p>{ said(text::START[0], &fills) }</p>
        <div class="tables">
            <Heat name="HC's H_res" caption="HC's H_res at the start (worked out)" matrix={a} {scale} shared=true legend=true numbers=true cell=48.0 marks={Vec::new()} hint=false />
            <Heat name="mHC's H_res" caption="mHC's H_res at the start (worked out)" matrix={b} {scale} shared=true legend=true numbers=true cell=48.0 marks={Vec::new()} hint=false />
        </div>
        <p>{ said(text::START[1], &fills) }</p>
        <div class="scroll">
            <table class="vectors">
                <caption>{"The product of the starting tables over the layers, in powers of ten (worked out)"}</caption>
                <thead><tr><th></th><th scope="col">{"HC, forward gain"}</th><th scope="col">{"HC, proxy"}</th><th scope="col">{"mHC, forward gain"}</th><th scope="col">{"mHC, proxy"}</th></tr></thead>
                <tbody>{ for rows }</tbody>
            </table>
        </div>
        <div class="said" role="status"><p>{ said(text::START[2], &fills) }</p></div>
    </> }
}

/// Part 2: the nine runs, the three at the depth picked side by side.
fn runs(r: &Run, d: usize, on: Callback<usize>) -> Html {
    let f = |s: usize, k: usize| r.fig(s, d, k);
    let fills = [
        ("depth", DEPTHS[d].to_string()),
        ("b", cut(f(0, 0))),
        ("h", cut(f(1, 0))),
        ("m", cut(f(2, 0))),
        ("bg", cut(f(0, 2))),
        ("hg", pow(f(1, 4))),
        ("hs", pow(r.at_start(d)[0])),
        ("off", cut(r.off)),
        ("bmax", cut(f(0, 3))),
        ("hmax", cut(f(1, 3))),
        ("mmax", cut(f(2, 3))),
    ];
    let names: Vec<(String, String)> = DEPTHS.iter().map(|l| (format!("{l} layers"), format!("the three runs at {l} layers, 800 steps each"))).collect();
    // The rows: a figure, how it is shown, and its column in the run's figures.
    let rows: [(&str, usize, bool); 9] = [
        ("loss at step 800", 0, false),
        ("smallest loss of any step", 1, false),
        ("gradient size at step 800", 2, false),
        ("largest gradient size of any step", 3, false),
        ("forward gain at step 800 (power of ten)", 4, true),
        ("largest forward gain of any step (power of ten)", 5, true),
        ("backward gain at step 800 (power of ten)", 6, true),
        ("proxy at step 800 (power of ten)", 7, true),
        ("NaN events", 8, false),
    ];
    let body = rows.iter().map(|(name, k, p10)| {
        let cell = |s: usize| if *p10 { pow(f(s, *k)) } else { cut(f(s, *k)) };
        html! { <tr><th scope="row">{ name.to_string() }</th><td>{ cell(0) }</td><td>{ cell(1) }</td><td>{ cell(2) }</td></tr> }
    });
    let stuck = f(0, 0) > 2.0;
    let lines = r.checks.iter().map(|c| html! { <li class={classes!("check", if c.ok { "ok" } else { "fail" })}><code>{ c.line.clone() }</code></li> });
    html! { <>
        { p(text::RUNS[0]) }
        { buttons(&names, d, on) }
        <div class="scroll">
            <table class="vectors">
                <caption>{ format!("The three runs at {} layers, 800 steps each (measured here: the fourth page's offline runs, read from their records)", DEPTHS[d]) }</caption>
                <thead><tr><th></th>{ for SCHEMES.iter().map(|s| html! { <th scope="col">{ s.to_string() }</th> }) }</tr></thead>
                <tbody>{ for body }</tbody>
            </table>
        </div>
        <div class="said" role="status">
            <p>{ said(text::RUNS[1], &fills) }</p>
            { if stuck { html! { <p>{ said(text::RUNS[2], &fills) }</p> } } else { html! {} } }
            <p>{ said(text::RUNS[3], &fills) }</p>
            <p>{ said(text::RUNS[4], &fills) }</p>
        </div>
        <h3>{"The program's checks"}</h3>
        <ul class="checks">{ for lines }</ul>
        <p>{ format!("The Check library's report, as X_eTaL printed it: {}.", r.summary) }</p>
    </> }
}

/// Part 3: beside the Python's run.
fn python(r: &Run) -> Html {
    let fills = [
        ("py_h48", cut(r.py(1, 2).0)),
        ("py_b", DEPTHS.iter().enumerate().map(|(d, _)| cut(r.py(0, d).0)).collect::<Vec<_>>().join(", ")),
    ];
    let rows = (0..3).flat_map(|s| (0..3).map(move |d| (s, d))).map(|(s, d)| {
        html! { <tr><th scope="row">{ format!("{}, {} layers", SCHEMES[s], DEPTHS[d]) }</th><td>{ cut(r.fig(s, d, 0)) }</td><td class="theirs">{ cut(r.py(s, d).0) }</td><td>{ pow(r.fig(s, d, 7)) }</td><td class="theirs">{ pow(r.py(s, d).1) }</td></tr> }
    });
    html! { <>
        { p(text::PYTHON[0]) }
        { p(text::PYTHON[1]) }
        <div class="charts">
            <div class="chart">{ picture(&r.charts[0], "The final loss by depth as a fraction of guessing evenly, in powers of ten: the three here (measured) and the Python's three (quoted)") }</div>
            <div class="chart">{ picture(&r.charts[1], "The proxy by depth: HC and mHC at the start (worked out), after training here (measured) and after training in the Python's run (quoted)") }</div>
        </div>
        <div class="scroll">
            <table class="vectors">
                <caption>{"Final losses and proxies after 800 steps: this page's runs (measured here; 16 tokens, guessing evenly 2.77) beside the Python's run (quoted from its results page; 256 tokens, guessing evenly 5.55)"}</caption>
                <thead><tr><th></th><th scope="col">{"loss, here"}</th><th scope="col">{"loss, the Python's"}</th><th scope="col">{"proxy, here"}</th><th scope="col">{"proxy, the Python's"}</th></tr></thead>
                <tbody>{ for rows }</tbody>
            </table>
        </div>
        <p class="note">{"The Python's figures: "}<a href={PY_RESULTS}>{"mHC-poc, docs/results.md"}</a>{" (its plain skip has no mixing table, so its proxy is given as 0)."}</p>
        { p(text::PYTHON[2]) }
        <p>{ said(text::PYTHON[3], &fills) }</p>
    </> }
}

/// Part 4: beside the paper, quoted.
fn paper() -> Html {
    let rows = text::PAPER_TABLE.iter().map(|(b, x, h, m)| html! { <tr><th scope="row">{ b.to_string() }</th><td class="theirs">{ x.to_string() }</td><td class="theirs">{ h.to_string() }</td><td class="theirs">{ m.to_string() }</td></tr> });
    html! { <>
        { p(text::PAPER[0]) }
        <div class="scroll">
            <table class="vectors">
                <caption>{"The paper's 27-billion-weight models on five benchmarks (quoted from its table 4)"}</caption>
                <thead><tr><th></th><th scope="col">{"baseline"}</th><th scope="col">{"HC"}</th><th scope="col">{"mHC"}</th></tr></thead>
                <tbody>{ for rows }</tbody>
            </table>
        </div>
        { p(text::PAPER[1]) }
        { p(text::PAPER[2]) }
        { p(text::PAPER[3]) }
    </> }
}

/// Part 5: the numbers each way trains, worked out.
fn counts(r: &Run) -> Html {
    let fills = [
        ("block", show(r.per_layer[0])),
        ("plain", show(r.per_layer[1])),
        ("static", show(r.per_layer[2])),
        ("dynamic", show(r.per_layer[3])),
        ("emb", show(r.per_layer[4])),
    ];
    let names = ["the plain skip", "HC or mHC", "mHC, input-dependent tables (the fifth page's)"];
    let rows = names.iter().enumerate().map(|(s, n)| html! { <tr><th scope="row">{ n.to_string() }</th><td>{ show(r.total(s, 0)) }</td><td>{ show(r.total(s, 1)) }</td><td>{ show(r.total(s, 2)) }</td></tr> });
    html! { <>
        <p>{ said(text::COUNTS[0], &fills) }</p>
        <div class="scroll">
            <table class="vectors">
                <caption>{"The numbers trained, in all (worked out from the toy's sizes)"}</caption>
                <thead><tr><th></th>{ for DEPTHS.iter().map(|l| html! { <th scope="col">{ format!("{l} layers") }</th> }) }</tr></thead>
                <tbody>{ for rows }</tbody>
            </table>
        </div>
        { p(text::COUNTS[1]) }
        { p(text::COUNTS[2]) }
    </> }
}

/// The Python's functions beside the X_eTaL.
fn beside() -> Html {
    let here = Linker::of(PROGRAM);
    let fourth = Linker::of(FOURTH_IMPORTS);
    let rows = text::BESIDE.iter().map(|(py, what, x, wh)| {
        let code = if *wh == "this page" { here.code(x) } else { fourth.code(x) };
        let place = if *wh == "this page" { html! { {"this page"} } } else { html! { <a href={FOURTH}>{"the fourth page"}</a> } };
        html! { <tr><th scope="row"><code>{ py.to_string() }</code></th><td class="what">{ what.to_string() }</td><td class="xtl">{ code }</td><td>{ place }</td></tr> }
    });
    html! {
        <div class="scroll">
            <table class="vectors beside">
                <caption>{"The Python (mHC-poc, "}<a href={PY_SRC}>{"mlx/src"}</a>{") beside the X_eTaL"}</caption>
                <thead><tr><th scope="col">{"the Python"}</th><th scope="col">{"what it does"}</th><th scope="col">{"the X_eTaL"}</th><th scope="col">{"where"}</th></tr></thead>
                <tbody>{ for rows }</tbody>
            </table>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    // Where the page starts may be named in its address (lesson::link): ?mode=explorer&depth=24.
    let mode = use_state(|| Mode::from_link().unwrap_or(Mode::Beginner));
    let depth = use_state(|| param("depth").and_then(|d| d.parse::<usize>().ok()).and_then(|d| DEPTHS.iter().position(|x| *x == d)).unwrap_or(2));
    let result = use_memo((), |_| {
        let t0 = now();
        let r = run();
        if let Some(doc) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) {
            let _ = doc.set_attribute("data-xetal-ms", &format!("{:.0}", now() - t0));
        }
        r
    });
    let linker = Linker::of(PROGRAM);
    let onmode = { let mode = mode.clone(); Callback::from(move |m| mode.set(m)) };
    let ondepth = { let depth = depth.clone(); Callback::from(move |i: usize| depth.set(i)) };
    let body = match result.as_ref() {
        Err(e) => html! { <pre class="err">{ e.clone() }</pre> },
        Ok(r) => html! { <>
            { part(1, "At the start: the arithmetic", start(r)) }
            { part(2, "After training: the nine runs", runs(r, *depth, ondepth)) }
            { part(3, "Beside the Python's run", python(r)) }
            { part(4, "Beside the paper", paper()) }
            { part(5, "The numbers each way trains", counts(r)) }
            { part(6, "What the toy showed, and what it did not", html! { <>
                <h3>{"Showed"}</h3>
                <ul>{ for text::SHOWED.iter().map(|t| li(t)) }</ul>
                <h3>{"Did not show"}</h3>
                <ul>{ for text::NOT_SHOWN.iter().map(|t| li(t)) }</ul>
            </> }) }
        </> },
    };
    let texts = text::all();
    let data = FILES.iter().map(|(n, _)| html! { <li><code>{ n.to_string() }</code></li> });
    html! {
        <ContextProvider<Here> context={Here("mhc-compared".into())}>
        <header>
            { header("mHC compared", text::LEDE) }
        </header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                { p(text::QUESTION[0]) }
                { p(text::QUESTION[1]) }
                <p class="hint">{"This is the last page of the depth track, after "}<a href="../residual-streams/">{"Residual streams"}</a>{", "}<a href="../sinkhorn/">{"Sinkhorn-Knopp"}</a>{", "}<a href="../gain-by-depth/">{"The gain by depth"}</a>{", "}<a href="../training-at-depth/">{"Training at depth"}</a>{" and "}<a href="../dynamic-mixing/">{"Dynamic mixing"}</a>{". "}<a href="../welcome/">{"The welcome page"}</a>{" shows how the lessons work. A word with a dotted underline opens a plain explanation. The paper: "}<a href="https://arxiv.org/abs/2512.24880">{"arXiv:2512.24880"}</a>{"; the Python re-implementation and its results: "}<a href="https://github.com/softwarewrighter/mHC-poc">{"mHC-poc"}</a>{"."}</p>
            </> }) }
            { frame(Frame::Background, "What you need before", html! { <>{ for text::BACKGROUND.iter().map(|t| p(t)) }</> }) }
            { frame(Frame::Rationale, "Why set them side by side", p(text::RATIONALE)) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ p(text::PURPOSE[0]) }{ p(text::PURPOSE[1]) }</> }) }
            <section class="panel">
                <h2>{"How much to show"}</h2>
                <Switch mode={*mode} onchange={onmode} />
                <p class="note">{"Advanced adds the program, the Python beside it, and the data. Switching keeps the depth you picked."}</p>
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
                    <h3>{"The Python beside the X_eTaL"}</h3>
                    { p(text::CODE[1]) }
                    { beside() }
                    <h3>{"The library functions the program uses"}</h3>
                    { table(PROGRAM) }
                    <h3>{"The words of X_eTaL itself that it uses"}</h3>
                    { words(PROGRAM) }
                    <h3>{"The program"}</h3>
                    { p(text::CODE[2]) }
                    { linker.block(PROGRAM) }
                    <h3>{"The data"}</h3>
                    { p(text::CODE[3]) }
                    <ul>{ for data }</ul>
                    <p class="note">{"What made the records (data/runs.txt):"}</p>
                    <pre class="trainer">{ RUNS }</pre>
                </section>
            }) }
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
