//! Build provenance for the footer (this repository's commit, the
//! pinned X_eTaL-ML commit and the X_eTaL commit it pins, the build
//! host and time), and where each library on the search path lives (its
//! repository, its path there, the pinned commits), for links to its
//! source. The libraries themselves are embedded by X_eTaL-ML's
//! microscope, which takes X_eTaL-libraries at this repository's pin
//! from MICROSCOPE_LIBS (.cargo/config.toml). Adapted from microscope's
//! build.rs.

use std::path::{Path, PathBuf};
use std::process::Command;

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

/// `$OUT_DIR/origins.rs`: where each library on the search path lives
/// (its file, the repository, its path there), and the full pinned
/// commits, for links to the source.
fn origins(root: &Path) {
    println!("cargo:rerun-if-changed={}", root.join("XETAL_LIBRARIES_COMMIT").display());
    let roots = [("own", root.join("libs"), "libs"), ("ml", root.join("work/xetal-ml/libs"), "libs"), ("libraries", root.join("work/xetal-libraries/libs"), "libs")];
    let mut code = String::from("pub const ORIGINS: &[(&str, &str, &str)] = &[\n");
    for (kind, dir, prefix) in &roots {
        let mut dirs: Vec<PathBuf> = std::fs::read_dir(dir).map(|d| d.filter_map(Result::ok).map(|e| e.path()).filter(|p| p.is_dir()).collect()).unwrap_or_default();
        dirs.sort();
        for d in dirs {
            let name = d.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
            for (file, _) in sources(&d) {
                code += &format!("    ({file:?}, {kind:?}, {:?}),\n", format!("{prefix}/{name}/src/{file}"));
            }
        }
    }
    code += "];\n";
    let full = |file: PathBuf| std::fs::read_to_string(file).unwrap_or_default().trim().to_string();
    code += &format!("pub const ML_COMMIT: &str = {:?};\n", full(root.join("XETAL_ML_COMMIT")));
    code += &format!("pub const LIBRARIES_COMMIT: &str = {:?};\n", full(root.join("XETAL_LIBRARIES_COMMIT")));
    let out = Path::new(&std::env::var("OUT_DIR").unwrap()).join("origins.rs");
    std::fs::write(out, code).unwrap();
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    origins(&root);
    println!("cargo:rustc-env=LESSON_SHA={}", run("git", &["rev-parse", "--short", "HEAD"]));
    println!("cargo:rustc-env=LESSON_HOST={}", run("hostname", &["-s"]));
    println!("cargo:rustc-env=LESSON_TIMESTAMP={}", run("date", &["-u", "+%Y%m%dT%H%M"]));
    println!("cargo:rustc-env=LESSON_ML_SHA={}", pinned(&root.join("XETAL_ML_COMMIT")));
    println!("cargo:rustc-env=LESSON_XETAL_SHA={}", pinned(&root.join("work/xetal-ml/XETAL_COMMIT")));
    println!("cargo:rustc-env=LESSON_LIBRARIES_SHA={}", pinned(&root.join("XETAL_LIBRARIES_COMMIT")));
    // .git/HEAD names the branch and does not change at a commit; its log does.
    println!("cargo:rerun-if-changed={}", root.join(".git/logs/HEAD").display());
}
