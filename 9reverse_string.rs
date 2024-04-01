fn reverse_str(str: &mut String) {
    let n = str.len();
    let mut i = 0;
    let mut j = n - 1;
    while i <= j {
        str.as_bytes_mut().swap(i, j);
        i += 1;
        j -= 1;
    }
}