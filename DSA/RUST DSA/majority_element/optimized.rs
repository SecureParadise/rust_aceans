// https://leetcode.com/problems/majority-element/

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut frequency = 1;
        for i in 1..n {
            if nums[i] == nums[i - 1] {
                frequency += 1;
            } else {
                frequency = 1;
            }
            if frequency > n / 2 {
                return nums[i];
            }
        }
        return nums[n / 2];
    }
}
