extern crate fid;
extern crate rand;

use fid::BitVector;
use rand::{Rng, SeedableRng, StdRng};

fn generate_random_vector(n: usize, p: f64) -> BitVector {
    let mut rng: StdRng = SeedableRng::from_seed([0; 32]);
    let mut bv = BitVector::new();
    for _ in 0..n {
        let b = rng.gen_bool(p);
        bv.push(b);
    }
    bv
}

fn main() {
    let test_cases = &[
        (1000000, 0.99),
        (1000000, 0.5),
        (1000000, 0.01),
        (100000000, 0.99),
        (100000000, 0.5),
        (100000000, 0.01),
    ];

    println!("n: # of nodes, p: density of 1s\n");

    for &(n, p) in test_cases {
        let bv = generate_random_vector(n, p);
        let size = bv.size();
        let rate = (size * 8) as f64 / n as f64;
        println!(
            "n = {}, p = {}: {} bytes ({} bit / orig bit)",
            n, p, size, rate
        );
    }
}
