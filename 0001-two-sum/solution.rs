impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hash_num: HashMap<i32,i32> = HashMap::new();
        for (k,v) in nums.into_iter().enumerate(){
            if hash_num.contains_key(&v){return vec![hash_num.get(&v).unwrap().clone(), k as i32];}
            hash_num.insert(target - v, k as i32);
        }
        Vec::new()
    }
}
