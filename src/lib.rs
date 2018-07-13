//! FID (Fully Indexable Dictionary) implementation for Rust
//! 
//! This crate provides a succinct bit vector that supports two kinds of bit operations in constant-time:
//! 
//! - `rank(i)` computes the number of 0s (or 1s) in [0..i)
//! - `select(r)` locates the (r+1)-th position of 0 (or 1).
//! 
//! Structures supporting these two operations are called FID (fully indexable dictionary).
//! 
//! # Basic usage
//! 
//! ```
//! use fid::{BitVector, FID};
//! 
//! let mut bv = BitVector::new();
//! // 01101101
//! bv.push(false); bv.push(true); bv.push(true); bv.push(false);
//! bv.push(true); bv.push(true); bv.push(false); bv.push(true);
//! 
//! assert_eq!(bv.rank0(5), 2);
//! assert_eq!(bv.rank1(5), 3);
//! assert_eq!(bv.select0(2), 6);
//! assert_eq!(bv.select1(2), 4);
//! ```
//! 
//! # About implementation
//! 
//! Compression and computation algorithms for `BitVector` are originally from [1] and its practical implementation ideas are from [2].
//! 
//! [1] Gonzalo Navarro and Eliana Providel. 2012. Fast, small, simple rank/select on bitmaps. In Proceedings of the 11th international conference on Experimental Algorithms (SEA'12), Ralf Klasing (Ed.). Springer-Verlag, Berlin, Heidelberg, 295-306. DOI=http://dx.doi.org/10.1007/978-3-642-30850-5_26
//!
//! [2] rsdic by Daisuke Okanohara. [https://github.com/hillbig/rsdic](https://github.com/hillbig/rsdic)

pub mod bit_vector;
pub mod bit_array;
pub mod fid;

pub use bit_array::BitArray;
pub use bit_vector::BitVector;
pub use fid::FID;