#![allow(dead_code)]
use std::collections::HashMap;
use std::iter;

const MAX_HSK_LEVEL: usize = 16;

type HskMap = HashMap<&'static str, usize>;

pub struct Stats {
    pub hskwords: Vec<HskMap>,
    pub hskcounts: Vec<usize>,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            hskwords: Vec::from_iter(iter::repeat(HskMap::default()).take(MAX_HSK_LEVEL)),
            hskcounts: Vec::from_iter(iter::repeat(0).take(MAX_HSK_LEVEL)),
        }
    }

    pub fn reset(&mut self) {
        self.hskcounts = Vec::from_iter(iter::repeat(0).take(MAX_HSK_LEVEL));
        self.hskwords.iter_mut().for_each(|hm| hm.clear());
    }

    pub fn update(&mut self, k: &'static str, hsk: u8) {
        let hsk = if hsk >= (MAX_HSK_LEVEL as u8) || hsk == 0 {
            MAX_HSK_LEVEL - 1
        } else {
            (hsk - 1) as usize
        };
        let ent = self.hskwords[hsk].entry(k).or_insert(0);
        *ent += 1;
        self.hskcounts[hsk] += 1;
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}
