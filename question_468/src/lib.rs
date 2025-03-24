pub struct Solution {}

/// <p>Given a string <code>queryIP</code>, return <code>&quot;IPv4&quot;</code> if IP is a valid IPv4 address, <code>&quot;IPv6&quot;</code> if IP is a valid IPv6 address or <code>&quot;Neither&quot;</code> if IP is not a correct IP of any type.</p>
///
/// <p><strong>A valid IPv4</strong> address is an IP in the form <code>&quot;x<sub>1</sub>.x<sub>2</sub>.x<sub>3</sub>.x<sub>4</sub>&quot;</code> where <code>0 &lt;= x<sub>i</sub> &lt;= 255</code> and <code>x<sub>i</sub></code> <strong>cannot contain</strong> leading zeros. For example, <code>&quot;192.168.1.1&quot;</code> and <code>&quot;192.168.1.0&quot;</code> are valid IPv4 addresses while <code>&quot;192.168.01.1&quot;</code>, <code>&quot;192.168.1.00&quot;</code>, and <code>&quot;192.168@1.1&quot;</code> are invalid IPv4 addresses.</p>
///
/// <p><strong>A valid IPv6</strong> address is an IP in the form <code>&quot;x<sub>1</sub>:x<sub>2</sub>:x<sub>3</sub>:x<sub>4</sub>:x<sub>5</sub>:x<sub>6</sub>:x<sub>7</sub>:x<sub>8</sub>&quot;</code> where:</p>
///
/// <ul>
/// 	<li><code>1 &lt;= x<sub>i</sub>.length &lt;= 4</code></li>
/// 	<li><code>x<sub>i</sub></code> is a <strong>hexadecimal string</strong> which may contain digits, lowercase English letter (<code>&#39;a&#39;</code> to <code>&#39;f&#39;</code>) and upper-case English letters (<code>&#39;A&#39;</code> to <code>&#39;F&#39;</code>).</li>
/// 	<li>Leading zeros are allowed in <code>x<sub>i</sub></code>.</li>
/// </ul>
///
/// <p>For example, &quot;<code>2001:0db8:85a3:0000:0000:8a2e:0370:7334&quot;</code> and &quot;<code>2001:db8:85a3:0:0:8A2E:0370:7334&quot;</code> are valid IPv6 addresses, while &quot;<code>2001:0db8:85a3::8A2E:037j:7334&quot;</code> and &quot;<code>02001:0db8:85a3:0000:0000:8a2e:0370:7334&quot;</code> are invalid IPv6 addresses.</p>
///
/// <p>&nbsp;</p>
/// <p><strong class="example">Example 1:</strong></p>
///
/// <pre>
/// <strong>Input:</strong> queryIP = &quot;172.16.254.1&quot;
/// <strong>Output:</strong> &quot;IPv4&quot;
/// <strong>Explanation:</strong> This is a valid IPv4 address, return &quot;IPv4&quot;.
/// </pre>
///
/// <p><strong class="example">Example 2:</strong></p>
///
/// <pre>
/// <strong>Input:</strong> queryIP = &quot;2001:0db8:85a3:0:0:8A2E:0370:7334&quot;
/// <strong>Output:</strong> &quot;IPv6&quot;
/// <strong>Explanation:</strong> This is a valid IPv6 address, return &quot;IPv6&quot;.
/// </pre>
///
/// <p><strong class="example">Example 3:</strong></p>
///
/// <pre>
/// <strong>Input:</strong> queryIP = &quot;256.256.256.256&quot;
/// <strong>Output:</strong> &quot;Neither&quot;
/// <strong>Explanation:</strong> This is neither a IPv4 address nor a IPv6 address.
/// </pre>
///
/// <p>&nbsp;</p>
/// <p><strong>Constraints:</strong></p>
///
/// <ul>
/// 	<li><code>queryIP</code> consists only of English letters, digits and the characters <code>&#39;.&#39;</code> and <code>&#39;:&#39;</code>.</li>
/// </ul>
///

impl Solution {
    fn validate_ipv4_address(query_ip: String) -> String {
        let mut chars = query_ip.chars();
        let mut number: i32 = -1;
        let mut dot_count = 0;
        loop {
            match chars.next() {
                Some('.') => {
                    if number == -1 || dot_count == 3 {
                        return "Neither".to_string();
                    }

                    number = -1;
                    dot_count += 1;
                }
                Some(c) => {
                    if c < '0' || c > '9' {
                        return "Neither".to_string();
                    }

                    let dig = c as i32 - '0' as i32;
                    if number == 0 {
                        return "Neither".to_string();
                    }

                    if number == -1 {
                        number = 0;
                    }

                    number = number * 10 + dig;
                    if number > 255 {
                        return "Neither".to_string();
                    }
                }
                None => {
                    if number == -1 {
                        return "Neither".to_string();
                    }

                    if dot_count != 3 {
                        return "Neither".to_string();
                    }

                    return "IPv4".to_string();
                }
            }
        }
    }

    fn validate_ipv6_address(query_ip: String) -> String {
        let mut chars = query_ip.chars();
        let mut hex_digits = 0;
        let mut dot_count = 0;

        loop {
            match chars.next() {
                Some(':') => {
                    if hex_digits == 0 || dot_count == 7 {
                        return "Neither".to_string();
                    }

                    hex_digits = 0;
                    dot_count += 1;
                }
                Some(c) => {
                    if (c < '0' || c > '9') && (c < 'a' || c > 'f') && (c < 'A' || c > 'F') {
                        return "Neither".to_string();
                    }

                    hex_digits += 1;
                    if hex_digits > 4 {
                        return "Neither".to_string();
                    }
                }
                None => {
                    if hex_digits == 0 {
                        return "Neither".to_string();
                    }

                    if dot_count != 7 {
                        return "Neither".to_string();
                    }

                    return "IPv6".to_string();
                }
            }
        }
    }

    pub fn valid_ip_address(query_ip: String) -> String {
        if query_ip.len() < 7 {
            return "Neither".to_string();
        }

        if query_ip[0..4].contains(".") {
            Self::validate_ipv4_address(query_ip)
        } else {
            Self::validate_ipv6_address(query_ip)
        }
    }
}
