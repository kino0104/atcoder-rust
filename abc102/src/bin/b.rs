use proconio::{
    input,
    marker::{Bytes, Chars},
};

fn main() {
    input! {
        n: i32,
        a: [i64; n],
    }

    let mut min_v: i64 = 1000000001;
    let mut max_v: i64 = 0;

    for i in a {
        if i > max_v {
            max_v = i;
        }
        if i < min_v {
            min_v = i;
        }
    }
    println!("{}", max_v - min_v);
}
