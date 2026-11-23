//! The experiment's model natively: the page runs the command line's
//! program for mHC; every scheme's short run checks its gradient and
//! begins exactly as its recorded run does (the records are the
//! program's, and a rerun reproduces them); the replay reads the nine
//! records; the measured results the page states hold; nothing goes
//! unexplained.

use training_at_depth_web::micro::{recorded, replay, run, schemes, with, DEPTHS, FIELDS, LIVE_STEPS, PRINTED, PROGRAM, STEPS};

#[test]
fn for_mhc_the_page_runs_the_command_lines_program() {
    let s = schemes();
    assert_eq!(with(&s[2]), PROGRAM);
    assert_eq!(include_str!("../../reg/cli-training-at-depth.out").lines().filter(|l| !l.starts_with("drawn ")).count(), PRINTED);
    for sc in &s[..2] {
        assert!(with(sc).lines().zip(PROGRAM.lines()).filter(|(a, b)| a != b).count() >= 1);
    }
}

#[test]
fn each_short_run_checks_its_gradient_and_begins_as_its_record() {
    for (k, sc) in schemes().iter().enumerate() {
        let r = run(sc).unwrap();
        assert!(r.checks.iter().all(|c| c.ok), "{}: {}", sc.title, r.summary);
        for i in 0..r.written.len() {
            let (w, c) = (r.written[i], r.central[i]);
            assert!((w - c).abs() <= 1e-8f64.max(1e-3 * c.abs()), "{} entry {i}: {w} vs {c}", sc.title);
        }
        assert_eq!(r.nans, 0.0);
        let rec = recorded(k, 12).unwrap();
        for t in 1..=LIVE_STEPS {
            for f in 0..FIELDS {
                assert_eq!(rec.at(t, f), r.record.at(t, f), "{} step {t} field {f}", sc.title);
            }
        }
        assert!(r.record.at(LIVE_STEPS, 0) < r.record.at(1, 0));
    }
}

#[test]
fn the_records_say_what_the_page_says() {
    for (k, _) in schemes().iter().enumerate() {
        for d in DEPTHS {
            let rec = recorded(k, d).unwrap();
            assert_eq!(rec.steps(), STEPS);
            assert!((1..=STEPS).all(|t| rec.at(t, 5) == 0.0), "a NaN step in {k} at {d}");
        }
    }
    let r48 = replay(48).unwrap();
    // The plain skip does not learn at 48 layers: its loss stays at the even guess, ln 16.
    assert!((r48.last(0, 0) - 16f64.ln()).abs() < 0.01, "{}", r48.last(0, 0));
    assert!(r48.last(0, 1) < 1e-3);
    // HC and mHC learn at 48 layers; HC's gain stays above 10^27, mHC's at 1.
    assert!(r48.last(1, 0) < 0.01 && r48.last(2, 0) < 0.01);
    assert!(r48.last(1, 2) > 1e27);
    assert!((r48.last(2, 2) - 1.0).abs() < 1e-4);
    assert_eq!(r48.nans, vec![0.0, 0.0, 0.0]);
    assert_eq!(r48.charts.len(), 4);
    for d in [12, 24] {
        let rp = replay(d).unwrap();
        for s in 0..3 {
            assert!(rp.last(s, 0) < 0.01, "{s} at {d}: {}", rp.last(s, 0));
        }
    }
}

#[test]
fn nothing_on_the_page_goes_unexplained() {
    use lesson::eli5::missing;
    use lesson::explain::{calls, definition, unexplained};
    use training_at_depth_web::micro::REPLAY;
    use training_at_depth_web::text;
    assert_eq!(missing(&text::all()), Vec::<String>::new());
    for prog in [PROGRAM, REPLAY] {
        assert_eq!(unexplained(prog), Vec::<String>::new());
        for (lib, name) in calls(prog) {
            assert!(definition(&lib, &name).is_some(), "no definition of {lib} {name}");
        }
    }
    for t in text::all() {
        assert_eq!(lesson::narrate::forbidden(t), Vec::<&str>::new(), "{t}");
    }
}
