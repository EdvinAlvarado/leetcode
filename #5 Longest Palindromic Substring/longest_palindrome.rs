pub fn longest_palindrome(s: String) -> String {
    if s.len() == 1 {
        return s;
    }
    let mut longest: (String, usize) = ("".to_string(), 0);
    for i in 0..s.len() {
        for j in (i..=s.len()).rev() {
            if longest.1 >= j-i {break;}
            let sl = &s[i..j];
            if sl.chars().eq(sl.chars().clone().rev()) {
                longest = (sl.to_string().clone(), j-i);
            }
        }
    }
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

    let res5 = longest_palindrome("kyyrjtdplseovzwjkykrjwhxquwxsfsorjiumvxjhjmgeueafubtonhlerrgsgohfosqssmizcuqryqomsipovhhodpfyudtusjhonlqabhxfahfcjqxyckycstcqwxvicwkjeuboerkmjshfgiglceycmycadpnvoeaurqatesivajoqdilynbcihnidbizwkuaoegmytopzdmvvoewvhebqzskseeubnretjgnmyjwwgcooytfojeuzcuyhsznbcaiqpwcyusyyywqmmvqzvvceylnuwcbxybhqpvjumzomnabrjgcfaabqmiotlfojnyuolostmtacbwmwlqdfkbfikusuqtupdwdrjwqmuudbcvtpieiwteqbeyfyqejglmxofdjksqmzeugwvuniaxdrunyunnqpbnfbgqemvamaxuhjbyzqmhalrprhnindrkbopwbwsjeqrmyqipnqvjqzpjalqyfvaavyhytetllzupxjwozdfpmjhjlrnitnjgapzrakcqahaqetwllaaiadalmxgvpawqpgecojxfvcgxsbrldktufdrogkogbltcezflyctklpqrjymqzyzmtlssnavzcquytcskcnjzzrytsvawkavzboncxlhqfiofuohehaygxidxsofhmhzygklliovnwqbwwiiyarxtoihvjkdrzqsnmhdtdlpckuayhtfyirnhkrhbrwkdymjrjklonyggqnxhfvtkqxoicakzsxmgczpwhpkzcntkcwhkdkxvfnjbvjjoumczjyvdgkfukfuldolqnauvoyhoheoqvpwoisniv".to_string());
    println!("{}", res5);
}
