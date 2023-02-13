impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len(){
            return false;
        }
        else{
            use std::collections::HashMap;
            let mut hs_s:HashMap<char,i32> = HashMap::new();
            let mut hs_t:HashMap<char,i32> = HashMap::new();
            let mut sv: Vec<char> = s.chars().collect();
            let mut tv: Vec<char> = t.chars().collect();

            for i in 0..s.len(){
                hs_s.insert(sv[i], 1 + if hs_s.contains_key(&sv[i]) { hs_s[&sv[i]] } else { 0 });
                hs_t.insert(tv[i], 1 + if hs_t.contains_key(&tv[i]) { hs_t[&tv[i]] } else { 0 });
            }
            for j in hs_s.keys(){
                if hs_s.get(&j) != hs_t.get(&j){
                    return false;
                }
            }
            return true;
        }
    }
}
