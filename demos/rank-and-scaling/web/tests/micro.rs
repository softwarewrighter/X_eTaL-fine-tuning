//! The sweep's model natively: the page runs the command line's
//! program at the default settings; the scales are the library's
//! rules; the narrator's sentences come from the measured table;
//! nothing goes unexplained.

use lesson::eli5::missing;
use lesson::explain::{calls, definition, unexplained};
use lesson::narrate::forbidden;
use rank_and_scaling_web::micro::{run, with, Settings, ALPHAS, PRINTED, PROGRAM, RANKS, STEPS};
use rank_and_scaling_web::text;

#[test]
fn at_the_default_settings_the_page_runs_the_command_lines_program() {
    assert_eq!(with(Settings::default()), PROGRAM);
    assert_eq!(include_str!("../../reg/cli-rank-and-scaling.out").lines().filter(|l| !l.starts_with("drawn ")).count(), PRINTED);
    let changed = with(Settings { alpha: 16.0, steps: 10 }).lines().zip(PROGRAM.lines()).filter(|(a, b)| a != b).count();
    assert_eq!(changed, 2);
    assert!(ALPHAS.contains(&Settings::default().alpha) && STEPS.contains(&Settings::default().steps));
}

#[test]
fn the_scales_are_the_librarys_rules_and_the_counts_are_analytical() {
    let s = run(Settings::default()).unwrap();
    assert_eq!(s.rows.len(), RANKS);
    for r in &s.rows {
        let k = r.rank as f64;
        assert!((r.by_r.s - 4.0 / k).abs() < 1e-12, "alpha / r at rank {}", r.rank);
        assert!((r.by_sqrt.s - 4.0 / k.sqrt()).abs() < 1e-12, "alpha / sqrt r at rank {}", r.rank);
        assert_eq!(r.trained, r.rank * 16);
    }
    // At rank 1 the two rules are one rule: the same run.
    assert_eq!(s.rows[0].by_r, s.rows[0].by_sqrt);
    // Before training the losses are lesson 3's.
    assert!((s.before.0 - 1.0108989398939692).abs() < 1e-9 && (s.before.1 - 0.6830932582254989).abs() < 1e-9);
    assert!(s.t_size > 0.0 && s.charts.iter().all(|c| c.starts_with("<svg")));
}

#[test]
fn the_narrator_speaks_from_the_table_without_a_verdict() {
    let s = run(Settings::default()).unwrap();
    let (best, every) = s.lowest_held(false);
    let held: Vec<f64> = s.rows.iter().map(|r| r.by_r.held).collect();
    assert_eq!(held.iter().cloned().fold(f64::INFINITY, f64::min), held[best - 1]);
    assert_eq!(every, held.windows(2).all(|w| w[1] < w[0]));
    let account = s.account();
    assert!(account.iter().any(|t| t.contains(&format!("lowest at rank {best}"))), "{account:?}");
    for t in account.iter().chain(s.step_said(4, 5).iter()) {
        assert_eq!(forbidden(t), Vec::<&str>::new(), "{t}");
        assert!(t.contains("measured") || t.contains("analytical"), "unmarked: {t}");
    }
    // What changed from rank 4 to 5 quotes both rows' counts.
    let said = s.step_said(4, 5).join(" ");
    assert!(said.contains("from 64 to 80"), "{said}");
    // The answer to the guess is the column's.
    assert_eq!(s.answer().contains("did not fall"), !every);
    assert!(account.iter().any(|t| t.contains("before any training")), "{account:?}");
}

#[test]
fn nothing_on_the_page_goes_unexplained() {
    assert_eq!(missing(&text::all()), Vec::<String>::new());
    assert_eq!(unexplained(PROGRAM), Vec::<String>::new());
    for (lib, name) in calls(PROGRAM) {
        assert!(definition(&lib, &name).is_some(), "no definition of {lib} {name}");
    }
    for t in text::SUMMARY.iter().chain(text::PURPOSE.iter()) {
        assert_eq!(forbidden(t), Vec::<&str>::new(), "{t}");
    }
}
