pub struct Solution {}

/// <p>You are given an integer array&nbsp;<code>nums</code>. You can choose <strong>exactly one</strong> index (<strong>0-indexed</strong>) and remove the element. Notice that the index of the elements may change after the removal.</p>
///
/// <p>For example, if <code>nums = [6,1,7,4,1]</code>:</p>
///
/// <ul>
/// 	<li>Choosing to remove index <code>1</code> results in <code>nums = [6,7,4,1]</code>.</li>
/// 	<li>Choosing to remove index <code>2</code> results in <code>nums = [6,1,4,1]</code>.</li>
/// 	<li>Choosing to remove index <code>4</code> results in <code>nums = [6,1,7,4]</code>.</li>
/// </ul>
///
/// <p>An array is <strong>fair</strong> if the sum of the odd-indexed values equals the sum of the even-indexed values.</p>
///
/// <p>Return the <em><strong>number</strong> of indices that you could choose such that after the removal, </em><code>nums</code><em> </em><em>is <strong>fair</strong>. </em></p>
///
/// <p>&nbsp;</p>
/// <p><strong class="example">Example 1:</strong></p>
///
/// <pre>
/// <strong>Input:</strong> nums = [2,1,6,4]
/// <strong>Output:</strong> 1
/// <strong>Explanation:</strong>
/// Remove index 0: [1,6,4] -&gt; Even sum: 1 + 4 = 5. Odd sum: 6. Not fair.
/// Remove index 1: [2,6,4] -&gt; Even sum: 2 + 4 = 6. Odd sum: 6. Fair.
/// Remove index 2: [2,1,4] -&gt; Even sum: 2 + 4 = 6. Odd sum: 1. Not fair.
/// Remove index 3: [2,1,6] -&gt; Even sum: 2 + 6 = 8. Odd sum: 1. Not fair.
/// There is 1 index that you can remove to make nums fair.
/// </pre>
///
/// <p><strong class="example">Example 2:</strong></p>
///
/// <pre>
/// <strong>Input:</strong> nums = [1,1,1]
/// <strong>Output:</strong> 3
/// <strong>Explanation:</strong>&nbsp;You can remove any index and the remaining array is fair.
/// </pre>
///
/// <p><strong class="example">Example 3:</strong></p>
///
/// <pre>
/// <strong>Input:</strong> nums = [1,2,3]
/// <strong>Output:</strong> 0
/// <strong>Explanation:</strong>&nbsp;You cannot make a fair array after removing any index.
/// </pre>
///
/// <p>&nbsp;</p>
/// <p><strong>Constraints:</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
/// 	<li><code>1 &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
/// </ul>
///

impl Solution {
    fn is_fair(
        removed_index: usize,
        cumulative_even: &Vec<i32>,
        cumulative_odd: &Vec<i32>,
    ) -> bool {
        if cumulative_odd.len() == 0 {
            return true;
        }

        let prev_even = if removed_index > 0 {
            cumulative_even[(removed_index - 1) >> 1] // (i - 1) / 2
        } else {
            0
        };

        let prev_odd = if removed_index > 1 {
            cumulative_odd[(removed_index >> 1) - 1] // i / 2 - 1
        } else {
            0
        };

        let current_even = cumulative_even[removed_index >> 1];
        let current_odd = if removed_index > 0 {
            cumulative_odd[(removed_index - 1) >> 1]
        } else {
            0
        };

        let last_even = cumulative_even.last().unwrap();
        let last_odd = cumulative_odd.last().unwrap();

        let cumulated_even = prev_even + last_odd - current_odd;
        let cumulated_odd = prev_odd + last_even - current_even;

        cumulated_even == cumulated_odd
    }

    /// (Even, Odd)
    fn get_cumulative_sum(nums: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        let mut cumulative_even = vec![0; (nums.len() + 1) >> 1];
        let mut cumulative_odd = vec![0; nums.len() >> 1];
        let mut index = 1;

        cumulative_even[0] = nums[0];
        if nums.len() == 1 {
            return (cumulative_even, cumulative_odd);
        }

        cumulative_odd[0] = nums[1];
        for i in 2..nums.len() {
            if i % 2 == 0 {
                cumulative_even[index] = cumulative_even[index - 1] + nums[i];
            } else {
                cumulative_odd[index] = cumulative_odd[index - 1] + nums[i];
                index += 1;
            }
        }

        (cumulative_even, cumulative_odd)
    }

    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let (cumulative_even, cumulative_odd) = Self::get_cumulative_sum(&nums);

        let mut total = 0;
        for i in 0..nums.len() {
            if Self::is_fair(i, &cumulative_even, &cumulative_odd) {
                total += 1;
            }
        }

        total
    }
}
