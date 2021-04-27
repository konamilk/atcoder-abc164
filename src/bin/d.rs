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
    let source = AutoSource::from("1817181712114");
    input!{
        // from source,
        s: Chars
    };

    let mut cum_mod:Vec<i32> = vec![];
    let mut fac_mod = 1;
    cum_mod.push(0);

    let mut val= [0; 2020];
    val[0] = 1;

    for (i, &add) in s.iter().rev().enumerate(){
        let a = add as i32 -48;
        let cum = (cum_mod[i] + a * fac_mod) % 2019;
        cum_mod.push(cum);
        val[cum as usize] += 1;
        fac_mod = (10 * fac_mod) % 2019;
    }

    let mut ans = 0;

    for i in 0..val.len() {
        ans += val[i] * (val[i] - 1) / 2;
    }
    println!("{}", ans);
}
