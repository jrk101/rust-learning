impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let space =' ';
        let mut space_count=0;
        let mut length=0;
        let mut space = true;
        let s_char:Vec<char> = s.chars().collect();
        for (i,ch) in s_char.iter().rev().enumerate(){
            if *ch==' ' && space{
                continue;
            }
            else{
                if *ch!=' '{
                    space=false;
                    length+=1;
                }
                else{
                    break;
                }
            }
        }
        length
    }
}