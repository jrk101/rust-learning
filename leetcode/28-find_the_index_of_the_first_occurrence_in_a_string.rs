impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack_char:Vec<char> = haystack.chars().collect();
        let needle_char:Vec<char> = needle.chars().collect();
        let mut occurrence=-1;
        let h_len=haystack_char.len();
        let n_len=needle_char.len();
        if h_len>=n_len{
            for c in 0..=h_len-n_len{
                let mut found=true;
                for i in 0..n_len{
                    if haystack_char[c+i]==needle_char[i]{
                        continue;
                    }
                    else{
                        found=false;
                        break;
                    }
                    
                }
                if found{
                    occurrence=c as i32;
                    break;
                }


            }
        }
        return occurrence
    }
}