//! The page, lesson 3, for new eyes: where the numbers of A and B come
//! from. The question, what is needed from the lessons before, why
//! one step is taken apart, and what the page is; the task (the frozen
//! table, the hidden shift, the examples trained on and held out, the
//! adapter at the start); one step of training in six stages, each
//! sentence quoting the run's numbers; training by Adam with its
//! record (X_eTaL's chart of the two losses, three experiments as
//! data, the record to step back through, what the record shows said
//! from measured numbers); what that showed and did not; and the code
//! (Advanced). Presentation only: every number is one X_eTaL printed,
//! and Rust never trains, differences or sums anything.

use web_sys::HtmlInputElement;
use yew::prelude::*;

use lesson::arithmetic::{rounded, show};
use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::explain::{table, words};
use lesson::heat::{key, largest, Heat, Mark, Matrix};
use lesson::link::{number, param};
use lesson::mode::{when, Mode, Switch};
use lesson::narrate::{held_out, moved};
use lesson::stepper::{flow, Stepper};
use microscope::chrome::picture;

use crate::micro::{pipeline, presets, program, run, Settings, Training, HELD, M, N, PROGRAM};
use crate::text;

/// The six stages of the step as the page tells them, and the
/// pipeline's stages each one shows.
const STAGES: [&str; 6] = ["p", "g", "b", "dA dB", "A2 B2", "W2"];
const SHOWN: [&[&str]; 6] = [&["h", "u", "p"], &["g"], &["b"], &["dA", "dB"], &["A2", "B2"], &["W2", "A3", "B3", "s2"]];
/// Which of the six each one is told from (for the picture's dots).
const READS: [&[usize]; 6] = [&[], &[1], &[2], &[3], &[4], &[]];

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

/// A sentence with its `{marks}` filled from the run, then drawn as prose.
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

/// A picture of a table, numbers written in, at a shared scale.
fn heat(name: &str, caption: String, m: Matrix, scale: f64, marks: Vec<(usize, Mark)>, cell: f64) -> Html {
    html! { <Heat name={name.to_string()} {caption} matrix={m} {scale} shared=true legend=false numbers=true {cell} {marks} hint=false /> }
}

/// Part 1: what training has.
fn task(t: &Training) -> Html {
    let s = &t.settings;
    let (w, tt) = (matrix(N, N, &t.w), matrix(N, N, &t.t));
    let (a, b) = (matrix(s.r, N, t.a_at(0)), matrix(N, s.r, t.b_at(0)));
    let xt = matrix(N, s.n, &Training::columns(&t.x, N, M, 0, s.n));
    let xh = matrix(N, HELD, &Training::columns(&t.x, N, M, s.n, M));
    let yt = matrix(N, s.n, &Training::columns(&t.y, N, M, 0, s.n));
    let yh = matrix(N, HELD, &Training::columns(&t.y, N, M, s.n, M));
    let tables = largest(&[&w, &tt, &a, &b]);
    let examples = largest(&[&xt, &xh, &yt, &yh]);
    html! { <>
        { p(text::TASK[0]) }
        <div class="tables">
            { heat("W", "W: the frozen table (8 rows, 8 columns)".into(), w, tables, vec![], 40.0) }
            { heat("T", "T: the hidden shift (8 rows, 8 columns); the examples come from W + T".into(), tt, tables, vec![], 40.0) }
        </div>
        { p(text::TASK[2]) }
        <div class="tables">
            { heat("A", format!("A at the start ({} rows, 8 columns): small numbers by a fixed rule", s.r), a, tables, vec![], 40.0) }
            { heat("B", format!("B at the start (8 rows, {} columns): all zeros", s.r), b, tables, vec![], 40.0) }
        </div>
        { key(tables, true) }
        { p(text::TASK[1]) }
        <div class="tables">
            { heat("x trained on", format!("the inputs trained on: {} examples, one per column", s.n), xt, examples, vec![], 34.0) }
            { heat("x held out", format!("the inputs held out: {HELD} examples"), xh, examples, vec![], 34.0) }
        </div>
        <div class="tables">
            { heat("y trained on", format!("the outputs wanted for the {} trained on", s.n), yt, examples, vec![], 34.0) }
            { heat("y held out", format!("the outputs wanted for the {HELD} held out"), yh, examples, vec![], 34.0) }
        </div>
        { key(examples, true) }
    </> }
}

/// The pictures of the pipeline's stages a told stage shows.
fn pictures(t: &Training, step: usize) -> Html {
    let s = &t.settings;
    let shape = |label: &str| -> (usize, usize) {
        match label {
            "h" | "b" => (s.r, s.n),
            "u" | "p" | "g" => (N, s.n),
            "dA" | "A2" | "A3" => (s.r, N),
            "dB" | "B2" | "B3" => (N, s.r),
            "s2" => (1, 1),
            _ => (N, N),
        }
    };
    let caption = |label: &str| -> String {
        match label {
            "h" => format!("h = A x: the squeeze, {} numbers per example", s.r),
            "u" => "u = B h: the spread".into(),
            "p" => "p: the prediction, W x + s u (one column per example)".into(),
            "g" => "g = p - y: the error, and the gradient at the output".into(),
            "b" => "b = B' g: the error carried back through B".into(),
            "dA" => "dA: A's gradient".into(),
            "dB" => "dB: B's gradient".into(),
            "A2" => "A2 = A - lr dA: A after the step".into(),
            "B2" => "B2 = B - lr dB: B after the step".into(),
            "W2" => "W2: the frozen table, given back".into(),
            "A3" => "A after the step".into(),
            "B3" => "B after the step".into(),
            "s2" => "s: the strength, given back".into(),
            other => other.into(),
        }
    };
    let mut maps: Vec<(String, String, Matrix)> = SHOWN[step]
        .iter()
        .map(|label| {
            let (rows, cols) = shape(label);
            (label.to_string(), caption(label), matrix(rows, cols, t.stage(label)))
        })
        .collect();
    // Beside the prediction, the outputs wanted; beside W2, W itself.
    if step == 0 {
        maps.push(("y".into(), "y: the outputs wanted".into(), matrix(N, s.n, &Training::columns(&t.y, N, M, 0, s.n))));
    }
    if step == 5 {
        maps.insert(1, ("W".into(), "W: the frozen table, before".into(), matrix(N, N, &t.w)));
    }
    let scale = largest(&maps.iter().map(|(_, _, m)| m).collect::<Vec<_>>());
    let drawn = maps.into_iter().map(|(name, caption, m)| {
        let written = SHOWN[step].contains(&name.as_str());
        let marks = if written { (0..m.items.len()).map(|k| (k, Mark::Written)).collect() } else { vec![] };
        let cell = if m.cols > N { 30.0 } else { 36.0 };
        html! { <div key={format!("{name}-{step}")}>{ heat(&name, caption, m, scale, marks, cell) }</div> }
    });
    html! { <>
        <div class="tables stages">{ for drawn }</div>
        { key(scale, true) }
    </> }
}

/// Part 2: one step of training, in six stages.
fn one_step(t: &Training, step: usize, onstep: Callback<usize>, mode: Mode, linker: &Linker) -> Html {
    let s = &t.settings;
    let fills = [
        ("before", cut(t.before)),
        ("after", cut(t.after)),
        ("lr", show(s.lr)),
        ("wmoved", show(t.w_moved)),
        ("off", if t.off == 0.0 { "0".to_string() } else { format!("{:.0e}", t.off) }),
    ];
    let shown: Vec<&str> = t.trace.stages.iter().filter(|st| SHOWN[step].contains(&st.label.as_str())).map(|st| st.text.as_str()).collect();
    html! { <>
        { p(text::STEP[0]) }
        { p(text::STEP[1]) }
        <div key={format!("flow-{step}")}>{ flow(&STAGES, step, READS[step]) }</div>
        <Stepper len={6} {step} {onstep} unit="Stage" />
        <p class="stage-says" role="status" key={format!("says-{step}")}>{ said(text::STAGES[step], &fills) }</p>
        { when(mode, Mode::Explorer, html! { <>
            <p class="note">{"This stage in the program:"}</p>
            { for shown.iter().map(|text| linker.block(text)) }
        </> }) }
        <p class="marks-key">{"Outlines:"}<span class="k written"></span>{"worked out in this stage"}</p>
        { pictures(t, step) }
        { when(mode, Mode::Advanced, html! { <>
            <h3>{"The step as mathematics"}</h3>
            <p class="note">{"Every stage of the step, drawn by X_eTaL-ML's Flow from the same text that is run: a row a stage."}</p>
            <div class="tex" data-tex={t.tex.clone()}>{ t.tex.clone() }</div>
        </> }) }
    </> }
}

/// What the record shows, said from its numbers: the two losses from
/// start to end, and whether the trained-on loss fell steadily.
fn reading(t: &Training) -> Html {
    let (lt, lh) = (&t.loss_t, &t.loss_h);
    let end = lt.len() - 1;
    let both = held_out((lt[0], lt[end]), (lh[0], lh[end]));
    // The largest rise between one record and the next, on the examples trained on.
    let rise = (1..lt.len()).map(|k| (lt[k] - lt[k - 1], k)).fold((0.0, 0), |a, b| if b.0 > a.0 { b } else { a });
    let fill = |text: &str, fills: &[(&str, String)]| fills.iter().fold(text.to_string(), |s, (m, v)| s.replace(&format!("{{{m}}}"), v));
    let steady = if rise.0 > 0.05 * lt[0] {
        fill(text::READING[0], &[("k0", (rise.1 - 1).to_string()), ("k1", rise.1.to_string()), ("s0", t.step_of(rise.1 - 1).to_string()), ("s1", t.step_of(rise.1).to_string()), ("a", cut(lt[rise.1 - 1])), ("b", cut(lt[rise.1]))])
    } else if rise.0 > 0.0 {
        text::READING[1].to_string()
    } else {
        text::READING[2].to_string()
    };
    // Where the held-out loss ended against where it started, in words of the record.
    let held = if lh[end] > lh[0] {
        let low = lh.iter().enumerate().fold((0, lh[0]), |a, (k, v)| if *v < a.1 { (k, *v) } else { a });
        let high = lh.iter().enumerate().fold((0, lh[0]), |a, (k, v)| if *v > a.1 { (k, *v) } else { a });
        fill(text::READING[3], &[("low", cut(low.1)), ("k", low.0.to_string()), ("s", t.step_of(low.0).to_string()), ("high", cut(high.1)), ("kh", high.0.to_string()), ("sh", t.step_of(high.0).to_string())])
    } else {
        String::new()
    };
    html! { <div class="said" role="status">
        <p>{ format!("{both} {steady} {held}").trim_end().to_string() }</p>
        <p class="note">{ format!("Said from the record alone: {} records, the start and then one every {} steps, {} steps in all. Every loss is per example, as X_eTaL printed it, to 6 decimals.", t.records(), s_every(t), t.step_of(end)) }</p>
    </div> }
}

fn s_every(t: &Training) -> usize {
    t.settings.every
}

/// Part 3: training with its record.
fn training(t: &Training, at: usize, onat: Callback<usize>, chosen: usize, onpreset: Callback<usize>) -> Html {
    let s = &t.settings;
    let buttons = presets().into_iter().enumerate().map(|(i, p)| {
        let onclick = { let onpreset = onpreset.clone(); Callback::from(move |_| onpreset.emit(i)) };
        html! { <button type="button" class={classes!("preset", (i == chosen).then_some("on"))} {onclick} aria-pressed={(i == chosen).to_string()}><b>{ p.title.clone() }</b><span>{ p.setup.clone() }</span></button> }
    });
    let onat = Callback::from(move |e: InputEvent| {
        if let Ok(k) = e.target_unchecked_into::<HtmlInputElement>().value().parse::<usize>() {
            onat.emit(k);
        }
    });
    let (a, b, d, tt) = (matrix(s.r, N, t.a_at(at)), matrix(N, s.r, t.b_at(at)), matrix(N, N, t.d_at(at)), matrix(N, N, &t.t));
    let scale = largest(&[&a, &b, &d, &tt]);
    let end = t.records() - 1;
    html! { <>
        { p(text::TRAIN[0]) }
        { p(text::TRAIN[1]) }
        <div class="presets">{ for buttons }</div>
        <p class="setup"><b>{ s.title.clone() }</b>{ format!(": rank {}, alpha {}, learning rate {}, {} examples trained on and {HELD} held out, {} steps, a record every {}.", s.r, s.alpha, s.lr, s.n, t.step_of(end), s.every) }</p>
        { picture(&t.chart, "The loss per example on the examples trained on and on the held-out ones, a record a point") }
        { reading(t) }
        { p(text::TRAIN[2]) }
        <label class="rank">{"record"}<input type="range" min="0" max={end.to_string()} step="1" value={at.to_string()} oninput={onat} aria-label={format!("The record to show, 0 to {end}")} /><output>{ format!("{at} of {end}, after {} steps", t.step_of(at)) }</output></label>
        <p class="said">{ format!("At record {at}, after {} steps: the loss per example is {} on the examples trained on and {} on the held-out ones.", t.step_of(at), cut(t.loss_t[at]), cut(t.loss_h[at])) }</p>
        <div class="tables" key={format!("rec-{at}-{chosen}")}>
            { heat("A", format!("A at record {at}"), a, scale, vec![], 36.0) }
            { heat("B", format!("B at record {at}"), b, scale, vec![], 36.0) }
            { heat("s B A", format!("s B A: the adapter's table at record {at} (times an input, it gives the correction)"), d, scale, vec![], 36.0) }
            { heat("T", "T: the hidden shift".into(), tt, scale, vec![], 36.0) }
        </div>
        { key(scale, true) }
        <p class="note">{ format!("{}.", moved("From record 0 to this one, the loss per example on the examples trained on", t.loss_t[0], t.loss_t[at])) }</p>
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    // Where the page starts may be named in its address (lesson::link):
    // ?mode=explorer&preset=2&stage=4&record=30 (the stage from 1, the preset from 0).
    let all = presets();
    let mode = use_state(|| Mode::from_link().unwrap_or(Mode::Beginner));
    let chosen = use_state(|| param("preset").and_then(|p| p.parse::<usize>().ok()).filter(|i| *i < 3).unwrap_or(0));
    let step = use_state(|| number("stage", 1, 6).map_or(0, |n| n - 1));
    let at = use_state(|| param("record").and_then(|p| p.parse::<usize>().ok()));
    let settings: Settings = all[*chosen].clone();
    let result = use_memo(settings.clone(), |s| run(s));
    let linker = Linker::of(PROGRAM);
    let onmode = { let mode = mode.clone(); Callback::from(move |m| mode.set(m)) };
    let onstep = { let step = step.clone(); Callback::from(move |s| step.set(s)) };
    let onat = { let at = at.clone(); Callback::from(move |k| at.set(Some(k))) };
    let onpreset = { let (chosen, at) = (chosen.clone(), at.clone()); Callback::from(move |i| { chosen.set(i); at.set(None); }) };
    let body = match result.as_ref() {
        Err(e) => html! { <pre class="err">{ e.clone() }</pre> },
        Ok(t) => {
            let end = t.records() - 1;
            let record = at.map_or(end, |k| k.min(end));
            html! { <>
                { part(1, "What training has", task(t)) }
                { part(2, "One step of training, in six stages", one_step(t, *step, onstep, *mode, &linker)) }
                { part(3, "Training, with its record", training(t, record, onat, *chosen, onpreset)) }
            </> }
        }
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("lora-training".into())}>
        <header>
            { header("LoRA training", text::LEDE) }
        </header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                { p(text::QUESTION[0]) }
                { p(text::QUESTION[1]) }
                <p class="hint">{"This is lesson 3. It builds on "}<a href="../lora-hello/">{"lesson 1, LoRA hello"}</a>{" and "}<a href="../lora-lab/">{"lesson 2, the LoRA lab"}</a>{"; "}<a href="../welcome/">{"the welcome page"}</a>{" shows how the lessons work. A word with a dotted underline opens a plain explanation."}</p>
            </> }) }
            { frame(Frame::Background, "What you need from the lessons before", html! { <>{ for text::BACKGROUND.iter().map(|t| p(t)) }</> }) }
            { frame(Frame::Rationale, "Why take one step apart", p(text::RATIONALE)) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ p(text::PURPOSE[0]) }{ p(text::PURPOSE[1]) }</> }) }
            <section class="panel">
                <h2>{"How much to show"}</h2>
                <Switch mode={*mode} onchange={onmode} />
                <p class="note">{"Explorer adds each stage as the program writes it. Advanced adds the step drawn as mathematics and the program. Switching keeps the experiment, the stage and the record you are on."}</p>
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
                    <h3>{"The step as a pipeline"}</h3>
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
