//! The page, framed for someone who has never trained a model: the
//! question (how do I teach my local model something new without it
//! forgetting, and without the cost?), the background, the idea, what
//! this page is; then the experiment (the input x and alpha in number
//! fields; the frozen layer's output, the correction the adapter adds
//! and the adapted output, row over row; the update's rank, the merge
//! and the numbers trained, here and on a real layer); then what it
//! showed against the question, where to go next, and the program.

use web_sys::HtmlInputElement;
use yew::prelude::*;

use lesson::chrome::{footer, frame, Frame};
use microscope::chrome::{header, panel};
use microscope::source::{block, NONE};

use crate::micro::{program, run, Hello, Settings};

/// A number as the page shows it: six decimals at most, no trailing zeros.
fn show(v: f64) -> String {
    let s = format!("{v:.6}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s == "-0" { "0".into() } else { s.to_string() }
}

/// A count with commas between its thousands.
fn commas(n: usize) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// A row of four numbers under a heading; `against` marks the cells
/// whose value differs from that row's (the frozen output).
fn row(name: &str, v: &[f64], against: Option<&[f64]>, class: &'static str) -> Html {
    let cells = v.iter().enumerate().map(|(i, x)| {
        let changed = against.is_some_and(|a| (a[i] - x).abs() > 1e-12);
        html! { <td class={classes!(changed.then_some("changed"))}>{ show(*x) }</td> }
    });
    html! { <tr class={class}><th scope="row">{name.to_string()}</th>{ for cells }</tr> }
}

/// A term and what it means, for the list of words.
fn term(word: &str, meaning: &str) -> Html {
    html! { <><dt>{word.to_string()}</dt><dd>{meaning.to_string()}</dd></> }
}

/// Before the experiment: the question, the background, the idea, and
/// what this page is.
fn before() -> Html {
    html! { <>
        { frame(Frame::Question, "The question", html! { <>
            <p>{"You have a language model running on your own machine, and you want to teach it something new: the conventions of your project, a subject it is weak in, a way of answering. Three worries come with that."}</p>
            <ul>
                <li><b>{"Will it forget? "}</b>{"Teaching a model one thing can damage what it already knew."}</li>
                <li><b>{"How long will it take? "}</b>{"Training a whole model is days of computing on hardware most people do not have."}</li>
                <li><b>{"What will it cost? "}</b>{"That computing is memory and electricity."}</li>
            </ul>
            <p>{"This page shows the idea most often used to answer all three, called LoRA, on one small piece of a model that you can read in full."}</p>
        </> }) }
        { frame(Frame::Background, "What you need to know first", html! { <>
            <p>{"A language model is, underneath, a very large collection of numbers called weights: several billion in a model that fits on a laptop. They are kept in tables (matrices). The model works by multiplying its input, itself a list of numbers, by one table after another; each such step is a layer. Everything the model knows is in those numbers."}</p>
            <p>{"Teaching it something new is called fine-tuning: the numbers are changed, a little at a time, so that the model's answers move toward the ones you want. The direct way changes all of them, and that is where the three worries come from."}</p>
            <ul>
                <li><b>{"Time, memory, electricity. "}</b>{"A number being trained needs more than itself: the direction to move it in (its gradient) and, with the usual method, two running averages. That is about four numbers held and updated for every one trained, billions of them, at every step of training."}</li>
                <li><b>{"Forgetting. "}</b>{"The new numbers replace the old ones, and what the model knew lived in the old ones. Training hard on one narrow thing can spoil abilities you never meant to touch. This is called catastrophic forgetting."}</li>
            </ul>
        </> }) }
        { frame(Frame::Rationale, "The idea: leave the model alone, and train a small correction", html! { <>
            <p>{"LoRA (low-rank adaptation) does not change the model's weights at all. Beside a table of weights, W, it puts a small correction made from two thin tables, A and B, and only those are trained. The layer then gives its old output plus the correction:"}</p>
            <p class="formula">{"y = W x + s B (A x)"}</p>
            <p>{"Read it from the right. A squeezes the input x down to a few numbers. B spreads those few back out to the size of the output. s is a strength setting. The result is added to W x, what the layer gave before."}</p>
            <p>{"Thin is the point. A correction that has to pass through so narrow a middle cannot say everything, but it needs very few numbers. LoRA's bet is that what a model must change to learn one new thing is small enough to fit."}</p>
            <dl class="terms">
                { term("weight", "one of the model's numbers") }
                { term("layer", "one step of the model: multiply the input by a table of weights") }
                { term("frozen", "not changed by training; W is frozen") }
                { term("adapter", "the small trained part added beside the frozen weights: A and B") }
                { term("rank (r)", "how many numbers the adapter squeezes the input into; the width of its middle") }
                { term("alpha", "the strength setting; the scale is s = alpha / r") }
            </dl>
        </> }) }
        { frame(Frame::Purpose, "What this page is, and is not", html! { <>
            <p>{"It is one layer, shrunk from thousands of rows to 4 x 4 so that every number is on the screen. W is the frozen layer. The adapter has rank 1: A is a single row of 4 numbers and B a single column of 4."}</p>
            <p>{"Nothing is trained here. The adapter's numbers are made up. The aim is to see what an adapter is, where it sits, what it adds to a layer's output and what it costs, before the lessons on how its numbers are learned. Every number below is computed by a short X_eTaL program running in your browser, shown at the end."}</p>
        </> }) }
    </> }
}

/// What the run gave: what the adapter changes, then the update, the
/// merge and the cost.
fn results(h: &Hello, s: Settings) -> Html {
    let moved = h.y.iter().zip(&h.base).filter(|(a, b)| (*a - *b).abs() > 1e-12).count();
    html! { <>
        { panel("What the adapter changes", "y := (W, A, B, s) lo:f_orward x", "The input goes two ways. Through the frozen W, as before the adapter existed. And through the adapter: A squeezes the four numbers of x into one, B spreads that one back over the four outputs, and s scales it. The two are added.", false, html! { <>
            <p>{ format!("A x = {}: the input squeezed to one number. Scaled by s = {} (alpha / r) and spread by B, it is what the adapter adds: {}.", show(h.h), show(h.s), h.correction.iter().map(|v| show(*v)).collect::<Vec<_>>().join(", ")) }</p>
            <table class="vectors">
                { row("x: the input", &s.x, None, "") }
                { row("W x: the frozen layer's output", &h.base, None, "") }
                { row("+ s B (A x): what the adapter adds", &h.correction, None, "") }
                { row("= y: the adapted layer's output", &h.y, Some(&h.base), "sum") }
            </table>
            <p class="muted">{ format!("{moved} of the 4 outputs changed (outlined){}. W itself did not change: no number of it is trained.", if h.adds_up { "; the two outputs differ by exactly the correction, checked by X_eTaL" } else { "" }) }</p>
        </> }) }
        { panel("The update, the merge and the cost", "lo:m_erge (W, A, B, s)", "Written out as one table, the adapter's correction is s B A, 4 x 4, but every row of it is a multiple of A: that is what rank 1 means. Once training is over it can be folded into W, giving one ordinary table that computes the same layer, so an adapted model runs as fast as the original.", false, html! { <>
            <table class="facts">
                <tr><td>{"the update's rank (lo:r_ank, by Gram-Schmidt)"}</td><td>{ h.rank }</td></tr>
                <tr><td>{"the merged table W + s B A gives the same output"}</td><td>{ if h.merged_same { "yes (within 1e-12)" } else { "no" } }</td></tr>
                <tr><td>{"numbers the adapter trains, of this layer's (lo:p_arams)"}</td><td>{ format!("{} of {}", h.trained.0, h.trained.1) }</td></tr>
                <tr><td>{"the same on a 4096 x 4096 layer, at rank 16"}</td><td>{ format!("{} of {} ({:.2}%)", commas(h.big.0), commas(h.big.1), h.percent) }</td></tr>
            </table>
            <p class="muted">{"Half of this tiny layer is no saving. The saving comes with size: 4096 x 4096 is one table of a 7-billion-parameter model that runs on a laptop, and 16 is an ordinary rank. The adapter's count grows with the table's sides, the table's own with their product. These are counts, worked out by X_eTaL from the sizes, not measurements of time or power."}</p>
        </> }) }
    </> }
}

/// After the experiment: what it showed against the question, and
/// where to go next.
fn after(result: &Result<Hello, String>) -> Html {
    let counts = match result {
        Ok(h) => format!("On this layer the adapter would train {} numbers of {}; on a 4096 x 4096 layer at rank 16, {} of {}, {:.2}%.", h.trained.0, h.trained.1, commas(h.big.0), commas(h.big.1), h.percent),
        Err(_) => "The counts are above, once the program has run.".to_string(),
    };
    html! { <>
        { frame(Frame::Summary, "What this showed, against the question", html! { <>
            <ul>
                <li><b>{"Will it forget? "}</b>{"W never changed, whatever you typed. The original layer is whole: with alpha at 0, or the adapter taken away, it answers exactly as before. Nothing is overwritten, so nothing is lost for good, and one model can carry a different adapter for each thing you teach it. What this does not promise: while an adapter is on, the output changes for every input its A responds to, wanted or not. Studies find that LoRA forgets less than full fine-tuning, not nothing (Biderman and others, 2024, \"LoRA Learns Less and Forgets Less\"). Measuring that is a later lesson."}</li>
                <li><b>{"How long, and at what cost? "}</b>{counts}{" Fewer numbers trained means less memory for gradients and running averages, less to compute at each step, and a small file to keep for each thing taught. It is cheaper, not free: the frozen weights still take part in every step."}</li>
                <li><b>{"What is the price? "}</b>{"A correction of rank r can only say so much. How much rank a task needs, and what too little looks like, is what the next lessons measure rather than assert."}</li>
            </ul>
            <p>{"In one sentence: instead of rewriting a model to teach it, you freeze it and train a thin correction beside it, which keeps the original intact and makes the training small."}</p>
        </> }) }
        { frame(Frame::Next, "Where next", html! { <>
            <p>{"This page showed what an adapter is. It did not show how A and B get their numbers, how to choose the rank, or how to tell whether the model learned and what it lost. Those are the lessons being built here: the forward pass one step at a time, a training step narrated, rank and strength measured, and LoRA beside its variants ("}<a href="../">{"the catalog"}</a>{" lists what is live)."}</p>
            <p>{"For the full mathematics of one adapted layer today, with its gradients and training curves, see "}<a href="https://softwarewrighter.github.io/X_eTaL-ML/lora-explorer/">{"the LoRA explorer"}</a>{" in X_eTaL-ML, whose library computes this page."}</p>
        </> }) }
    </> }
}

#[function_component(App)]
pub fn app() -> Html {
    let settings = use_state(Settings::default);
    let s = *settings;
    let result = use_memo(s, |s| run(*s));
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
        field(format!("x{i}"), s.x[i], Callback::from(move |v| { let mut n = *st; n.x[i] = v; st.set(n); }))
    });
    let alpha = { let st = settings.clone(); field("alpha".into(), s.alpha, Callback::from(move |v| st.set(Settings { alpha: v, ..*st }))) };
    let reset = { let st = settings.clone(); Callback::from(move |_| st.set(Settings::default())) };
    let body = match result.as_ref() {
        Err(e) => html! { <pre class="err">{ e.clone() }</pre> },
        Ok(h) => results(h, s),
    };
    html! {
        <>
        <header>
            { header("LoRA hello", "How do you teach a model something new without it forgetting what it knows, and without retraining all of it? The idea, on one layer small enough to read: freeze the weights and train a small correction beside them. Computed by X_eTaL in your browser.") }
        </header>
        <main>
            { before() }
            <section class="panel try">
                <h2>{"Try it"}</h2>
                <p class="note">{"x0 to x3 are the four numbers of the input. alpha is the strength of the adapter's correction (s = alpha / r, and the rank r is 1 here)."}</p>
                <div class="controls">
                    { for inputs }
                    { alpha }
                    <button onclick={reset}>{"Reset"}</button>
                </div>
                <ol class="tries">
                    <li>{"Before you touch anything, predict: with alpha at 0, what will the adapted output y be? Set alpha to 0 and look."}</li>
                    <li>{"Change one number of the input. Which rows change? Does anything ever change W?"}</li>
                    <li>{"Double alpha. What happens to what the adapter adds?"}</li>
                </ol>
            </section>
            { body }
            { after(result.as_ref()) }
            <section class="panel code">
                <h2>{"The program"}</h2>
                <p class="note">{"Everything the page runs, in your browser, at each setting: lora-hello.xtl, with x and alpha as you set them. Each line without a name on its left prints one of the values above."}</p>
                { block(&program(s), NONE) }
            </section>
        </main>
        { footer() }
        </>
    }
}
