pub fn longest_palindrome(s: String) -> String {
    if s.len() == 1 {
        return s;
    }
    
    for win_size in (1..=s.len()).rev() {
        match s.as_bytes()
                .windows(win_size)
                .find(|sl| {
                    sl.iter().eq(sl.clone().iter().rev())
                }) {
            Some(sl) => return std::str::from_utf8(sl).unwrap_or("").to_string(),
            None => (),
        }
    }
    "".to_string()
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

    let res5 = longest_palindrome("kyyrjtdplseovzwjkykrjwhxquwxsfsorjiumvxjhjmgeueafubtonhlerrgsgohfosqssmizcuqryqomsipovhhodpfyudtusjhonlqabhxfahfcjqxyckycstcqwxvicwkjeuboerkmjshfgiglceycmycadpnvoeaurqatesivajoqdilynbcihnidbizwkuaoegmytopzdmvvoewvhebqzskseeubnretjgnmyjwwgcooytfojeuzcuyhsznbcaiqpwcyusyyywqmmvqzvvceylnuwcbxybhqpvjumzomnabrjgcfaabqmiotlfojnyuolostmtacbwmwlqdfkbfikusuqtupdwdrjwqmuudbcvtpieiwteqbeyfyqejglmxofdjksqmzeugwvuniaxdrunyunnqpbnfbgqemvamaxuhjbyzqmhalrprhnindrkbopwbwsjeqrmyqipnqvjqzpjalqyfvaavyhytetllzupxjwozdfpmjhjlrnitnjgapzrakcqahaqetwllaaiadalmxgvpawqpgecojxfvcgxsbrldktufdrogkogbltcezflyctklpqrjymqzyzmtlssnavzcquytcskcnjzzrytsvawkavzboncxlhqfiofuohehaygxidxsofhmhzygklliovnwqbwwiiyarxtoihvjkdrzqsnmhdtdlpckuayhtfyirnhkrhbrwkdymjrjklonyggqnxhfvtkqxoicakzsxmgczpwhpkzcntkcwhkdkxvfnjbvjjoumczjyvdgkfukfuldolqnauvoyhoheoqvpwoisniv".to_string());
    println!("{}", res5);
}
