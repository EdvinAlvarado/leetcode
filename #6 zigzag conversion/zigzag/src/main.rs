mod zigzag;
// use crate::zigzag;

fn main() {
    // let s =  zigzag::convert("abcdefghijklmnopqrstuvwxyz".to_string(), 4);
    let s =  zigzag::convert("PAYPALISHIRING".to_string(), 4);
    println!("Hello, world!\n{}",s);
}
