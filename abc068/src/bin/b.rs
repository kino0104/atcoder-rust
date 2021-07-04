use proconio::{
    input,
    marker::{Bytes, Chars},
};

fn main() {
    input! {
        n: i32,
    }

    let res = if n >= 64 {
        64
    } else if n >= 32 {
        32
    } else if n >= 16 {
        16
    } else if n >= 8 {
        8
    } else if n >= 4 {
        4
    } else if n >= 2 {
        2
    } else {
        1
    };
    println!("{}", res);
}
