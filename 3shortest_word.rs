fn shortest_word_length(s: &str) -> usize {
    let mut ans = usize::MAX;
    let mut word = String::new();
    
    for c in s.chars() {
        if c == ' ' && !word.is_empty() {
            ans = ans.min(word.len());
            word.clear();
        } else {
            word.push(c);
        }
    }
    
    if !word.is_empty() {
        ans = ans.min(word.len());
    }
    
    ans
}
