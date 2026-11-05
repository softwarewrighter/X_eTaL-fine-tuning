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
