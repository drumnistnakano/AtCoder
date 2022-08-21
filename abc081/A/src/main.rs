use proconio::input;

fn main(){
    input! {
        n: String
    }

    println!("{}", n.chars().filter(|&x| x == '1').count());
}
