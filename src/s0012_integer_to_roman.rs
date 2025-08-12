struct Solution {}
impl Solution {
    const MAP: [(i32, &str); 13] = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
    ];

    pub fn int_to_roman(mut num: i32) -> String {
        let mut result = String::with_capacity(15);
        for (k, v) in &Self::MAP {
            if (num as f32) / (*k as f32) >= 1.0 {
                for i in 0..(num/k) {
                    result.push_str(v);
                }
                num = num % k;
                if num == 0 {
                    break;
                }
            }
        }
        
        return result;
    }
}