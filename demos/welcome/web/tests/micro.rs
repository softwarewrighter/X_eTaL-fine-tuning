//! The welcome page's model natively: the page runs the command line's
//! program and reads its stages back; what the tour shows is what the
//! arithmetic must give; nothing goes unexplained.

use lesson::arithmetic::adds_up;
use lesson::eli5::missing;
use lesson::explain::{calls, definition};
use lesson::narrate::{forbidden, moved};
use welcome_web::micro::{pipeline, program, run, PRINTED, PROGRAM};
use welcome_web::text;

#[test]
fn the_page_runs_the_command_lines_program_and_asks_for_its_stages() {
    let p = program();
    assert!(p.starts_with(PROGRAM.trim_end()));
    let extra: Vec<&str> = p[PROGRAM.trim_end().len()..].lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).collect();
    assert!(extra[0].starts_with("@ flow:s_tages< c#\""), "{extra:?}");
    assert!(pipeline().starts_with("y := @ flow:p_ipe< c#\"") && pipeline().ends_with("\"\"#"));
    let golden = include_str!("../../reg/cli-welcome.out");
    assert_eq!(golden.lines().count(), PRINTED);
}

#[test]
fn the_trace_is_the_calculations_run() {
    let g = run().unwrap();
    let s = &g.trace.stages;
    assert_eq!(s.len(), 4);
    assert_eq!(s[0].items, [1.0, 2.0, 3.0, 4.0]);
    // Stage 2: boiled down to one number, 0.5 - 1 + 3 + 1.
    assert_eq!(s[1].items, [3.5]);
    // Stage 3: that number times each number of B.
    assert_eq!(s[2].items, [3.5, 0.0, -3.5, 1.75]);
    for i in 0..4 {
        assert_eq!(s[2].items[i], s[1].items[0] * g.b[i]);
    }
    // The last stage is the program's y.
    assert_eq!(s[3].items, g.y);
    // What each uses: stage 2 uses x; stage 3 stage 2; the output x and stage 3.
    assert_eq!(s.iter().map(|st| st.reads.clone()).collect::<Vec<_>>(), [vec![], vec![1], vec![2], vec![1, 3]]);
}

#[test]
fn the_sum_written_out_is_stage_two() {
    let g = run().unwrap();
    assert_eq!(g.products, [0.5, -1.0, 3.0, 1.0]);
    assert!(adds_up(&g.products, g.sum, 1e-12));
    assert_eq!(g.sum, g.trace.stages[1].items[0]);
}

#[test]
fn the_change_in_words_comes_from_two_runs_and_judges_nothing() {
    let g = run().unwrap();
    // Counting the spread-out list four times instead of twice adds
    // two more of its first number (3.5) to the first output.
    assert!((g.stronger - (g.y[0] + 2.0 * g.trace.stages[2].items[0])).abs() < 1e-12);
    let said = moved("The first number of the output", g.y[0], g.stronger);
    assert_eq!(said, "The first number of the output rose from 13.544698 to 20.544698");
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
