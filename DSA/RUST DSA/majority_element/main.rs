// https://leetcode.com/problems/majority-element/

// Boyer-Moore Majority Vote Algorithm
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut element = nums[0];
        let mut frequency = 1;
        for &num in nums.iter().skip(1) {
            if frequency == 0 {
                element = num;
                frequency = 1;
            } else if num == element {
                frequency += 1;
            } else {
                frequency -= 1;
            }
        }
        element
    }
}
fn main(){
    let nums = [2,2,1,1,1,2,2];
}