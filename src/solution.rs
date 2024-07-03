impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        if nums.len() <= 4 {
            return 0;
        }

        nums.sort();

        let mut ans = nums[nums.len() - 1] - nums[0];

        for i in 0..=3 {
            ans = ans.min(nums[nums.len() - 4 + i] - nums[i]);
        }

        return ans;
    }
}

pub struct Solution {}