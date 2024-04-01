fn find_median_sorted_array(nums: &Vec<i32>) -> f64 {
    let n = nums.len();
    if n == 0 {
        return 0.0;
    }

    if n % 2 == 0 { // If even number of elements
        return (nums[n/2 - 1] + nums[n/2]) as f64 / 2.0;
    } else { // If odd number of elements
        return nums[n/2] as f64;
    }
}