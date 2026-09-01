impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let len=digits.len();
        for i in (0..len).rev(){
            digits[i]+=1;
            if digits[i]<10{
                break;
            }
            else{
                if i==0{
                    digits[i]=0;
                    digits.insert(0,1);
                    break;
                }
                else{
                    digits[i]=0;                }
            }
        }
        digits
    }
}