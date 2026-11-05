//! The libraries a page here runs with beyond X_eTaL-ML's own: this
//! repository's (libs/<Name>/src) and every library of
//! X_eTaL-libraries at this repository's pin (XETAL_LIBRARIES_COMMIT),
//! embedded at build time and put in microscope's in-memory store,
//! where `u_se<` finds them as the command line does through
//! XETAL_PATH. They replace the seven microscope embeds at X_eTaL-ML's
//! libraries pin and add the rest (ask FM1, docs/xetal-ML-asks.md), so
//! the browser and the command line run the same libraries.

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
