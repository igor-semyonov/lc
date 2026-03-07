fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
pub fn find_kth_bit(n: i32, k: i32) -> char {
    let nth_string = find_nth_string(n);
    nth_string
        .chars()
        .nth(k as usize - 1)
        .unwrap()
}

fn find_nth_string(n: i32) -> String {
    let mut sn = String::from("0"); // Start s_1 = "0"
    for _ in 1..n {
        sn = find_next_nth_string(&sn);
    }
    sn
}

#[allow(dead_code)]
fn find_next_nth_string(s: &str) -> String {
    let mut result = String::from(s);
    result.push('1');
    result.extend(
        s.chars()
            .rev()
            .map(
                |b| match b {
                    '0' => '1',
                    '1' => '0',
                    _ => unreachable!("something broke"),
                },
            ),
    );

    result
}

#[cfg(test)]
mod tests;
