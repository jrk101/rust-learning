impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut map:HashMap<i32,i32> = HashMap::new();
        let vec_len = nums.len() as i32;
        let last_index=nums.len()-1;
        if k == vec_len{
            *nums.iter().max().unwrap_or(&-1)
        }
        else{
            for element in nums.iter(){
                let count=map.entry(*element).or_insert(0);
                *count+=1;
            }
            if k==1{
                let mut x=-1;
                for (key,value) in map.iter(){
                    if *value== 1{
                        x=x.max(*key)
                    }
                }
                return x;
            }
            let first_element_value=map.get(&nums[0]).unwrap();
            let last_element_value=map.get(&nums[nums.len()-1]).unwrap();
            if (*first_element_value == 1){
                if (*last_element_value == 1){
                    std::cmp::max(nums[0],nums[last_index])
                }
                else {
                    nums[0]
                }
            }
            else if (*last_element_value == 1) & (*first_element_value != 1){
                nums[last_index]
            }
            else{
                -1
            }
        }
    }
}