use proconio::input;

fn main() {
    input! {
        n: isize,
    }
    let mut num = 0;

    for x in 0..998244354 {
        if (n - x) % 998244353 == 0 {
            num = x;
            break;
        }
    }

    println!("{}", num);
}
