//! The lesson's model natively: the page runs the command line's
//! program at the first preset; one round of the pipeline is the
//! loop's first; the paper's start is exact after one round and the
//! lopsided one within a millionth after eleven; the nearly split one
//! is not within a millionth after twenty, and the checks say so; the
//! eight checks pass where the sums reached 1; every setting replays
//! the same; nothing goes unexplained.

use sinkhorn_web::micro::{check, presets, program, run, with, Run, Settings, CHECKS, PERMS, PRINTED, PROGRAM, S, TMAX};

#[test]
fn at_the_first_preset_the_page_runs_the_command_lines_program() {
    let d = Settings::default();
    assert_eq!(with(&d), PROGRAM);
    assert!(program(&d).starts_with(PROGRAM.trim_end()));
    assert_eq!(include_str!("../../reg/cli-sinkhorn.out").lines().filter(|l| !l.starts_with("drawn ")).count(), PRINTED);
    for p in presets().iter().skip(1) {
        let changed = with(p).lines().zip(PROGRAM.lines()).filter(|(a, b)| a != b).count();
        assert_eq!(changed, 1, "{}: {changed} lines changed", p.title);
    }
}

#[test]
fn the_papers_start_is_exact_after_one_round() {
    let r = run(&Settings::default()).unwrap();
    // The logits 2I - 1; A is e on the diagonal and 1/e off it; every sum is e + 3/e.
    for i in 0..S {
        for j in 0..S {
            assert_eq!(r.logits[i * S + j], if i == j { 1.0 } else { -1.0 });
            let want = if i == j { std::f64::consts::E } else { 1.0 / std::f64::consts::E };
            assert!((r.a[i * S + j] - want).abs() < 1e-12);
        }
    }
    let sum = std::f64::consts::E + 3.0 / std::f64::consts::E;
    for v in r.rows_a.iter().chain(r.cols_a.iter()) {
        assert!((v - sum).abs() < 1e-12);
    }
    // The pipeline's round is the loop's first, and both are H already: one round lands.
    assert_eq!(r.off, 0.0);
    assert_eq!(Run::at(&r.cs, 1, S * S), &r.h[..]);
    assert_eq!(r.rounds, 1);
    assert!(r.row_off.iter().all(|v| *v == 0.0));
    assert_eq!(format!("{:.3}", r.h[0]), "0.711");
    assert_eq!(format!("{:.3}", r.h[1]), "0.096");
    // Its decomposition: the identity with weight 0.711 and three reorderings with 0.096 each.
    assert_eq!(r.tables(), 4);
    assert_eq!(&r.orderings[..S], &[1, 2, 3, 4]);
    assert!((r.weights[0] - r.h[0]).abs() < 1e-12);
    assert!((r.weights_sum - 1.0).abs() < 1e-12);
    assert_eq!(r.rest, 0.0);
    assert!(r.passed());
    assert_eq!(r.summary, "8 checks, all passed");
}

#[test]
fn the_lopsided_start_converges_in_eleven_rounds_and_passes() {
    let r = run(&presets()[1]).unwrap();
    assert_eq!(r.rounds, 11);
    // Each round leaves the rows less off than the round before.
    for t in 1..TMAX {
        assert!(r.row_off[t] < r.row_off[t - 1], "round {}", t + 1);
    }
    assert!(r.row_off[TMAX - 1] < 1e-10);
    assert!(r.off < 1e-15, "{}", r.off);
    // A's largest singular value is larger than 1; H's is 1 to the digits kept.
    assert!(r.sigma_a > 1.0);
    assert!((r.sigma_h - 1.0).abs() < 1e-9, "{}", r.sigma_h);
    assert!(r.size_hv <= r.size_v);
    // Ten permutation tables carry weight, summing to 1 within a millionth.
    assert_eq!(r.tables(), PERMS);
    assert!((r.weights_sum - 1.0).abs() < 1e-6);
    assert!(r.rest < 1e-6);
    assert!(r.passed(), "{}", r.summary);
}

#[test]
fn the_nearly_split_start_is_not_within_a_millionth_after_twenty_rounds() {
    let r = run(&presets()[2]).unwrap();
    assert_eq!(r.rounds, TMAX + 1);
    assert!(r.row_off[TMAX - 1] > 1e-6 && r.row_off[TMAX - 1] < 1e-3, "{}", r.row_off[TMAX - 1]);
    // The columns are 1 after every column step regardless.
    for v in &r.cols_c {
        assert!((v - 1.0).abs() < 1e-12);
    }
    assert!(!r.passed());
    let failed: Vec<&str> = r.checks.iter().filter(|c| !c.ok).map(|c| c.name.as_str()).collect();
    assert_eq!(failed, ["every row of H sums to 1", "every row of H B sums to 1", "the weights of the permutation tables sum to 1", "nothing of H is left after the permutation tables found"]);
    assert_eq!(r.summary, "8 checks, 4 FAILED");
}

#[test]
fn the_records_are_what_the_page_reads() {
    let r = run(&presets()[1]).unwrap();
    assert_eq!(r.checks.len(), CHECKS);
    let labels: Vec<&str> = r.trace.stages.iter().map(|st| st.label.as_str()).collect();
    assert_eq!(labels, ["A", "R", "C"]);
    assert_eq!(r.stage("A"), &r.a[..]);
    assert_eq!(r.stage("R"), Run::at(&r.rs, 1, S * S));
    assert_eq!(r.stage("C"), &r.round[..]);
    // After the row step every row sums to 1; after the column step every column does (to rounding).
    for v in r.rows_r.iter().chain(r.cols_c.iter()) {
        assert!((v - 1.0).abs() < 1e-12);
    }
    // The last table of the record is H.
    assert_eq!(Run::at(&r.cs, TMAX, S * S), &r.h[..]);
    assert!(r.chart.starts_with("<svg"));
    assert!(r.tex.contains("\\sum"));
    assert_eq!(check("ok: a").unwrap().ok, true);
    assert_eq!(check("FAIL: a: expected true, got 0").unwrap().name, "a");
    assert!(check("3 checks").is_err());
}

#[test]
fn every_setting_replays_the_same() {
    for p in presets() {
        let (a, b) = (run(&p).unwrap(), run(&p).unwrap());
        assert_eq!(a, b, "{}", p.title);
    }
}

#[test]
fn nothing_on_the_page_goes_unexplained() {
    use lesson::eli5::missing;
    use lesson::explain::{calls, definition, unexplained};
    use sinkhorn_web::text;
    assert_eq!(missing(&text::all()), Vec::<String>::new());
    assert_eq!(unexplained(PROGRAM), Vec::<String>::new());
    for (lib, name) in calls(PROGRAM) {
        assert!(definition(&lib, &name).is_some(), "no definition of {lib} {name}");
    }
    // What the page says calls no start better or worse.
    for t in text::all() {
        assert_eq!(lesson::narrate::forbidden(t), Vec::<&str>::new(), "{t}");
    }
    // The Python is quoted line for line against the loop.
    assert_eq!(text::PYTHON.len(), text::LOOP.len());
    assert_eq!(text::PYTHON.len(), text::COMPARE.len());
}

#[test]
fn the_texts_quote_numbers_the_page_fills() {
    use sinkhorn_web::text;
    let marks = |texts: &[&str]| -> Vec<String> { texts.iter().flat_map(|s| s.split('{').skip(1).map(|p| p.split('}').next().unwrap().to_string())).collect() };
    assert_eq!(marks(&text::START), ["rowA0", "workedR", "rowsR1", "colsR1", "colsC1", "rowsC1", "off"]);
    assert_eq!(marks(&text::ROUNDS), ["tmax", "rounds", "tmax", "rowOff20", "colOff20", "tmax", "rowOff20"]);
    assert_eq!(marks(&text::ROUND), ["t", "tmax", "colsR", "rowsC", "rowOff", "t", "tmax", "rowsA", "colsR", "rowOff", "t", "tmax", "rowsC", "rowOff"]);
    assert_eq!(marks(&text::CHECKS), ["tmax", "sigmaA", "sigmaH", "sizeV", "sizeHv", "rowsHB", "colsHB", "tables", "weights", "weightsSum", "rest", "summary", "summary", "tmax", "weightsSum", "rest", "sigmaH", "tmax", "rowsHB", "colsHB"]);
}
