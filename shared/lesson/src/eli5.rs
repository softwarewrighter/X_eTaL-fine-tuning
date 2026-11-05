//! Every idea explained simply, where the reader asks for it (the
//! user, 2026-11-05: "links to hidden sections which use ELI5 to
//! explain each concept"; the pages are for beginners who know neither
//! machine learning nor X_eTaL).
//!
//! `glossary.tsv` holds the ideas: an id, the term, a plain
//! explanation. A page writes its prose with marks: `[shown|id]` makes
//! `shown` a link that opens the explanation of `id` under it, and
//! `**words**` is bold. `prose` draws it; `index` lists every idea a
//! page links, each a section closed until opened; `missing` finds
//! marks that name no idea (a page's test).

use yew::prelude::*;

/// One idea: its id, the term as a heading, and the explanation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Concept {
    pub id: &'static str,
    pub term: &'static str,
    pub simply: &'static str,
}

const GLOSSARY: &str = include_str!("../glossary.tsv");

/// Every idea of the glossary, in its order.
pub fn concepts() -> Vec<Concept> {
    GLOSSARY
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .filter_map(|l| {
            let mut f = l.split('\t');
            Some(Concept { id: f.next()?, term: f.next()?, simply: f.next()? })
        })
        .collect()
}

/// The idea with this id.
pub fn concept(id: &str) -> Option<Concept> {
    concepts().into_iter().find(|c| c.id == id)
}

/// A piece of marked prose.
#[derive(Clone, Debug, PartialEq)]
pub enum Piece {
    Text(String),
    Bold(String),
    Term { shown: String, id: String },
}

/// Marked prose taken apart: `[shown|id]` a term, `**words**` bold,
/// the rest text. A `[` without `|id]` after it is ordinary text.
pub fn pieces(text: &str) -> Vec<Piece> {
    let mut out = Vec::new();
    let mut plain = String::new();
    let mut rest = text;
    let flush = |plain: &mut String, out: &mut Vec<Piece>| {
        if !plain.is_empty() {
            out.push(Piece::Text(std::mem::take(plain)));
        }
    };
    while !rest.is_empty() {
        if let Some(after) = rest.strip_prefix("**") {
            if let Some(end) = after.find("**") {
                flush(&mut plain, &mut out);
                out.push(Piece::Bold(after[..end].to_string()));
                rest = &after[end + 2..];
                continue;
            }
        }
        if let Some(after) = rest.strip_prefix('[') {
            if let Some(end) = after.find(']') {
                if let Some((shown, id)) = after[..end].split_once('|') {
                    flush(&mut plain, &mut out);
                    out.push(Piece::Term { shown: shown.to_string(), id: id.to_string() });
                    rest = &after[end + 1..];
                    continue;
                }
            }
        }
        let c = rest.chars().next().unwrap();
        plain.push(c);
        rest = &rest[c.len_utf8()..];
    }
    flush(&mut plain, &mut out);
    out
}

/// The ids the texts link, each once, in the order first met.
pub fn used(texts: &[&str]) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for t in texts {
        for p in pieces(t) {
            if let Piece::Term { id, .. } = p {
                if !ids.contains(&id) {
                    ids.push(id);
                }
            }
        }
    }
    ids
}

/// The ids the texts link that the glossary does not have.
pub fn missing(texts: &[&str]) -> Vec<String> {
    used(texts).into_iter().filter(|id| concept(id).is_none()).collect()
}

#[derive(Properties, PartialEq)]
pub struct TermProps {
    pub id: AttrValue,
    pub shown: AttrValue,
}

/// A word that opens its explanation: a button (so it works by
/// keyboard and by touch), and under it, once asked for, the idea
/// explained simply. Asked again, it closes.
#[function_component(Term)]
pub fn term(props: &TermProps) -> Html {
    let open = use_state(|| false);
    let Some(c) = concept(&props.id) else {
        return html! { {props.shown.clone()} };
    };
    let onclick = { let open = open.clone(); Callback::from(move |_| open.set(!*open)) };
    html! { <>
        <button type="button" class={classes!("term", open.then_some("open"))} aria-expanded={open.to_string()} title="Explain this simply" {onclick}>{props.shown.clone()}</button>
        if *open {
            <span class="eli5" role="note"><b>{c.term}{": "}</b>{c.simply}</span>
        }
    </> }
}

/// Marked prose drawn: text, bold, and terms that open.
pub fn prose(text: &str) -> Html {
    pieces(text)
        .into_iter()
        .map(|p| match p {
            Piece::Text(t) => html! { {t} },
            Piece::Bold(t) => html! { <b>{t}</b> },
            Piece::Term { shown, id } => html! { <Term id={id} shown={shown} /> },
        })
        .collect()
}

/// A paragraph of marked prose.
pub fn p(text: &str) -> Html {
    html! { <p>{ prose(text) }</p> }
}

/// A list item of marked prose.
pub fn li(text: &str) -> Html {
    html! { <li>{ prose(text) }</li> }
}

/// Every idea the texts link, each a section closed until opened: the
/// place to look a word up, and the whole of what the page explains.
pub fn index(texts: &[&str]) -> Html {
    let items = used(texts).into_iter().filter_map(|id| concept(&id)).map(|c| {
        html! { <details class="idea" id={format!("idea-{}", c.id)}><summary>{c.term}</summary><p>{c.simply}</p></details> }
    });
    html! { <div class="ideas">{ for items }</div> }
}
