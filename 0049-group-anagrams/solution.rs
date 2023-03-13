impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        strs.into_iter().fold(HashMap::new(),|mut accumulator, word|{
            let val = word.chars().fold([0u8;26],|mut accumulator, c|{
                accumulator[c as usize - 'a' as usize] +=1;
                accumulator
            });
            accumulator.entry(val).or_insert(vec![]).push(word);
            accumulator
        }).into_values().collect()
    }
}
