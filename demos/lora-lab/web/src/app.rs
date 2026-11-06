//! The page, lesson 2, for new eyes: the question lesson 1 left open
//! (how is the correction worked out, and what does the rank change?),
//! what is needed from lesson 1, why it is taken number by number, and
//! what the page is; then the layer's numbers; the calculation a stage
//! at a time, any worked-out number's sum written out when its square
//! is clicked; the formulas with letters to pick (Explorer); the rank
//! to change, with what changed said from the two runs; what the
//! adapter changed; what that showed and did not; and the code
//! (Advanced). Presentation only: every number is one X_eTaL printed.

use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;

use lesson::arithmetic::{rounded, show, symbolic, term, written_to};
use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::explain::{table, words};
use lesson::heat::{column, key, largest, Heat, Mark, Matrix};
use lesson::link::{number, param};
use lesson::mode::{when, Mode, Switch};
use lesson::narrate::{count, moved};
use lesson::predict::Predict;
use lesson::stepper::{flow, Stepper};

use crate::micro::{pipeline, program, run, Lab, Settings, N, PROGRAM};
use crate::text;

/// What each stage's value is called, and its caption.
const NAMES: [&str; 4] = ["x", "h", "u", "y"];
/// The tables (and the strength) each stage reads, besides earlier stages.
const TABLES: [&str; 4] = ["", "the table A", "the table B", "the table W and the strength s"];
const CAPTIONS: [&str; 4] = ["x: the input", "h: the squeeze", "u: the spread", "y: the output"];

/// How many decimals a written-out sum shows.
const PLACES: usize = 3;

/// A numbered part of the lesson.
fn part(n: usize, title: &str, body: Html) -> Html {
    html! {
        <section class="panel part">
            <h2><span class="n">{ n }</span>{ title.to_string() }</h2>
            { body }
        </section>
    }
}

/// A number as a sum shows it.
fn cut(v: f64) -> String {
    show(rounded(v, PLACES))
}

/// A row of numbers under a heading, to 3 decimals; `against` marks the
/// cells whose value differs from that row's.
fn row(name: &str, v: &[f64], against: Option<&[f64]>, class: &'static str) -> Html {
    let cells = v.iter().enumerate().map(|(i, x)| {
        let changed = against.is_some_and(|a| (a[i] - x).abs() > 1e-12);
        html! { <td class={classes!(changed.then_some("changed"))}>{ cut(*x) }</td> }
    });
    html! { <tr class={class}><th scope="row">{name.to_string()}</th>{ for cells }</tr> }
}

/// The layer's numbers: W, A, B and the input, one scale for all.
fn layer(g: &Lab, chosen: &Option<String>) -> Html {
    let (w, a, b) = (Matrix { rows: N, cols: N, items: g.w.clone() }, Matrix { rows: g.r, cols: N, items: g.a.clone() }, Matrix { rows: N, cols: g.r, items: g.b.clone() });
    let x = column(&g.trace.stages[0].items);
    let scale = largest(&[&w, &a, &b, &x]);
    let on = |name: &str| chosen.as_deref() == Some(name);
    html! { <>
        { p(text::LAYER[0]) }
        { p(text::LAYER[1]) }
        <div class="tables">
            <Heat name="W" caption="W: the frozen table (8 rows, 8 columns)" matrix={w} {scale} shared=true legend=false numbers=true cell={42.0} chosen={on("W")} meaning="How much that output takes from that number of the input." />
            <Heat name="A" caption={format!("A: the adapter's first table ({} rows, 8 columns)", g.r)} matrix={a} {scale} shared=true legend=false numbers=true cell={42.0} chosen={on("A")} />
            <Heat name="B" caption={format!("B: the adapter's second table (8 rows, {} columns)", g.r)} matrix={b} {scale} shared=true legend=false numbers=true cell={42.0} chosen={on("B")} />
            <Heat name="x" caption="x: the input (8 numbers)" matrix={x} {scale} shared=true legend=false numbers=true cell={42.0} />
        </div>
        { key(scale, true) }
    </> }
}

/// The sum behind the square last clicked, written out.
fn sum_of(g: &Lab, picked: Option<(usize, usize)>, step: usize, mode: Mode) -> Html {
    let Some((stage, k)) = picked.filter(|(stage, _)| *stage <= step) else { return html! { <p class="hint">{ text::SUMS[0] }</p> } };
    let Some((a, b, products, total)) = g.sum(stage, k) else { return html! { <p class="hint">{ text::SUMS[0] }</p> } };
    let n = k + 1;
    let (name, symbols) = match stage {
        1 => (format!("h({n})"), symbolic("h", "A", "x", n, N)),
        2 => (format!("u({n})"), symbolic("u", "B", "h", n, g.r)),
        _ => (format!("row {n} of W, times x"), symbolic("W x", "W", "x", n, N)),
    };
    let u = &g.trace.stages[2].items;
    html! { <>
        { when(mode, Mode::Advanced, symbols) }
        <p class="worked">{ written_to(PLACES, &name, &a, &b, &products, total) }</p>
        if stage == 3 {
            <p>{ text::SUMS[2] }</p>
            <p class="worked">{ format!("y({n}) = {} + {} \u{00d7} {} = {} + {} = {}", term(rounded(g.base[k], PLACES)), term(rounded(g.s, PLACES)), term(rounded(u[k], PLACES)), term(rounded(g.base[k], PLACES)), term(rounded(g.correction[k], PLACES)), cut(g.y[k])) }</p>
        }
        <p class="note">{ text::SUMS[1] }</p>
    </> }
}

/// The calculation a stage at a time, with each stage's numbers as a
/// picture whose squares can be clicked.
fn stepping(g: &Lab, step: usize, onstep: Callback<usize>, picked: Option<(usize, usize)>, onpick: Callback<(usize, usize)>, chosen: &Option<String>, mode: Mode, linker: &Linker) -> Html {
    let stages = &g.trace.stages;
    let maps: Vec<Matrix> = stages.iter().map(|s| column(&s.items)).collect();
    let scale = largest(&maps.iter().collect::<Vec<_>>());
    let reads = &stages[step].reads;
    let drawn = maps.iter().enumerate().map(|(i, m)| {
        let mark = if i == step { Some(Mark::Written) } else if reads.contains(&(i + 1)) { Some(Mark::Read) } else { None };
        let marks: Vec<(usize, Mark)> = mark.map(|mk| (0..m.items.len()).map(|k| (k, mk)).collect()).unwrap_or_default();
        let pick = { let onpick = onpick.clone(); Callback::from(move |k| onpick.emit((i, k))) };
        html! { <Heat key={format!("{i}-{step}-{}", g.r)} name={NAMES[i]} caption={CAPTIONS[i]} matrix={m.clone()} {scale} shared=true {marks} legend=false numbers=true cell={46.0} pending={i > step} hint=false onpick={pick} chosen={chosen.as_deref() == Some(NAMES[i])} /> }
    });
    let uses: Vec<&str> = reads.iter().map(|r| NAMES[r - 1]).collect();
    html! { <>
        { p(text::STEPS[0]) }
        { p(text::STEPS[1]) }
        <div key={format!("flow-{step}")}>{ flow(&NAMES, step, reads) }</div>
        <Stepper len={stages.len()} {step} {onstep} unit="Stage" />
        <p class="stage-says" role="status" key={format!("says-{step}")}>{ prose(text::STAGES[step]) }</p>
        { when(mode, Mode::Explorer, html! {
            <p class="note">{ if uses.is_empty() { "This stage uses no earlier stage.".to_string() } else { format!("This stage uses: {}, and {}.", uses.join(" and "), TABLES[step]) } }</p>
        }) }
        { when(mode, Mode::Advanced, html! { <>
            <p class="note">{"This stage in the program:"}</p>
            { linker.block(&stages[step].text) }
        </> }) }
        <p class="marks-key">{"Outlines:"}<span class="k written"></span>{"being worked out"}<span class="k read"></span>{"used to work it out"}</p>
        <div class="tables stages">{ for drawn }</div>
        { key(scale, true) }
        <h3>{"Where a number comes from"}</h3>
        { sum_of(g, picked, step, mode) }
    </> }
}

/// The three formulas with their letters to pick, and what the letter
/// picked stands for.
fn formulas(g: &Lab, chosen: &Option<String>, onchoose: Callback<String>) -> Html {
    // A click on a marked letter of a drawn formula: the nearest element marked as a symbol names it.
    let onclick = {
        let onchoose = onchoose.clone();
        Callback::from(move |e: MouseEvent| {
            let name = e.target_dyn_into::<Element>().and_then(|el| el.closest(".sym").ok().flatten()).and_then(|sym| sym.class_name().split_whitespace().find_map(|c| c.strip_prefix("sym-").map(str::to_string)));
            if let Some(name) = name {
                onchoose.emit(name);
            }
        })
    };
    // The same choices as buttons, for the keyboard and for touch.
    let letters = text::SYMBOLS.iter().map(|(name, _, _)| {
        let (onchoose, name) = (onchoose.clone(), name.to_string());
        let on = chosen.as_deref() == Some(name.as_str());
        html! { <button type="button" class={classes!(on.then_some("on"))} aria-pressed={on.to_string()} onclick={Callback::from({ let name = name.clone(); move |_| onchoose.emit(name.clone()) })}>{ name.clone() }</button> }
    });
    let what = chosen.as_deref().and_then(|c| text::SYMBOLS.iter().find(|(n, _, _)| *n == c));
    html! { <>
        { p(text::FORMULA[0]) }
        <div class="formulas" {onclick}>
            // The letter picked, shown picked in every formula.
            { for chosen.iter().map(|c| html! { <style>{ format!(".formulas .sym-{c} {{ color: var(--accent); font-weight: 700; border-bottom-style: solid; }}") }</style> }) }
            { for g.tex.iter().zip(["h: the squeeze", "u: the spread", "y: the output"]).map(|(t, stage)| html! { <div class="line"><span class="which">{ stage }</span><div class="tex" data-tex={t.clone()}>{ t.clone() }</div></div> }) }
        </div>
        <div class="letters" role="group" aria-label="The letters of the formulas">{ for letters }</div>
        <p class="said" role="status">{
            match what {
                Some((name, picture, meaning)) => html! { <><b>{ format!("{name}: ") }</b>{ *meaning }{ if picture.is_empty() { "" } else { " Its picture is outlined above." } }</> },
                None => html! { <span class="hint">{ text::FORMULA[2] }</span> },
            }
        }</p>
        { p(text::FORMULA[1]) }
    </> }
}

/// What changed when the rank moved, said from the two runs.
fn said(g: &Lab, before: Option<(usize, usize, f64)>) -> Html {
    let Some((r0, n0, s0)) = before.filter(|(r0, _, _)| *r0 != g.r) else {
        return html! { <p class="said">{ format!("At rank {}: the adapter has {} numbers, beside the {} of the table W itself.", g.r, g.trained.0, g.trained.1) }</p> };
    };
    html! {
        <div class="said" role="status">
            <p><b>{ format!("Rank {r0} to {}. ", g.r) }</b>{ format!("{}.", count("The numbers the adapter has", n0, g.trained.0, Some((g.trained.1, "the table W itself")))) }</p>
            <p>{ format!("Stage h now has {} {}: A has {} {}, and B has {} {}.", g.r, if g.r == 1 { "number" } else { "numbers" }, g.r, if g.r == 1 { "row" } else { "rows" }, g.r, if g.r == 1 { "column" } else { "columns" }) }</p>
            <p>{ format!("{}: it is alpha divided by the rank, which keeps the correction from growing just because the rank did.", moved("The strength s", s0, g.s)) }</p>
            <p>{ text::CHANGED[1] }</p>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    // Where the page starts may be named in its address (lesson::link):
    // ?mode=explorer&rank=5&stage=3&pick=2.1&letter=A (stage and cell from 1).
    let mode = use_state(|| Mode::from_link().unwrap_or(Mode::Beginner));
    let settings = use_state(|| Settings { r: number("rank", 1, N).unwrap_or(Settings::default().r) });
    let step = use_state(|| number("stage", 1, 4).map_or(0, |n| n - 1));
    let picked = use_state(|| param("pick").and_then(|p| p.split_once('.').and_then(|(a, b)| Some((a.parse::<usize>().ok()?.checked_sub(1)?, b.parse::<usize>().ok()?.checked_sub(1)?)))).filter(|(stage, k)| *stage < 4 && *k < N));
    let chosen = use_state(|| param("letter").filter(|l| text::SYMBOLS.iter().any(|(n, _, _)| n == l)));
    // The rank, the count and the strength before the last change of rank.
    let before = use_state(|| None::<(usize, usize, f64)>);
    let s = *settings;
    let result = use_memo(s, |s| run(*s));
    // A run at rank 5, for the answer to the question about the rank.
    let at_five = use_memo((), |_| run(Settings { r: 5 }).map(|g| g.trace.stages[1].items.len()).unwrap_or(0));
    let linker = Linker::of(PROGRAM);
    let onmode = { let mode = mode.clone(); Callback::from(move |m| mode.set(m)) };
    let onstep = { let step = step.clone(); Callback::from(move |s| step.set(s)) };
    let onpick = { let picked = picked.clone(); Callback::from(move |at| picked.set(Some(at))) };
    let onchoose = { let chosen = chosen.clone(); Callback::from(move |name| chosen.set(Some(name))) };
    let onrank = {
        let (settings, before, picked, result) = (settings.clone(), before.clone(), picked.clone(), result.clone());
        Callback::from(move |e: InputEvent| {
            if let Ok(r) = e.target_unchecked_into::<HtmlInputElement>().value().parse::<usize>() {
                if (1..=N).contains(&r) && r != settings.r {
                    if let Ok(g) = result.as_ref() {
                        before.set(Some((g.r, g.trained.0, g.s)));
                    }
                    // A square picked at the old rank may not exist at the new one.
                    picked.set(None);
                    settings.set(Settings { r });
                }
            }
        })
    };
    let choices: Vec<AttrValue> = ["2", "5", "8", "40"].iter().map(|c| AttrValue::from(*c)).collect();
    let right = choices.iter().position(|c| c.as_str() == at_five.to_string()).unwrap_or(1);
    let body = match result.as_ref() {
        Err(e) => html! { <pre class="err">{ e.clone() }</pre> },
        Ok(g) => html! { <>
            { part(1, "The layer's numbers", layer(g, &chosen)) }
            { part(2, "The calculation, one stage at a time", stepping(g, *step, onstep, *picked, onpick, &chosen, *mode, &linker)) }
            { when(*mode, Mode::Explorer, part(3, "The formulas, letter by letter", formulas(g, &chosen, onchoose))) }
            { part(if mode.shows(Mode::Explorer) { 4 } else { 3 }, "Change the rank", html! { <>
                { p(text::RANK[0]) }
                <Predict question={text::RANK[1]} choices={choices} {right} why={text::RANK[2]} />
                <label class="rank">{"rank"}<input type="range" min="1" max="8" step="1" value={s.r.to_string()} oninput={onrank} aria-label="The rank of the adapter, 1 to 8" /><output>{ s.r }</output></label>
                { said(g, *before) }
                { p(text::RANK[3]) }
            </> }) }
            { part(if mode.shows(Mode::Explorer) { 5 } else { 4 }, "What the adapter changed", html! { <>
                { p(text::CHANGED[0]) }
                <div class="scroll"><table class="vectors">
                    { row("W x: what the frozen table gives", &g.base, None, "") }
                    { row("+ s B (A x): the correction", &g.correction, None, "") }
                    { row("= y: the output", &g.y, Some(&g.base), "sum") }
                </table></div>
                <p class="muted">{ if g.same { "The program also works the whole layer out in one go, with the LoRA library's function from lesson 1, and compares: the two agree, which the program prints as a 1. " } else { "" } }{"Numbers here are shown to 3 decimals."}</p>
            </> }) }
        </> },
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("lora-lab".into())}>
        <header>
            { header("LoRA lab", text::LEDE) }
        </header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                { p(text::QUESTION[0]) }
                { p(text::QUESTION[1]) }
                <p class="hint">{"This is lesson 2. If you have not read "}<a href="../lora-hello/">{"lesson 1, LoRA hello"}</a>{", start there; and "}<a href="../welcome/">{"the welcome page"}</a>{" shows how the lessons work. A word with a dotted underline opens a plain explanation."}</p>
            </> }) }
            { frame(Frame::Background, "What you need from lesson 1", html! { <>{ p(text::BACKGROUND[0]) }{ p(text::BACKGROUND[1]) }</> }) }
            { frame(Frame::Rationale, "Why go through it number by number", p(text::RATIONALE)) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ p(text::PURPOSE[0]) }{ p(text::PURPOSE[1]) }</> }) }
            <section class="panel">
                <h2>{"How much to show"}</h2>
                <Switch mode={*mode} onchange={onmode} />
                <p class="note">{"Explorer adds the formulas, with letters to pick. Advanced adds each sum in symbols and the program. Switching keeps the rank and the stage you are on."}</p>
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
                    { linker.block(&program(s)) }
                </section>
            }) }
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
