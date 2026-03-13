fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
pub fn find_kth_bit(n: i32, k: i32) -> char {
    let mut n_local = n;
    let x_n = find_nth_num(n);
    match (x_n
        >> (BITS - x_n.leading_zeros() - (k as u32 - 1)))
        & 1
    {
        0 => '0',
        1 => '1',
        _ => unreachable!("something went wrong"),
    }
}

const BITS: u32 = 128;
#[allow(dead_code)]
fn find_next_nth_num(x: u128) -> u128 {
    let len = BITS - x.leading_zeros() + 1;
    (x << (len + 1))
        + (1 << len)
        + ((!x).reverse_bits() >> (BITS - len))
}

fn find_nth_num(n: i32, start: bool) -> u128 {
    let mut xn = if start {
        1
    } else {
        0
    }; // Start x_1 = 0
    for _ in 1..n {
        xn = find_next_nth_num(xn);
    }
    xn
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
