pub fn longest_palindrome(s: String) -> String {
    if s.len() == 1 {
        return s;
    }
    let mut l: usize = 0;
    let mut r: usize = 0;
    let mut longest: (String, usize) = ("".to_string(), 0);
    for i in 0..s.len() {
        if longest.1 >= s.len()/2 + 1 {break;}

        // Even starter 2-letter palindrome
        if let Some(left) = &s.get(i..i+1) {
            while let Some(right) = &s.get(i+r+1..i+r+2) {
                if left == right {
                    r += 1;
                } else {
                    break;
                }
            }
        }

        if i as i32 - l as i32 - 1 >= 0 {
            // after the starter even check, all checks are just the same as odds
            // Check expanding odd palindrome
            let left = &s.get(i-l-1..i-l);
            let right = &s.get(i+r..i+r+1);
            while left.is_some() && right.is_some() {
                if left.unwrap() == right.unwrap() {
                    r += 1;
                    l += 1;
                } else {break;}
                // println!("\t{} {} {}", i, i-l, i+r);
                if i as i32 - l as i32 - 1 < 0 || i+r >= s.len() {break;}
                let left = &s.get(i-l-1..i-l);
                let right = &s.get(i+r..i+r+1);
            }
        }

        print!("{}\t{:?} {}\t{:?} {}\t{:?}", s, s.get(i-l..i-l+1), i-l, s.get(i+r-1..i+r), i+r, s.get(i-l..i+r));
        if longest.1 < r+l {
            longest = (s.get(i-l..i+r).expect("final slice is wroing").to_string(), r+l);
        }
        println!("\t{:?}", longest);
        l = 0;
        r = 1;
    }
    println!("");
    longest.0
}

fn main() {
    let res1 = longest_palindrome("babad".to_string());
    assert_eq!("bab".to_string(), res1);

    let res2 = longest_palindrome("cbbd".to_string());
    assert_eq!("bb".to_string(), res2);

    let res3 = longest_palindrome("a".to_string());
    assert_eq!("a".to_string(), res3);

    let res4 = longest_palindrome("ac".to_string());
    assert_eq!("a".to_string(), res4);

    let res5 = longest_palindrome("ccc".to_string());
    assert_eq!("ccc".to_string(), res5);

    let res6 = longest_palindrome("caba".to_string());
    assert_eq!("aba".to_string(), res6);

    let res7 = longest_palindrome("abcbe".to_string());
    assert_eq!("bcb".to_string(), res7);
}
