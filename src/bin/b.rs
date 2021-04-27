use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        mut a: i64,
        b: i64,
        mut c: i64,
        d: i64,
    };

    loop {
        c = c - b;
        if c <= 0 {
            println!("Yes");
            return
        }
        a = a - d;
        if a <= 0 {
            println!("No");
            return

        }

    }
}
