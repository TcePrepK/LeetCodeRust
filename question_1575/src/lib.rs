pub struct Solution {}

/// <p>You are given a rectangular cake of size <code>h x w</code> and two arrays of integers <code>horizontalCuts</code> and <code>verticalCuts</code> where:</p>
///
/// <ul>
/// 	<li><code>horizontalCuts[i]</code> is the distance from the top of the rectangular cake to the <code>i<sup>th</sup></code> horizontal cut and similarly, and</li>
/// 	<li><code>verticalCuts[j]</code> is the distance from the left of the rectangular cake to the <code>j<sup>th</sup></code> vertical cut.</li>
/// </ul>
///
/// <p>Return <em>the maximum area of a piece of cake after you cut at each horizontal and vertical position provided in the arrays</em> <code>horizontalCuts</code> <em>and</em> <code>verticalCuts</code>. Since the answer can be a large number, return this <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>
///
/// <p>&nbsp;</p>
/// <p><strong class="example">Example 1:</strong></p>
/// <img alt="" src="https://assets.leetcode.com/uploads/2020/05/14/leetcode_max_area_2.png" style="width: 225px; height: 240px;" />
/// <pre>
/// <strong>Input:</strong> h = 5, w = 4, horizontalCuts = [1,2,4], verticalCuts = [1,3]
/// <strong>Output:</strong> 4
/// <strong>Explanation:</strong> The figure above represents the given rectangular cake. Red lines are the horizontal and vertical cuts. After you cut the cake, the green piece of cake has the maximum area.
/// </pre>
///
/// <p><strong class="example">Example 2:</strong></p>
/// <img alt="" src="https://assets.leetcode.com/uploads/2020/05/14/leetcode_max_area_3.png" style="width: 225px; height: 240px;" />
/// <pre>
/// <strong>Input:</strong> h = 5, w = 4, horizontalCuts = [3,1], verticalCuts = [1]
/// <strong>Output:</strong> 6
/// <strong>Explanation:</strong> The figure above represents the given rectangular cake. Red lines are the horizontal and vertical cuts. After you cut the cake, the green and yellow pieces of cake have the maximum area.
/// </pre>
///
/// <p><strong class="example">Example 3:</strong></p>
///
/// <pre>
/// <strong>Input:</strong> h = 5, w = 4, horizontalCuts = [3], verticalCuts = [3]
/// <strong>Output:</strong> 9
/// </pre>
///
/// <p>&nbsp;</p>
/// <p><strong>Constraints:</strong></p>
///
/// <ul>
/// 	<li><code>2 &lt;= h, w &lt;= 10<sup>9</sup></code></li>
/// 	<li><code>1 &lt;= horizontalCuts.length &lt;= min(h - 1, 10<sup>5</sup>)</code></li>
/// 	<li><code>1 &lt;= verticalCuts.length &lt;= min(w - 1, 10<sup>5</sup>)</code></li>
/// 	<li><code>1 &lt;= horizontalCuts[i] &lt; h</code></li>
/// 	<li><code>1 &lt;= verticalCuts[i] &lt; w</code></li>
/// 	<li>All the elements in <code>horizontalCuts</code> are distinct.</li>
/// 	<li>All the elements in <code>verticalCuts</code> are distinct.</li>
/// </ul>
///

impl Solution {
    fn calculate_longest_side(length: i32, cuts: &Vec<i32>) -> u32 {
        let mut longest_side = 0;
        let mut remaining_length = length;
        let mut last_cut = 0;
        for cut in cuts {
            let side = cut - last_cut;
            if side > longest_side {
                longest_side = side;
            }

            if longest_side > remaining_length {
                break;
            }

            last_cut = *cut;
            remaining_length -= side;
        }
        if remaining_length > longest_side {
            longest_side = remaining_length;
        }

        longest_side as u32
    }

    pub fn max_area(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
        horizontal_cuts.sort();
        vertical_cuts.sort();

        let horizontal_longest_side = Self::calculate_longest_side(h, &horizontal_cuts);
        let vertical_longest_side = Self::calculate_longest_side(w, &vertical_cuts);

        (horizontal_longest_side as u64 * vertical_longest_side as u64 % 1_000_000_007) as i32
    }
}
