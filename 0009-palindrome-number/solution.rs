impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x == 0 {
            return true
        }
        if x < 0 || x % 10 == 0 {
            return false;
        }
        let mut hi: i32 = 10i32.pow(x.ilog10());
        let mut lo = 10;
        while hi >= lo {
            if (x / hi) % 10 != (x % lo) / (lo / 10) {
                return false;
            }
            hi /= 10;
            lo *= 10;
        }
        true
    }
}
