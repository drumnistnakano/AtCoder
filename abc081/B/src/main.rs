use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [i32; n],
    }

    let mut counter = 0;
    let mut _arr = arr;

    'countup: loop {
        for m in _arr.iter() {
            if m % 2 != 0 {
                break 'countup;
            }
        }
        counter += 1;
        _arr = _arr.iter().map(|x| x/2).collect();
    }

    println!("{}", counter);
}
