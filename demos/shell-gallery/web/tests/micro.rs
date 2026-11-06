//! The gallery's model natively: the page runs the command line's
//! program and reads its pipeline's stages back; what the parts show
//! is what the arithmetic must give; nothing goes unexplained.

use lesson::arithmetic::adds_up;
use lesson::eli5::missing;
use lesson::explain::{calls, definition};
use lesson::narrate::{count, forbidden};
use shell_gallery_web::micro::{pipeline, program, run, PRINTED, PROGRAM};
use shell_gallery_web::text;

#[test]
fn the_page_runs_the_command_lines_program_and_asks_for_its_stages() {
    let p = program();
    assert!(p.starts_with(PROGRAM.trim_end()));
    assert_eq!(p[PROGRAM.trim_end().len()..].lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).count(), 5, "one statement more: the pipeline's record");
    assert!(pipeline().starts_with("y := @ flow:p_ipe< c#\"") && pipeline().ends_with("\"\"#"));
    let golden = include_str!("../../reg/cli-shell-gallery.out");
    assert_eq!(golden.lines().count(), PRINTED);
}

#[test]
fn the_trace_is_the_pipelines_run() {
    let g = run().unwrap();
    let s = &g.trace.stages;
    assert_eq!(s.len(), 4);
    assert_eq!(s[0].items, [1.0, 2.0, 3.0, 4.0]);
    // The squeeze: A x = 0.5 - 1 + 3 + 1.
    assert_eq!(s[1].items, [3.5]);
    // The spread: 3.5 times each number of B.
    assert_eq!(s[2].items, [3.5, 0.0, -3.5, 1.75]);
    for i in 0..4 {
        assert_eq!(s[2].items[i], s[1].items[0] * g.b[i]);
    }
    // The last stage is the program's y.
    assert_eq!(s[3].items, g.y);
    // What each reads: the squeeze reads x; the spread the squeeze; the output x and the spread.
    assert_eq!(s.iter().map(|st| st.reads.clone()).collect::<Vec<_>>(), [vec![], vec![1], vec![2], vec![1, 3]]);
}

#[test]
fn the_sum_written_out_is_x_etals() {
    let g = run().unwrap();
    assert!(adds_up(&g.row1, g.row1_sum, 1e-12));
    // The output's first number is that sum plus the strength (2) times the spread.
    assert!((g.y[0] - (g.row1_sum + 2.0 * g.trace.stages[2].items[0])).abs() < 1e-12);
}

#[test]
fn the_change_in_words_comes_from_the_counts_and_judges_nothing() {
    let g = run().unwrap();
    assert_eq!(g.counts, (8, 16, 16));
    let said = count("the numbers it trains", g.counts.0, g.counts.1, Some((g.counts.2, "the table itself")));
    assert_eq!(said, "the numbers it trains went from 8 to 16, as many as the 16 of the table itself");
    assert_eq!(forbidden(&said), Vec::<&str>::new());
}

#[test]
fn nothing_on_the_page_goes_unexplained() {
    assert_eq!(missing(&text::all()), Vec::<String>::new());
    for (lib, name) in calls(PROGRAM) {
        assert!(definition(&lib, &name).is_some(), "no definition of {lib} {name}");
    }
    assert_eq!(text::STAGES.len(), 4);
}
