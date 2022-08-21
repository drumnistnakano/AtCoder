use proconio::input;

fn main() {
    input! {
        x: [i32; 5],
    }
    let index = x.iter().position(|&m| m == 0).unwrap();
    println!("{:?}", index + 1);
}
