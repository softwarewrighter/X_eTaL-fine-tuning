//! The page's model natively: the page runs the command line's
//! program, its settings change two lines only, and what it reads back
//! is what the arithmetic must give.

use lora_hello_web::micro::{program, run, Settings, PRINTED, PROGRAM};

#[test]
fn at_the_defaults_the_page_runs_the_command_lines_program() {
    assert_eq!(program(Settings::default()), PROGRAM);
}

#[test]
fn the_page_reads_what_the_command_line_prints() {
    let golden = include_str!("../../reg/cli-lora-hello.out");
    assert_eq!(golden.lines().count(), PRINTED);
    let h = run(Settings::default()).unwrap();
    let line = |v: &[f64]| v.iter().map(|x| format!("{x:?}")).collect::<Vec<_>>().join(" ");
    let lines: Vec<&str> = golden.lines().collect();
    assert_eq!(line(&[h.s, h.h]), lines[0]);
    assert_eq!(line(&h.base), lines[1]);
    assert_eq!(line(&h.y), lines[2]);
    assert_eq!(line(&h.correction), lines[3]);
}

#[test]
fn the_adapter_adds_seven_times_b() {
    // A x = 0.5 - 1 + 3 + 1 = 3.5, s = 2: the correction is 7 B.
    let h = run(Settings::default()).unwrap();
    assert_eq!((h.s, h.h), (2.0, 3.5));
    assert_eq!(h.correction, [7.0, 0.0, -7.0, 3.5]);
    assert!(h.adds_up && h.merged_same);
    assert_eq!(h.rank, 1);
    assert_eq!(h.trained, (8, 16));
    // On a 4096 x 4096 layer at rank 16: 16 (4096 + 4096) of 4096 squared.
    assert_eq!(h.big, (131_072, 16_777_216));
    assert_eq!(h.percent, 0.78125);
    for i in 0..4 {
        assert!((h.y[i] - h.base[i] - h.correction[i]).abs() < 1e-12);
    }
}

#[test]
fn the_settings_change_x_and_alpha_only() {
    let s = Settings { x: [0.0, 0.0, 2.0, -1.0], alpha: 0.5 };
    let p = program(s);
    assert!(p.contains("x := 0.0 0.0 2.0 -1.0\n") && p.contains("s := 0.5 lo:s_caling 1\n"), "{p}");
    let changed = p.lines().zip(PROGRAM.lines()).filter(|(a, b)| a != b).count();
    assert_eq!(changed, 2);
    // A x = 2 - 0.25 = 1.75, s = 0.5: the correction is 0.875 B.
    let h = run(s).unwrap();
    assert_eq!((h.s, h.h), (0.5, 1.75));
    assert_eq!(h.correction, [0.875, 0.0, -0.875, 0.4375]);
    assert!(h.adds_up && h.merged_same);
}

#[test]
fn alpha_at_zero_leaves_the_frozen_layer() {
    let h = run(Settings { alpha: 0.0, ..Settings::default() }).unwrap();
    assert_eq!(h.y, h.base);
    assert_eq!(h.rank, 0);
}
