impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result:Vec<i32> = Vec::new();
        for (i,element) in nums.iter().enumerate(){
            for (j,second_element) in nums.iter().skip(i+1).enumerate(){
                if element+second_element == target{
                    let i:i32=i as i32;
                    let j:i32=j as i32;
                    result= vec![i,j+i+1];
                    break;
                }
            }
        }
        return result
    }
}