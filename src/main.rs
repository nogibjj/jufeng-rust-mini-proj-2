use std::collections::HashMap;

fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut pre_sum = Vec::new();
    pre_sum.push(0);
    for num in nums {
        pre_sum.push(pre_sum.last().unwrap() + num);
    }

    let mut result = 0;
    let mut map = HashMap::new();
    for s in pre_sum {
        if map.contains_key(&(s - k)) {
            result += map[&(s - k)];
        }
        map.entry(s).and_modify(|v| *v += 1).or_insert(1);
    }
    result
}



fn main() {
    let nums = vec![1, 2, 3];
    let k = 3;
    println!("{}", subarray_sum(nums, k));
}
