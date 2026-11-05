//! Build provenance for the footer (this repository's commit, the
//! pinned X_eTaL-ML commit and the X_eTaL commit it pins, the build
//! host and time), and the libraries the pages' store lacks: this
//! repository's own (libs/<Name>/src) and the general ones named in
//! GENERAL, which X_eTaL-ML's microscope does not embed (ask FM1 in
//! docs/xetal-ML-asks.md). Adapted from microscope's build.rs.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Libraries of X_eTaL-libraries the pages here import that X_eTaL-ML
/// does not link (it embeds Check, Plot, Strings, Format, Lists,
/// Equations and Parsing).
const GENERAL: &[&str] = &["Matrix"];

fn run(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".into())
}

/// The first seven characters of the commit a pin file holds.
fn pinned(file: &Path) -> String {
    println!("cargo:rerun-if-changed={}", file.display());
    match std::fs::read_to_string(file).unwrap_or_default().trim() {
        "" => "unknown".into(),
        sha => sha.chars().take(7).collect(),
    }
}

/// A library directory's sources: Name.xtl (functions), Name.xtlm (macros).
fn sources(dir: &Path) -> Vec<(String, PathBuf)> {
    let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
    ["xtl", "xtlm"]
        .into_iter()
        .filter_map(|ext| {
            let src = dir.join("src").join(format!("{name}.{ext}"));
            src.is_file().then(|| (format!("{name}.{ext}"), src))
        })
        .collect()
}

/// `$OUT_DIR/libs.rs`: each library to add as (file, source).
fn libraries(root: &Path) {
    let own = root.join("libs");
    println!("cargo:rerun-if-changed={}", own.display());
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(&own)
        .map(|d| d.filter_map(Result::ok).map(|e| e.path()).filter(|p| p.is_dir()).collect())
        .unwrap_or_default();
    let general = root.join("work/xetal-ml/work/xetal-libraries/libs");
    dirs.extend(GENERAL.iter().map(|name| general.join(name)));
    let mut found: Vec<(String, PathBuf)> = dirs.iter().flat_map(|d| sources(d)).collect();
    found.sort();
    for name in GENERAL {
        assert!(found.iter().any(|(f, _)| f.starts_with(&format!("{name}."))), "no library {name} at the pin (just xetal)");
    }
    let mut code = String::from("pub const LIBRARIES: &[(&str, &str)] = &[\n");
    for (file, src) in &found {
        println!("cargo:rerun-if-changed={}", src.display());
        code += &format!("    ({file:?}, include_str!({:?})),\n", src.canonicalize().unwrap());
    }
    code += "];\n";
    let out = Path::new(&std::env::var("OUT_DIR").unwrap()).join("libs.rs");
    std::fs::write(out, code).unwrap();
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    libraries(&root);
    println!("cargo:rustc-env=LESSON_SHA={}", run("git", &["rev-parse", "--short", "HEAD"]));
    println!("cargo:rustc-env=LESSON_HOST={}", run("hostname", &["-s"]));
    println!("cargo:rustc-env=LESSON_TIMESTAMP={}", run("date", &["-u", "+%Y%m%dT%H%M"]));
    println!("cargo:rustc-env=LESSON_ML_SHA={}", pinned(&root.join("XETAL_ML_COMMIT")));
    println!("cargo:rustc-env=LESSON_XETAL_SHA={}", pinned(&root.join("work/xetal-ml/XETAL_COMMIT")));
    // .git/HEAD names the branch and does not change at a commit; its log does.
    println!("cargo:rerun-if-changed={}", root.join(".git/logs/HEAD").display());
}
