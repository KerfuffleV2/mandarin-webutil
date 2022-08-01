#![allow(dead_code, unused_variables)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Syllable {
    pub init: Initial,
    pub fin: Final,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Initial {
    Hh, // None
    B,
    P,
    M,
    F,
    D,
    T,
    N,
    L,
    Z,
    C,
    S,
    Zh,
    Ch,
    Sh,
    R,
    J,
    Q,
    X,
    G,
    K,
    H,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Final {
    A,
    Ai,
    Ao,
    An,
    Ang,
    E,
    Ei,
    En,
    Eng,
    Er,
    O,
    Ou,
    Ong,
    I,
    Ir,
    Ia,
    Iao,
    Ie,
    Iou,
    Ian,
    Iang,
    In,
    Ing,
    Iong,
    U,
    Ua,
    Uai,
    Uei,
    Uo,
    Uan,
    Uang,
    Uen,
    Ueng,
    V,
    Ve,
    Van,
    Vn,
}

#[inline]
pub fn is_zhuyin_char(c: char) -> bool {
    ('\u{3100}'..='\u{312f}').contains(&c) || ('\u{31a0}'..='\u{31bf}').contains(&c)
}

impl Final {
    const PINYIN_NO_INITIAL: &'static [&'static str] = &[
        "a", "ai", "ao", "an", "ang", "e", "ei", "en", "eng", "er", "o", "ou", "ong", "yi", "yir",
        "ya", "yao", "ye", "you", "yan", "yang", "yin", "ying", "yong", "wu", "wa", "wai", "wei",
        "wo", "wan", "wang", "wen", "weng", "yu", "yue", "yuan", "yun",
    ];
    const PINYIN: &'static [&'static str] = &[
        "a", "ai", "ao", "an", "ang", "e", "ei", "en", "eng", "er", "o", "ou", "ong", "i", "i",
        "ia", "iao", "ie", "iu", "ian", "iang", "in", "ing", "iong", "u", "ua", "uai", "ui", "uo",
        "uan", "uang", "un", "eng", "u", "ue", "uan", "un",
    ];

    const ZHUYIN: &'static [&'static str] = &[
        "ㄚ",    // a
        "ㄞ",    // ai
        "ㄠ",    // ao
        "ㄢ",    // an
        "ㄤ",    // ang
        "ㄜ",    // e
        "ㄟ",    // ei
        "ㄣ",    // en
        "ㄥ",    // eng
        "ㄦ",    // er
        "ㄛ",    // o
        "ㄡ",    // ou
        "ㄨㄥ", // ong
        "ㄧ",    // i
        "",       // i(r)
        "ㄧㄚ", // ya
        "ㄧㄠ", // yao
        "ㄧㄝ", // ye
        "ㄧㄡ", // you
        "ㄧㄢ", // yan
        "ㄧㄤ", // yang
        "ㄧㄣ", // yin
        "ㄧㄥ", // ying
        "ㄩㄥ", // yong
        "ㄨ",    // wu
        "ㄨㄚ", // wa
        "ㄨㄞ", // wai
        "ㄨㄟ", // wei
        "ㄨㄛ", // wo
        "ㄨㄢ", // wan
        "ㄨㄤ", // wang
        "ㄨㄣ", // wen
        "ㄨㄥ", // weng
        "ㄩ",    // yu
        "ㄩㄝ", // yue
        "ㄩㄢ", // yuan
        "ㄩㄣ", // yun
    ];

    const IPA: &'static [&'static str] = &[
        "ɑ",        // a
        "aɪ̯",     // ai
        "ɑʊ̯",    // ao
        "an",        // an
        "ɑŋ",      // ang
        "ɯ̯ʌ",    // e
        "eɪ̯",     // ei
        "ən",       // en
        "əŋ",      // eng
        "ɑɻ",      // er
        "ɔ",        // o
        "ɤʊ̯",    // ou
        "ʊŋ",      // ong
        "i",         // i
        "ɿ",        // i(r)
        "i̯ɑ",     // ya
        "i̯ɑʊ̯", // yao
        "iɛ",       // ye
        "i̯ɤʊ̯", // you
        "iɛn",      // yan
        "i̯ɑŋ",   // yang
        "in",        // yin
        "iŋ",       // ying
        "i̯ʊŋ",   // yong
        "u",         // wu
        "u̯ɑ",     // wa
        "u̯aɪ̯",  // wai
        "u̯eɪ̯",  // wei
        "u̯ɔ",     // wo
        "u̯an",     // wan
        "u̯ɑŋ",   // wang
        "u̯ən",    // wen
        "u̯əŋ",   // weng
        "y",         // yu
        "y̯œ",     // yue
        "y̯ɛn",    // yuan
        "yn",        // yun
    ];

    pub fn fix_with_initial(&self, ini: Initial) -> Self {
        if ini >= Initial::Z && ini <= Initial::R && self == &Self::I {
            Self::Ir
        } else {
            *self
        }
    }

    pub fn from_pinyin(s: impl AsRef<str>, ini: Initial) -> Option<Final> {
        let mut c = s
            .as_ref()
            .chars()
            .take(16)
            .filter(|c| c.is_alphabetic())
            .flat_map(|c| c.to_lowercase());
        let c1 = if ini == Initial::Hh {
            match c.next()? {
                'y' => 'i',
                'w' => 'u',
                c1 => c1,
            }
        } else {
            c.next().unwrap_or_default()
        };
        Some(match c1 {
            'i' if ini >= Initial::Z && ini <= Initial::R => Self::Ir,
            'i' => match c.next().unwrap_or_default() {
                'a' => match c.next().unwrap_or_default() {
                    'o' => Self::Iao,
                    'n' => match c.next() {
                        Some('g') => Self::Iang,
                        _ => Self::Ian,
                    },
                    _ => Self::Ia,
                },
                'e' => Self::Ie,
                'u' if ini == Initial::Hh => match c.next().unwrap_or_default() {
                    'e' => Self::Ve,
                    'a' => {
                        if c.next() == Some('n') {
                            Self::Van
                        } else {
                            return None;
                        }
                    }
                    'n' => Self::Vn,
                    _ => Self::V,
                },
                'u' => Self::Iou,
                'o' => match c.next().unwrap_or_default() {
                    'u' if ini == Initial::Hh => Self::Iou,
                    'n' => {
                        if c.next() == Some('g') {
                            Self::Iong
                        } else {
                            return None;
                        }
                    }
                    _ => return None,
                },
                'n' => match c.next() {
                    Some('g') => Self::Ing,
                    _ => Self::In,
                },
                'i' if ini == Initial::Hh => {
                    if c.next() == Some('n') {
                        Self::In
                    } else {
                        Self::I
                    }
                }
                _ => Self::I,
            },
            'ü' | 'v' => {
                if c.next() == Some('e') {
                    Self::Ve
                } else {
                    Self::V
                }
            }
            'u' if ini == Initial::J || ini == Initial::Q || ini == Initial::X => {
                match c.next().unwrap_or_default() {
                    'a' => {
                        if c.next() == Some('n') {
                            Self::Van
                        } else {
                            return None;
                        }
                    }
                    'e' => Self::Ve,
                    'n' => Self::Vn,
                    _ => Self::V,
                }
            }
            'u' => match c.next().unwrap_or_default() {
                'u' if ini == Initial::Hh => Self::U, // wu
                'e' if ini == Initial::Hh => match c.next().unwrap_or_default() {
                    // wei, wen, weng
                    'i' => Self::Uei,
                    'n' => {
                        if c.next() == Some('g') {
                            Self::Ueng
                        } else {
                            Self::Uen
                        }
                    }
                    _ => return None,
                },
                'a' => match c.next().unwrap_or_default() {
                    'i' => Self::Uai,
                    'n' => {
                        if c.next() == Some('g') {
                            Self::Uang
                        } else {
                            Self::Uan
                        }
                    }
                    _ => Self::Ua,
                },
                'n' => Self::Uen,
                'o' => Self::Uo,
                'i' => Self::Uei,
                _ => Self::U,
            },
            'a' => match c.next() {
                Some('i') => Self::Ai,
                Some('o') => Self::Ao,
                Some('n') => match c.next() {
                    Some('g') => Self::Ang,
                    _ => Self::An,
                },
                _ => Self::A,
            },
            'e' => match c.next() {
                Some('i') => Self::Ei,
                Some('r') if ini == Initial::Hh => Self::Er,
                Some('n') => match c.next() {
                    Some('g') => Self::Eng,
                    _ => Self::En,
                },
                _ => Self::E,
            },
            'o' => match c.next() {
                Some('u') => Self::Ou,
                Some('n') => match c.next()? {
                    'g' => Self::Ong,
                    _ => return None,
                },
                _ => Self::O,
            },
            _ => {
                if ini == Initial::R {
                    Self::Er
                } else {
                    return None;
                }
            }
        })
    }

    // TBD?
    /*
    pub fn from_ipa(s: impl AsRef<str>) -> Option<Final> {
      let s = s.as_ref();
      let mut chars = s.chars();
      /*
        "ɯ̯ʌ",    // e
        "eɪ̯",     // ei
        "ɔ",        // o
        "ɤʊ̯",    // ou
        "ʊŋ",      // ong
        "ɿ",        // i(r)
        "u",         // wu
        "ɪ̯"
       */
      Some(match chars.next()? {
        'ɑ' => match chars.next() {
          'ʊ̯' => Final::Ao,
          'ŋ' => Final::Ang,
          'ɻ' => Final::Er,
          _ => Final::A,
        },        // a

        'a' => match chars.next()? {
          'ɪ̯' => Final::Ai,
          'n' => Final::An,
          _ => return None,
        },     // ai

        "u̯ɑ" => None,     // wa
        "u̯aɪ̯" => None,  // wai
        "u̯eɪ̯" => None,  // wei
        "u̯ɔ" => None,     // wo
        "u̯an" => None,     // wan
        "u̯ɑŋ" => None,   // wang
        "u̯ən" => None,    // wen
        "u̯əŋ" => None,   // weng

        "i̯ɑ" => None,     // ya
        "i̯ɑʊ̯" => None, // yao
        "i̯ɤʊ̯" => None, // you
        "i̯ɑŋ" => None,   // yang
        "i̯ʊŋ" => None,   // yong

        "i" => None,         // i
        "iɛ" => None,       // ye
        "iɛn" => None,      // yan
        "in" => None,        // yin
        "iŋ" => None,       // ying

        "ən" => None,       // en
        "əŋ" => None,      // eng

        "y̯œ" => None,     // yue
        "y̯ɛn" => None,    // yuan

        "y" => None,         // yu
        "yn" => None,        // yun
        _ => None,
      })
      // None
    }
    */

    #[inline]
    pub fn pinyin(&self, ini: Initial) -> &'static str {
        match ini {
            Initial::Hh => return Self::PINYIN_NO_INITIAL[*self as usize],
            Initial::L | Initial::N => {
                if *self == Self::V {
                    return "ü";
                } else if *self == Self::Ve {
                    return "üe";
                }
            }
            _ => (),
        }
        Self::PINYIN[*self as usize]
    }

    #[inline]
    pub fn zhuyin(&self) -> &'static str {
        Self::ZHUYIN[*self as usize]
    }

    #[inline]
    pub fn ipa(&self) -> &'static str {
        Self::IPA[*self as usize]
    }
}

impl Initial {
    const PINYIN: &'static [&'static str] = &[
        "", "b", "p", "m", "f", "d", "t", "n", "l", "z", "c", "s", "zh", "ch", "sh", "r", "j", "q",
        "x", "g", "k", "h",
    ];

    const ZHUYIN: &'static [&'static str] = &[
        "", "ㄅ", "ㄆ", "ㄇ", "ㄈ", "ㄉ", "ㄊ", "ㄋ", "ㄌ", "ㄗ", "ㄘ", "ㄙ", "ㄓ", "ㄔ", "ㄕ",
        "ㄖ", "ㄐ", "ㄑ", "ㄒ", "ㄍ", "ㄎ", "ㄏ",
    ];

    const IPA: &'static [&'static str] = &[
        "", "p", "pʰ", "m", "f", "t", "tʰ", "n", "l", "ts", "tsʰ", "s", "tʂ", "tʂʰ", "ʂ", "ʐ",
        "tɕ", "tɕʰ", "ɕ", "k", "kʰ", "x",
    ];

    pub fn from_pinyin(s: impl AsRef<str>) -> Option<Self> {
        let mut chars = s.as_ref().chars().flat_map(|c| c.to_lowercase());
        Some(match chars.next()? {
            'b' => Self::B,
            'p' => Self::P,
            'm' => Self::M,
            'f' => Self::F,
            'd' => Self::D,
            't' => Self::T,
            'n' => Self::N,
            'l' => Self::L,
            'z' => {
                if chars.next()? == 'h' {
                    Self::Zh
                } else {
                    Self::Z
                }
            }
            'c' => {
                if chars.next()? == 'h' {
                    Self::Ch
                } else {
                    Self::C
                }
            }
            's' => {
                if chars.next()? == 'h' {
                    Self::Sh
                } else {
                    Self::S
                }
            }
            'r' => Self::R,
            'j' => Self::J,
            'q' => Self::Q,
            'x' => Self::X,
            'g' => Self::G,
            'k' => Self::K,
            'h' => Self::H,
            c => {
                if c.is_alphabetic() {
                    Self::Hh
                } else {
                    return None;
                }
            }
        })
    }

    pub fn from_zhuyin(s: impl AsRef<str>) -> Option<Self> {
        // Bopomofo: U+3100–U+312F, U+31A0–U+31BF
        let c = match s.as_ref().chars().next() {
            Some(c) if is_zhuyin_char(c) => c,
            _ => return None,
        };
        Some(match c {
            'ㄅ' => Self::B,
            'ㄆ' => Self::P,
            'ㄇ' => Self::M,
            'ㄈ' => Self::F,
            'ㄉ' => Self::D,
            'ㄊ' => Self::T,
            'ㄋ' => Self::N,
            'ㄌ' => Self::L,
            'ㄗ' => Self::Z,
            'ㄘ' => Self::C,
            'ㄙ' => Self::S,
            'ㄓ' => Self::Zh,
            'ㄔ' => Self::Ch,
            'ㄕ' => Self::Sh,
            'ㄖ' => Self::R,
            'ㄐ' => Self::J,
            'ㄑ' => Self::Q,
            'ㄒ' => Self::X,
            'ㄍ' => Self::G,
            'ㄎ' => Self::K,
            'ㄏ' => Self::H,
            _ => Self::Hh,
        })
    }

    pub fn from_ipa(s: impl AsRef<str>) -> Option<Self> {
        let mut chars = s.as_ref().chars();
        Some(match chars.next()? {
            'ɕ' => Self::X,
            'f' => Self::F,
            'k' => {
                if chars.next()? == 'ʰ' {
                    Self::K
                } else {
                    Self::G
                }
            }
            'l' => Self::L,
            'm' => Self::M,
            'n' => Self::N,
            'p' => {
                if chars.next()? == 'ʰ' {
                    Self::P
                } else {
                    Self::B
                }
            }
            's' => Self::S,
            'ʂ' => Self::Sh,
            't' => match chars.next()? {
                'ʰ' => Self::T,
                'ɕ' => {
                    if chars.next()? == 'ʰ' {
                        Self::Q
                    } else {
                        Self::J
                    }
                }
                's' => {
                    if chars.next()? == 'ʰ' {
                        Self::C
                    } else {
                        Self::Z
                    }
                }
                'ʂ' => {
                    if chars.next()? == 'ʰ' {
                        Self::Ch
                    } else {
                        Self::Zh
                    }
                }

                _ => Self::D,
            },
            'x' => Self::H,
            'ʐ' => Self::R,
            _ => Self::Hh,
        })
    }

    #[inline]
    pub fn pinyin(&self) -> &'static str {
        Self::PINYIN[*self as usize]
    }

    #[inline]
    pub fn zhuyin(&self) -> &'static str {
        Self::ZHUYIN[*self as usize]
    }

    #[inline]
    pub fn ipa(&self) -> &'static str {
        Self::IPA[*self as usize]
    }
}

impl Syllable {
    pub fn from_pinyin(s: impl AsRef<str>) -> Option<Self> {
        let s = s.as_ref().trim_start();
        let mut init = Initial::from_pinyin(s)?;
        let is = init.pinyin();
        let fin = Final::from_pinyin(
            if init == Initial::Hh {
                s
            } else if s.len() <= is.len() {
                ""
            } else {
                &s[is.len()..]
            },
            init,
        )?;
        if init == Initial::R && fin == Final::Er {
            init = Initial::Hh;
        }
        Some(Self { init, fin })
    }

    pub fn pinyin(&self) -> String {
        [self.init.pinyin(), self.fin.pinyin(self.init)]
            .into_iter()
            .collect::<String>()
    }

    pub fn zhuyin(&self) -> String {
        [self.init.zhuyin(), self.fin.zhuyin()]
            .into_iter()
            .collect::<String>()
    }

    pub fn ipa(&self) -> String {
        [self.init.ipa(), self.fin.ipa()]
            .into_iter()
            .collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const PINYIN_WORDS: &str = include_str!("../test_data/pinyin.lst");

    #[test]
    pub fn test_pinyin() {
        assert_eq!(Initial::from_pinyin("wo3"), Some(Initial::Hh));
        assert_eq!(Initial::from_pinyin("chi1"), Some(Initial::Ch));
        assert_eq!(Final::from_pinyin("wo3", Initial::Hh), Some(Final::Uo));
        assert_eq!(
            Syllable::from_pinyin("r1"),
            Some(Syllable {
                init: Initial::Hh,
                fin: Final::Er
            })
        );
        assert_eq!(
            Syllable::from_pinyin("xia1"),
            Some(Syllable {
                init: Initial::X,
                fin: Final::Ia
            })
        );
        assert_eq!(
            Syllable::from_pinyin("yu1"),
            Some(Syllable {
                init: Initial::Hh,
                fin: Final::V
            })
        );
        assert_eq!(
            Syllable::from_pinyin("qu1"),
            Some(Syllable {
                init: Initial::Q,
                fin: Final::V
            })
        );
    }

    #[test]
    // FIXME: Only verifies that parsing succeeds.
    pub fn test_pinyin_words() {
        let lines = PINYIN_WORDS
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty() && !l.starts_with('#'));
        for line in lines {
            let w = Syllable::from_pinyin(line);

            assert_ne!(w, None);
            println!("{line}\t=>\t{:?}", w.unwrap());
        }
    }
}
