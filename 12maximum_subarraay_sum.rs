
fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut maxi = i32::MIN;
    
    for &num in nums.iter() {
        sum += num;
        maxi = cmp::max(maxi, sum);
        
        if sum < 0 {
            sum = 0;
        }
    }
    
    maxi
}