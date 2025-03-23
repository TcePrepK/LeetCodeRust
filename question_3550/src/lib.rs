pub struct Solution {}

/// <p>You are given a <code>m x n</code> 2D array <code>board</code> representing a chessboard, where <code>board[i][j]</code> represents the <strong>value</strong> of the cell <code>(i, j)</code>.</p>
///
/// <p>Rooks in the <strong>same</strong> row or column <strong>attack</strong> each other. You need to place <em>three</em> rooks on the chessboard such that the rooks <strong>do not</strong> <strong>attack</strong> each other.</p>
///
/// <p>Return the <strong>maximum</strong> sum of the cell <strong>values</strong> on which the rooks are placed.</p>
///
/// <p>&nbsp;</p>
/// <p><strong class="example">Example 1:</strong></p>
///
/// <div class="example-block">
/// <p><strong>Input:</strong> <span class="example-io">board = </span>[[-3,1,1,1],[-3,1,-3,1],[-3,2,1,1]]</p>
///
/// <p><strong>Output:</strong> 4</p>
///
/// <p><strong>Explanation:</strong></p>
///
/// <p><img alt="" src="https://assets.leetcode.com/uploads/2024/08/08/rooks2.png" style="width: 294px; height: 450px;" /></p>
///
/// <p>We can place the rooks in the cells <code>(0, 2)</code>, <code>(1, 3)</code>, and <code>(2, 1)</code> for a sum of <code>1 + 1 + 2 = 4</code>.</p>
/// </div>
///
/// <p><strong class="example">Example 2:</strong></p>
///
/// <div class="example-block">
/// <p><strong>Input:</strong> <span class="example-io">board = [[1,2,3],[4,5,6],[7,8,9]]</span></p>
///
/// <p><strong>Output:</strong> <span class="example-io">15</span></p>
///
/// <p><strong>Explanation:</strong></p>
///
/// <p>We can place the rooks in the cells <code>(0, 0)</code>, <code>(1, 1)</code>, and <code>(2, 2)</code> for a sum of <code>1 + 5 + 9 = 15</code>.</p>
/// </div>
///
/// <p><strong class="example">Example 3:</strong></p>
///
/// <div class="example-block">
/// <p><strong>Input:</strong> <span class="example-io">board = [[1,1,1],[1,1,1],[1,1,1]]</span></p>
///
/// <p><strong>Output:</strong> <span class="example-io">3</span></p>
///
/// <p><strong>Explanation:</strong></p>
///
/// <p>We can place the rooks in the cells <code>(0, 2)</code>, <code>(1, 1)</code>, and <code>(2, 0)</code> for a sum of <code>1 + 1 + 1 = 3</code>.</p>
/// </div>
///
/// <p>&nbsp;</p>
/// <p><strong>Constraints:</strong></p>
///
/// <ul>
/// 	<li><code>3 &lt;= m == board.length &lt;= 100</code></li>
/// 	<li><code>3 &lt;= n == board[i].length &lt;= 100</code></li>
/// 	<li><code>-10<sup>9</sup> &lt;= board[i][j] &lt;= 10<sup>9</sup></code></li>
/// </ul>
///

impl Solution {
    pub fn maximum_value_sum(board: Vec<Vec<i32>>) -> i64 {
        let mut max_three: Vec<Vec<(i32, usize)>> = vec![vec![(i32::MIN, 0); 3]; board.len()];

        for y in 0..board.len() {
            for x in 0..board[y].len() {
                let value = (board[y][x], x);
                if value.0 < max_three[y][0].0 {
                    continue;
                }

                if value.0 < max_three[y][1].0 {
                    max_three[y][0] = value;
                    continue;
                }
                max_three[y][0] = max_three[y][1];

                if value.0 < max_three[y][2].0 {
                    max_three[y][1] = value;
                    continue;
                }
                max_three[y][1] = max_three[y][2];
                max_three[y][2] = value;
            }
        }

        let mut max_sum: i64 = i64::MIN;
        for c in 2..board.len() {
            for b in 1..c {
                for a in 0..b {
                    let row_a = &max_three[a];
                    let row_b = &max_three[b];
                    let row_c = &max_three[c];

                    for rook_one in row_a.iter().rev() {
                        for rook_two in row_b.iter().rev() {
                            if rook_two.1 == rook_one.1 {
                                continue;
                            }

                            for rook_three in row_c.iter().rev() {
                                if rook_three.1 == rook_one.1 || rook_three.1 == rook_two.1 {
                                    continue;
                                }

                                let sum =
                                    rook_one.0 as i64 + rook_two.0 as i64 + rook_three.0 as i64;
                                if sum > max_sum {
                                    max_sum = sum;
                                }
                            }
                        }
                    }
                }
            }
        }

        max_sum
    }
}
