use proconio::{
    input,
    marker::{Bytes, Chars},
};

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }
    let mut res = 0;
    let mut exist_odd = false;

    loop {
        for n in &a {
            if n % 2 != 0 {
                exist_odd = true;
            }
        }

        if exist_odd {
            break;
        }

        for x in &mut a {
            *x /= 2
        }

        res += 1;
    }
    println!("{}", res);
}
