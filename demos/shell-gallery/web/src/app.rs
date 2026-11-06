//! The page: each part of shared/lesson working on one run's trace.
//! Framed as every page is; the depth switch; a question first; the
//! pipeline stepped through with its values as heat maps (what a stage
//! writes outlined solid, what it reads dashed); a sum written out; a
//! change in words; then the ideas and, for those who want it, the
//! code. Presentation only: every number is one X_eTaL printed.

use yew::prelude::*;

use lesson::arithmetic::{numeric, symbolic};
use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, p, prose, Here};
use lesson::heat::{column, key, largest, Heat, Mark, Matrix};
use lesson::mode::{when, Mode, Switch};
use lesson::narrate::count;
use lesson::predict::Predict;
use lesson::stepper::Stepper;

use crate::micro::{pipeline, program, run, Gallery, PROGRAM};
use crate::text;

/// What each stage's value is called on the page.
const NAMES: [&str; 4] = ["x", "h", "u", "y"];
const CAPTIONS: [&str; 4] = ["x: the input", "h: the squeeze", "u: the spread", "y: the output"];

/// The pipeline stepped through: the controls, what the stage on show
/// does, its code (Advanced), and every stage's value as a heat map.
fn stepping(g: &Gallery, step: usize, onstep: Callback<usize>, mode: Mode, linker: &Linker) -> Html {
    let stages = &g.trace.stages;
    let maps: Vec<Matrix> = stages.iter().map(|s| column(&s.items)).collect();
    let scale = largest(&maps.iter().collect::<Vec<_>>());
    let reads = &stages[step].reads;
    let drawn = maps.iter().enumerate().map(|(i, m)| {
        // The stage on show writes its own value; the stages it reads are marked read.
        let mark = if i == step { Some(Mark::Written) } else if reads.contains(&(i + 1)) { Some(Mark::Read) } else { None };
        let marks: Vec<(usize, Mark)> = mark.map(|mk| (0..m.items.len()).map(|k| (k, mk)).collect()).unwrap_or_default();
        html! { <Heat name={NAMES[i]} caption={CAPTIONS[i]} matrix={m.clone()} {scale} shared=true {marks} legend=false /> }
    });
    html! {
        <section class="panel">
            <h2>{"A computation, stepped through"}</h2>
            { p(text::STEPPING) }
            <Stepper len={stages.len()} {step} {onstep} />
            <p class="stage-says" role="status">{ prose(text::STAGES[step]) }</p>
            { when(mode, Mode::Advanced, html! { <>
                <p class="note">{"This stage in the program:"}</p>
                { linker.block(&stages[step].text) }
            </> }) }
            <p class="marks-key">{"In the pictures:"}<span class="k written"></span>{"worked out by this stage"}<span class="k read"></span>{"read by this stage"}</p>
            <div class="tables stages">{ for drawn }</div>
            { key(scale, true) }
            { when(mode, Mode::Explorer, html! { <>
                <h3>{"The tables the stages use"}</h3>
                <div class="tables">
                    <Heat name="W" caption="W: the frozen table" matrix={Matrix { rows: 4, cols: 4, items: g.w.clone() }} scale={largest(&[&Matrix { rows: 4, cols: 4, items: g.w.clone() }])} meaning="How much the output of that row takes from the input of that column." />
                    <Heat name="A" caption="A: the one row of the adapter" matrix={Matrix { rows: 1, cols: 4, items: g.a.clone() }} scale={largest(&[&Matrix { rows: 1, cols: 4, items: g.a.clone() }])} legend=false />
                    <Heat name="B" caption="B: the one column of the adapter" matrix={column(&g.b)} scale={largest(&[&column(&g.b)])} legend=false />
                </div>
            </> }) }
        </section>
    }
}

/// The parts that need the run: the question, the stepper, the sum,
/// the change in words.
fn parts(g: &Gallery, step: usize, onstep: Callback<usize>, mode: Mode, linker: &Linker) -> Html {
    let x = &g.trace.stages[0].items;
    // How many numbers the squeeze gives, from the run: the answer to the question.
    let out = g.trace.stages[1].items.len();
    let choices: Vec<AttrValue> = ["1", "4", "16"].iter().map(|c| AttrValue::from(*c)).collect();
    let right = choices.iter().position(|c| c.as_str() == out.to_string()).unwrap_or(0);
    let (r1, r2, table) = g.counts;
    html! { <>
        <section class="panel">
            <h2>{"A question first"}</h2>
            <Predict question={text::PREDICT[0]} {choices} {right} why={text::PREDICT[1]} />
        </section>
        { stepping(g, step, onstep, mode, linker) }
        <section class="panel">
            <h2>{"A sum written out"}</h2>
            { p(text::SUM) }
            { when(mode, Mode::Advanced, symbolic("W x", "W", "x", 1, 4)) }
            { numeric("row 1 of W, times x", &g.w[..4], x, &g.row1, g.row1_sum) }
        </section>
        <section class="panel">
            <h2>{"A change, in words"}</h2>
            { p(text::CHANGE) }
            <p class="stage-says">{ format!("Raise the rank of the adapter from 1 to 2: {}.", count("the numbers it trains", r1, r2, Some((table, "the table itself")))) }</p>
        </section>
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    let mode = use_state(|| Mode::Beginner);
    let step = use_state(|| 0usize);
    let result = use_memo((), |_| run());
    let linker = Linker::of(PROGRAM);
    let onmode = { let mode = mode.clone(); Callback::from(move |m| mode.set(m)) };
    let onstep = { let step = step.clone(); Callback::from(move |s| step.set(s)) };
    let body = match result.as_ref() {
        Err(e) => html! { <pre class="err">{ e.clone() }</pre> },
        Ok(g) => parts(g, *step, onstep, *mode, &linker),
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("shell-gallery".into())}>
        <header>
            { header("Shell gallery", text::LEDE) }
        </header>
        <main>
            { frame(Frame::Question, "What is a lesson made of?", p(text::QUESTION)) }
            { frame(Frame::Background, "The computation on show", p(text::BACKGROUND)) }
            { frame(Frame::Rationale, "One run, only arranged", p(text::RATIONALE)) }
            { frame(Frame::Purpose, "What this page is, and is not", p(text::PURPOSE)) }
            <section class="panel">
                <h2>{"Three depths"}</h2>
                { p(text::MODES) }
                <Switch mode={*mode} onchange={onmode} />
            </section>
            { body }
            { frame(Frame::Summary, "What was shown", p(text::SUMMARY)) }
            { frame(Frame::Next, "Where next", html! { <p>{ text::NEXT }{" "}<a href="../lora-hello/">{"LoRA hello"}</a>{"."}</p> }) }
            <section class="panel">
                <h2>{"Every idea on this page, explained simply"}</h2>
                { index(&texts) }
            </section>
            { when(*mode, Mode::Advanced, html! {
                <section class="panel code">
                    <h2>{"The code, for those who want it"}</h2>
                    <p class="note">{"The pipeline, as the program writes it, then the whole program the page runs. Each function name links to its entry in the cross-reference."}</p>
                    { linker.block(pipeline()) }
                    { linker.block(&program()) }
                </section>
            }) }
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
