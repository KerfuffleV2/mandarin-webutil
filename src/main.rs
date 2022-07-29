#![allow(non_snake_case)]
use std::cmp::Ordering;

use dioxus::{fermi::*, prelude::*};

use chinese_dictionary as cd;
use once_cell::sync::Lazy;
use regex::Regex;

static VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    #[cfg(all(feature = "web", feature = "desktop"))]
    compile_error!("Only one target feature may be enabled.");
    #[cfg(feature = "web")]
    dioxus::web::launch(App);
    #[cfg(feature = "desktop")]
    dioxus::desktop::launch(App);
}

#[derive(Debug, PartialEq, Clone)]
struct Config {
    hints: bool,
    tone_color: bool,
    hsk: bool,
    simplified: bool,
    wordspace: bool,
    tooltips: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hints: true,
            tone_color: true,
            simplified: true,
            hsk: true,
            wordspace: true,
            tooltips: true,
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
struct InputState(String);

static INPUT: Atom<String> = |_| String::default();
static WORDS: Atom<Vec<Segment>> = |_| Vec::default();
static CONFIG: Atom<Config> = |_| Config::default();

fn App(cx: Scope) -> Element {
    let version = VERSION.unwrap_or("UNKNOWN");

    cx.render(rsx! {
        style { [include_str!("../assets/styles.css")] }
        Settings { }
        p { }
        h3 { "Enter Simplified Chinese text:" }
        TextInput { }
        PrettyChinese { }
        p {
            small {
                "Mandarin Webutil v{version} | "
                a { href: "https://github.com/kerfufflev2/mandarin-webutil/", "GitHub Repo" }
            }
        }
    })
}

fn Settings(cx: Scope) -> Element {
    let cfg = use_atom_state(&cx, CONFIG);

    cx.render(rsx! {
        div {
            class: "settings",
            h3 { "Settings:" }
            input {
                id: "cfgSimplified",
                r#type: "checkbox",
                checked: "{cfg.simplified}",
                onchange: move |_| cfg.modify(|currcfg| Config { simplified: !cfg.simplified, ..currcfg.clone() }),
            }
            label {
                r#for: "cfgSimplified",
                "Simplified"
            }
            input {
                id: "cfgHints",
                r#type: "checkbox",
                checked: "{cfg.hints}",
                onchange: move |_| cfg.modify(|currcfg| Config { hints: !cfg.hints, ..currcfg.clone() }),
            }
            label {
                r#for: "cfgHints",
                "Hints"
            }
            input {
                id: "cfgTonecolor",
                r#type: "checkbox",
                checked: "{cfg.tone_color}",
                onchange: move |_| cfg.modify(|currcfg| Config { tone_color: !cfg.tone_color, ..currcfg.clone() }),
            }
            label {
                r#for: "cfgTonecolor",
                "Tone color"
            }
            input {
                id: "cfgHsk",
                r#type: "checkbox",
                checked: "{cfg.hsk}",
                onchange: move |_| cfg.modify(|currcfg| Config { hsk: !cfg.hsk, ..currcfg.clone() }),
            }
            label {
                r#for: "cfgHsk",
                "HSK"
            }
            input {
                id: "cfgWordspace",
                r#type: "checkbox",
                checked: "{cfg.wordspace}",
                onchange: move |_| cfg.modify(|currcfg| Config { wordspace: !cfg.wordspace, ..currcfg.clone() }),
            }
            label {
                r#for: "cfgWordspace",
                "Word spacing"
            }
            input {
                id: "cfgTooltips",
                r#type: "checkbox",
                checked: "{cfg.tooltips}",
                onchange: move |_| cfg.modify(|currcfg| Config { tooltips: !cfg.tooltips, ..currcfg.clone() }),
            }
            label {
                r#for: "cfgTooltips",
                "Tooltips"
            }
        }
    })
}

fn TextInput(cx: Scope) -> Element {
    let input = use_atom_state(&cx, INPUT);
    let words = use_atom_state(&cx, WORDS);
    cx.render(rsx! {
        textarea {
            cols: "100", rows: "15",
            oninput: move |evt| {
                words.set(make_words(evt.value.as_str()));
                input.set(evt.value.clone());
            },
            "{input}"
        }
    })
}

fn PrettyChinese(cx: Scope) -> Element {
    let words = use_read(&cx, WORDS).as_slice();

    cx.render(rsx! {
        h3 { "Output:" }
        div {
            words.iter().map(|word| {
                rsx! { Chinese { word: word } }
            })
        }
    })
}

#[inline_props]
#[allow(unused_variables)]
fn WordSpan<'a>(
    cx: Scope<'a>,
    defs: &'a [&'static cd::WordEntry],
    children: Element<'a>,
) -> Element<'a> {
    let cfg = use_read(&cx, CONFIG);
    let word = defs[0];
    let hsk = if cfg.hsk { word.hsk } else { 99 };
    let wordspacing = if cfg.wordspace { "" } else { "unspaced" };

    if !cfg.tooltips {
        return cx.render(rsx! {
            span {
                class: "word{wordspacing} hsk{hsk}",
                &cx.props.children
            }
        });
    }

    let tooltip = defs
        .iter()
        .enumerate()
        .map(|(idx, thisreading)| {
            let defs = thisreading
                .english
                .iter()
                .enumerate()
                .map(|(idx, d)| format!("  {}. {d}\n", idx + 1))
                .collect::<String>();
            format!(
                "({}) {} {} [trad. {}]{}:\n{defs}\n",
                idx + 1,
                thisreading.simplified,
                thisreading.traditional,
                thisreading.pinyin_marks,
                if idx == 0 && thisreading.hsk > 0 {
                    format!(" (HSK {})", thisreading.hsk)
                } else {
                    String::default()
                }
            )
        })
        .collect::<String>();

    cx.render(rsx! {
        span {
            title: "{tooltip}",
            class: "word{wordspacing} hsk{hsk}",
            &cx.props.children
        }
    })
}

#[inline_props]
fn Chinese<'a>(cx: Scope, word: &'a Segment) -> Element<'a> {
    let cfg = use_read(&cx, CONFIG);
    let defs = match word {
        Segment::Break => return cx.render(rsx! { br { } }),
        Segment::Plain(plain) => {
            return cx.render(rsx! {
                span { class: "tone5 plain", "{plain}" }
            })
        }
        Segment::Chinese(ref defs) => defs,
    };
    let thisword = defs[0];

    let cchars = if cfg.simplified {
        thisword.simplified.chars()
    } else {
        thisword.traditional.chars()
    };
    let pwords = cchars.zip(
        thisword
            .pinyin_marks
            .split_whitespace()
            .zip(thisword.tone_marks.clone()),
    );
    let hide_hints = !cfg.hints;
    let tone_color = cfg.tone_color;

    cx.render(rsx! {
        WordSpan {
            defs: defs,
            ruby {
                pwords.into_iter().map(|(c, (pinyin, mut tone))| {
                    if !tone_color { tone = 99 }
                    let linkchars = if cfg.simplified { &thisword.simplified } else { &thisword.traditional };
                    rsx! {
                        ruby {
                            a {
                                href: "https://www.mdbg.net/chinese/dictionary?page=worddict&wdrst=0&wdqb={linkchars}",
                                class: "wordlink tone{tone}",
                                target: "_blank",
                                "{c}"
                            }
                            rt { class: "tone{tone}", hidden: "{hide_hints}", "{pinyin}" }
                        }
                    }
                })
            }
        }
    })
}

#[derive(Debug)]
enum Segment {
    Chinese(Vec<&'static cd::WordEntry>),
    Plain(String),
    Break,
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Chinese(l0), Self::Chinese(r0)) => l0
                .iter()
                .zip(r0.iter())
                .all(|(wel, wer)| wel.word_id == wer.word_id),
            (Self::Plain(l0), Self::Plain(r0)) => l0 == r0,
            (Self::Break, Self::Break) => true,
            _ => false,
        }
    }
}

#[allow(dead_code)]
impl Segment {
    pub fn as_chinese(&self) -> Option<&[&'static cd::WordEntry]> {
        match self {
            Segment::Chinese(ref v) => Some(v),
            _ => None,
        }
    }

    pub fn as_plain(&self) -> Option<&str> {
        match self {
            Segment::Plain(s) => Some(s.as_str()),
            _ => None,
        }
    }
}

fn sort_defs(defs: &mut [&cd::WordEntry]) {
    static SUX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?i)^\s*(:?(?:archaic|old)\s+variant\s+)|(?:\(archaic\))\s*$")
            .expect("Internal error: Could not compile regex")
    });
    let sucky = |we: &cd::WordEntry| {
        we.english.is_empty()
            || we
                .pinyin_numbers
                .chars()
                .next()
                .expect("Internal error: No pinyin for definition")
                .is_ascii_uppercase()
            || SUX.is_match(&we.english[0])
    };
    defs.sort_by(|w1, w2| {
        if sucky(w1) {
            Ordering::Greater
        } else if sucky(w2) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
}

fn make_words(s: &str) -> Vec<Segment> {
    static REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?s)([\p{Han}]+)|([^\p{Han}]+)")
            .expect("Internal error: Could not compile regex")
    });

    REGEX
        .captures_iter(s)
        .flat_map(|chunk| {
            if let Some(ch) = chunk.get(1) {
                cd::tokenize(ch.as_str())
                    .into_iter()
                    .map(|chword| {
                        let mut qr = cd::query_by_simplified(chword);
                        if qr.is_empty() && cd::is_traditional(chword) {
                            qr = cd::query_by_traditional(chword);
                        }
                        if qr.is_empty() {
                            return Segment::Plain(chword.to_owned());
                        }
                        sort_defs(&mut qr);
                        Segment::Chinese(qr)
                    })
                    .collect::<Vec<_>>()
            } else {
                chunk
                    .get(2)
                    .map(|rmatch| rmatch.as_str())
                    .unwrap_or_default()
                    .split_inclusive('\n')
                    .flat_map(|pchunk| {
                        let pchunk = pchunk.to_owned();
                        if pchunk.ends_with('\n') {
                            vec![Segment::Plain(pchunk), Segment::Break]
                        } else {
                            vec![Segment::Plain(pchunk)]
                        }
                    })
                    .collect::<Vec<_>>()
            }
        })
        .collect::<Vec<_>>()
}
