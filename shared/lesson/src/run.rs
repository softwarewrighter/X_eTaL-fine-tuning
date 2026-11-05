//! Running an X_eTaL program as microscope does, with this
//! repository's libraries in the store first.

/// Run `src` and return its output lines, which must number `lines`;
/// an X_eTaL error (or a different count) is the Err.
pub fn output(src: &str, lines: usize) -> Result<Vec<String>, String> {
    crate::libs::install();
    microscope::run::output(src, lines)
}

/// Run `src` and return its output lines, however many; an X_eTaL
/// error is the Err.
pub fn printed(src: &str) -> Result<Vec<String>, String> {
    crate::libs::install();
    microscope::run::printed(src)
}
