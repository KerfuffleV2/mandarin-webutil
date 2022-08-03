use dioxus::prelude::*;
use futures::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Clipboard;

use crate::{
    config::{Config, Hint},
    input::InputAction,
    phonetic as ph,
    words::Segment,
};

pub fn get() -> Option<Clipboard> {
    web_sys::window().and_then(|w| w.navigator().clipboard())
}

pub async fn copy(txt: impl AsRef<str>) {
    if let Some(cb) = self::get() {
        let _ = JsFuture::from(cb.write_text(txt.as_ref())).await;
    }
}

pub async fn read() -> String {
    if let Some(cb) = self::get() {
        JsFuture::from(cb.read_text())
            .await
            .map(|v| v.as_string())
            .unwrap_or_default()
    } else {
        None
    }
    .unwrap_or_default()
}

pub enum PasteAction {
    Append,
    Replace,
}

pub async fn paste_service(
    mut rx: UnboundedReceiver<PasteAction>,
    input_task: CoroutineHandle<InputAction>,
) {
    while let Some(msg) = rx.next().await {
        let clipval = self::read().await;
        input_task.send(match msg {
            PasteAction::Replace => InputAction::Set {
                refresh: true,
                s: clipval,
            },
            PasteAction::Append => InputAction::Append {
                refresh: true,
                s: clipval,
            },
        });
    }
}

pub enum CopyAction {
    Characters,
    Annotations,
}

pub async fn copy_service(
    mut rx: UnboundedReceiver<CopyAction>,
    cfg: UseRef<Config>,
    segments: UseRef<Vec<Segment>>,
) {
    let mut result = String::with_capacity(1024);
    while let Some(msg) = rx.next().await {
        let currcfg = cfg.read().to_owned();
        let is_chars = matches!(msg, CopyAction::Characters);
        let is_simp = currcfg.simplified;
        let hint_typ = currcfg.hint;

        result.clear();
        let segs = segments.read().to_owned();

        for seg in segs.iter() {
            match seg {
                Segment::Chinese(words) => {
                    if words.is_empty() {
                        continue;
                    }
                    let we = words[0];
                    if is_chars {
                        result.push_str(if is_simp {
                            &we.simplified
                        } else {
                            &we.traditional
                        });
                    } else {
                        we.pinyin_numbers
                            .split_whitespace()
                            .zip(we.tone_marks.clone())
                            .zip(we.pinyin_marks.split_whitespace())
                            .for_each(|((pinyin, tone), pinyintm)| {
                                let phon =
                                    ph::Syllable::from_pinyin(pinyin).unwrap_or(ph::Syllable {
                                        init: ph::Initial::Q,
                                        fin: ph::Final::A,
                                    });
                                if let Some(hint) = crate::words::generate_hint(
                                    hint_typ, &phon, tone, we.hsk, pinyintm,
                                ) {
                                    result.push_str(&hint);
                                    result.push(' ');
                                }
                            })
                    }
                }
                Segment::Plain(txt) => result.push_str(txt.as_str()),
                Segment::Break => result.push('\n'),
            }
        }
        self::copy(result.trim_end()).await;
    }
}

#[inline_props]
pub fn ClipboardFunctions(cx: Scope, cfg: UseRef<Config>) -> Element {
    let cfg = cfg.read();
    let paste_task = use_coroutine_handle::<PasteAction>(&cx).expect("Could not get paste task");
    let copy_task = use_coroutine_handle::<CopyAction>(&cx).expect("Could not get copy task");

    cx.render(rsx! {
        "[Copy "
        button {
            title: "Copy characters",
            onclick: move |_| copy_task.send(CopyAction::Characters),
            "子"
        }
        (cfg.hint != Hint::Off).then(|| {
            let anntyp = Hint::OPTIONS[cfg.hint as usize];
            rsx! {
                " | "
                button {
                    title: "Copy annotations",
                    onclick: move |_| copy_task.send(CopyAction::Annotations),
                    "{anntyp}"
                }
            }
        })

        "], [Paste "
        button {
            title: "Append clipboard",
            onclick: move |_| paste_task.send(PasteAction::Append),
            "+"
        }
        " | "
        button {
            title: "Replace with clipboard",
            onclick: move |_| paste_task.send(PasteAction::Replace),
            "="
        }
        "]"
    })
}
