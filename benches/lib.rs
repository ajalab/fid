#![feature(test)]

extern crate fid;
extern crate rand;
extern crate test;

use fid::{BitVector, FID};
use rand::{Rng, SeedableRng, StdRng};
use test::Bencher;

const TRIALS: u64 = 10000;

#[bench]
fn rank_1000000_dense(b: &mut Bencher) {
    bench_rank1(1000000, 0.99, b);
}

#[bench]
fn rank_1000000_normal(b: &mut Bencher) {
    bench_rank1(1000000, 0.5, b);
}

#[bench]
fn rank_1000000_sparse(b: &mut Bencher) {
    bench_rank1(1000000, 0.01, b);
}

#[bench]
fn rank_100000000_dense(b: &mut Bencher) {
    bench_rank1(100000000, 0.99, b);
}

#[bench]
fn rank_100000000_normal(b: &mut Bencher) {
    bench_rank1(100000000, 0.5, b);
}

#[bench]
fn rank_100000000_sparse(b: &mut Bencher) {
    bench_rank1(100000000, 0.01, b);
}

#[bench]
fn select_1000000_dense(b: &mut Bencher) {
    bench_select1(1000000, 0.99, b);
}

#[bench]
fn select_1000000_normal(b: &mut Bencher) {
    bench_select1(1000000, 0.5, b);
}

#[bench]
fn select_1000000_sparse(b: &mut Bencher) {
    bench_select1(1000000, 0.01, b);
}

#[bench]
fn select_100000000_dense(b: &mut Bencher) {
    bench_select1(100000000, 0.99, b);
}

#[bench]
fn select_100000000_normal(b: &mut Bencher) {
    bench_select1(100000000, 0.5, b);
}

#[bench]
fn select_100000000_sparse(b: &mut Bencher) {
    bench_select1(100000000, 0.01, b);
}

fn bench_rank1(n: u64, p: f64, b: &mut Bencher) {
    let mut rng: StdRng = SeedableRng::from_seed([0; 32]);
    let mut bv = BitVector::new();
    for _ in 0..n {
        let b = rng.gen_bool(p);
        bv.push(b);
    }
    b.iter(|| {
        for _ in 0..TRIALS {
            bv.rank1(rng.gen_range(0, n));
        }
    })
}

fn bench_select1(n: u64, p: f64, b: &mut Bencher) {
    let mut rng: StdRng = SeedableRng::from_seed([0; 32]);
    let mut bv = BitVector::new();
    let mut rank = 0;
    for _ in 0..n {
        let b = rng.gen_bool(p);
        bv.push(b);
        rank += b as u64;
    }
    b.iter(|| {
        for _ in 0..TRIALS {
            bv.select1(rng.gen_range(0, rank));
        }
    })
}
