fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut ans = String::new();
    if strs.is_empty() {
        return ans;
    }
    let first_str = &strs[0];
    for (i, ch) in first_str.chars().enumerate() {
        let mut match_found = true;
        for str in strs.iter().skip(1) {
            if i >= str.len() || ch != str.chars().nth(i).unwrap() {
                match_found = false;
                break;
            }
        }
        if !match_found {
            break;
        } else {
            ans.push(ch);
        }
    }
    ans
}