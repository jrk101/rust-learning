impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut flag=1;
        let mut rev=0;
        if x<0{
            flag=0;
        }
        else{
            let mut num=x;
            let mut digit;
            while num!=0{
                digit = num%10;
                rev=rev*10+digit;
                num=num/10;
            }
        }
        if flag==1 && x==rev{
            return true
        }
        else{
            return false
        }
    }
}