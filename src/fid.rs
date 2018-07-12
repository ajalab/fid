pub trait FID {
    fn len(&self) -> u64;

    fn rank(&self, c: u8, i: u64) -> u64 {
        if c == 0 {
            self.rank0(i)
        } else {
            self.rank1(i)
        }
    }

    fn rank0(&self, i: u64) -> u64 {
        i + 1 - self.rank1(i)
    }

    fn rank1(&self, i: u64) -> u64 {
        i + 1 - self.rank0(i)
    }

    fn select(&self, c: u8, i: u64) -> u64 {
        let (mut s, mut e) = (0, self.len());

        while e - s > 1 {
            let m = (s + e) / 2;
            let r = self.rank(c, m);
            if i + 1 <= r {
                e = m
            } else {
                s = m
            }
        }
        return s;
    }

    fn select0(&self, i: u64) -> u64 {
        self.select(0, i)
    }

    fn select1(&self, i: u64) -> u64 {
        self.select(1, i)
    }
}
