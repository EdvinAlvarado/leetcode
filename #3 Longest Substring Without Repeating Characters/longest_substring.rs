
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut top = 0;
    for (i,c) in s.chars().enumerate() {
        let mut substr: String = c.to_string();
        let mut temp = 1;
        for cc in s.chars().skip(i+1) {
            if substr.contains(cc) {
                break;
            }
            else {
                temp += 1;
                substr.push(cc);
            }
        }
        top = top.max(temp);
    }
    top
}


fn main() {
    println!("{}", length_of_longest_substring("abcabcbb".to_string()))
}
