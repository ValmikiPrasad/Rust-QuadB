use std::collections::BinaryHeap;

fn find_kth_largest(nums: Vec<i32>, k: usize) -> i32 {
    let mut pq = BinaryHeap::new();

    for num in nums {
        pq.push(num);
    }

    for _ in 0..k - 1 {
        pq.pop();
    }

    pq.peek().cloned().unwrap()
}