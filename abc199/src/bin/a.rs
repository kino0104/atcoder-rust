use proconio::{
    input,
    marker::{Bytes, Chars},
};

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }

    if a * a + b * b < c * c {
        println!("Yes");
    } else {
        println!("No");
    }
}
