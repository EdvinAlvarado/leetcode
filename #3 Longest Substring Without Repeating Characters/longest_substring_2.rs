use std::collections::HashMap;


pub fn length_of_longest_substring(s: String) -> i32 {
    let mut top = 0;
    let mut i: usize = 0;
    let mut map = HashMap::new();
    for j in 0..s.len() {
        match map.get(&s.chars().nth(j).unwrap()) {
            Some(match_j) => {i = i.max(match_j + 1);},
            None => {},
        }
        map.insert(s.chars().nth(j).unwrap(), j);
        top = top.max(j-i+1);
    }
    top as i32
}


fn main() {
    println!("{}", length_of_longest_substring("abcabcbb".to_string()))
}
