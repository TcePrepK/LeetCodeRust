pub struct Solution {}

/// <p>Given the string <code>s</code>, return the size of the longest substring containing each vowel an even number of times. That is, &#39;a&#39;, &#39;e&#39;, &#39;i&#39;, &#39;o&#39;, and &#39;u&#39; must appear an even number of times.</p>
///
/// <p>&nbsp;</p>
/// <p><strong class="example">Example 1:</strong></p>
///
/// <pre>
/// <strong>Input:</strong> s = &quot;eleetminicoworoep&quot;
/// <strong>Output:</strong> 13
/// <strong>Explanation: </strong>The longest substring is &quot;leetminicowor&quot; which contains two each of the vowels: <strong>e</strong>, <strong>i</strong> and <strong>o</strong> and zero of the vowels: <strong>a</strong> and <strong>u</strong>.
/// </pre>
///
/// <p><strong class="example">Example 2:</strong></p>
///
/// <pre>
/// <strong>Input:</strong> s = &quot;leetcodeisgreat&quot;
/// <strong>Output:</strong> 5
/// <strong>Explanation:</strong> The longest substring is &quot;leetc&quot; which contains two e&#39;s.
/// </pre>
///
/// <p><strong class="example">Example 3:</strong></p>
///
/// <pre>
/// <strong>Input:</strong> s = &quot;bcbcbc&quot;
/// <strong>Output:</strong> 6
/// <strong>Explanation:</strong> In this case, the given string &quot;bcbcbc&quot; is the longest because all vowels: <strong>a</strong>, <strong>e</strong>, <strong>i</strong>, <strong>o</strong> and <strong>u</strong> appear zero times.
/// </pre>
///
/// <p>&nbsp;</p>
/// <p><strong>Constraints:</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= s.length &lt;= 5 x 10^5</code></li>
/// 	<li><code>s</code>&nbsp;contains only lowercase English letters.</li>
/// </ul>
///

impl Solution {
    fn char_to_hash(c: char) -> u8 {
        match c {
            'a' => 0,
            'e' => 1,
            'i' => 2,
            'o' => 3,
            'u' => 4,
            _ => panic!("Invalid character"),
        }
    }

    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut mask: usize = 0b00000;
        let mut map: [usize; 32] = [0; 32];
        let mut index: usize = 1;
        let mut chars = s.chars().into_iter();
        let mut longest = 0;

        map[mask] = index;
        while index <= s.len() {
            let char = chars.next();
            match char {
                Some('a' | 'e' | 'i' | 'o' | 'u') => {
                    mask ^= 1 << Solution::char_to_hash(char.unwrap());
                }
                _ => {}
            }

            index += 1;
            if map[mask] == 0 {
                map[mask] = index;
            } else {
                longest = longest.max(index - map[mask]);
            }
        }

        longest as i32
    }
}
