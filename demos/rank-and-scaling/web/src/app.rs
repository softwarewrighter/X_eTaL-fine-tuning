//! The page, lesson 4, for new eyes: what the rank buys and costs,
//! and whether the rule for the strength matters. The question, what
//! is needed from the lessons before, why a sweep, and what the page
//! is; a prediction first; the sweep's table with every figure marked
//! measured or analytical, X_eTaL's charts, and the alpha and steps
//! sliders that rerun it; a rank to read about, with what changed from
//! the rank before said from the rows; the account of the whole table
//! from its numbers; what that showed and did not; and the code
//! (Advanced). Presentation only: every number is one X_eTaL printed,
//! and the sentences compare numbers without judging them.

use web_sys::HtmlInputElement;
use yew::prelude::*;

use lesson::arithmetic::{commas, show};
use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, Here};
use lesson::explain::{table, words};
use lesson::link::{number, param};
use lesson::mode::{when, Mode, Switch};
use lesson::predict::Predict;
use microscope::chrome::picture;

use crate::micro::{run, with, Settings, Sweep, ALPHAS, PROGRAM, RANKS, STEPS};
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

/// A number as the table shows it, to 3 decimals.
fn cut(v: f64) -> String {
    show((v * 1000.0).round() / 1000.0)
}

/// A slider over a few values, with the value shown.
fn slider(label: &str, values: &[f64], at: usize, onat: Callback<usize>, aria: String) -> Html {
    let oninput = Callback::from(move |e: InputEvent| {
        if let Ok(k) = e.target_unchecked_into::<HtmlInputElement>().value().parse::<usize>() {
            onat.emit(k);
        }
    });
    html! { <label class="rank">{ label.to_string() }<input type="range" min="0" max={(values.len() - 1).to_string()} step="1" value={at.to_string()} {oninput} aria-label={aria} /><output>{ show(values[at]) }</output></label> }
}

/// The sweep's table, every figure marked A (analytical) or M (measured).
fn sweep_table(s: &Sweep, chosen: usize) -> Html {
    let head = |t: &str, mark: &'static str| html! { <th scope="col">{ t.to_string() }<span class={classes!("mark", mark)}>{ mark }</span></th> };
    let rows = s.rows.iter().map(|r| {
        html! { <tr class={classes!((r.rank == chosen).then_some("chosen"))}>
            <th scope="row">{ r.rank }</th>
            <td>{ commas(r.trained) }</td>
            <td>{ cut(r.by_r.s) }</td><td>{ cut(r.by_r.train) }</td><td>{ cut(r.by_r.held) }</td><td>{ cut(r.by_r.size) }</td>
            <td>{ cut(r.by_sqrt.s) }</td><td>{ cut(r.by_sqrt.train) }</td><td>{ cut(r.by_sqrt.held) }</td><td>{ cut(r.by_sqrt.size) }</td>
        </tr> }
    });
    html! { <>
        <div class="scroll"><table class="sweep">
            <tr><th></th><th></th><th scope="colgroup" colspan="4">{"s = alpha / r"}</th><th scope="colgroup" colspan="4">{"s = alpha / sqrt r"}</th></tr>
            <tr>
                { head("rank", "A") }{ head("numbers trained", "A") }
                { head("s", "A") }{ head("loss, trained on", "M") }{ head("loss, held out", "M") }{ head("size of s B A", "M") }
                { head("s", "A") }{ head("loss, trained on", "M") }{ head("loss, held out", "M") }{ head("size of s B A", "M") }
            </tr>
            { for rows }
        </table></div>
        <p class="note">{ format!("A: analytical, from the setup alone. M: measured, after {} steps with alpha {}. Before any training the loss per example is {} on the examples trained on and {} on the held-out ones, at every rank. T, the shift the adapter is to come to, has size {}. Numbers to 3 decimals.", s.settings.steps, show(s.settings.alpha), cut(s.before.0), cut(s.before.1), cut(s.t_size)) }</p>
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    // Where the page starts may be named in its address (lesson::link):
    // ?mode=explorer&alpha=2&steps=1&rank=5 (alpha and steps as positions, from 0; the rank itself).
    let mode = use_state(|| Mode::from_link().unwrap_or(Mode::Beginner));
    let alpha = use_state(|| param("alpha").and_then(|v| v.parse::<usize>().ok()).filter(|i| *i < ALPHAS.len()).unwrap_or(2));
    let steps = use_state(|| param("steps").and_then(|v| v.parse::<usize>().ok()).filter(|i| *i < STEPS.len()).unwrap_or(2));
    let rank = use_state(|| number("rank", 1, RANKS).unwrap_or(4));
    let settings = Settings { alpha: ALPHAS[*alpha], steps: STEPS[*steps] };
    let result = use_memo(settings, |s| run(*s));
    let linker = Linker::of(PROGRAM);
    let onmode = { let mode = mode.clone(); Callback::from(move |m| mode.set(m)) };
    let onalpha = { let alpha = alpha.clone(); Callback::from(move |k| alpha.set(k)) };
    let onsteps = { let steps = steps.clone(); Callback::from(move |k| steps.set(k)) };
    let onrank = {
        let rank = rank.clone();
        Callback::from(move |e: InputEvent| {
            if let Ok(r) = e.target_unchecked_into::<HtmlInputElement>().value().parse::<usize>() {
                if (1..=RANKS).contains(&r) {
                    rank.set(r);
                }
            }
        })
    };
    let steps_values: Vec<f64> = STEPS.iter().map(|s| *s as f64).collect();
    let body = match result.as_ref() {
        Err(e) => html! { <pre class="err">{ e.clone() }</pre> },
        Ok(s) => {
            let (_, every) = s.lowest_held(false);
            let choices: Vec<AttrValue> = [text::PREDICT[1], "No: from some rank to the next it rises", "It stays the same at every rank"].iter().map(|c| AttrValue::from(*c)).collect();
            let right = if every { 0 } else { 1 };
            let r = *rank;
            html! { <>
                { part(1, "A guess first", html! { <>
                    { p(text::PREDICT[0]) }
                    <Predict question={format!("After {} steps with alpha {}, under s = alpha / r: does the held-out loss fall from each rank to the next?", s.settings.steps, show(s.settings.alpha))} {choices} {right} why={text::PREDICT[2].replace("{answer}", &s.answer())} />
                </> }) }
                { part(2, "The sweep", html! { <>
                    { p(text::TABLE[0]) }
                    { p(text::TABLE[1]) }
                    { slider("alpha", &ALPHAS, *alpha, onalpha, "alpha, the number the strength is set from".into()) }
                    { slider("steps", &steps_values, *steps, onsteps, "how many steps each rank is trained for".into()) }
                    { sweep_table(s, r) }
                    { p(text::TABLE[2]) }
                    <div class="charts">
                        { picture(&s.charts[0], "The loss per example on the examples trained on and on the held-out ones, by rank, under s = alpha / r") }
                        { picture(&s.charts[1], "The same under s = alpha / sqrt r") }
                        { picture(&s.charts[2], "The size of s B A by rank under each rule, beside the size of T") }
                    </div>
                </> }) }
                { part(3, "A rank to read about", html! { <>
                    { p(text::READ[0]) }
                    <label class="rank">{"rank"}<input type="range" min="1" max={RANKS.to_string()} step="1" value={r.to_string()} oninput={onrank} aria-label="The rank to read about, 1 to 8" /><output>{ r }</output></label>
                    <div class="said" role="status">
                        if r == 1 {
                            <p>{ format!("Rank 1 is the smallest: {} numbers trained (analytical). Under s = alpha / r the loss on the examples trained on after {} steps is {} and on the held-out ones {}; under alpha / sqrt r, {} and {} (measured). At rank 1 the two rules give the same strength, {}, so any difference between the two rows is in the run alone.", commas(s.at(1).trained), s.settings.steps, cut(s.at(1).by_r.train), cut(s.at(1).by_r.held), cut(s.at(1).by_sqrt.train), cut(s.at(1).by_sqrt.held), cut(s.at(1).by_r.s)) }</p>
                        } else {
                            { for s.step_said(r - 1, r).iter().map(|t| html! { <p>{ t.clone() }</p> }) }
                        }
                    </div>
                    { p(text::READ[1]) }
                    <div class="account">
                        { for s.account().iter().map(|t| html! { <p>{ t.clone() }</p> }) }
                    </div>
                </> }) }
            </> }
        }
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("rank-and-scaling".into())}>
        <header>
            { header("Rank and scaling", text::LEDE) }
        </header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                { p(text::QUESTION[0]) }
                { p(text::QUESTION[1]) }
                <p class="hint">{"This is lesson 4. It builds on "}<a href="../lora-lab/">{"lesson 2, the LoRA lab"}</a>{" and "}<a href="../lora-training/">{"lesson 3, LoRA training"}</a>{"; "}<a href="../welcome/">{"the welcome page"}</a>{" shows how the lessons work. A word with a dotted underline opens a plain explanation."}</p>
            </> }) }
            { frame(Frame::Background, "What you need from the lessons before", html! { <>{ for text::BACKGROUND.iter().map(|t| p(t)) }</> }) }
            { frame(Frame::Rationale, "Why sweep it", p(text::RATIONALE)) }
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
                    <h3>{"The sweep"}</h3>
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
