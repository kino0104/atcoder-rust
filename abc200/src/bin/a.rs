use proconio::{
    input,
    marker::{Bytes, Chars},
};

fn main() {
    input! {
        n: i32,
    }

    let ans = if n % 100 == 0 { n / 100 } else { n / 100 + 1 };
    println!("{}", ans);
}
