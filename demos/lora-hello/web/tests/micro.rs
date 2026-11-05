//! The page's model natively: the page runs the command line's
//! program, its settings change two lines only, and what it reads back
//! is what the arithmetic must give.

use lesson::code::builtins;
use lesson::eli5::{missing, used};
use lesson::explain::{calls, definition, imports};
use lora_hello_web::micro::{program, run, Settings, PRINTED, PROGRAM};
use lora_hello_web::text;

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
    assert_eq!(line(&h.w), lines[0]);
    assert_eq!(line(&h.a), lines[1]);
    assert_eq!(line(&h.b), lines[2]);
    assert_eq!(line(&[h.s, h.h]), lines[3]);
    assert_eq!(line(&h.products), lines[4]);
    assert_eq!(line(&[h.scaled]), lines[5]);
    assert_eq!(line(&h.w_row), lines[6]);
    assert_eq!(line(&h.base), lines[7]);
    assert_eq!(line(&h.y), lines[8]);
    assert_eq!(line(&h.correction), lines[9]);
    assert_eq!(line(&h.table), lines[11]);
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

#[test]
fn the_squeeze_is_written_out_from_the_runs_numbers() {
    // A x: each number of A times the matching number of x; h is their sum.
    let h = run(Settings::default()).unwrap();
    assert_eq!(h.a, [0.5, -0.5, 1.0, 0.25]);
    assert_eq!(h.b, [1.0, 0.0, -1.0, 0.5]);
    assert_eq!(h.products, [0.5, -1.0, 3.0, 1.0]);
    assert_eq!(h.products.iter().sum::<f64>(), h.h);
    assert_eq!(h.w.len(), 16);
    // The strength, then the spread: 3.5 x 2 = 7, and 7 times each number of B.
    assert_eq!(h.scaled, 7.0);
    for i in 0..4 {
        assert_eq!(h.correction[i], h.scaled * h.b[i]);
    }
    // Row 1 of W times x, written out, adds up to the first output of the frozen layer.
    assert!((h.w_row.iter().sum::<f64>() - h.base[0]).abs() < 1e-12);
    // The correction as a table: every row is A times one number (2, 0, -2, 1).
    for (r, k) in [2.0, 0.0, -2.0, 1.0].iter().enumerate() {
        for c in 0..4 {
            assert_eq!(h.table[r * 4 + c], k * h.a[c]);
        }
    }
}

// New eyes (docs/new-eyes.md): the reader knows neither machine
// learning nor X_eTaL, so nothing the page shows goes unexplained.

#[test]
fn every_word_that_opens_names_an_idea_the_glossary_has() {
    let texts = text::all();
    assert_eq!(missing(&texts), Vec::<String>::new());
    assert!(used(&texts).len() >= 20, "{:?}", used(&texts));
}

#[test]
fn every_library_function_the_program_calls_is_explained_from_its_source() {
    assert_eq!(imports(PROGRAM).iter().map(|i| (i.alias.as_str(), i.library.as_str())).collect::<Vec<_>>(), [("lo:", "LoRA")]);
    let calls = calls(PROGRAM);
    let names: Vec<&str> = calls.iter().map(|(_, n)| n.as_str()).collect();
    assert_eq!(names, ["s_caling", "f_orward", "d_elta", "m_erge", "r_ank", "p_arams"]);
    for (lib, name) in &calls {
        let d = definition(lib, name).unwrap_or_else(|| panic!("no definition of {lib} {name}"));
        assert!(d.source.starts_with(&format!("l:{name} := ")), "{d:?}");
        assert!(!d.doc.is_empty() && !d.doc.contains(">>"), "{d:?}");
        assert!(d.source_url.contains("/X_eTaL-ML/blob/") && d.source_url.ends_with(&format!("#L{}", d.line)), "{d:?}");
        assert!(d.xref.starts_with("../doc/work-xetal-ml-libs-LoRA-src-LoRA.xtl.html#l."), "{d:?}");
    }
    // The two the page has asides for are shown as the library has them.
    assert_eq!(definition("LoRA", "m_erge").unwrap().source, "l:m_erge := { (W, A, B, s) -> W + l:d_elta (A, B, s) }");
    assert!(definition("LoRA", "f_orward").unwrap().source.contains("B '+ '* i_nner A '+ '* i_nner x"));
}

#[test]
fn every_built_in_word_of_the_program_is_explained() {
    let explained: Vec<&str> = text::BUILTINS.iter().map(|(w, _)| *w).collect();
    // m_ax is explained with r_/, the word it is quoted for.
    let unexplained: Vec<String> = builtins(PROGRAM).into_iter().filter(|b| !explained.contains(&b.as_str()) && b != "m_ax").collect();
    assert_eq!(unexplained, Vec::<String>::new());
}
