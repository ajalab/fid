# FID

[![Crates.io](https://img.shields.io/crates/v/fid.svg)](https://crates.io/crates/fid)
[![docs.rs](https://docs.rs/fid/badge.svg)](https://docs.rs/fid)
[![Build Status](https://travis-ci.com/ajalab/fid.svg?branch=master)](https://travis-ci.com/ajalab/fid)

 This crate provides a succinct data structure for bit vectors that support two kinds of bit operations in constant-time:

 - `rank(i)` computes the number of 0s (or 1s) in [0..i)
 - `select(r)` locates the (r+1)-th position of 0 (or 1).

 Structures supporting these two operations are called FID (fully indexable dictionary).

## Usage

In your `Cargo.toml`
```
[dependencies]
fid = "0.1"
```
then
```rust
extern crate fid;

use fid::{BitVector, FID};

let mut bv = BitVector::new();
// 01101101
bv.push(false); bv.push(true); bv.push(true); bv.push(false);
bv.push(true); bv.push(true); bv.push(false); bv.push(true);

assert_eq!(bv.rank0(5), 2);
assert_eq!(bv.rank1(5), 3);
assert_eq!(bv.select0(2), 6);
assert_eq!(bv.select1(2), 4);
```

## Credits

The basic compression and computation algorithms for `BitVector` are originally from [1], and its practical implementation techniques are from [2].

In `BitVector`, bits are divided in small and large blocks.
Each small block is identified by a class (number of 1s in the block) and an index within the class. Classes are stored in ceil(log(SBLOCK_WIDTH + 1)) bits.
Indices are stored in log(C(SBLOCK_WIDTH, index)) bits with enumerative code if its compressed size is less than MAX_CODE_SIZE.
Otherwise the bit pattern of the small block is explicitly stored as an index for the sake of efficiency.
This idea originally comes from [2]. For each large block, we store the number of 1s up to its beginning and a pointer for the index of the first small block.

[1] Gonzalo Navarro and Eliana Providel. 2012. Fast, small, simple rank/select on bitmaps. In Proceedings of the 11th international conference on Experimental Algorithms (SEA'12), Ralf Klasing (Ed.). Springer-Verlag, Berlin, Heidelberg, 295-306. DOI=http://dx.doi.org/10.1007/978-3-642-30850-5_26

[2] rsdic by Daisuke Okanohara. [https://github.com/hillbig/rsdic](https://github.com/hillbig/rsdic)

## Benchmark

10,000 operations on bit vectors of length (1,000,000 and 100,000,000) and of density (dense: 99%, normal: 50%, sparse: 1% 1s).

```
$ rustup nightly run cargo bench
running 12 tests
test rank_100000000_dense    ... bench:     752,410 ns/iter (+/- 39,871)
test rank_100000000_normal   ... bench:     865,107 ns/iter (+/- 34,210)
test rank_100000000_sparse   ... bench:     714,583 ns/iter (+/- 17,977)
test rank_1000000_dense      ... bench:     670,544 ns/iter (+/- 18,139)
test rank_1000000_normal     ... bench:     376,054 ns/iter (+/- 8,969)
test rank_1000000_sparse     ... bench:     635,294 ns/iter (+/- 15,752)
test select_100000000_dense  ... bench:   1,026,957 ns/iter (+/- 740,011)
test select_100000000_normal ... bench:   2,193,391 ns/iter (+/- 63,561)
test select_100000000_sparse ... bench:   1,971,993 ns/iter (+/- 60,703)
test select_1000000_dense    ... bench:     805,135 ns/iter (+/- 20,085)
test select_1000000_normal   ... bench:   1,456,985 ns/iter (+/- 33,205)
test select_1000000_sparse   ... bench:   1,791,824 ns/iter (+/- 44,174)
```
