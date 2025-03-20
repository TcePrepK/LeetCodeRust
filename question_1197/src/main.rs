use question_1197::Solution;

fn main() {
    println!(
        "Example 1 -> {}",
        Solution::parse_bool_expr("&(|(f))".into())
    );
    println!(
        "Example 2 -> {}",
        Solution::parse_bool_expr("|(f,f,f,t)".into())
    );
    println!(
        "Example 3 -> {}",
        Solution::parse_bool_expr("!(&(f,t))".into())
    );
    println!(
        "Example 4 -> {}",
        Solution::parse_bool_expr("|(f,&(t,t))".into())
    );
}
