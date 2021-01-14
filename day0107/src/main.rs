use std::io;
use rand::Rng;
fn main() {
    println!("请输入数字");
    let mut number_ = String::new();
    io::stdin().read_line(&mut number_).expect("无法读取行");

    let rand_number = rand::thread_rng().gen_range(1,101);
    println!("数字是:{}{}",number_,rand_number);
}
