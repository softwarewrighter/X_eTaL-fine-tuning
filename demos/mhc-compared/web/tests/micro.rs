//! The summary's model natively: the page runs the command line's
//! program on the same data; the arithmetic at the start and the
//! records' figures are what the page says of them; the Python's
//! figures are read as quoted; nothing on the page goes unexplained.

use mhc_compared_web::micro::{run, FIELDS, PRINTED, PROGRAM};

#[test]
fn the_page_runs_the_command_lines_program() {
    assert_eq!(include_str!("../../reg/cli-mhc-compared.out").lines().filter(|l| !l.starts_with("drawn ")).count(), PRINTED);
    let r = run().unwrap();
    assert!(r.checks.iter().all(|c| c.ok), "{:?}", r.checks);
    assert_eq!(r.summary, "6 checks, all passed");
    assert_eq!(r.figures.len(), 9 * FIELDS);
}

#[test]
fn the_start_is_the_third_pages_arithmetic() {
    let r = run().unwrap();
    // exp(2I - 1) and its Sinkhorn projection: e, 1/e; 0.711, 0.096.
    assert_eq!(format!("{:.3} {:.3} {:.3} {:.3}", r.hc[0], r.hc[1], r.mhc[0], r.mhc[1]), "2.718 0.368 0.711 0.096");
    // HC's proxy at 12, 24, 48 layers: 6.39, 13.37, 27.35; mHC's forward gain 1 and proxy log10(1/4).
    let p: Vec<String> = (0..3).map(|d| format!("{:.2}", r.at_start(d)[1])).collect();
    assert_eq!(p, ["6.39", "13.37", "27.35"]);
    assert!((0..3).all(|d| r.at_start(d)[2].abs() < 1e-9));
    assert_eq!(format!("{:.3}", r.at_start(2)[3]), "-0.602");
}

#[test]
fn the_records_say_what_the_page_says() {
    let r = run().unwrap();
    // The plain skip at 48 layers did not learn; every other run did.
    assert!(r.fig(0, 2, 0) > 2.7);
    for (s, d) in [(0, 0), (0, 1), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2)] {
        assert!(r.fig(s, d, 0) < 0.01, "{s} {d}");
    }
    // HC's forward gain after training near the start's; mHC's within a hundred-thousandth of 1.
    for d in 0..3 {
        assert!((r.fig(1, d, 4) - r.at_start(d)[0]).abs() < 0.5, "{d}");
    }
    assert!(r.off < 1e-5);
    // No NaN event.
    assert!((0..9).all(|k| r.figures[k * FIELDS + 8] == 0.0));
    // The Python's figures, as quoted.
    assert_eq!(r.py(1, 2), (5.54, 27.3));
    assert_eq!(r.py(2, 2), (0.0002, -0.6));
    // The counts: a block of 280, static mixing 36, dynamic 795.
    assert_eq!(r.per_layer, [280.0, 3.0, 36.0, 795.0, 256.0]);
}

#[test]
fn nothing_on_the_page_goes_unexplained() {
    use lesson::eli5::missing;
    use lesson::explain::{calls, definition, unexplained};
    use mhc_compared_web::text;
    assert_eq!(missing(&text::all()), Vec::<String>::new());
    assert_eq!(unexplained(PROGRAM), Vec::<String>::new());
    for (lib, name) in calls(PROGRAM) {
        assert!(definition(&lib, &name).is_some(), "no definition of {lib} {name}");
    }
    for t in text::all() {
        assert_eq!(lesson::narrate::forbidden(t), Vec::<&str>::new(), "{t}");
    }
    for (py, what, x, _) in text::BESIDE {
        for t in [py, what, x] {
            assert_eq!(lesson::narrate::forbidden(t), Vec::<&str>::new(), "{t}");
        }
    }
    // The lines quoted as this page's are in its program.
    for (_, _, x, wh) in text::BESIDE {
        if wh == "this page" {
            assert!(PROGRAM.contains(x), "not in the program: {x}");
        }
    }
}
