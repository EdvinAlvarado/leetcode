pub fn reverse(x: i32) -> i32 {
    let sol: i32 = x.abs()
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    if x < 0 {-sol} else {sol}
}
