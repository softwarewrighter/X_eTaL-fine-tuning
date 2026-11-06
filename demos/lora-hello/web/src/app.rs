//! The page, for a reader with new eyes (one who knows neither machine
//! learning nor X_eTaL, and may not program): the question, the
//! background, the idea, what this page is; the layer's numbers; one
//! calculation worked start to finish, with the settings beside it;
//! the correction as a table, the merge and the cost; what it showed
//! and did not; where next; then every idea explained simply, and,
//! for those who want it, the code: how to read it, the functions it
//! uses, the program. Words open plain explanations; the code behind
//! each result is folded into an aside with the library function it
//! calls explained from the library's own source; every function in
//! the code links to the cross-reference.
//! Presentation only: every number shown is one X_eTaL printed.

use web_sys::HtmlInputElement;
use yew::prelude::*;

use lesson::arithmetic::{commas, list, numeric, show};
use lesson::chrome::{footer, frame, header, Frame};
use lesson::code::Linker;
use lesson::eli5::{index, li, p, prose, Here};
use lesson::explain::{definition, function, imports, table};

use crate::micro::{line, program, run, Hello, Settings, PROGRAM};
use crate::text;

/// A row of numbers under a heading; `against` marks the cells whose
/// value differs from that row's (the frozen output).
fn row(name: &str, v: &[f64], against: Option<&[f64]>, class: &'static str) -> Html {
    let cells = v.iter().enumerate().map(|(i, x)| {
        let changed = against.is_some_and(|a| (a[i] - x).abs() > 1e-12);
        html! { <td class={classes!(changed.then_some("changed"))}>{ show(*x) }</td> }
    });
    html! { <tr class={class}><th scope="row">{name.to_string()}</th>{ for cells }</tr> }
}

/// A table of 4 rows of 4, row by row.
fn square(v: &[f64]) -> Html {
    let rows = v.chunks(4).enumerate().map(|(i, r)| row(&format!("row {}", i + 1), r, None, ""));
    html! { <table class="vectors">{ for rows }</table> }
}

/// A piece of code and what it means, for a reading table.
fn read(linker: &Linker, code: &str, meaning: &str) -> Html {
    html! { <tr><td>{ linker.code(code) }</td><td>{ prose(meaning) }</td></tr> }
}

/// Before the experiment: the question, the background, the idea, and
/// what this page is.
fn before() -> Html {
    html! { <>
        { frame(Frame::Question, "The question", html! { <>
            <p class="hint">{ text::HINT }</p>
            { p(text::QUESTION[0]) }
            <ul>{ li(text::QUESTION[1]) }{ li(text::QUESTION[2]) }{ li(text::QUESTION[3]) }</ul>
            { p(text::QUESTION[4]) }
        </> }) }
        { frame(Frame::Background, "What you need to know first", html! { <>
            { p(text::BACKGROUND[0]) }
            { p(text::BACKGROUND[1]) }
            <ul>{ li(text::BACKGROUND[2]) }{ li(text::BACKGROUND[3]) }</ul>
        </> }) }
        { frame(Frame::Rationale, "The idea: leave the model alone, and train a small correction", html! { <>
            { p(text::RATIONALE[0]) }
            <p class="formula">{"y = W x + s B (A x)"}</p>
            { p(text::RATIONALE[1]) }
            { p(text::RATIONALE[2]) }
            { p(text::RATIONALE[3]) }
            { p(text::RATIONALE[4]) }
        </> }) }
        { frame(Frame::Purpose, "What this page is, and is not", html! { <>
            { p(text::PURPOSE[0]) }
            { p(text::PURPOSE[1]) }
        </> }) }
    </> }
}

/// The whole layer: W's 16 numbers, A's row, B's column.
fn numbers(h: &Hello) -> Html {
    html! {
        <section class="panel">
            <h2>{"The layer's numbers"}</h2>
            { p(text::NUMBERS) }
            <div class="tables">
                <figure><figcaption>{"W: the frozen table (4 rows, 4 columns)"}</figcaption>{ square(&h.w) }</figure>
                <figure><figcaption>{"A: the one row of the adapter"}</figcaption><table class="vectors">{ row("row 1", &h.a, None, "") }</table></figure>
                <figure><figcaption>{"B: the one column of the adapter"}</figcaption><table class="vectors">{ for h.b.iter().enumerate().map(|(i, v)| row(&format!("row {}", i + 1), &[*v], None, "")) }</table></figure>
            </div>
        </section>
    }
}

/// The code behind the calculation, and `lo:f_orward` explained from
/// the library's own source: folded away until opened.
fn forward_aside(linker: &Linker) -> Html {
    let Some(d) = definition("LoRA", "f_orward") else { return html! {} };
    let import = imports(PROGRAM).into_iter().next().map(|i| i.line).unwrap_or_default();
    function("lo:", &d,
        html! { <>
            { p("In the program, the whole calculation above is two lines. The first opens the LoRA [library|library]; the second hands [the layer and its adapter|tuple], and the input x, to the function named forward of that library, and calls what comes back y.") }
            { linker.block(&format!("{import}\n{}", line("y := "))) }
            { p("The function forward works out what the adapted layer gives for an input: what the frozen layer gives, plus the [correction|correction]. It is the formula y = W x + s B (A x) as one line of code.") }
        </> },
        html! { <>
            <p>{"Piece by piece, from the right, as X_eTaL reads it:"}</p>
            <table class="read">
                { read(linker, "{ (W, A, B, s) x -> ... }", "The curly brackets make a [function|function]. Before the arrow is what it is given: [the layer and its adapter as one bundle|tuple] on its left, the input x on its right. After the arrow is what it works out.") }
                { read(linker, "A '+ '* i_nner x", "[A times x|inner]: the squeeze (step 2 above). It is rightmost, so it is worked out first.") }
                { read(linker, "B '+ '* i_nner", "B times that: the spread (step 4).") }
                { read(linker, "s *", "times [the strength|strength] s (step 3; the order of these two does not change the result).") }
                { read(linker, "(W '+ '* i_nner x) +", "what the frozen layer gives, W times x (step 1), added on (step 5). The round brackets make it one piece.") }
            </table>
            { p("Notice what is not there: the table s B A is never built. The input goes through A and then B, which is far less arithmetic than making a full table first. That is how an adapter is used while it is being trained.") }
        </> })
}

/// The code behind the merge, and `lo:m_erge` (with `lo:d_elta`, which
/// it calls) explained from the library's own source: folded away.
fn merge_aside(linker: &Linker) -> Html {
    let (Some(m), Some(d)) = (definition("LoRA", "m_erge"), definition("LoRA", "d_elta")) else { return html! {} };
    let within = Linker::within("LoRA", "");
    function("lo:", &m,
        html! { <>
            { p("In the program, the table of the correction and the merged table are one line each: two functions of the LoRA [library|library], named delta and merge. (The library calls the correction \"the update\"; delta is the usual name for a change.)") }
            { linker.block(&format!("{}\n{}", line("table := "), line("merged := "))) }
            { p("The function merge gives back one ordinary table, W + s B A, that does the work of W and the adapter together: what you make once training is over, so that the adapted model runs as fast as the original. This page calls it to check that the [merged|merge] table really gives the same output.") }
        </> },
        html! { <>
            <p>{"Piece by piece:"}</p>
            <table class="read">
                { read(linker, "{ (W, A, B, s) -> ... }", "A [function|function] given [one bundle|tuple]: the layer and its adapter.") }
                { read(&within, "l:d_elta (A, B, s)", "Another function of the same library (inside a library, a small raised l before a name means \"one of my own\"): the correction written out as one full table, s B A.") }
                { read(linker, "W +", "added to the frozen table, number by number. This makes a new table; W itself is not changed.") }
            </table>
            <p>{ format!("And here is that other function, delta, exactly as the library has it (line {} of its file):", d.line) }</p>
            { within.block(&d.source) }
            <table class="read">
                { read(linker, "B '+ '* i_nner A", "[B times A|inner]: this time the full table is built, 4 rows and 4 columns here.") }
                { read(linker, "s *", "times [the strength|strength] s.") }
            </table>
            <p class="links"><a href={d.source_url.clone()} target="_blank">{"The source of delta on GitHub"}</a>{" \u{00b7} "}<a href={d.xref.clone()}>{"delta in the cross-reference"}</a></p>
        </> })
}

/// One calculation, start to finish, from the run's numbers.
fn calculation(h: &Hello, s: Settings, controls: Html, linker: &Linker) -> Html {
    let moved = h.y.iter().zip(&h.base).filter(|(a, b)| (*a - *b).abs() > 1e-12).count();
    let still: Vec<String> = h.y.iter().zip(&h.base).enumerate().filter(|(_, (a, b))| (*a - *b).abs() <= 1e-12).map(|(i, _)| (i + 1).to_string()).collect();
    let spread: Vec<String> = h.b.iter().map(|b| format!("{} \u{00d7} {}", show(h.scaled), show(*b))).collect();
    html! {
        <section class="panel calc">
            <h2>{"One calculation, start to finish"}</h2>
            { p(text::CALC[0]) }
            { controls }
            { p(text::CALC[1]) }
            { p(text::CALC[2]) }
            { numeric("row 1 of W, times x", &h.w[..4], &s.x, &h.w_row, h.base[0]) }
            { p(text::CALC[3]) }
            <p class="worked">{ format!("W x = {}", list(&h.base)) }</p>
            { p(text::CALC[4]) }
            { numeric("A x", &h.a, &s.x, &h.products, h.h) }
            { p(text::CALC[5]) }
            <p class="worked">{ format!("{} \u{00d7} {} = {}", show(h.h), show(h.s), show(h.scaled)) }</p>
            { p(text::CALC[6]) }
            <p class="worked">{ format!("{} = {}", spread.join(", "), list(&h.correction)) }</p>
            { p(text::CALC[7]) }
            <table class="vectors">
                { row("x: the input", &s.x, None, "") }
                { row("W x: what the frozen layer gives", &h.base, None, "") }
                { row("+ the correction, s B (A x)", &h.correction, None, "") }
                { row("= y: what the adapted layer gives", &h.y, Some(&h.base), "sum") }
            </table>
            <p class="muted">{ format!("{moved} of the 4 outputs changed (outlined).{}{} ", if still.is_empty() { String::new() } else { format!(" Output {} did not: the number of B for it is 0, so the correction adds nothing there.", still.join(" and ")) }, if h.adds_up { " The program checks that the two outputs differ by exactly the correction: they do." } else { "" }) }{ text::CALC[8] }</p>
            <h3>{"Things to try"}</h3>
            <ol class="tries">{ li(text::TRIES[0]) }{ li(text::TRIES[1]) }{ li(text::TRIES[2]) }</ol>
            { forward_aside(linker) }
        </section>
    }
}

/// The correction as a table, the merge, and the cost.
fn merging(h: &Hello, linker: &Linker) -> Html {
    html! {
        <section class="panel">
            <h2>{"The correction as a table, the merge, and the cost"}</h2>
            { p(text::TABLE[0]) }
            <div class="tables">
                <figure><figcaption>{ format!("s B A: the correction as a table. Each row is A ({}) times one number.", list(&h.a)) }</figcaption>{ square(&h.table) }</figure>
            </div>
            { p(text::TABLE[1]) }
            { p(text::TABLE[2]) }
            { merge_aside(linker) }
            <h3>{"What it costs"}</h3>
            { p(text::COST[0]) }
            { p(text::COST[1]) }
            <table class="facts">
                <tr><td>{ prose(text::FACTS[0]) }</td><td>{ h.rank }</td></tr>
                <tr><td>{ prose(text::FACTS[1]) }</td><td>{ if h.merged_same { "yes" } else { "no" } }</td></tr>
                <tr><td>{ prose(text::FACTS[2]) }</td><td>{ format!("{} beside {}", h.trained.0, h.trained.1) }</td></tr>
                <tr><td>{ prose(text::FACTS[3]) }</td><td>{ format!("{} beside {} ({:.2}%)", commas(h.big.0), commas(h.big.1), h.percent) }</td></tr>
            </table>
        </section>
    }
}

/// After the experiment: what it showed against the question, and
/// where to go next.
fn after(result: &Result<Hello, String>) -> Html {
    let counts = match result {
        Ok(h) => format!("on this layer the adapter would train {} numbers beside {}; on a table of 4096 rows and 4096 columns at rank 16, {} beside {}, which is {:.2}%. ", h.trained.0, h.trained.1, commas(h.big.0), commas(h.big.1), h.percent),
        Err(_) => "the counts above. ".to_string(),
    };
    html! { <>
        { frame(Frame::Summary, "What this showed, and what it did not", html! { <>
            <ul>
                { li(text::SUMMARY[0]) }
                <li>{ prose(text::SUMMARY[1]) }{counts}{ prose(text::SUMMARY[2]) }</li>
                { li(text::SUMMARY[3]) }
            </ul>
            { p(text::SUMMARY[4]) }
        </> }) }
        { frame(Frame::Next, "Where next", html! { <>
            { p(text::NEXT[0]) }
            <p>{"See "}<a href="../">{"the catalog"}</a>{" for what is live. For the full mathematics of one adapted layer today, with its training, see "}<a href="https://softwarewrighter.github.io/X_eTaL-ML/lora-explorer/">{"the LoRA explorer"}</a>{" in X_eTaL-ML, whose library does the arithmetic of this page."}</p>
        </> }) }
    </> }
}

/// The reference at the end: every idea, then the code for those who
/// want it (how to read it, the functions it uses, the program).
fn reference(s: Settings, linker: &Linker) -> Html {
    let texts = text::all();
    let signs = text::SIGNS.iter().map(|(code, meaning)| read(linker, code, meaning));
    let words = text::BUILTINS.iter().map(|(word, meaning)| html! { <tr><td>{ linker.code(word) }</td><td>{ *meaning }</td></tr> });
    html! { <>
        <section class="panel">
            <h2>{"Every idea on this page, explained simply"}</h2>
            <p class="note">{"The same explanations the underlined words open, in one place. Open any of them."}</p>
            { index(&texts) }
        </section>
        <section class="panel code" id="code">
            <h2>{"The code, for those who want it"}</h2>
            { p(text::READING[0]) }
            <h3>{"How to read it"}</h3>
            { p(text::READING[1]) }
            <table class="functions"><tr><th>{"What you see"}</th><th>{"What it means"}</th></tr>{ for signs }</table>
            { p(text::READING[2]) }
            <p class="links">
                <a href="https://github.com/softwarewrighter/X_eTaL/blob/main/docs/learn-xetal.md" target="_blank">{"Learn X_eTaL in five minutes"}</a>{" \u{00b7} "}
                <a href="https://softwarewrighter.github.io/X_eTaL/" target="_blank">{"Try X_eTaL in your browser"}</a>{" \u{00b7} "}
                <a href="../doc/demos-lora-hello-lora-hello.xtl.html">{"This program in the cross-reference"}</a>
            </p>
            <h3>{"The library functions the program uses"}</h3>
            <p class="note">{"The program opens one library, LoRA, which is part of X_eTaL-ML, and uses six of its functions. Each is described here in the library's own words (written for programmers), with links to its source and to its entry in the cross-reference. Two of them, forward and merge, are explained in plain words in the asides above."}</p>
            { table(PROGRAM) }
            <h3>{"The words of X_eTaL itself that it uses"}</h3>
            <table class="functions"><tr><th>{"Word"}</th><th>{"What it does"}</th></tr>{ for words }</table>
            <h3>{"The program"}</h3>
            { p(text::READING[3]) }
            { linker.block(&program(s)) }
        </section>
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    let settings = use_state(Settings::default);
    let s = *settings;
    let result = use_memo(s, |s| run(*s));
    let linker = Linker::of(PROGRAM);
    // A number field: an empty or unfinished entry keeps the last value.
    let field = |label: String, value: f64, set: Callback<f64>| {
        let oninput = Callback::from(move |e: InputEvent| {
            if let Ok(v) = e.target_unchecked_into::<HtmlInputElement>().value().parse::<f64>() {
                if v.is_finite() { set.emit(v); }
            }
        });
        html! { <label>{label}<input type="number" step="0.5" value={show(value)} {oninput} /></label> }
    };
    let inputs = (0..4).map(|i| {
        let st = settings.clone();
        field(format!("x{}", i + 1), s.x[i], Callback::from(move |v| { let mut n = *st; n.x[i] = v; st.set(n); }))
    });
    let alpha = { let st = settings.clone(); field("alpha".into(), s.alpha, Callback::from(move |v| st.set(Settings { alpha: v, ..*st }))) };
    let reset = { let st = settings.clone(); Callback::from(move |_| st.set(Settings::default())) };
    let controls = html! {
        <div class="controls">
            { for inputs }
            { alpha }
            <button onclick={reset}>{"Reset"}</button>
        </div>
    };
    let body = match result.as_ref() {
        Err(e) => html! { <pre class="err">{ e.clone() }</pre> },
        Ok(h) => html! { <>{ numbers(h) }{ calculation(h, s, controls, &linker) }{ merging(h, &linker) }</> },
    };
    html! {
        <ContextProvider<Here> context={Here("lora-hello".into())}>
        <header>
            { header("LoRA hello", text::LEDE) }
        </header>
        <main>
            { before() }
            { body }
            { after(result.as_ref()) }
            { reference(s, &linker) }
        </main>
        { footer() }
        </ContextProvider<Here>>
    }
}
