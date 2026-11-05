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

/// The X_eTaL-libraries commit this repository pins.
pub fn libraries_pin() -> &'static str {
    env!("LESSON_LIBRARIES_SHA")
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
            <span>{ format!("X_eTaL-libraries {}", libraries_pin()) }</span>{ sep() }
            <span>{ format!("X_eTaL {xetal}") }</span>{ sep() }
            <span class="build" data-sha={sha()}>{ build() }</span>
        </footer>
    }
}

/// The parts that frame every page (the user's review of the first
/// one, 2026-11-05: a page may not jump into its subject): the
/// question in a newcomer's words, the background needed, the
/// rationale (why this approach), the purpose (what this page is and
/// is not), and after the experiment a summary and what is next.
/// scripts/browser-check.sh fails a page that lacks one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Frame {
    Question,
    Background,
    Rationale,
    Purpose,
    Summary,
    Next,
}

/// Every frame, in the order a page shows them.
pub const FRAMES: [Frame; 6] = [Frame::Question, Frame::Background, Frame::Rationale, Frame::Purpose, Frame::Summary, Frame::Next];

impl Frame {
    /// Its name in the page (`data-frame`), which the browser check reads.
    pub fn name(self) -> &'static str {
        match self {
            Frame::Question => "question",
            Frame::Background => "background",
            Frame::Rationale => "rationale",
            Frame::Purpose => "purpose",
            Frame::Summary => "summary",
            Frame::Next => "next",
        }
    }
}

/// A framing section: a titled panel of prose, marked with its kind.
pub fn frame(kind: Frame, title: &str, body: Html) -> Html {
    html! {
        <section class="panel frame" data-frame={kind.name()}>
            <h2>{title.to_string()}</h2>
            {body}
        </section>
    }
}

/// An aside: something a reader may want explained and another may
/// skip, hidden until its summary is opened (keyboard and touch too).
pub fn aside(id: &str, summary: &str, body: Html) -> Html {
    html! {
        <details class="aside" data-aside={id.to_string()}>
            <summary>{summary.to_string()}</summary>
            <div class="aside-body">{body}</div>
        </details>
    }
}

/// A panel as microscope's (`microscope::chrome::panel`), its heading's
/// code given already drawn, so it can be linked (`code::Linker`).
pub fn panel(title: &str, code: Html, note: &str, body: Html) -> Html {
    html! {
        <section class="panel">
            <h2>{title.to_string()}{" "}{code}</h2>
            <p class="note">{note.to_string()}</p>
            {body}
        </section>
    }
}
