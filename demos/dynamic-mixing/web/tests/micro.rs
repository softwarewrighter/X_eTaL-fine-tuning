//! The lesson's model natively: the page runs the command line's
//! program at the paper's gating factor; at 0 the two inputs make the
//! same tables, the earlier pages' static ones; at small factors H_res
//! is doubly stochastic and at 1 and above twenty rounds do not reach
//! it; the gradient checks at every factor; nothing goes unexplained.

use dynamic_mixing_web::micro::{run, with, ALPHAS, PAPER, PRINTED, PROGRAM, S};

#[test]
fn at_the_papers_start_the_page_runs_the_command_lines_program() {
    assert_eq!(ALPHAS[PAPER], 0.01);
    assert_eq!(with(0.01), PROGRAM);
    assert_eq!(include_str!("../../reg/cli-dynamic-mixing.out").lines().count(), PRINTED);
}

#[test]
fn at_zero_the_tables_are_static() {
    let r = run(0.0).unwrap();
    assert_eq!(r.apart, [0.0, 0.0, 0.0]);
    // The biases alone: sigmoid 0 = 0.5, twice that 1, and the Sinkhorn projection of exp(2I - 1): 0.711 and 0.096.
    assert!(r.a.h_pre.iter().all(|v| (v - 0.5).abs() < 1e-12));
    assert!(r.a.h_post.iter().all(|v| (v - 1.0).abs() < 1e-12));
    assert_eq!(format!("{:.3} {:.3}", r.a.h_res[0], r.a.h_res[1]), "0.711 0.096");
}

#[test]
fn the_gating_factor_lets_the_input_in_and_the_gradient_checks() {
    let mut last = 0.0;
    for a in ALPHAS {
        let r = run(a).unwrap();
        assert_eq!(r.checks[0].ok, true, "the gradient at {a}: {}", r.checks[0].line);
        for i in 0..r.written.len() {
            let (w, c) = (r.written[i], r.central[i]);
            assert!((w - c).abs() <= 1e-8f64.max(1e-3 * c.abs()), "{a} entry {i}: {w} vs {c}");
        }
        // The two inputs' tables move apart as the factor grows.
        assert!(r.apart[0] >= last, "{a}");
        last = r.apart[0];
        // Both inputs' H_res within a millionth of doubly stochastic below 1, not at 1 and above.
        assert_eq!(r.checks[1].ok, a < 1.0, "H_res at {a}: {}", r.checks[1].line);
        assert_eq!(r.a.h_res.len(), S * S);
    }
    let r = run(0.01).unwrap();
    assert!(r.apart.iter().all(|d| *d > 0.001 && *d < 0.05), "{:?}", r.apart);
}

#[test]
fn nothing_on_the_page_goes_unexplained() {
    use dynamic_mixing_web::text;
    use lesson::eli5::missing;
    use lesson::explain::{calls, definition, unexplained};
    assert_eq!(missing(&text::all()), Vec::<String>::new());
    assert_eq!(unexplained(PROGRAM), Vec::<String>::new());
    for (lib, name) in calls(PROGRAM) {
        assert!(definition(&lib, &name).is_some(), "no definition of {lib} {name}");
    }
    for t in text::all() {
        assert_eq!(lesson::narrate::forbidden(t), Vec::<&str>::new(), "{t}");
    }
}
