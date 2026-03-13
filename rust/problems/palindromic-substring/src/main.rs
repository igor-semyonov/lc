fn main() {
    let s = String::from("There are four lights!");
    println!(
        "{}",
        longest_palindrome(s)
    );
}

fn longest_palindrome(s: String) -> String {
    (1..=s.len())
        .rev()
        .flat_map(
            |window_size| {
                s.chars()
                    .collect::<Vec<char>>()
                    .windows(window_size)
                    .flat_map(
                        |substring| {
                            if substring
                                .iter()
                                .zip(
                                    substring
                                        .iter()
                                        .rev(),
                                )
                                .all(|(a, b)| a == b)
                            {
                                Some(
                                    substring
                                        .iter()
                                        .collect(),
                                )
                            } else {
                                None
                            }
                        },
                    )
                    .next()
            },
        )
        .next()
        .unwrap_or(String::from(""))
}

#[cfg(test)]
mod tests;
