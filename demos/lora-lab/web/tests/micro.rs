//! The lab's model natively: the page runs the command line's program;
//! the trace is the pipeline's run; every sum the page can write out
//! adds up to the number it is shown for; the rank changes what it
//! must; nothing goes unexplained.

use lesson::arithmetic::adds_up;
use lesson::eli5::missing;
use lesson::explain::{calls, definition, unexplained};
use lora_lab_web::micro::{formulas, program, run, with, Settings, N, PRINTED, PROGRAM};
use lora_lab_web::text;

#[test]
fn at_the_default_rank_the_page_runs_the_command_lines_program() {
    assert_eq!(with(Settings::default()), PROGRAM);
    assert!(program(Settings::default()).starts_with(PROGRAM.trim_end()));
    assert_eq!(include_str!("../../reg/cli-lora-lab.out").lines().count(), PRINTED);
    // The rank is one line of the program.
    let changed = with(Settings { r: 5 }).lines().zip(PROGRAM.lines()).filter(|(a, b)| a != b).count();
    assert_eq!(changed, 1);
}

#[test]
fn the_formulas_are_the_pipelines_own_equations_marked() {
    let f = formulas();
    assert_eq!(f.len(), 3);
    assert_eq!(f[0], "\"h[k] | r\" e:t_exSymbols< \"sum j:=0..7 | A[k,j] * x[j]\"");
    let lab = run(Settings::default()).unwrap();
    for (tex, symbols) in lab.tex.iter().zip([vec!["h", "A", "x"], vec!["u", "B", "h"], vec!["y", "W", "x", "s", "u"]]) {
        for s in symbols {
            assert!(tex.contains(&format!("\\htmlClass{{sym sym-{s}}}")), "{s} not marked in {tex}");
        }
    }
    // Every symbol a formula marks is one the page explains.
    for tex in &lab.tex {
        for part in tex.split("sym sym-").skip(1) {
            let name = part.split('}').next().unwrap();
            assert!(text::SYMBOLS.iter().any(|(n, _, _)| *n == name), "symbol {name} not explained");
        }
    }
}

#[test]
fn the_trace_is_the_layers_run_and_the_library_agrees() {
    let lab = run(Settings::default()).unwrap();
    let s = &lab.trace.stages;
    assert_eq!(s.iter().map(|st| st.items.len()).collect::<Vec<_>>(), [N, 2, N, N]);
    assert_eq!(s[3].items, lab.y);
    assert!(lab.same, "the pipeline's output is not lo:f_orward's");
    assert_eq!(s.iter().map(|st| st.reads.clone()).collect::<Vec<_>>(), [vec![], vec![1], vec![2], vec![1, 3]]);
    // The output is what the frozen table gives plus the correction.
    for i in 0..N {
        assert!((lab.y[i] - (lab.base[i] + lab.correction[i])).abs() < 1e-12);
    }
}

#[test]
fn every_sum_the_page_can_write_out_adds_up() {
    for r in [1, 2, 5, 8] {
        let lab = run(Settings { r }).unwrap();
        let sizes = [N, r, N, N];
        assert!(lab.sum(0, 0).is_none(), "the input is given, not worked out");
        for stage in 1..4 {
            for k in 0..sizes[stage] {
                let (a, b, products, total) = lab.sum(stage, k).unwrap();
                assert_eq!((a.len(), b.len()), (products.len(), products.len()));
                for (p, (a, b)) in products.iter().zip(a.iter().zip(&b)) {
                    assert!((p - a * b).abs() < 1e-12, "rank {r} stage {stage} cell {k}");
                }
                assert!(adds_up(&products, total, 1e-12), "rank {r} stage {stage} cell {k}");
            }
        }
    }
}

#[test]
fn the_rank_changes_the_squeeze_and_the_count_and_nothing_of_w() {
    let (one, four, eight) = (run(Settings { r: 1 }).unwrap(), run(Settings { r: 4 }).unwrap(), run(Settings { r: 8 }).unwrap());
    assert_eq!((one.trace.stages[1].items.len(), four.trace.stages[1].items.len()), (1, 4));
    // r (8 + 8): 16 at rank 1; at rank 4 as many as the table's 64; at 8 twice as many.
    assert_eq!((one.trained, four.trained, eight.trained), ((16, 64), (64, 64), (128, 64)));
    assert_eq!(one.w, eight.w);
    assert_eq!(one.base, eight.base);
}

#[test]
fn nothing_on_the_page_goes_unexplained() {
    assert_eq!(missing(&text::all()), Vec::<String>::new());
    for (lib, name) in calls(PROGRAM) {
        assert!(definition(&lib, &name).is_some(), "no definition of {lib} {name}");
    }
    assert_eq!(unexplained(PROGRAM), Vec::<String>::new());
    assert_eq!(text::STAGES.len(), 4);
}
