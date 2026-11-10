//! The page, lesson 5, for new eyes: LoRA beside LoRA-FA in two
//! experiments kept apart. The question, what is needed from the
//! lessons before, why, and what the page is; Experiment A
//! (approximating a known matrix): a guess, the two fits charted
//! against the lowest loss possible at the rank and LoRA-FA's floor,
//! T and the two learned updates as pictures, the spectrum, and the
//! account from the numbers; Experiment B (learning from examples): a
//! guess, the three methods' losses before and after, the charts and
//! the account, with the first retention measure; what that showed and
//! did not; the code (Advanced). Presentation only: every number is one
//! X_eTaL printed, and the sentences compare numbers without judging.

use web_sys::HtmlInputElement;
use yew::prelude::*;

use lesson::arithmetic::{commas, show};
use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, Here};
use lesson::explain::{table, words};
use lesson::heat::{key, largest, Heat, Matrix};
use lesson::link::number;
use lesson::mode::{when, Mode, Switch};
use lesson::predict::Predict;
use microscope::chrome::picture;

use crate::micro::{q, run, with, Run, Settings, EXAMPLES, METHODS, PROGRAM, RANKS};
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

/// A slider over whole numbers, with the value shown.
fn slider(label: &str, low: usize, high: usize, at: usize, onat: Callback<usize>, aria: &'static str) -> Html {
    let oninput = Callback::from(move |e: InputEvent| {
        if let Ok(k) = e.target_unchecked_into::<HtmlInputElement>().value().parse::<usize>() {
            if (low..=high).contains(&k) {
                onat.emit(k);
            }
        }
    });
    html! { <label class="rank">{ label.to_string() }<input type="range" min={low.to_string()} max={high.to_string()} step="1" value={at.to_string()} {oninput} aria-label={aria} /><output>{ at }</output></label> }
}

/// A mark on a figure: A (analytical) or M (measured).
fn mark(m: &'static str) -> Html {
    html! { <span class={classes!("mark", m)}>{ m }</span> }
}

/// The spectrum's table: T's singular values and the lowest loss
/// possible at each rank, all analytical.
fn spectrum(r: &Run) -> Html {
    html! { <>
        <div class="scroll"><table class="sweep">
            <tr><th scope="row">{"rank"}{ mark("A") }</th>{ for (1..=RANKS).map(|k| html! { <td class={classes!((k == r.settings.rank).then_some("chosen"))}>{ k }</td> }) }</tr>
            <tr><th scope="row">{"singular value"}{ mark("A") }</th>{ for r.singular.iter().map(|v| html! { <td>{ q(*v) }</td> }) }</tr>
            <tr><th scope="row">{"lowest loss possible"}{ mark("A") }</th>{ for r.lowest.iter().map(|v| html! { <td>{ q(*v) }</td> }) }</tr>
        </table></div>
        <p class="note">{ format!("Numbers to 3 decimals; the rank set is marked. The lowest loss possible at rank k is half the sum of the squared singular values past the k-th. The check of the spectrum code: run on a matrix built from the singular values 5, 3, 2, 1, 0.5 and three 0s, its largest error is {}.", show(r.check)) }</p>
    </> }
}

/// The three tables of Experiment A on one scale: T, LoRA's update, LoRA-FA's.
fn updates(r: &Run) -> Html {
    let m = |v: &Vec<f64>| Matrix { rows: 8, cols: 8, items: v.clone() };
    let (t, l, f) = (m(&r.t), m(&r.lora.update), m(&r.fa.update));
    let scale = largest(&[&t, &l, &f]);
    html! { <>
        <div class="tables">
            <Heat name="T" caption="T: the shift to come to (8 by 8, rank 4)" matrix={t} {scale} shared=true legend=false hint=false />
            <Heat name="LoRA" caption="s B A as LoRA learned it, A and B moved" matrix={l} {scale} shared=true legend=false hint=false />
            <Heat name="LoRA-FA" caption="s B A as LoRA-FA learned it, B alone moved" matrix={f} {scale} shared=true legend=false hint=false />
        </div>
        { key(scale, true) }
    </> }
}

/// Experiment B's table: a method a row, the three losses before and
/// after, the numbers trained.
fn learn_table(r: &Run) -> Html {
    let counts = [r.lora.trained, r.fa.trained, 64];
    let rows = r.records().into_iter().zip(METHODS).zip(counts).map(|((rec, name), n)| {
        let (a, b) = (rec.first(), rec.last());
        html! { <tr>
            <th scope="row">{ name }</th>
            <td>{ commas(n) }</td>
            <td>{ q(a[0]) }</td><td>{ q(b[0]) }</td>
            <td>{ q(a[1]) }</td><td>{ q(b[1]) }</td>
            <td>{ q(a[2]) }</td><td>{ q(b[2]) }</td>
            <td>{ q(rec.change) }</td>
        </tr> }
    });
    html! { <>
        <div class="scroll"><table class="sweep">
            <tr><th></th><th></th><th scope="colgroup" colspan="2">{"trained on"}</th><th scope="colgroup" colspan="2">{"held out"}</th><th scope="colgroup" colspan="2">{"old examples"}</th><th></th></tr>
            <tr>
                <th scope="col">{"method"}</th><th scope="col">{"numbers trained"}{ mark("A") }</th>
                <th scope="col">{"before"}{ mark("M") }</th><th scope="col">{"after"}{ mark("M") }</th>
                <th scope="col">{"before"}{ mark("M") }</th><th scope="col">{"after"}{ mark("M") }</th>
                <th scope="col">{"before"}{ mark("M") }</th><th scope="col">{"after"}{ mark("M") }</th>
                <th scope="col">{"size of the change"}{ mark("M") }</th>
            </tr>
            { for rows }
        </table></div>
        <p class="note">{ format!("Losses per example, to 3 decimals, before any training and after {} steps on {} examples at rank {}. The size of the change is the size of s B A, or of the moved W less W. A: analytical, from the setup alone. M: measured.", r.steps(), r.settings.n, r.settings.rank) }</p>
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    // Where the page starts may be named in its address (lesson::link): ?mode=explorer&rank=2&n=3.
    let mode = use_state(|| Mode::from_link().unwrap_or(Mode::Beginner));
    let rank = use_state(|| number("rank", 1, RANKS).unwrap_or(4));
    let n = use_state(|| number("n", EXAMPLES.0, EXAMPLES.1).unwrap_or(8));
    let settings = Settings { rank: *rank, n: *n };
    let result = use_memo(settings, |s| run(*s));
    let linker = Linker::of(PROGRAM);
    let onmode = { let mode = mode.clone(); Callback::from(move |m| mode.set(m)) };
    let onrank = { let rank = rank.clone(); Callback::from(move |k| rank.set(k)) };
    let onn = { let n = n.clone(); Callback::from(move |k| n.set(k)) };
    let body = match result.as_ref() {
        Err(e) => html! { <pre class="err">{ e.clone() }</pre> },
        Ok(r) => {
            let choices_a: Vec<AttrValue> = [text::PREDICT_A[1], "No: LoRA-FA's loss stays higher", "No: LoRA-FA's loss ends lower"].iter().map(|c| AttrValue::from(*c)).collect();
            let right_a = if r.fit_same() { 0 } else if r.end(true) > r.end(false) { 1 } else { 2 };
            let choices_b: Vec<AttrValue> = METHODS.iter().map(|c| AttrValue::from(*c)).collect();
            let (right_b, _) = r.old_extremes();
            html! { <>
                <section class="panel">
                    <h2>{"The settings"}</h2>
                    { slider("rank", 1, RANKS, r.settings.rank, onrank, "The rank of the adapter, 1 to 8, for both experiments") }
                    { slider("examples trained on", EXAMPLES.0, EXAMPLES.1, r.settings.n, onn, "How many of the 12 examples are trained on in Experiment B, 2 to 8") }
                    <p class="note">{"A move runs the whole program again; the sentences below are written again from the new numbers."}</p>
                </section>
                { part(1, "Experiment A: approximating a known matrix. A guess first", html! { <>
                    { p(text::PREDICT_A[0]) }
                    <Predict question={format!("At rank {}, after {} steps fitting T: does LoRA-FA come to the same loss as LoRA?", r.settings.rank, r.steps())} choices={choices_a} right={right_a} why={text::PREDICT_A[2].replace("{answer}", &r.fit_answer())} />
                </> }) }
                { part(2, "Experiment A: the two fits", html! { <>
                    { p(text::FIT[0]) }
                    { picture(&r.charts[0], "The loss while fitting T, a record every 10 steps: LoRA, LoRA-FA, the lowest loss possible at this rank and LoRA-FA's floor") }
                    { updates(r) }
                    { p(text::FIT[1]) }
                    { spectrum(r) }
                    { p(text::FIT[2]) }
                    <div class="account">{ for r.fit_said().iter().map(|t| html! { <p>{ t.clone() }</p> }) }</div>
                </> }) }
                { part(3, "Experiment B: learning from examples. A guess first", html! { <>
                    { p(text::PREDICT_B[0]) }
                    <Predict question={format!("After {} steps on {} examples at rank {}: which moved the layer's outputs for the old examples least?", r.steps(), r.settings.n, r.settings.rank)} choices={choices_b} right={right_b} why={text::PREDICT_B[2].replace("{answer}", &r.old_answer())} />
                </> }) }
                { part(4, "Experiment B: the three runs", html! { <>
                    { p(text::LEARN[0]) }
                    { learn_table(r) }
                    <div class="charts">
                        { picture(&r.charts[1], "The loss per example on the examples trained on, a record every 10 steps: LoRA, LoRA-FA, full fine-tuning") }
                        { picture(&r.charts[2], "The same on the held-out examples") }
                        { picture(&r.charts[3], "The same on the old examples: the retention measure") }
                    </div>
                    { p(text::LEARN[1]) }
                    { p(text::LEARN[2]) }
                    <div class="account">{ for r.learn_said().iter().map(|t| html! { <p>{ t.clone() }</p> }) }</div>
                </> }) }
            </> }
        }
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("lora-vs-fa".into())}>
        <header>
            { header("LoRA beside LoRA-FA", text::LEDE) }
        </header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                { p(text::QUESTION[0]) }
                { p(text::QUESTION[1]) }
                <p class="hint">{"This is lesson 5. It builds on "}<a href="../lora-training/">{"lesson 3, LoRA training"}</a>{" and "}<a href="../rank-and-scaling/">{"lesson 4, rank and scaling"}</a>{"; "}<a href="../welcome/">{"the welcome page"}</a>{" shows how the lessons work. A word with a dotted underline opens a plain explanation."}</p>
            </> }) }
            { frame(Frame::Background, "What you need from the lessons before", html! { <>{ for text::BACKGROUND.iter().map(|t| p(t)) }</> }) }
            { frame(Frame::Rationale, "Why these two experiments", p(text::RATIONALE)) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ p(text::PURPOSE[0]) }{ p(text::PURPOSE[1]) }</> }) }
            <section class="panel">
                <h2>{"How much to show"}</h2>
                <Switch mode={*mode} onchange={onmode} />
                <p class="note">{"Advanced adds the program. Switching keeps the sliders where they are."}</p>
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
                    { p(text::CODE[1]) }
                    <h3>{"The library functions the program uses"}</h3>
                    { table(PROGRAM) }
                    <h3>{"The words of X_eTaL itself that it uses"}</h3>
                    { words(PROGRAM) }
                    <h3>{"The program"}</h3>
                    { p(text::CODE[2]) }
                    { linker.block(&with(settings)) }
                </section>
            }) }
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
