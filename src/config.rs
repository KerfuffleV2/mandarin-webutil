use dioxus::{events::FormEvent, fermi::*, prelude::*};

pub static CONFIG: Atom<Config> = |_| Config::default();

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hint {
    Off,
    Pinyin,
    PinyinInit,
    PinyinFin,
    Zhuyin,
    Ipa,
    Raw,
    ToneMark,
    Hsk,
    PinyinTM,
}

impl Hint {
    pub const OPTIONS: &'static [&'static str] = &[
        "off",
        "pinyin",
        "pinyin initial",
        "pinyin final",
        "zhuyin",
        "IPA",
        "raw",
        "tone",
        "HSK",
        "pinyin tm",
    ];
}

impl From<usize> for Hint {
    fn from(val: usize) -> Self {
        match val {
            1 => Self::Pinyin,
            2 => Self::PinyinInit,
            3 => Self::PinyinFin,
            4 => Self::Zhuyin,
            5 => Self::Ipa,
            6 => Self::Raw,
            7 => Self::ToneMark,
            8 => Self::Hsk,
            9 => Self::PinyinTM,
            _ => Self::Off,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    pub hint: Hint,
    pub tonecolor: bool,
    pub hsk: bool,
    pub simplified: bool,
    pub wordspace: bool,
    pub tooltips: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hint: Hint::Pinyin,
            tonecolor: true,
            simplified: true,
            hsk: true,
            wordspace: true,
            tooltips: true,
        }
    }
}

#[derive(Props)]
pub struct BooleanOptionProps<'a> {
    label: &'a str,
    current: bool,
    onchange: EventHandler<'a, FormEvent>,
}

pub fn BooleanOption<'a>(cx: Scope<'a, BooleanOptionProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
      label {
        "{cx.props.label}: "
        input {
          r#type: "checkbox",
          checked: "{cx.props.current}",
          onchange: move |evt| cx.props.onchange.call(evt)
        }
      }

    })
}

#[derive(Props)]
pub struct MultiOptionProps<'a> {
    label: &'a str,
    options: &'a [&'a str],
    current: usize,
    onchange: EventHandler<'a, FormEvent>,
}

pub fn MultiOption<'a>(cx: Scope<'a, MultiOptionProps<'a>>) -> Element {
    let curr = cx.props.current;
    cx.render(rsx! {
        label {
          "{cx.props.label}: "
          select {
            onchange: move |evt| cx.props.onchange.call(evt),
            cx.props.options.iter().enumerate().map(|(idx, opt)| {
              let selected = idx == curr;
              rsx! {
                option {
                  value: "{idx}",
                  selected: "{selected}",
                  "{opt}"
                }
              }
            })
          }
        }


    })
}
