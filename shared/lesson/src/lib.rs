//! The teaching shell of the X_eTaL fine-tuning pages, on top of
//! X_eTaL-ML's `microscope` (which runs X_eTaL, reads arrays back and
//! draws decorated source).
//!
//! - `chrome`: this repository's header and footer, the frames of a page,
//!   asides
//! - `code`: X_eTaL drawn with every function linked to the cross-reference
//! - `eli5`: every idea explained simply, opened from the word itself
//! - `explain`: a library function explained from its own source
//!
//! Written for a reader with new eyes: one who knows neither machine
//! learning nor X_eTaL (docs/new-eyes.md). The stepper, heat maps,
//! the narrator of changes and the rest of docs/plan.md's F5 come
//! with the step lesson-shell.

pub mod chrome;
pub mod code;
pub mod eli5;
pub mod explain;
