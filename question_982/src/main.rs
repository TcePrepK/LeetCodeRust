use question_982::Solution;

fn main() {
    println!(
        "Example 1 -> {}",
        Solution::min_increment_for_unique([1, 2, 2].into())
    );
    println!(
        "Example 2 -> {}",
        Solution::min_increment_for_unique([3, 2, 1, 2, 1, 7].into())
    );
    println!(
        "Example 3 -> {}",
        Solution::min_increment_for_unique([7, 2, 7, 2, 1, 4, 3, 1, 4, 8].into())
    );
}
