impl Solution {
    //aqui debo tomar el string que me llega descomponerlo, comprar y asignar los valores
    pub fn roman_to_int(s: String) -> i32 {
        s.chars().fold((0,0), |(sum,prev),c|{
            let val = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };
            (sum + if prev < val { val - prev * 2 } else { val }, val)
        }).0
    }
}
