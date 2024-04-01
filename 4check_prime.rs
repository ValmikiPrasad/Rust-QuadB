fn check_prime(n: u32) -> bool {
    let mut flag = false;
    if n == 1 {
        flag = true;
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            flag = true;
            break;
        }
        i += 1;
    }
    !flag
}
