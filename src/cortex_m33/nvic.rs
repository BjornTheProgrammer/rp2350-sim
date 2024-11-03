use std::ops::{Index, IndexMut};

pub struct Nvic {
    ipr0: u32,
    ipr1: u32,
    ipr2: u32,
    ipr3: u32,
    ipr4: u32,
    ipr5: u32,
    ipr6: u32,
    ipr7: u32,
}

impl Nvic {
    pub fn new() -> Self {
        Self {
            ipr0: 0,
            ipr1: 0,
            ipr2: 0,
            ipr3: 0,
            ipr4: 0,
            ipr5: 0,
            ipr6: 0,
            ipr7: 0,
        }
    }
}

impl Index<usize> for Nvic {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.ipr0,
            1 => &self.ipr1,
            2 => &self.ipr2,
            3 => &self.ipr3,
            4 => &self.ipr4,
            5 => &self.ipr5,
            6 => &self.ipr6,
            7 => &self.ipr7,
            _ => panic!("Specified nvic of '{}' does not exist", index),
        }
    }
}

impl IndexMut<usize> for Nvic {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.ipr0,
            1 => &mut self.ipr1,
            2 => &mut self.ipr2,
            3 => &mut self.ipr3,
            4 => &mut self.ipr4,
            5 => &mut self.ipr5,
            6 => &mut self.ipr6,
            7 => &mut self.ipr7,
            _ => panic!("Specified nvic of '{}' does not exist", index),
        }
    }
}
