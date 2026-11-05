//! Around every page: this repository's footer. microscope's own
//! footer names X_eTaL-ML and its commit (ask FM2 in
//! docs/xetal-ML-asks.md), so the pages here use this one; the header,
//! panels and notices are microscope's.

use yew::prelude::*;

pub const REPO: &str = "https://github.com/softwarewrighter/X_eTaL-fine-tuning";
pub const ML: &str = "https://github.com/softwarewrighter/X_eTaL-ML";

/// The build's provenance, compact and without labels, as the catalog's
/// footer shows it: the build host, this repository's short commit and
/// the time, yyyymmddThhmm (UTC, no zone shown).
pub fn build() -> String {
    format!("{} {} {}", env!("LESSON_HOST"), sha(), env!("LESSON_TIMESTAMP"))
}

/// This repository's short commit at the build.
pub fn sha() -> &'static str {
    env!("LESSON_SHA")
}

/// The pins: the X_eTaL-ML commit this repository pins, and the X_eTaL
/// commit that one pins.
pub fn pins() -> (&'static str, &'static str) {
    (env!("LESSON_ML_SHA"), env!("LESSON_XETAL_SHA"))
}

fn sep() -> Html {
    html! { <span class="sep">{ "\u{00b7}" }</span> }
}

/// The footer, as the sibling repositories' live pages have it:
/// copyright, license, the repository on GitHub, the way back to the
/// catalog, the cross-reference (pages/doc), the pins and the build
/// (host, short commit, time). A link to the literate documents goes
/// here when they exist.
pub fn footer() -> Html {
    let (ml, xetal) = pins();
    html! {
        <footer>
            <span>{ "Copyright (c) 2026 Michael A Wright" }</span>{ sep() }
            <span>{ "MIT License" }</span>{ sep() }
            <a href={REPO} target="_blank">{ "GitHub" }</a>{ sep() }
            <a href="../">{ "All lessons" }</a>{ sep() }
            <a href="../doc/">{ "Cross-reference" }</a>{ sep() }
            <span>{ "X_eTaL-ML " }<a href={format!("{ML}/commit/{ml}")} target="_blank">{ ml }</a></span>{ sep() }
            <span>{ format!("X_eTaL {xetal}") }</span>{ sep() }
            <span class="build" data-sha={sha()}>{ build() }</span>
        </footer>
    }
}
