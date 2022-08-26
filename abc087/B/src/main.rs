use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize
    }

    let mut counter: i32 = 0;

    for i in 0..a+1 {
        for j in 0..b+1 {
            for k in 0..c+1 {
                if (500*i + 100*j+ 50*k) == x {
                    counter += 1;
                }
            }
        }
    }

    println!("{}", counter);
}
