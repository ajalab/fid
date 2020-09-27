use std::fmt;
use std::mem;

use serde::{Deserialize, Serialize};

type Block = u64;
const BLOCK_SIZE: usize = mem::size_of::<Block>() * 8;

#[derive(Serialize, Deserialize, Clone)]
pub struct BitArray {
    blocks: Vec<Block>,
}

impl BitArray {
    /// Returns a zero-cleared bit array of size `size`.
    pub fn new(size: usize) -> Self {
        let n_blocks = (size + BLOCK_SIZE - 1) / BLOCK_SIZE;
        BitArray {
            blocks: (0..n_blocks).map(|_| 0).collect(),
        }
    }

    pub fn with_word_size(word_size: usize, len: usize) -> Self {
        BitArray::new(word_size * len)
    }

    /// Returns the size (byte) of the array.
    pub fn size(&self) -> usize {
        mem::size_of::<Block>() * self.blocks.len()
    }

    /// Sets the bit at position `i` to `b`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ba = fid::BitArray::new(8);
    /// ba.set_bit(3, true);
    /// assert_eq!(ba.get_bit(3), true);
    /// assert_eq!(ba.get_bit(4), false);
    /// ba.set_bit(256, true);  // automatically extended
    /// assert_eq!(ba.get_bit(256), true);
    /// ```
    pub fn set_bit(&mut self, i: usize, b: bool) {
        if i >= self.blocks.len() * BLOCK_SIZE {
            self.resize(i + 1);
        }

        let k = i / BLOCK_SIZE;
        let p = i % BLOCK_SIZE;
        let mask: Block = 1 << (BLOCK_SIZE - p - 1);

        if b {
            self.blocks[k] |= mask;
        } else {
            self.blocks[k] &= !mask;
        }
    }

    /// Gets the bit at position `i`.
    ///
    /// # Panics
    /// Panics if the specified position exceeds the capacity.
    pub fn get_bit(&self, i: usize) -> bool {
        debug_assert!(i < self.blocks.len() * BLOCK_SIZE);

        let k = i / BLOCK_SIZE;
        let p = i % BLOCK_SIZE;

        ((self.blocks[k] << p) >> (BLOCK_SIZE - 1)) == 1
    }

    /// Gets the slice of size `slice_size` at position `i`.
    ///
    /// # Panics
    /// Panics if `slice_size` is greater than 64.
    pub fn set_slice(&mut self, i: usize, slice_size: usize, slice: u64) {
        debug_assert!(slice_size <= 64);
        if slice_size == 0 {
            return;
        }
        if i + slice_size > self.blocks.len() * BLOCK_SIZE {
            self.resize(i + slice_size);
        }

        let k = i / BLOCK_SIZE;
        let p = i % BLOCK_SIZE;
        let excess = (i + slice_size).saturating_sub((k + 1) * BLOCK_SIZE);
        if excess == 0 {
            let mask = slice << (BLOCK_SIZE - p - slice_size);
            self.blocks[k] |= mask as Block;
        } else {
            let mask1 = slice >> excess;
            let mask2 = (slice & (!0 >> (BLOCK_SIZE - excess))) << (BLOCK_SIZE - excess);
            self.blocks[k] |= mask1 as Block;
            self.blocks[k + 1] |= mask2 as Block;
        }
    }

    /// Sets the `i`-th word of size `word_size` to `word`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ba = fid::BitArray::new(128);
    /// ba.set_word(0, 12, 0b0101_1010_1100);
    /// assert_eq!(ba.get_word(0, 12), 0b0101_1010_1100);
    /// ba.set_word(5, 12, 0b1010_0101_0011);
    /// assert_eq!(ba.get_word(5, 12), 0b1010_0101_0011);
    /// ```
    #[inline]
    pub fn set_word(&mut self, i: usize, word_size: usize, word: u64) {
        self.set_slice(i * word_size, word_size, word);
    }

    /// Gets the slice of size `slice_size` at position `i`.
    ///
    /// # Panics
    /// Panics if the end position of the slice exceeds the capacity or `slice_size` is greater than 64.
    pub fn get_slice(&self, i: usize, slice_size: usize) -> u64 {
        debug_assert!(slice_size <= 64);
        debug_assert!(i + slice_size <= self.blocks.len() * BLOCK_SIZE);

        if slice_size == 0 {
            return 0;
        }

        let k = i / BLOCK_SIZE;
        let p = i % BLOCK_SIZE;
        let excess = (i + slice_size).saturating_sub((k + 1) * BLOCK_SIZE);
        if excess == 0 {
            (self.blocks[k] & (!0 >> p)) >> (BLOCK_SIZE - p - slice_size)
        } else {
            let w1 = self.blocks[k] & (!0 >> p);
            let w2 = self.blocks[k + 1] >> (BLOCK_SIZE - excess);
            w1 << excess | w2
        }
    }

    /// Gets the `i`-th word of size `word_size`.
    pub fn get_word(&self, i: usize, word_size: usize) -> u64 {
        self.get_slice(i * word_size, word_size)
    }

    /// Resizes the array.
    pub fn resize(&mut self, new_size: usize) {
        self.blocks
            .resize((new_size + BLOCK_SIZE - 1) / BLOCK_SIZE, 0);
    }
}

impl fmt::Debug for BitArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.blocks
            .iter()
            .map(|b| writeln!(f, "{:0w$b}", b, w = BLOCK_SIZE))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_word_get_word() {
        let word_size = 7;
        let mut ba = BitArray::with_word_size(word_size, 128);
        for i in 0..128 {
            ba.set_word(i, word_size, i as u64);
        }
        for i in 0..128 {
            assert_eq!(ba.get_word(i, word_size), i as u64);
        }
    }

    #[test]
    fn set_bit_get_word() {
        let points = &[3, 5, 6, 7];
        let mut ba = BitArray::new(8);
        for &p in points {
            ba.set_bit(p, true);
        }
        assert_eq!(ba.get_word(0, 4), 1);
        assert_eq!(ba.get_word(1, 4), 7);
    }

    #[test]
    fn set_bit_get_bit() {
        let mut ba = BitArray::new(256);
        let points = &[2, 3, 5, 8, 13, 21, 34, 55, 89, 144];

        for &p in points {
            ba.set_bit(p, true);
        }

        let mut j = 0;
        for i in 0..145 {
            if i == points[j] {
                assert_eq!(ba.get_bit(i), true);
                j += 1;
            } else {
                assert_eq!(ba.get_bit(i), false);
            }
        }
    }

    #[test]
    fn extend_with_resize() {
        let mut ba = BitArray::new(BLOCK_SIZE * 4);
        assert_eq!(ba.blocks.len(), 4);
        ba.resize(BLOCK_SIZE * 5);
        assert_eq!(ba.blocks.len(), 5);
        ba.resize(BLOCK_SIZE * 6 + 7);
        assert_eq!(ba.blocks.len(), 7);
    }

    #[test]
    fn shrink_with_resize() {
        let mut ba = BitArray::new(BLOCK_SIZE * 4);
        assert_eq!(ba.blocks.len(), 4);
        ba.resize(BLOCK_SIZE * 3);
        assert_eq!(ba.blocks.len(), 3);
        ba.resize(BLOCK_SIZE + 3);
        assert_eq!(ba.blocks.len(), 2);
    }
}
