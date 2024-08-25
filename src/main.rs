use std::io::{self, Write};

fn longest_common_subsequence(str1: &str, str2: &str) -> u64 {
    let mut dp = vec![vec![0 as u64; str2.len() + 1]; str1.len() + 1];

    for (i, c) in str2.char_indices().rev() {
        for (j, k) in str1.char_indices().rev() {
            if c == k {
                dp[j][i] = 1 + dp[j + 1][i + 1];
            } else {
                dp[j][i] = dp[j + 1][i].max(dp[j][i + 1]);
            }
        }
    }

    return dp[0][0];
}

fn main() {
    let mut first_string = String::new();
    let mut second_string = String::new();

    print!("Enter the first string: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut first_string)
        .expect("Failed to read line.");

    print!("Enter the second string: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut second_string)
        .expect("Failed to read line.");

    println!(
        "Longest common subsequence has lenght {}.",
        longest_common_subsequence(first_string.trim(), second_string.trim())
    )
}
