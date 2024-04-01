fn first_occurrence(arr: &[i32], x: i32) -> i32 {
    let mut ans = -1;
    let mut s = 0;
    let mut e = arr.len() as i32 - 1;

    while s <= e {
        let mid = s + (e - s) / 2;
        if arr[mid as usize] == x {
            ans = mid;
            e = mid - 1;
        } else if arr[mid as usize] < x {
            s = mid + 1;
        } else {
            e = mid - 1;
        }
    }
    ans
}