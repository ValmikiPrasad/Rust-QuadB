fn merge(nums1: Vec<i32>, m: i32, nums2: Vec<i32>, n: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut ans = Vec::new();
    
    while i < m && j < n {
        if nums1[i as usize] < nums2[j as usize] {
            ans.push(nums1[i as usize]);
            i += 1;
        } else {
            ans.push(nums2[j as usize]);
            j += 1;
        }
    }
    
    while i < m {
        ans.push(nums1[i as usize]);
        i += 1;
    }
    
    while j < n {
        ans.push(nums2[j as usize]);
        j += 1;
    }
    
    ans
}
