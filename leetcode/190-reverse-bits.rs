impl Solution {
    pub fn reverse_bits(mut num: i32) -> i32 {
        let mut rev=0;
        let mut last_digit=0;
        for i in (0..32){
            last_digit=num&1;
            num=num>>1;
            rev=rev<<1;
            rev=rev|last_digit;

        }
        return rev
    }
}