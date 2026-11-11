//! The model natively: the program's counts against the formulas the
//! families' definitions give; the map's rows match the families; the
//! prose names no family better than another; nothing goes
//! unexplained.

use lesson::eli5::missing;
use lesson::explain::{calls, definition, unexplained};
use lesson::narrate::forbidden;
use landscape_web::micro::{run, share, LAYERS, PRINTED, PROGRAM, ROWS};
use landscape_web::text;

#[test]
fn the_counts_follow_the_definitions() {
    let r = run().unwrap();
    assert_eq!(include_str!("../../reg/cli-landscape.out").lines().count(), PRINTED);
    for (j, (d, k)) in LAYERS.iter().enumerate() {
        let all = d * d;
        let lora = k * (d + d);
        let want: [usize; 9] = [all, all / 8, lora, k * d, lora, lora + d, d + k, all, 0];
        for (i, w) in want.iter().enumerate() {
            assert_eq!(r.trained[i][j], *w, "trained, row {} at {d}", ROWS[i].name);
            // Adam holds three per number trained; a forward-only method and an edit hold none.
            let factor = if i < 7 { 3 } else { 0 };
            assert_eq!(r.held[i][j], factor * w, "held, row {} at {d}", ROWS[i].name);
        }
        let stored: [usize; 9] = [all, all / 8, lora, k * d, lora, lora + d, d + k, all, 2 * d];
        for (i, s) in stored.iter().enumerate() {
            assert_eq!(r.stored[i][j], *s, "stored, row {} at {d}", ROWS[i].name);
        }
        assert_eq!(r.bytes[j], 2.0 * all as f64);
        assert_eq!(r.bytes[2 + j], 0.5 * all as f64);
        assert_eq!(r.shares[j], lora as f64 / all as f64);
        assert_eq!(r.shares[2 + j], (k * d) as f64 / all as f64);
    }
    assert_eq!((r.trained[2][1], r.trained[0][1]), (131_072, 16_777_216));
    assert_eq!(share(r.shares[0]), "as many as the layer itself");
    assert_eq!(share(r.shares[1]), "1 in 128");
}

#[test]
fn the_map_is_consistent() {
    assert_eq!(text::FAMILIES.len(), text::WORRIES.len());
    assert!(ROWS.iter().all(|r| r.family < text::FAMILIES.len()));
    // Every live lesson named on the map is a demo of this repository; every other cell names its saga or is held in reserve.
    let live: Vec<&str> = text::CURRICULUM.iter().filter(|l| !l.slug.is_empty()).map(|l| l.slug).collect();
    for row in text::GRID.iter() {
        for cells in row.iter() {
            for c in cells.iter() {
                if !c.slug.is_empty() {
                    assert!(live.contains(&c.slug), "{} is not live", c.slug);
                    assert!(std::path::Path::new(&format!("../../{}/demo.toml", c.slug)).exists(), "no demo {}", c.slug);
                }
            }
        }
    }
    for w in text::WORRIES.iter() {
        for x in [&w.forgetting, &w.time, &w.cost] {
            assert_eq!(x.measured.is_empty(), x.slug.is_empty(), "{}", x.measured);
            if !x.slug.is_empty() {
                assert!(live.contains(&x.slug));
            }
        }
    }
    assert_eq!(text::CURRICULUM.iter().filter(|l| !l.slug.is_empty()).count(), 6);
}

#[test]
fn the_prose_judges_no_family() {
    let r = run().unwrap();
    for t in r.said() {
        assert_eq!(forbidden(&t), Vec::<&str>::new(), "{t}");
        assert!(t.contains("analytical"), "unmarked: {t}");
    }
    let mut texts = text::all();
    for f in text::FAMILIES.iter() {
        texts.push(f.arithmetic);
    }
    for w in text::WORRIES.iter() {
        for x in [&w.forgetting, &w.time, &w.cost] {
            texts.extend([x.gives, x.measured, x.to_come]);
        }
    }
    for t in texts {
        assert_eq!(forbidden(t), Vec::<&str>::new(), "{t}");
    }
}

#[test]
fn nothing_on_the_page_goes_unexplained() {
    assert_eq!(missing(&text::all()), Vec::<String>::new());
    assert_eq!(unexplained(PROGRAM), Vec::<String>::new());
    for (lib, name) in calls(PROGRAM) {
        assert!(definition(&lib, &name).is_some(), "no definition of {lib} {name}");
    }
}
