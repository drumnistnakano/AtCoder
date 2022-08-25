use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
    }

    println!("{}", arr[0]);
    for m in arr.iter() {
        println!("{:?}", m);
    }
}
