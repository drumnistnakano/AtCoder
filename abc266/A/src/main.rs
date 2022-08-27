use proconio::input;

fn main() {
    input! {
        n: String,
    }
    let num: usize = ((n.chars().count())+1)/2;
    let num_v: Vec<char> = n.chars().collect();
    println!("{}", num_v[num-1]);
}
