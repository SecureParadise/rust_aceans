// https://leetcode.com/problems/majority-element/

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        for i in 0..n {
            let mut frequency: usize = 0;
            for j in 0..n {
                if nums[j] == nums[i] {
                    frequency += 1;
                }
            }
            if frequency > n / 2 {
                return nums[i];
            }
        }
        return -1;
    }
}
