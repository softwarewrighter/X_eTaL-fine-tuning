//! The model natively: the page runs the command line's program at the
//! default settings; the projection's identity holds (LoRA-FA's
//! remainder is 0 within tolerance, LoRA's is not); LoRA comes to the
//! lowest loss possible and LoRA-FA to its floor; the spectrum code
//! passes its check; the narrator speaks from the numbers without a
//! verdict; nothing goes unexplained.

use lesson::eli5::missing;
use lesson::explain::{calls, definition, unexplained};
use lesson::narrate::forbidden;
use lora_vs_fa_web::micro::{run, with, Settings, CHUNKS, EXAMPLES, PRINTED, PROGRAM, RANKS};
use lora_vs_fa_web::text;

#[test]
fn at_the_default_settings_the_page_runs_the_command_lines_program() {
    assert_eq!(with(Settings::default()), PROGRAM);
    assert_eq!(include_str!("../../reg/cli-lora-vs-fa.out").lines().filter(|l| !l.starts_with("drawn ")).count(), PRINTED);
    let changed = with(Settings { rank: 2, n: 3 }).lines().zip(PROGRAM.lines()).filter(|(a, b)| a != b).count();
    assert_eq!(changed, 2);
    assert!((1..=RANKS).contains(&Settings::default().rank) && (EXAMPLES.0..=EXAMPLES.1).contains(&Settings::default().n));
}

#[test]
fn the_projection_and_the_floors_hold() {
    let r = run(Settings::default()).unwrap();
    // The spectrum code recovers known singular values.
    assert!(r.check < 1e-6, "check {}", r.check);
    assert!((r.singular[0] - 1.4025087826714349).abs() < 1e-9 && r.singular[4..].iter().all(|v| *v < 1e-6));
    // LoRA-FA's update never left the frozen A's row space; LoRA's did; T is not in it.
    assert!(r.fa.remainder < 1e-9, "LoRA-FA's remainder {}", r.fa.remainder);
    assert!(r.lora.remainder > 0.1 && r.t_remainder > 0.1);
    // At rank 4 LoRA comes to the lowest loss possible (0 on a rank-4 T) and LoRA-FA to its floor.
    assert!(r.lowest_here.abs() < 1e-9 && r.end(false) < 1e-9, "LoRA ended at {}", r.end(false));
    assert!((r.end(true) - r.floor).abs() < 1e-9, "LoRA-FA ended at {} against its floor {}", r.end(true), r.floor);
    assert!((r.floor - 0.5 * r.t_remainder * r.t_remainder).abs() < 1e-9);
    // The counts are analytical, the records have the start and every chunk, the charts drew.
    assert_eq!((r.lora.trained, r.fa.trained, r.lora.held_by_adam, r.fa.held_by_adam), (64, 32, 128, 64));
    for rec in r.records() {
        assert_eq!(rec.rows.len(), CHUNKS + 1);
        assert!(rec.first()[2] == 0.0 && rec.change > 0.0);
    }
    assert!((r.learn_lora.first()[0] - 1.0108989398939692).abs() < 1e-9 && (r.learn_lora.first()[1] - 0.6830932582254989).abs() < 1e-9);
    assert!(r.charts.len() == 4 && r.charts.iter().all(|c| c.starts_with("<svg")));
}

#[test]
fn another_rank_keeps_the_identities() {
    let r = run(Settings { rank: 2, n: 3 }).unwrap();
    assert!(r.fa.remainder < 1e-9 && r.lora.remainder > 0.01);
    // At rank 2 the lowest loss possible is above 0, and LoRA comes near it.
    assert!(r.lowest_here > 0.1 && (r.end(false) - r.lowest_here).abs() < 1e-3, "LoRA {} against {}", r.end(false), r.lowest_here);
    assert_eq!((r.lora.trained, r.fa.trained), (32, 16));
}

#[test]
fn the_narrator_speaks_from_the_numbers_without_a_verdict() {
    let r = run(Settings::default()).unwrap();
    let texts: Vec<String> = r.fit_said().into_iter().chain(r.learn_said()).chain([r.fit_answer(), r.old_answer()]).collect();
    for t in &texts {
        assert_eq!(forbidden(t), Vec::<&str>::new(), "{t}");
        assert!(t.contains("measured") || t.contains("analytical"), "unmarked: {t}");
    }
    assert!(texts[2].contains("its update did not leave the subspace"), "{}", texts[2]);
    assert_eq!(r.fit_same(), false);
    let (lo, hi) = r.old_extremes();
    assert_ne!(lo, hi);
    let old: Vec<f64> = r.records().iter().map(|x| x.last()[2]).collect();
    assert_eq!(old.iter().cloned().fold(f64::INFINITY, f64::min), old[lo]);
    assert!(r.old_answer().contains("The lowest is"));
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
