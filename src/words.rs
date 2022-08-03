#![allow(non_snake_case)]
use std::{borrow::Cow, cmp::Ordering};

use crate::phonetic::Initial;
use chinese_dictionary as cd;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{config::Hint, phonetic as ph, stats::Stats};

pub fn generate_hint(
    hint: Hint,
    phon: &ph::Syllable,
    tone: u8,
    hsk: u8,
    pin: &'static str,
) -> Option<Cow<'static, str>> {
    match hint {
        Hint::Off => None,
        Hint::Pinyin => Some(Cow::from(phon.pinyin())),
        Hint::PinyinInit => Some(if phon.init != Initial::Hh {
            Cow::from(phon.init.pinyin())
        } else {
            Cow::from(&phon.fin.pinyin(phon.init)[0..1])
        }),
        Hint::PinyinFin => Some(Cow::from({
            let mut result = phon.fin.pinyin(phon.init);
            if !result.is_empty()
                && phon.init == Initial::Hh
                && (result.starts_with('y') || result.starts_with('w'))
            {
                result = &result[1..];
            }
            result
        })),
        Hint::Zhuyin => Some(Cow::from(phon.zhuyin())),
        Hint::Ipa => Some(Cow::from(phon.ipa())),
        Hint::Raw => Some({
            let inistr = if phon.init == ph::Initial::Hh {
                String::default()
            } else {
                format!("{:?}", phon.init)
            };
            let mut result = format!("{inistr}{:?}", phon.fin);
            result.make_ascii_lowercase();
            Cow::from(result)
        }),
        Hint::ToneMark => Some(Cow::from(tone.to_string())),
        Hint::Hsk => Some(Cow::from(hsk.to_string())),
        Hint::PinyinTM => Some(Cow::from(pin)),
    }
}

#[derive(Debug, Clone)]
pub enum Segment {
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

pub fn make_words(s: &str) -> (Vec<Segment>, Stats) {
    static REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?s)([\p{Han}]+)|([^\p{Han}]+)")
            .expect("Internal error: Could not compile regex")
    });
    let mut stats = Stats::new();

    (
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
                            if !qr.is_empty() {
                                let w = &qr[0];
                                stats.update(&w.simplified, w.hsk);
                            }
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
            .collect::<Vec<_>>(),
        stats,
    )
}
