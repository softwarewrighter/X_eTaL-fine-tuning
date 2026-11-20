//! The lesson's model natively: the page runs the command line's
//! program at the paper's start; HC's forward gain is its row sum to
//! the power of the depth and its proxy the Python's analytical
//! figures; mHC's gains are 1 and its proxy tends to a quarter; mHC's
//! table does not move with the shift; HC's row sum crosses 1 between
//! two settings of the slider; nothing goes unexplained.

use gain_by_depth_web::micro::{run, with, L, PAPER, PRINTED, PROGRAM, S, SHIFTS};

#[test]
fn at_the_papers_start_the_page_runs_the_command_lines_program() {
    assert_eq!(SHIFTS[PAPER], 0.0);
    assert_eq!(with(0.0), PROGRAM);
    assert_eq!(include_str!("../../reg/cli-gain-by-depth.out").lines().filter(|l| !l.starts_with("drawn ")).count(), PRINTED);
    for c in SHIFTS {
        let changed = with(c).lines().zip(PROGRAM.lines()).filter(|(a, b)| a != b).count();
        assert_eq!(changed, usize::from(c != 0.0), "{c}");
    }
}

#[test]
fn hc_grows_by_its_row_sum_and_mhc_stays_at_one() {
    let r = run(0.0).unwrap();
    let s = std::f64::consts::E + 3.0 / std::f64::consts::E;
    for v in &r.hc.rows {
        assert!((v - s).abs() < 1e-12);
    }
    for k in 1..=L {
        let want = s.powi(k as i32);
        assert!((r.hc.forward[k - 1] / want - 1.0).abs() < 1e-9, "{k}");
        assert!((r.hc.backward[k - 1] / want - 1.0).abs() < 1e-9, "{k}");
        assert!((r.line[k - 1] - k as f64 * s.log10()).abs() < 1e-9);
        assert!((r.mhc.forward[k - 1] - 1.0).abs() < 1e-9);
        assert!((r.mhc.backward[k - 1] - 1.0).abs() < 1e-9);
    }
    // The Python's analytical figures: 6.39, 13.37, 27.35 against -0.602.
    assert_eq!(format!("{:.2}", r.hc.proxy[11]), "6.39");
    assert_eq!(format!("{:.2}", r.hc.proxy[23]), "13.37");
    assert_eq!(format!("{:.2}", r.hc.proxy[47]), "27.35");
    assert!((r.mhc.proxy[L - 1] - 0.25f64.log10()).abs() < 1e-6);
    assert!(r.checks.iter().all(|c| c.ok), "{}", r.summary);
    assert_eq!(r.summary, "4 checks, all passed");
    assert!(r.chart.starts_with("<svg"));
    assert_eq!(r.hc.product(1), &r.hc.table[..]);
    assert_eq!(r.mhc.product(L).len(), S * S);
}

#[test]
fn the_shift_moves_hc_and_not_mhc() {
    let base = run(0.0).unwrap();
    let mut crossed = 0;
    for (i, c) in SHIFTS.iter().enumerate() {
        let r = run(*c).unwrap();
        for k in 0..S * S {
            assert!((r.mhc.table[k] - base.mhc.table[k]).abs() < 1e-12, "{c}");
        }
        assert!((r.sweep[i] - r.hc.rows[0]).abs() < 1e-9 * r.hc.rows[0], "{c}");
        assert!((r.sweep_gain[i] / r.hc.forward[L - 1] - 1.0).abs() < 1e-9, "{c}");
        assert!(r.checks.iter().all(|c| c.ok), "{}", r.summary);
        if i > 0 && (r.sweep[i - 1] < 1.0) != (r.sweep[i] < 1.0) {
            crossed = i;
        }
    }
    // The row sum crosses 1 between -1.5 and -1.25, the settings SHIFT[2] names.
    assert_eq!(crossed, 3);
    assert_eq!(SHIFTS[2], -1.5);
}

#[test]
fn every_setting_replays_the_same() {
    for c in [-2.0, 0.0, 1.0] {
        assert_eq!(run(c).unwrap(), run(c).unwrap());
    }
}

#[test]
fn nothing_on_the_page_goes_unexplained() {
    use gain_by_depth_web::text;
    use lesson::eli5::missing;
    use lesson::explain::{calls, definition, unexplained};
    assert_eq!(missing(&text::all()), Vec::<String>::new());
    assert_eq!(unexplained(PROGRAM), Vec::<String>::new());
    for (lib, name) in calls(PROGRAM) {
        assert!(definition(&lib, &name).is_some(), "no definition of {lib} {name}");
    }
    for t in text::all() {
        assert_eq!(lesson::narrate::forbidden(t), Vec::<&str>::new(), "{t}");
    }
}
