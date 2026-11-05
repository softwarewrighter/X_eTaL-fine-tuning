//! The shell natively: a program run through microscope finds the
//! libraries of X_eTaL-ML (LoRA), a general library microscope embeds
//! (Check) and one it does not, added here (Matrix); the footer names
//! this repository and the pins.

use lesson::chrome::{build, pins, FRAMES, REPO};
use lesson::libs::added;
use lesson::run::{output, printed};

#[test]
fn a_program_imports_a_library_of_xetal_ml() {
    let out = output("\"lo:\" u_se< \"LoRA\"\n2 lo:p_arams 8 8\n", 1).unwrap();
    assert_eq!(out, ["32"]);
}

#[test]
fn the_adapted_layer_is_the_frozen_one_plus_the_correction() {
    // demos/lora-hello's layer: A x is 3.5, s is 2, so the adapter adds 7 B.
    let src = "\"lo:\" u_se< \"LoRA\"\n\
        A := 1 4 r_eshape 0.5 -0.5 1.0 0.25\n\
        B := 4 1 r_eshape 1.0 0.0 -1.0 0.5\n\
        (lo:d_elta (A, B, 2.0)) '+ '* i_nner 1.0 2.0 3.0 4.0\n";
    assert_eq!(output(src, 1).unwrap(), ["7.0 0.0 -7.0 3.5"]);
}

#[test]
fn a_general_library_microscope_embeds_is_found() {
    assert_eq!(output("\"k:\" u_se< \"Check\"\n6 k:i_s 2 * 3\n", 1).unwrap(), ["ok"]);
}

#[test]
fn a_general_library_added_here_is_found() {
    // Ask FM1: Matrix is not among the libraries microscope embeds.
    assert!(added().contains(&"Matrix.xtl"), "{:?}", added());
    assert_eq!(output("\"mx:\" u_se< \"Matrix\"\nmx:t_race mx:i_dentity 3\n", 1).unwrap(), ["3"]);
}

#[test]
fn an_error_is_reported_not_swallowed() {
    assert!(printed("1 +\n").is_err());
}

#[test]
fn the_footer_names_this_repository_and_the_pins() {
    assert!(REPO.ends_with("/X_eTaL-fine-tuning"));
    let (ml, xetal) = pins();
    let pin = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../XETAL_ML_COMMIT")).unwrap();
    assert_eq!(ml, &pin.trim()[..7]);
    let x = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../work/xetal-ml/XETAL_COMMIT")).unwrap();
    assert_eq!(xetal, &x.trim()[..7]);
    // Compact, no labels: host, short commit, yyyymmddThhmm.
    let b = build();
    let parts: Vec<&str> = b.split(' ').collect();
    assert_eq!(parts.len(), 3, "{b}");
    assert!(parts[1].len() == 7 && parts[1].chars().all(|c| c.is_ascii_hexdigit()), "{b}");
    let t = parts[2];
    assert!(t.len() == 13 && t.as_bytes()[8] == b'T' && t.chars().filter(|c| c.is_ascii_digit()).count() == 12, "{b}");
}

#[test]
fn the_frames_are_those_the_browser_check_requires() {
    // scripts/browser-check.sh names them in its FRAMES line.
    let script = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../scripts/browser-check.sh")).unwrap();
    let names: Vec<&str> = FRAMES.iter().map(|f| f.name()).collect();
    assert_eq!(names, ["question", "background", "rationale", "purpose", "summary", "next"]);
    assert!(script.contains(&format!("FRAMES = {names:?}")), "browser-check.sh does not require {names:?}");
}

// New eyes (docs/new-eyes.md): ideas explained simply, library
// functions explained from their own source, code linked.

#[test]
fn the_glossary_is_well_formed() {
    let all = lesson::eli5::concepts();
    assert!(all.len() >= 30, "{}", all.len());
    let mut ids: Vec<&str> = all.iter().map(|c| c.id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), all.len(), "an id twice");
    for c in &all {
        assert!(!c.id.is_empty() && !c.term.is_empty(), "{c:?}");
        assert!(c.id.chars().all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-'), "{c:?}");
        // An explanation is a few plain sentences, not a line and not an essay.
        let words = c.simply.split_whitespace().count();
        assert!((15..=75).contains(&words), "{}: {words} words", c.id);
    }
    // Every line of the file is an idea or a comment: three fields.
    for l in include_str!("../glossary.tsv").lines().filter(|l| !l.starts_with('#')) {
        assert_eq!(l.split('\t').count(), 3, "{l}");
    }
}

#[test]
fn marked_prose_is_taken_apart() {
    use lesson::eli5::{missing, pieces, used, Piece};
    let text = "A **bold** start, a [weight|weight] and [the table|table]; [plain] stays.";
    assert_eq!(
        pieces(text),
        [
            Piece::Text("A ".into()),
            Piece::Bold("bold".into()),
            Piece::Text(" start, a ".into()),
            Piece::Term { shown: "weight".into(), id: "weight".into() },
            Piece::Text(" and ".into()),
            Piece::Term { shown: "the table".into(), id: "table".into() },
            Piece::Text("; [plain] stays.".into()),
        ]
    );
    assert_eq!(used(&[text, "[again|weight]"]), ["weight", "table"]);
    assert_eq!(missing(&["[x|no-such-idea] [w|weight]"]), ["no-such-idea"]);
}

#[test]
fn a_library_function_is_explained_from_its_own_source() {
    use lesson::explain::{calls, definition, home, imports, xref_page};
    let program = "\"lo:\" u_se< \"LoRA\"\n# lo:n_ot code, a comment\ny := (W, A, B, s) lo:f_orward x\nlo:r_ank lo:d_elta (A, B, s)\n";
    let ims = imports(program);
    assert_eq!((ims[0].alias.as_str(), ims[0].library.as_str(), ims[0].line.as_str()), ("lo:", "LoRA", "\"lo:\" u_se< \"LoRA\""));
    assert_eq!(calls(program), [("LoRA".to_string(), "f_orward".to_string()), ("LoRA".to_string(), "r_ank".to_string()), ("LoRA".to_string(), "d_elta".to_string())]);
    let d = definition("LoRA", "f_orward").unwrap();
    // The definition is a line of the library's file, at the line said.
    let source = microscope::libs::LIBRARIES.iter().find(|(f, _)| *f == "LoRA.xtl").unwrap().1;
    assert_eq!(source.lines().nth(d.line - 1).unwrap(), d.source);
    assert!(d.doc.starts_with("`(W, A, B, s) f_orward x`: the adapted layer"), "{}", d.doc);
    assert!(d.source_url.starts_with("https://github.com/softwarewrighter/X_eTaL-ML/blob/") && d.source_url.ends_with(&format!("/libs/LoRA/src/LoRA.xtl#L{}", d.line)), "{}", d.source_url);
    assert_eq!(d.xref, "../doc/work-xetal-ml-libs-LoRA-src-LoRA.xtl.html#l.f_orward");
    // A function of several lines is taken whole.
    assert!(definition("LoRA", "r_ank").unwrap().source.lines().count() > 1);
    assert_eq!(definition("LoRA", "n_o_such"), None);
    assert_eq!(home("LoRA"), "X_eTaL-ML");
    // A general library, added here, is found too, in its own repository.
    assert_eq!(home("Matrix"), "X_eTaL-libraries");
    assert_eq!(xref_page("Matrix").unwrap(), "../doc/work-xetal-ml-work-xetal-libraries-libs-Matrix-src-Matrix.xtl.html");
    assert!(definition("Matrix", "t_race").unwrap().source_url.contains("/X_eTaL-libraries/blob/"));
}

#[test]
fn functions_in_code_link_to_the_cross_reference() {
    use lesson::code::{anchor, builtins, Linker};
    use xetal_play::Class;
    // As `xetal doc` names its anchors.
    assert_eq!(anchor("r_eshape"), "r_eshape");
    assert_eq!(anchor("r_/"), "r_-2f");
    assert_eq!(anchor("+"), "-2b");
    assert_eq!(anchor("s:i_f<"), "s.i_f-3c");
    let l = Linker::of("\"lo:\" u_se< \"LoRA\"\n");
    assert_eq!(l.href(Class::Builtin, "i_nner").unwrap(), "../doc/builtins.html#i_nner");
    assert_eq!(l.href(Class::Symbol, "*").unwrap(), "../doc/builtins.html#-2a");
    assert_eq!(l.href(Class::LibFunc, "lo:m_erge").unwrap(), "../doc/work-xetal-ml-libs-LoRA-src-LoRA.xtl.html#l.m_erge");
    // Not functions: no link. A library the program did not import: none.
    assert_eq!(l.href(Class::Symbol, "("), None);
    assert_eq!(l.href(Class::Builtin, "'"), None);
    assert_eq!(l.href(Class::LibFunc, "zz:f_oo"), None);
    // Inside a library, l: is the library itself.
    assert_eq!(Linker::within("LoRA", "").href(Class::LibFunc, "l:d_elta").unwrap(), "../doc/work-xetal-ml-libs-LoRA-src-LoRA.xtl.html#l.d_elta");
    assert_eq!(builtins("h := A '+ '* i_nner x\n4 4 r_eshape 'm_ax r_/ h\n"), ["i_nner", "r_eshape", "m_ax", "r_/"]);
}
