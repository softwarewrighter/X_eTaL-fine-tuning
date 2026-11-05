//! The libraries a page here imports that microscope's store lacks:
//! this repository's own (libs/<Name>/src) and the general ones named
//! in build.rs (Matrix), embedded at build time and put in
//! microscope's in-memory store, where `u_se<` finds them as the
//! command line does through XETAL_PATH. Ask FM1 (docs/xetal-ML-asks.md):
//! microscope embeds X_eTaL-ML's libraries and the seven of
//! X_eTaL-libraries it links only.

use std::sync::Once;

include!(concat!(env!("OUT_DIR"), "/libs.rs"));

/// The names of the libraries added, as their files: `Matrix.xtl`.
pub fn added() -> Vec<&'static str> {
    LIBRARIES.iter().map(|(file, _)| *file).collect()
}

/// Put the libraries in the store (once; later calls do nothing).
/// `run`'s functions call it; a page that calls microscope's own
/// `run` must call it first.
pub fn install() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        for (file, source) in LIBRARIES {
            microscope::libs::add(file, source);
        }
    });
}
