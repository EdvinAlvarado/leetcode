pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {return false;}
    let sol: i32 = x.abs()
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    x == sol
}
