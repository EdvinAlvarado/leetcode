mod reverse_integer;
use crate::reverse_integer::reverse;

fn main() {
    println!("{}", reverse(123));
    println!("{}", reverse(-123));
    println!("{}", reverse(120));
    println!("{}", reverse(1534236469));
}
