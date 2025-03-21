use question_1575::Solution;

fn main() {
    println!(
        "Example 1 -> {}",
        Solution::max_area(5.into(), 4.into(), [1, 2, 4].into(), [1, 3].into())
    );
    println!(
        "Example 2 -> {}",
        Solution::max_area(5.into(), 4.into(), [3, 1].into(), [1].into())
    );
    println!(
        "Example 3 -> {}",
        Solution::max_area(5.into(), 4.into(), [3].into(), [3].into())
    );
}
