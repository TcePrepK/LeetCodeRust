use question_468::Solution;

fn main() {
    println!(
        "Example 1 -> {}",
        Solution::valid_ip_address("172.16.254.1".into())
    );
    println!(
        "Example 2 -> {}",
        Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".into())
    );
    println!(
        "Example 3 -> {}",
        Solution::valid_ip_address("256.256.256.256".into())
    );
    println!(
        "Example 4 -> {}",
        Solution::valid_ip_address("2001:db8:85a3:0::8a2E:0370:7334".into())
    );
}
