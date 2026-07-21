impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let length = nums.len();
        let mut j=0;
        for i in 0..length{
            if nums[j]==0&&nums[i]!=0{
                nums.swap(i,j);
                j+=1;
            }
            if nums[j]!=0 && nums[i]!=0{
                j+=1;
            }
        }
    }
}