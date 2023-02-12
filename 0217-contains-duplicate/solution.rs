impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut hashset: HashSet<i32> = HashSet::new();

        for &n in nums.iter(){
            if hashset.contains(&n){
                return true;
            } 
            hashset.insert(n);
        };
        false
    }
}
