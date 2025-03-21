pub struct Solution {}

/// <p>You are given an integer array <code>nums</code>. In one move, you can pick an index <code>i</code> where <code>0 &lt;= i &lt; nums.length</code> and increment <code>nums[i]</code> by <code>1</code>.</p>
///
/// <p>Return <em>the minimum number of moves to make every value in </em><code>nums</code><em> <strong>unique</strong></em>.</p>
///
/// <p>The test cases are generated so that the answer fits in a 32-bit integer.</p>
///
/// <p>&nbsp;</p>
/// <p><strong class="example">Example 1:</strong></p>
///
/// <pre>
/// <strong>Input:</strong> nums = [1,2,2]
/// <strong>Output:</strong> 1
/// <strong>Explanation:</strong> After 1 move, the array could be [1, 2, 3].
/// </pre>
///
/// <p><strong class="example">Example 2:</strong></p>
///
/// <pre>
/// <strong>Input:</strong> nums = [3,2,1,2,1,7]
/// <strong>Output:</strong> 6
/// <strong>Explanation:</strong> After 6 moves, the array could be [3, 4, 1, 2, 5, 7].
/// It can be shown that it is impossible for the array to have all unique values with 5 or less moves.
/// </pre>
///
/// <p>&nbsp;</p>
/// <p><strong>Constraints:</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
/// 	<li><code>0 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
/// </ul>
///

impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut highest_num = -1;
        let mut total_inc = 0;
        for i in 0..nums.len() {
            let mut num = nums[i];
            if num > highest_num {
                highest_num = num;
                continue;
            }

            // Index already exists
            total_inc += highest_num - num + 1;
            highest_num += 1;
        }

        total_inc
    }
}
