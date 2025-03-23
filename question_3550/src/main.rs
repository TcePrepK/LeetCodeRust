use question_3550::Solution;

fn main() {
    println!(
        "Example 1 -> {}",
        Solution::maximum_value_sum(
            [
                [-3, 1, 1, 1].into(),
                [-3, 1, -3, 1].into(),
                [-3, 2, 1, 1].into()
            ]
            .into()
        )
    );
    println!(
        "Example 2 -> {}",
        Solution::maximum_value_sum([[1, 2, 3].into(), [4, 5, 6].into(), [7, 8, 9].into()].into())
    );
    println!(
        "Example 3 -> {}",
        Solution::maximum_value_sum([[1, 1, 1].into(), [1, 1, 1].into(), [1, 1, 1].into()].into())
    );
    println!(
        "Example 4 -> {}",
        Solution::maximum_value_sum(
            [
                [-53, -86, -80].into(),
                [-28, 16, -42].into(),
                [-88, 38, -66].into()
            ]
            .into()
        )
    );
    println!(
        "Example 5 -> {}",
        Solution::maximum_value_sum(
            [
                [-24, -3, -64].into(),
                [13, -8, -74].into(),
                [23, 65, -99].into()
            ]
            .into()
        )
    );
}
