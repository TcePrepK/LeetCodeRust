use question_1473::Solution;

fn main() {
    println!(
        "Example 1 -> {}",
        Solution::find_the_longest_substring("eleetminicoworoep".into())
    );
    println!(
        "Example 2 -> {}",
        Solution::find_the_longest_substring("leetcodeisgreat".into())
    );
    println!(
        "Example 3 -> {}",
        Solution::find_the_longest_substring("bcbcbc".into())
    );
}
