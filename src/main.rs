#![allow(non_snake_case)]
#[cfg(feature = "desktop")]
const _DUMMY: () = compile_error!("Desktop feature currently non-functional");

use dioxus::{core::to_owned, events::FormEvent, prelude::*};

use chinese_dictionary as cd;

mod clipboard;
mod config;
mod input;
mod phonetic;
mod stats;
mod words;

use crate::{
    config::*,
    input::*,
    phonetic as ph,
    stats::Stats,
    words::{generate_hint, Segment},
};

static VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    #[cfg(all(feature = "web", feature = "desktop"))]
    compile_error!("Only one target feature may be enabled.");
    #[cfg(feature = "web")]
    dioxus::web::launch(App);
    #[cfg(feature = "desktop")]
    dioxus::desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let version = VERSION.unwrap_or("UNKNOWN");
    let config = use_ref(&cx, Config::default);
    let segments = use_ref(&cx, Vec::default);
    let stats = use_ref(&cx, Stats::default);

    use_coroutine(&cx, {
        to_owned![segments, stats];
        |rx| input::input_service(rx, segments, stats)
    });

    use_coroutine(&cx, {
        let input_task =
            use_coroutine_handle::<InputAction>(&cx).expect("Could not get input task");
        to_owned![input_task];
        move |rx| clipboard::paste_service(rx, input_task)
    });

    use_coroutine(&cx, {
        to_owned![config, segments];
        |rx| clipboard::copy_service(rx, config, segments)
    });

    cx.render(rsx! {
        style { [include_str!("../assets/styles.css")] }
        Settings { cfg: config.clone() }
        p { }
        h3 { "Enter Simplified Chinese text:" }
        div { ClipboardFunctions { cfg: config.clone() } }
        TextInput { }
        SimpleStats { stats: stats.clone() }
        PrettyChinese { cfg: config.clone(), words: segments.clone() }
        p {
            small {
                "Mandarin Webutil v{version} | "
                a { href: "https://github.com/kerfufflev2/mandarin-webutil/", "GitHub Repo" }
            }
        }
    })
}

#[cfg(not(feature = "web"))]
fn ClipboardFunctions(cx: Scope) -> Element {
    None
}

#[cfg(feature = "web")]
use clipboard::ClipboardFunctions;

macro_rules! cfg_toggle {
    ($cfg:ident, $field:ident) => {
        move |_| {
            let _currval_ = (($cfg).read().$field);
            ($cfg).write().$field = !_currval_;
        }
    };
}

#[inline_props]
fn Settings(cx: Scope, cfg: UseRef<Config>) -> Element {
    let currcfg = cfg.read();
    cx.render(rsx! {
        div {
            class: "settings",
            h3 { "Settings:" }
            BooleanOption {
                label: "Simplified",
                current: currcfg.simplified,
                onchange: cfg_toggle!(cfg, simplified),
            }
            MultiOption {
                label: "Hint",
                current: currcfg.hint as usize,
                options: Hint::OPTIONS,
                onchange: |evt: FormEvent| {
                    cfg.with_mut(move |cfg| {
                        cfg.hint = evt.data.value.parse::<usize>()
                            .unwrap_or(0).into()
                    });
                    cfg.needs_update();
                }
            }
            BooleanOption {
                label: "Tone colors",
                current: currcfg.tonecolor,
                onchange: cfg_toggle!(cfg, tonecolor),
            }
            BooleanOption {
                label: "Hsk",
                current: currcfg.hsk,
                onchange: cfg_toggle!(cfg, hsk),
            }
            BooleanOption {
                label: "Word spacing",
                current: currcfg.wordspace,
                onchange: cfg_toggle!(cfg, wordspace),
            }
            BooleanOption {
                label: "Tooltips",
                current: currcfg.tooltips,
                onchange: cfg_toggle!(cfg, tooltips),
            }
        }
    })
}

#[inline_props]
fn TextInput(cx: Scope) -> Element {
    let input_task = use_coroutine_handle::<InputAction>(&cx).expect("Could not get input task");

    cx.render(rsx! {
        textarea {
            id: "input",
            cols: "100",
            rows: "15",
            autofocus: "true",
            oninput: move |evt| {
                input_task.send(InputAction::Set { refresh: false, s: evt.value.clone() });
            },
        }
    })
}

#[inline_props]
fn SimpleStats(cx: Scope, stats: UseRef<Stats>) -> Element {
    let stats = stats.read();
    let unique_words = stats.hskwords.iter().fold(0, |acc, m| acc + m.len());
    let total_words = stats.hskcounts.iter().fold(0, |acc, c| acc + *c);
    let avghsk = if unique_words == 0 {
        0.0
    } else {
        (stats
            .hskwords
            .iter()
            .enumerate()
            .map(|(idx, m)| (idx + 1) * m.len())
            .sum::<usize>() as f32)
            / (unique_words as f32)
    };
    cx.render(rsx! {
        p {
            b {
                "Words tot/uniq: {total_words}/{unique_words}"
                (total_words > 0).then(|| rsx! { ", avg HSK: {avghsk:.2}" }),
            }

            small {
                stats.hskwords.iter().zip(stats.hskcounts.iter()).enumerate()
                    .filter(|(_, (_, cnt))| **cnt > 0)
                    .map(|(idx, (m, cnt))| {
                        let idx = idx + 1;
                        let unique = m.len();
                        let pct = ((unique as f32) * 100f32) / (unique_words as f32);
                        rsx! {
                            ", "
                            if idx < 15 {
                                rsx! {
                                    "HSK"
                                    b { "{idx}" }
                                }
                            } else {
                                rsx! { "other" }
                            }
                            "("
                            b { "{cnt}" }
                            "/"
                            b { "{unique}" }
                            "/"
                            b { "{pct:.0}" }
                            "%)"
                        }
                    })
                }
        }
    })
}

#[inline_props]
fn PrettyChinese(cx: Scope, cfg: UseRef<Config>, words: UseRef<Vec<Segment>>) -> Element {
    cx.render(rsx! {
        div {
            words.read().iter().cloned().map(|word| {
                rsx! { Chinese { cfg: cfg.clone(), word: word } }
            })
        }
    })
}

#[inline_props]
#[allow(unused_variables)]
fn WordSpan<'a>(
    cx: Scope<'a>,
    cfg: UseRef<Config>,
    defs: Vec<&'static cd::WordEntry>,
    children: Element<'a>,
) -> Element {
    let cfg = cfg.read();
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
                thisreading.pinyin_marks,
                thisreading.traditional,
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
fn Chinese(cx: Scope, cfg: UseRef<Config>, word: Segment) -> Element {
    let word = word.clone();
    let currcfg = cfg.read();
    let defs = match word {
        Segment::Break => return cx.render(rsx! { br { } }),
        Segment::Plain(plain) => {
            return cx.render(rsx! {
                span { class: "tone5 plain", "{plain}" }
            })
        }
        Segment::Chinese(ref defs) => defs,
    };
    let defs = defs.clone();
    let thisword = defs[0];

    let cchars = if currcfg.simplified {
        thisword.simplified.chars()
    } else {
        thisword.traditional.chars()
    };
    let pwords = cchars.zip(
        thisword
            .pinyin_numbers
            .split_whitespace()
            .zip(thisword.pinyin_marks.split_whitespace())
            .zip(thisword.tone_marks.clone()),
    );
    let tone_color = currcfg.tonecolor;
    let hints = pwords
        .into_iter()
        .map(|(c, ((pinyin, pinyintm), mut tone))| {
            let pinyin = &(*pinyin).to_string();
            let pinyintm = pinyintm;
            if !tone_color {
                tone = 99
            }
            let linkchars = if currcfg.simplified {
                &thisword.simplified
            } else {
                &thisword.traditional
            };
            let phon = ph::Syllable::from_pinyin(pinyin).unwrap_or(ph::Syllable {
                init: ph::Initial::Q,
                fin: ph::Final::A,
            });
            let maybehint_top = generate_hint(currcfg.hint, &phon, tone, thisword.hsk, pinyintm);
            (
                c,
                linkchars.to_owned(),
                maybehint_top.map(|h| h.to_string()),
                tone,
            )
        });
    let output = rsx! {
        WordSpan {
            cfg: cx.props.cfg.clone(),
            defs: defs,
            ruby {
                hints.map(|(c, linkchars, maybehint_top, tone)| {
                    rsx! {
                        ruby {
                            a {
                                href: "https://www.mdbg.net/chinese/dictionary?page=worddict&wdrst=0&wdqb={linkchars}",
                                class: "wordlink tone{tone}",
                                target: "_blank",
                                "{c}"
                            }
                            maybehint_top.map(|hint| rsx! {
                                rt { class: "tone{tone}", "{hint}" }
                            })
                        }
                    }
                })
            }
        }
    };

    cx.render(output)
}
