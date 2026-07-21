impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;
        let mut roman:HashMap<char,i32>=HashMap::new();
        roman.insert('I',1);
        roman.insert('V',5);
        roman.insert('X',10);
        roman.insert('L',50);
        roman.insert('C',100);
        roman.insert('D',500);
        roman.insert('M',1000);
        let length=s.chars().count();
        let mut result:i32=0;
        let letters:Vec<char>=s.chars().collect();
        for i in 0..length-1{
            if roman.get(&letters[i]).unwrap()<roman.get(&letters[i+1]).unwrap(){
                result-=roman.get(&letters[i]).unwrap();
            }
            else{
                result+=roman.get(&letters[i]).unwrap();
            }
        }
        result+=roman.get(&letters[length-1]).unwrap();
        return result
        
    }
}