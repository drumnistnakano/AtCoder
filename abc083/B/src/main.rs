use proconio::input;

fn caluculateSum(i: usize) -> usize {
    let mut sum = 0;
    let mut _i = i;
    while _i > 0 {
        sum += _i%10;
        _i /= 10;
    }
    return sum;
}

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let mut total = 0;
    for i in 1..n+1 {
        let sum = caluculateSum(i);
        if a <= sum && sum <= b {
            println!("{}", i);
            total += i;
        }
    }

    println!("{}", total);
}
