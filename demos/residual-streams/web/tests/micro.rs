//! The lesson's model natively: the page runs the command line's
//! program at the first preset; the layer's trace is the pipeline's
//! run and agrees with the matrix products; the skip keeps the gap
//! that the plain stack loses; H_res's rows sum to 3.82 for HC and to
//! 1 for mHC; every setting replays the same; nothing goes unexplained.

use residual_streams_web::micro::{presets, program, run, with, Settings, D, L, PRINTED, PROGRAM, S};

#[test]
fn at_the_first_preset_the_page_runs_the_command_lines_program() {
    let d = Settings::default();
    assert_eq!(with(&d), PROGRAM);
    assert!(program(&d).starts_with(PROGRAM.trim_end()));
    assert_eq!(include_str!("../../reg/cli-residual-streams.out").lines().filter(|l| !l.starts_with("drawn ")).count(), PRINTED);
    for p in presets().iter().skip(1) {
        let changed = with(p).lines().zip(PROGRAM.lines()).filter(|(a, b)| a != b).count();
        assert_eq!(changed, 1, "{}: {changed} lines changed", p.title);
    }
    let hc = Settings { mhc: false, ..d };
    assert_eq!(with(&hc).lines().zip(PROGRAM.lines()).filter(|(a, b)| a != b).count(), 1);
}

#[test]
fn the_skip_keeps_the_gap_the_plain_stack_loses() {
    let r = run(&Settings::default()).unwrap();
    assert_eq!(r.x0.len(), D);
    assert_eq!(r.plain_gap.len(), L);
    assert_eq!(r.skip_gap.len(), L);
    // Two lists a hundredth apart: the second list's first number is the first's plus 0.01, the rest the same.
    assert!((r.x1[0] - r.x0[0] - 0.01).abs() < 1e-12);
    assert_eq!(r.x1[1..], r.x0[1..]);
    // Replaced: the gap is gone (to the last digit) by layer 48; added to: it is within a factor of 2 of what was sent in.
    assert!(r.plain_gap[L - 1] < 1e-10, "{}", r.plain_gap[L - 1]);
    assert!(r.plain_gap[11] < 1e-6, "{}", r.plain_gap[11]);
    assert!(r.skip_gap[L - 1] > 0.5 && r.skip_gap[L - 1] < 2.0, "{}", r.skip_gap[L - 1]);
    assert!(r.skip_gap.iter().all(|g| *g > 0.5));
    assert!(r.chart.starts_with("<svg"));
}

#[test]
fn the_mixing_tables_are_as_the_page_says() {
    let r = run(&Settings::default()).unwrap();
    // The logits: 1 on the diagonal, -1 off it.
    for i in 0..S {
        for j in 0..S {
            assert_eq!(r.logits[i * S + j], if i == j { 1.0 } else { -1.0 });
        }
    }
    // HC's H_res is e on the diagonal and 1/e off it; its rows and columns sum to e + 3/e.
    assert!((r.h_res_hc[0] - std::f64::consts::E).abs() < 1e-12);
    assert!((r.h_res_hc[1] - 1.0 / std::f64::consts::E).abs() < 1e-12);
    let row = std::f64::consts::E + 3.0 / std::f64::consts::E;
    for v in r.rows_hc.iter().chain(r.cols_hc.iter()) {
        assert!((v - row).abs() < 1e-12, "{v}");
    }
    assert_eq!(format!("{:.2}", row), "3.82");
    // mHC's rows and columns sum to 1 (to the last digit after 20 rounds).
    for v in r.rows_mhc.iter().chain(r.cols_mhc.iter()) {
        assert!((v - 1.0).abs() < 1e-12, "{v}");
    }
    // H_pre's rows and H_post sum to 1; the softmax of (1, -1, -1, -1) puts 0.71 on the diagonal.
    for i in 0..S {
        let s: f64 = r.h_pre[i * S..(i + 1) * S].iter().sum();
        assert!((s - 1.0).abs() < 1e-12);
    }
    assert!((r.h_post.iter().sum::<f64>() - 1.0).abs() < 1e-12);
    assert_eq!(format!("{:.2}", r.h_post[0]), "0.71");
    assert_eq!(r.h_post[0], r.h_pre[0]);
    // At the start every stream is the list sent in.
    for s in 0..S {
        assert_eq!(r.x_start[s * D..(s + 1) * D], r.x0[..]);
    }
}

#[test]
fn the_layer_is_the_pipeline_and_agrees_with_the_matrix_products() {
    for mhc in [true, false] {
        let r = run(&Settings { mhc, ..Settings::default() }).unwrap();
        assert!(r.off < 1e-12, "the pipeline's layer is off the matrix products by {}", r.off);
        let labels: Vec<&str> = r.trace.stages.iter().map(|st| st.label.as_str()).collect();
        assert_eq!(labels, ["X", "P", "m", "z", "h", "f", "Fs", "R", "X2"]);
        assert_eq!(r.stage("X"), &r.x_after_1[..]);
        assert_eq!(r.stage("X2"), &r.x_after_2[..]);
        assert_eq!(r.stage("m").len(), D);
        // The worked products are row 1 of each table times column 1 of X, and they sum to the stage's first number.
        for (products, table, stage) in [(&r.products_pre[..], &r.h_pre[..], "P"), (&r.products_res[..], r.h_res(), "R")] {
            for t in 0..S {
                assert!((products[t] - table[t] * r.stage("X")[t * D]).abs() < 1e-12);
            }
            assert!((products.iter().sum::<f64>() - r.stage(stage)[0]).abs() < 1e-12);
        }
        // The totals: X's 32 numbers, and R's, which mHC's columns keep and HC's multiply by their sum.
        assert!((r.total_x - r.stage("X").iter().sum::<f64>()).abs() < 1e-9);
        assert!((r.total_r - r.stage("R").iter().sum::<f64>()).abs() < 1e-9);
        assert_eq!(r.stage("z").len(), 16);
        // The shares: row s of Fs is f times H_post's weight for s.
        let (f, fs) = (r.stage("f"), r.stage("Fs"));
        for s in 0..S {
            for j in 0..D {
                assert!((fs[s * D + j] - f[j] * r.h_post[s]).abs() < 1e-12);
            }
        }
        // The new state is the mixed carried state plus the shares.
        let (x2, rr) = (r.stage("X2"), r.stage("R"));
        for k in 0..S * D {
            assert!((x2[k] - rr[k] - fs[k]).abs() < 1e-12);
        }
        // The carried state mixed by the table chosen: HC's inflates the row sums, mHC's keeps them.
        let x = r.stage("X");
        let (sum_x, sum_r): (f64, f64) = (x.iter().sum(), rr.iter().sum());
        if mhc {
            assert!((sum_x - sum_r).abs() < 1e-9, "{sum_x} vs {sum_r}");
        } else {
            assert!((sum_r / sum_x - r.rows_hc[0]).abs() < 1e-9, "{sum_x} vs {sum_r}");
        }
        assert!(r.tex.contains("\\sum"));
    }
}

#[test]
fn every_setting_replays_the_same() {
    for p in presets() {
        for mhc in [true, false] {
            let s = Settings { mhc, ..p.clone() };
            let (a, b) = (run(&s).unwrap(), run(&s).unwrap());
            assert_eq!(a, b, "{} {mhc}", p.title);
        }
    }
}

#[test]
fn nothing_on_the_page_goes_unexplained() {
    use lesson::eli5::missing;
    use lesson::explain::{calls, definition, unexplained};
    use residual_streams_web::text;
    assert_eq!(missing(&text::all()), Vec::<String>::new());
    assert_eq!(unexplained(PROGRAM), Vec::<String>::new());
    for (lib, name) in calls(PROGRAM) {
        assert!(definition(&lib, &name).is_some(), "no definition of {lib} {name}");
    }
    // What the run says and what the stages say calls no setting better or worse.
    for t in text::all() {
        assert_eq!(lesson::narrate::forbidden(t), Vec::<&str>::new(), "{t}");
    }
}

#[test]
fn the_stages_and_the_reading_quote_numbers_the_page_fills() {
    use residual_streams_web::text;
    let marks = |texts: &[&str]| -> Vec<String> { texts.iter().flat_map(|s| s.split('{').skip(1).map(|p| p.split('}').next().unwrap().to_string())).collect() };
    assert_eq!(marks(&text::STAGES), ["hpost0", "hpost1", "hpre0", "hpre1", "workedP", "hpost0", "hpost1", "workedR", "totalX", "totalR", "rowsum", "rowsum", "off"]);
    assert_eq!(marks(&text::STACK), ["x0", "x1"]);
    assert_eq!(marks(&text::STREAMS), ["hpre0", "hpre1", "e", "inv", "rowsum", "hpre0", "hpre1", "rowsum", "rowsum", "hpost0", "hpost1"]);
    assert_eq!(marks(&text::READING), ["p1", "p12", "p48", "s1", "s12", "s48", "psize48", "ssize48"]);
}
