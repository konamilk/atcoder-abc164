use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use smallvec::alloc::collections::BTreeSet;

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        n: usize,
        s: [String; n]
    };

    let mut bts = BTreeSet::<String>::new();

    for si in s {
        bts.insert(si);
    }

    println!("{}", bts.len())
}
