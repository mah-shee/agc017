#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n],
    }
    let mut count_even = 0;
    for i in a.iter() {
        if i % 2 == 0 {
            count_even += 1;
        }
    }
    if count_even == n && p == 0 {
        println!("{}", 2usize.pow(n as u32));
    } else if count_even == n && p == 1 {
        println!("0");
    } else {
        println!("{}", 2usize.pow((n - 1) as u32));
    }
}
