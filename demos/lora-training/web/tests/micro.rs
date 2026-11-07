//! The training's model natively: the page runs the command line's
//! program at the first preset; the step's trace is the pipeline's run
//! and agrees with the library; each preset replays to the same record;
//! W never moves; nothing goes unexplained.

use lora_training_web::micro::{presets, program, run, with, Settings, HELD, N, PRINTED, PROGRAM};

#[test]
fn at_the_first_preset_the_page_runs_the_command_lines_program() {
    let d = Settings::default();
    assert_eq!(with(&d), PROGRAM);
    assert!(program(&d).starts_with(PROGRAM.trim_end()));
    assert_eq!(include_str!("../../reg/cli-lora-training.out").lines().filter(|l| !l.starts_with("drawn ")).count(), PRINTED);
    for p in presets().iter().skip(1) {
        let changed = with(p).lines().zip(PROGRAM.lines()).filter(|(a, b)| a != b).count();
        assert!(changed >= 1 && changed <= 6, "{}: {changed} lines changed", p.title);
    }
}

#[test]
fn the_step_is_the_librarys_and_w_does_not_move() {
    let t = run(&Settings::default()).unwrap();
    let s = &t.settings;
    assert!(t.off < 1e-12, "the pipeline's step is off the library's by {}", t.off);
    assert_eq!(t.w_moved, 0.0);
    assert!(t.after < t.before, "the step did not lower the loss: {} -> {}", t.before, t.after);
    let labels: Vec<&str> = t.trace.stages.iter().map(|st| st.label.as_str()).collect();
    assert_eq!(labels, ["x", "h", "u", "p", "g", "b", "dA", "dB", "A2", "B2", "W2", "A3", "B3", "s2"]);
    assert_eq!(t.stage("p").len(), N * s.n);
    assert_eq!(t.stage("dA").len(), s.r * N);
    // At the start B is 0, so the error carried back through it, and A's gradient, are 0: only B moves on the first step.
    assert!(t.stage("b").iter().all(|v| *v == 0.0));
    assert!(t.stage("dA").iter().all(|v| *v == 0.0));
    assert_eq!(t.stage("A2"), t.a_at(0));
    assert!(t.stage("dB").iter().any(|v| *v != 0.0));
    // The error is the prediction less the output wanted, and the loss per example is half its squares' sum over n.
    let g = t.stage("g");
    let half: f64 = 0.5 * g.iter().map(|v| v * v).sum::<f64>() / s.n as f64;
    assert!((half - t.before).abs() < 1e-9, "{half} vs {}", t.before);
    assert!(t.tex.contains("\\\\") || t.tex.contains("\\sum"));
    assert!(t.chart.starts_with("<svg"));
}

#[test]
fn every_preset_replays_to_the_same_record() {
    for p in presets() {
        let (a, b) = (run(&p).unwrap(), run(&p).unwrap());
        assert_eq!(a, b, "{}", p.title);
        assert_eq!(a.records(), p.chunks + 1);
        assert_eq!(a.loss_t.len(), a.records());
        assert_eq!(a.loss_h.len(), a.records());
        assert_eq!(a.rec_a.len(), a.records() * p.r * N);
        assert_eq!(a.rec_d.len(), a.records() * N * N);
        // The start is recorded first: B at 0 and so the correction.
        assert!(a.b_at(0).iter().all(|v| *v == 0.0));
        assert!(a.d_at(0).iter().all(|v| *v == 0.0));
        assert_eq!(a.loss_t[0], a.before);
        assert_eq!(p.n + HELD <= 12, true);
    }
}

#[test]
fn nothing_on_the_page_goes_unexplained() {
    use lesson::eli5::missing;
    use lesson::explain::{calls, definition, unexplained};
    use lora_training_web::text;
    assert_eq!(missing(&text::all()), Vec::<String>::new());
    assert_eq!(unexplained(PROGRAM), Vec::<String>::new());
    // Every library function the program calls has a definition the page can show.
    for (lib, name) in calls(PROGRAM) {
        assert!(definition(&lib, &name).is_some(), "no definition of {lib} {name}");
    }
    // What the record says calls no setting better or worse.
    for t in text::READING.iter().chain(text::STAGES.iter()) {
        assert_eq!(lesson::narrate::forbidden(t), Vec::<&str>::new(), "{t}");
    }
}

#[test]
fn the_six_stages_quote_numbers_the_page_fills() {
    use lora_training_web::text;
    let marks: Vec<&str> = text::STAGES.iter().flat_map(|s| s.split('{').skip(1).map(|p| p.split('}').next().unwrap())).collect();
    assert_eq!(marks, ["before", "lr", "after", "before", "wmoved", "off"]);
}
