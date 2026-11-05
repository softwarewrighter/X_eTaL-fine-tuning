//! The teaching shell of the X_eTaL fine-tuning pages, on top of
//! X_eTaL-ML's `microscope` (which runs X_eTaL, reads arrays back and
//! draws decorated source).
//!
//! - `libs`: this repository's libraries, and the general ones
//!   microscope does not embed, importable with `u_se<`
//! - `run`: microscope's runs, with those libraries in the store
//! - `chrome`: this repository's footer
//!
//! The glossary, the stepper, heat maps, the narrator of changes and
//! the rest of docs/plan.md's F5 come with the step lesson-shell.

pub mod chrome;
pub mod libs;
pub mod run;
