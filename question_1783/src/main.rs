use question_1783::Solution;

fn main() {
    println!(
        "Example 1 -> {}",
        Solution::ways_to_make_fair([2, 1, 6, 4].into())
    );
    println!(
        "Example 2 -> {}",
        Solution::ways_to_make_fair([1, 1, 1].into())
    );
    println!(
        "Example 3 -> {}",
        Solution::ways_to_make_fair([1, 2, 3].into())
    );
    println!("Example 3 -> {}", Solution::ways_to_make_fair([1].into()));
}
