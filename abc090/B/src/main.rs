use proconio::input;

fn reverseNum(num: usize) -> usize {
    let mut str_num: String = "".to_string();
    let mut _num = num;
    while _num > 0 {
        let remainder = _num % 10;
        str_num.push_str(&remainder.to_string());
        _num /= 10;
    }
    return str_num.parse().unwrap();
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut counter = 0;
    for num in a..b+1 {
        let r_num = reverseNum(num);
        if num == r_num  {
            counter += 1;
        }
    }
    println!("{}", counter);
}
