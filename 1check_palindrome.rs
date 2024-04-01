fn is_palindrome(s: &str) -> bool {
    let mut i = 0;
    let mut j = s.len() - 1;
    let chars: Vec<char> = s.chars().collect();
    
    while i <= j {
        if chars[i] != chars[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}
