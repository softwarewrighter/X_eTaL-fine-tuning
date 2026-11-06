//! A computation stepped through, forward and back, from a recorded
//! trace (docs/plan.md, F6: a trace, not a re-implementation). The
//! trace is what X_eTaL-ML's Flow reports for a pipeline (`@
//! flow:s_tages<`): every stage's value and shape, from one run.
//! Stepping only moves a position in that record, so going back, or
//! playing it again, runs nothing and changes no value.

use gloo_timers::callback::Timeout;
use microscope::stages::{inputs, stage_ranges, values};
use yew::prelude::*;

/// One stage of a pipeline, as its run recorded it.
#[derive(Clone, Debug, PartialEq)]
pub struct Stage {
    /// Its number, from 1 (the data first).
    pub number: usize,
    /// The name the pipeline gives its value (`h`), or "" for the last.
    pub label: String,
    pub shape: Vec<usize>,
    pub items: Vec<f64>,
    /// The earlier stages it reads, by number.
    pub reads: Vec<usize>,
    /// Its text in the pipeline, as written.
    pub text: String,
}

/// A pipeline's stages in order, from one run.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Trace {
    pub stages: Vec<Stage>,
}

/// The trace of a pipeline: `line` is what `@ flow:s_tages<` printed
/// for it, `statement` the pipeline as the program writes it (from
/// which each stage's text, and so what it reads, is taken).
pub fn trace(line: &str, statement: &str) -> Result<Trace, String> {
    let vals = values(line)?;
    let ranges = stage_ranges(statement);
    if ranges.len() != vals.len() {
        return Err(format!("the pipeline has {} stages and its run recorded {}", ranges.len(), vals.len()));
    }
    let labels: Vec<(usize, String)> = vals.iter().filter(|v| !v.label.is_empty()).map(|v| (v.stage, v.label.clone())).collect();
    let stages = vals
        .into_iter()
        .zip(ranges)
        .map(|(v, (from, to))| {
            let text = statement[from..to].to_string();
            Stage { number: v.stage, reads: inputs(&text, v.stage, &labels), label: v.label, shape: v.shape, items: v.items, text }
        })
        .collect();
    Ok(Trace { stages })
}

/// The step after `step` of `len` (from 0), staying on the last.
pub fn next(step: usize, len: usize) -> usize {
    (step + 1).min(len.saturating_sub(1))
}

/// The step before `step`, staying on the first.
pub fn previous(step: usize) -> usize {
    step.saturating_sub(1)
}

#[derive(Properties, PartialEq)]
pub struct StepperProps {
    /// How many steps the trace has.
    pub len: usize,
    /// The step shown, from 0.
    pub step: usize,
    /// Asked to show another step.
    pub onstep: Callback<usize>,
    /// How long a step is held while playing, in milliseconds.
    #[prop_or(1600)]
    pub hold: u32,
}

/// The controls: to the start, back, play or pause, forward, and where
/// in the trace the page is. Buttons, so they work by keyboard and
/// touch. Playing moves forward one step at a time and stops at the end.
#[function_component(Stepper)]
pub fn stepper(props: &StepperProps) -> Html {
    let playing = use_state(|| false);
    let (len, step) = (props.len, props.step);
    {
        let (playing, onstep, hold) = (playing.clone(), props.onstep.clone(), props.hold);
        use_effect_with((*playing, step, len), move |&(on, step, len)| {
            let timer = (on && step + 1 < len).then(|| Timeout::new(hold, move || onstep.emit(step + 1)));
            if on && step + 1 >= len {
                playing.set(false);
            }
            move || drop(timer)
        });
    }
    let go = |to: usize| { let onstep = props.onstep.clone(); Callback::from(move |_| onstep.emit(to)) };
    let toggle = {
        let (playing, onstep) = (playing.clone(), props.onstep.clone());
        Callback::from(move |_| {
            // Play from the end starts over.
            if !*playing && step + 1 >= len {
                onstep.emit(0);
            }
            playing.set(!*playing);
        })
    };
    html! {
        <div class="stepper" role="group" aria-label="Step through the computation">
            <button type="button" onclick={go(0)} disabled={step == 0}>{"Start"}</button>
            <button type="button" onclick={go(previous(step))} disabled={step == 0}>{"Back"}</button>
            <button type="button" class="play" onclick={toggle} aria-pressed={playing.to_string()}>{ if *playing { "Pause" } else { "Play" } }</button>
            <button type="button" onclick={go(next(step, len))} disabled={step + 1 >= len}>{"Next"}</button>
            <span class="where" role="status">{ format!("Step {} of {}", step + 1, len) }</span>
        </div>
    }
}
