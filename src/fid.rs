/// A type that supports rank and support operations.
pub trait FID {
    /// Returns the total number of bits.
    fn len(&self) -> u64;

    /// Returns true if the structure is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Compute the number of bits in [0..i).
    fn rank(&self, b: bool, i: u64) -> u64 {
        if b {
            self.rank1(i)
        } else {
            self.rank0(i)
        }
    }

    /// Compute the number of 0s in [0..i).
    fn rank0(&self, i: u64) -> u64 {
        i - self.rank1(i)
    }

    /// Compute the number of 0s in [0..i).
    fn rank1(&self, i: u64) -> u64 {
        i - self.rank0(i)
    }

    /// Locate the position of the (r + 1)-th bit.
    fn select(&self, b: bool, r: u64) -> u64 {
        let (mut s, mut e) = (0, self.len());

        while e - s > 1 {
            let m = (s + e) / 2;
            let rank = self.rank(b, m);
            if r < rank {
                e = m
            } else {
                s = m
            }
        }
        s
    }

    /// Locate the position of the (r + 1)-th 0.
    fn select0(&self, r: u64) -> u64 {
        self.select(false, r)
    }

    /// Locate the position of the (r + 1)-th 1.
    fn select1(&self, r: u64) -> u64 {
        self.select(true, r)
    }

    /// Returns the i-th bit.
    fn get(&self, i: u64) -> bool {
        self.rank1(i + 1) - self.rank1(i) > 0
    }
}
