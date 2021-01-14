use std::fs::OpenOptions;
fn main() {
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/home/keeyu/.water")
        .unwrap();
    let mut buf: [u8];
    fd.read(&mut buf);
}
