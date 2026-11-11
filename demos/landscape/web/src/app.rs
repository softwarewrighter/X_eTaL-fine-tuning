//! The page, lesson 6, for new eyes: the map of the ways to change a
//! model. The question, the background, why a map now and what the
//! page is; the six families, each in plain words with one line of
//! arithmetic and where it stands in this course; the grid of the two
//! questions (where the change lives, how it is found) with every
//! lesson, live or to come, in its cell; the accounting X_eTaL
//! computed, every figure analytical; the three worries against what
//! has been measured; the curriculum; what this showed and did not;
//! and the code (Advanced). Presentation only: every count is one
//! X_eTaL printed, and the sentences compare counts without judging
//! the families.

use yew::prelude::*;

use lesson::arithmetic::commas;
use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::explain::{table, words};
use lesson::mode::{when, Mode, Switch};

use crate::micro::{bytes, run, Run, LAYERS, PROGRAM, ROWS};
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

/// A lesson live here, as a link, or to come, with its saga.
fn lesson_link(what: &str, slug: &str, saga: &str) -> Html {
    if !slug.is_empty() {
        html! { <a href={format!("../{slug}/")}>{ what.to_string() }</a> }
    } else if !saga.is_empty() {
        html! { <>{ what.to_string() }<span class="saga">{ format!("to come, {saga}") }</span></> }
    } else {
        html! { <>{ what.to_string() }<span class="saga">{ "held in reserve" }</span></> }
    }
}

/// The six families.
fn families() -> Html {
    html! { <>
        { for text::FAMILIES.iter().enumerate().map(|(i, f)| html! {
            <div class="family" id={format!("family-{}", i + 1)}>
                <h3>{ format!("{}. {}", i + 1, f.name) }</h3>
                { p(f.said) }
                <p class="arith">{ f.arithmetic }</p>
                <p class="status">{ prose(f.status) }</p>
            </div>
        }) }
    </> }
}

/// The grid: where the change may live against how it is found.
fn grid() -> Html {
    html! { <div class="scroll"><table class="grid">
        <tr><th scope="col">{"where the change may live"}</th>{ for text::FOUND.iter().map(|f| html! { <th scope="col">{ f.to_string() }</th> }) }</tr>
        { for text::LIVES.iter().zip(text::GRID.iter()).map(|(row, cells)| html! {
            <tr>
                <th scope="row">{ row.to_string() }</th>
                { for cells.iter().map(|cell| if cell.is_empty() {
                    html! { <td class="none">{"no lesson planned"}</td> }
                } else {
                    html! { <td><ul>{ for cell.iter().map(|c| html! { <li>{ lesson_link(c.what, c.slug, c.saga) }</li> }) }</ul></td> }
                }) }
            </tr>
        }) }
    </table></div> }
}

/// The accounting, every figure marked A (analytical).
fn counts(r: &Run) -> Html {
    let head = |t: &str| html! { <th scope="col">{ t.to_string() }<span class="mark">{"A"}</span></th> };
    let layer = |j: usize| format!("{} x {}, rank {}", LAYERS[j].0, LAYERS[j].0, LAYERS[j].1);
    html! { <div class="scroll"><table class="counts">
        <tr><th></th><th></th>{ for (0..2).map(|j| html! { <th scope="colgroup" colspan="3">{ layer(j) }</th> }) }<th></th></tr>
        <tr><th scope="col">{"method"}</th><th scope="col">{"family"}</th>
            { head("trained") }{ head("held while training") }{ head("stored apart") }
            { head("trained") }{ head("held while training") }{ head("stored apart") }
            <th scope="col">{"what is stored"}</th></tr>
        { for ROWS.iter().enumerate().map(|(i, row)| html! {
            <tr>
                <th scope="row">{ row.name }</th>
                <td class="note"><a href={format!("#family-{}", row.family + 1)}>{ format!("{}", row.family + 1) }</a></td>
                <td>{ commas(r.trained[i][0]) }</td><td>{ commas(r.held[i][0]) }</td><td>{ commas(r.stored[i][0]) }</td>
                <td>{ commas(r.trained[i][1]) }</td><td>{ commas(r.held[i][1]) }</td><td>{ commas(r.stored[i][1]) }</td>
                <td class="note">{ row.stored_note }</td>
            </tr>
        }) }
        <tr>
            <th scope="row">{"the layer W itself"}</th><td class="note">{"all"}</td>
            <td colspan="3" class="note">{ format!("{} at 16 bits a number; {} at 4 bits", bytes(r.bytes[0]), bytes(r.bytes[2])) }</td>
            <td colspan="3" class="note">{ format!("{} at 16 bits; {} at 4 bits", bytes(r.bytes[1]), bytes(r.bytes[3])) }</td>
            <td class="note">{"QLoRA keeps the frozen base at 4 bits, plus a scale per block"}</td>
        </tr>
    </table></div> }
}

/// The three worries against each family.
fn worries() -> Html {
    let cell = |w: &text::Worry| html! { <td><ul>
        <li>{ "by definition: " }{ w.gives }</li>
        if !w.measured.is_empty() {
            <li>{ "measured here: " }<a href={format!("../{}/", w.slug)}>{ w.measured }</a></li>
        } else {
            <li>{ "measured here: nothing yet" }</li>
        }
        if !w.to_come.is_empty() {
            <li>{ "to measure: " }{ w.to_come }</li>
        }
    </ul></td> };
    html! { <div class="scroll"><table class="worries">
        <tr><th scope="col">{"family"}</th><th scope="col">{"forgetting"}</th><th scope="col">{"time"}</th><th scope="col">{"cost"}</th></tr>
        { for text::FAMILIES.iter().zip(text::WORRIES.iter()).enumerate().map(|(i, (f, w))| html! {
            <tr><th scope="row"><a href={format!("#family-{}", i + 1)}>{ format!("{}. {}", i + 1, f.name) }</a></th>{ cell(&w.forgetting) }{ cell(&w.time) }{ cell(&w.cost) }</tr>
        }) }
    </table></div> }
}

/// The curriculum.
fn curriculum() -> Html {
    html! { <table class="curriculum">
        { for text::CURRICULUM.iter().map(|l| html! {
            <tr><td class="n">{ l.number }</td><td>{ lesson_link(l.title, l.slug, l.saga) }</td></tr>
        }) }
    </table> }
}

#[function_component(App)]
pub fn app() -> Html {
    // Where the page starts may be named in its address (lesson::link): ?mode=advanced.
    let mode = use_state(|| Mode::from_link().unwrap_or(Mode::Beginner));
    let result = use_memo((), |_| run());
    let linker = Linker::of(PROGRAM);
    let onmode = { let mode = mode.clone(); Callback::from(move |m| mode.set(m)) };
    let body = match result.as_ref() {
        Err(e) => html! { <pre class="err">{ e.clone() }</pre> },
        Ok(r) => html! { <>
            { part(1, "The families", families()) }
            { part(2, "The two questions", html! { <>
                { p(text::GRID_NOTE[0]) }
                { grid() }
                { p(text::GRID_NOTE[1]) }
            </> }) }
            { part(3, "The account", html! { <>
                { p(text::ACCOUNT[0]) }
                { counts(r) }
                { p(text::ACCOUNT[1]) }
                { p(text::ACCOUNT[2]) }
                <div class="account">
                    { for r.said().iter().map(|t| html! { <p>{ t.clone() }</p> }) }
                </div>
            </> }) }
            { part(4, "The three worries", worries()) }
            { part(5, "The curriculum", curriculum()) }
        </> },
    };
    let texts = text::all();
    html! {
        <ContextProvider<Here> context={Here("landscape".into())}>
        <header>
            { header("The landscape", text::LEDE) }
        </header>
        <main>
            { frame(Frame::Question, "The question", html! { <>
                { p(text::QUESTION[0]) }
                { p(text::QUESTION[1]) }
                <p class="hint">{"This is lesson 6, the last of the first group. It assumes "}<a href="../lora-vs-fa/">{"lesson 5"}</a>{" and the lessons before it; "}<a href="../welcome/">{"the welcome page"}</a>{" shows how the lessons work. A word with a dotted underline opens a plain explanation."}</p>
            </> }) }
            { frame(Frame::Background, "What you need from the lessons before", html! { <>{ for text::BACKGROUND.iter().map(|t| p(t)) }</> }) }
            { frame(Frame::Rationale, "Why a map now", p(text::RATIONALE)) }
            { frame(Frame::Purpose, "What this page is, and is not", html! { <>{ p(text::PURPOSE[0]) }{ p(text::PURPOSE[1]) }</> }) }
            <section class="panel">
                <h2>{"How much to show"}</h2>
                <Switch mode={*mode} onchange={onmode} />
                <p class="note">{"Advanced adds the program."}</p>
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
                    { linker.block(PROGRAM) }
                </section>
            }) }
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
