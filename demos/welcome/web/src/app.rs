//! The page: a welcome for someone new. It greets them, says what the
//! lessons are and why they are built as they are, then gives them the
//! seven things they will meet in every lesson to try, each on one
//! small calculation: words that open, a guess first, the calculation
//! stepped through (animated: the numbers a stage works out appear in
//! order, a dot travels to it from each stage it uses), pictures of
//! the numbers, a sum written out, a change in words, and how much to
//! show. Presentation only: every number is one X_eTaL printed.

use yew::prelude::*;

use lesson::arithmetic::{numeric, show, symbolic};
use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, p, prose, Here};
use lesson::heat::{brief, column, key, largest, Heat, Mark, Matrix};
use lesson::mode::{when, Mode, Switch};
use lesson::narrate::moved;
use lesson::predict::Predict;
use lesson::stepper::{flow, Stepper};

use crate::micro::{pipeline, program, run, Welcome, PROGRAM};
use crate::text;

/// What each stage's value is called, and its caption.
const NAMES: [&str; 4] = ["x", "h", "u", "y"];
const CAPTIONS: [&str; 4] = ["x: the input", "h: boiled down", "u: spread out", "y: the output"];

/// A numbered part of the tour.
fn part(n: usize, title: &str, body: Html) -> Html {
    html! {
        <section class="panel part">
            <h2><span class="n">{ n }</span>{ title.to_string() }</h2>
            { body }
        </section>
    }
}

/// The calculation stepped through: the stages in a row with the dots
/// that travel between them, the controls, what the stage on show
/// does, and every stage's numbers as a picture.
fn stepping(g: &Welcome, step: usize, onstep: Callback<usize>, mode: Mode, linker: &Linker) -> Html {
    let stages = &g.trace.stages;
    let maps: Vec<Matrix> = stages.iter().map(|s| column(&s.items)).collect();
    let scale = largest(&maps.iter().collect::<Vec<_>>());
    let reads = &stages[step].reads;
    let drawn = maps.iter().enumerate().map(|(i, m)| {
        // The stage on show works its own numbers out; the stages it uses are marked read.
        let mark = if i == step { Some(Mark::Written) } else if reads.contains(&(i + 1)) { Some(Mark::Read) } else { None };
        let marks: Vec<(usize, Mark)> = mark.map(|mk| (0..m.items.len()).map(|k| (k, mk)).collect()).unwrap_or_default();
        // A new key at each step, so the numbers of the stage come in afresh.
        // A stage not reached yet is drawn empty: its numbers appear when it is worked out.
        html! { <Heat key={format!("{i}-{step}")} name={NAMES[i]} caption={CAPTIONS[i]} matrix={m.clone()} {scale} shared=true {marks} legend=false numbers=true cell={52.0} pending={i > step} hint=false /> }
    });
    let uses: Vec<&str> = reads.iter().map(|r| NAMES[r - 1]).collect();
    html! { <>
        { p(text::STEPS[0]) }
        { p(text::STEPS[1]) }
        { p(text::STEPS[2]) }
        <div key={format!("flow-{step}")}>{ flow(&NAMES, step, reads) }</div>
        <Stepper len={stages.len()} {step} {onstep} unit="Stage" />
        <p class="stage-says" role="status" key={format!("says-{step}")}>{ prose(text::STAGES[step]) }</p>
        { when(mode, Mode::Explorer, html! {
            <p class="note">{ if uses.is_empty() { "This stage uses no earlier stage.".to_string() } else { format!("This stage uses: {}.", uses.join(" and ")) } }</p>
        }) }
        { when(mode, Mode::Advanced, html! { <>
            <p class="note">{"This stage in the program:"}</p>
            { linker.block(&stages[step].text) }
        </> }) }
        <p class="marks-key">{"Outlines:"}<span class="k written"></span>{"being worked out"}<span class="k read"></span>{"used to work it out"}</p>
        <div class="tables stages">{ for drawn }</div>
        { key(scale, true) }
        <p class="note">{ text::STEPS[3] }</p>
    </> }
}

/// The parts that need the run.
fn tour(g: &Welcome, step: usize, onstep: Callback<usize>, mode: Mode, linker: &Linker) -> Html {
    let x = &g.trace.stages[0].items;
    // How many numbers stage 2 gives, from the run: the answer to the guess.
    let out = g.trace.stages[1].items.len();
    let choices: Vec<AttrValue> = ["1", "4", "16"].iter().map(|c| AttrValue::from(*c)).collect();
    let right = choices.iter().position(|c| c.as_str() == out.to_string()).unwrap_or(0);
    let (w, a, b) = (Matrix { rows: 4, cols: 4, items: g.w.clone() }, Matrix { rows: 1, cols: 4, items: g.a.clone() }, column(&g.b));
    let tables = largest(&[&w, &a, &b]);
    html! { <>
        { part(2, "A guess first", html! { <>
            { p(text::GUESS[0]) }
            <Predict question={text::GUESS[1]} {choices} {right} why={text::GUESS[2]} />
        </> }) }
        { part(3, "A calculation, step by step", stepping(g, step, onstep, mode, linker)) }
        { part(4, "Pictures of the numbers", html! { <>
            { p(text::PICTURES[0]) }
            { p(text::PICTURES[1]) }
            <div class="tables">
                <Heat name="A" caption="A: the small table of stage h (1 row)" matrix={a} scale={tables} shared=true legend=false numbers=true cell={52.0} />
                <Heat name="B" caption="B: the small table of stage u (1 column)" matrix={b} scale={tables} shared=true legend=false numbers=true cell={52.0} />
                <Heat name="W" caption="W: the larger table of stage y (4 rows, 4 columns)" matrix={w} scale={tables} shared=true legend=false numbers=true cell={52.0} />
            </div>
            { key(tables, true) }
        </> }) }
        { part(5, "A sum written out", html! { <>
            { p(text::SUM[0]) }
            { when(mode, Mode::Advanced, symbolic("h", "A", "x", 1, 4)) }
            { numeric("h", &g.a, x, &g.products, g.sum) }
            { p(text::SUM[1]) }
        </> }) }
        { part(6, "A change, in words", html! { <>
            { p(text::CHANGE[0]) }
            { p(text::CHANGE[1]) }
            <p class="said">{ format!("{}.", moved("The first number of the output", g.y[0], g.stronger)) }</p>
            <p class="note">{ format!("({} is the first number of stage y; its square in part 3 shows it rounded, as {}. Both numbers here come from working the calculation out, once each way.)", show(g.y[0]), brief(g.y[0])) }</p>
        </> }) }
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    // Where the page starts may be named in its address (lesson::link): ?mode=explorer&stage=3.
    let mode = use_state(|| Mode::from_link().unwrap_or(Mode::Beginner));
    let step = use_state(|| lesson::link::number("stage", 1, 4).map_or(0, |n| n - 1));
    let result = use_memo((), |_| run());
    let linker = Linker::of(PROGRAM);
    let onmode = { let mode = mode.clone(); Callback::from(move |m| mode.set(m)) };
    let onstep = { let step = step.clone(); Callback::from(move |s| step.set(s)) };
    let body = match result.as_ref() {
        Err(e) => html! { <pre class="err">{ e.clone() }</pre> },
        Ok(g) => tour(g, *step, onstep, *mode, &linker),
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("welcome".into())}>
        <header>
            { header("Welcome: how these lessons work", text::LEDE) }
        </header>
        <main>
            { frame(Frame::Question, "Welcome", html! { <>{ p(text::QUESTION[0]) }{ p(text::QUESTION[1]) }</> }) }
            { frame(Frame::Background, "What a lesson is like", p(text::BACKGROUND)) }
            { frame(Frame::Purpose, "What to do on this page", p(text::PURPOSE)) }
            { part(1, "Words that open", html! { <>{ p(text::WORDS[0]) }{ p(text::WORDS[1]) }</> }) }
            { body }
            { part(7, "Three depths", html! { <>
                { p(text::DEPTHS) }
                <Switch mode={*mode} onchange={onmode} />
            </> }) }
            { frame(Frame::Rationale, "Why the lessons work this way", p(text::RATIONALE)) }
            { frame(Frame::Summary, "What you have seen", p(text::SUMMARY)) }
            { frame(Frame::Next, "Where next", html! { <>
                { p(text::NEXT) }
                <p><a class="go" href="../lora-hello/">{"Start the first lesson: LoRA hello"}</a></p>
            </> }) }
            <section class="panel">
                <h2>{"Every idea on this page, explained simply"}</h2>
                <p class="note">{"The same explanations the underlined words open, in one place."}</p>
                { index(&texts) }
            </section>
            { when(*mode, Mode::Advanced, html! {
                <section class="panel code">
                    <h2>{"The code, for those who want it"}</h2>
                    <p class="note">{"The calculation as the program writes it, then the whole program the page runs. Each function name links to its entry in the cross-reference. The first lesson says how to read it."}</p>
                    { linker.block(pipeline()) }
                    { linker.block(&program()) }
                </section>
            }) }
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
