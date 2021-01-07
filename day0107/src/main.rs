use std::io;
fn main() {
    println!("请输入数字");
    let mut number_ = String::new();
    io::stdin().read_line(&mut number_).expect("无法读取行");
    println!("数字是:{}",number_);
}
