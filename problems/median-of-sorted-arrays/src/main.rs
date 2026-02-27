#[cfg(test)]
mod tests;

fn main() {
    let m = find_median_sorted_arrays(
        // (0..10_000).collect(),
        (i32::MIN..0).collect(),
        (0..i32::MAX).collect(),
    );
    println!("{m:?}");
}

pub fn find_median_sorted_arrays(
    nums1: Vec<i32>,
    nums2: Vec<i32>,
) -> f64 {
    fn median(
        a1: &[i32],
        a2: &[i32],
        low: usize,
        high: usize,
        total_len: &usize,
    ) -> f64 {
        let p1 = (low + high) / 2;
        let p2 = total_len.div_ceil(2) - p1;

        let a1_left_max = if p1 == 0 {
            i32::MIN
        } else {
            a1[p1 - 1]
        };
        let a1_right_min = if p1 >= a1.len() {
            i32::MAX
        } else {
            a1[p1]
        };
        let a2_left_max = if p2 == 0 {
            i32::MIN
        } else {
            a2[p2 - 1]
        };
        let a2_right_min = if p2 >= a2.len() {
            i32::MAX
        } else {
            a2[p2]
        };

        if a1_left_max <= a2_right_min
            && a2_left_max <= a1_right_min
        {
            match total_len % 2 {
                0 => {
                    (a1_left_max.max(a2_left_max)
                        + a1_right_min.min(a2_right_min))
                        as f64
                        / 2.0
                }
                1 => a1_left_max.max(a2_left_max) as f64,
                _ => unreachable!(
                    "Remainder when dividing by 2 should be 0 or 1."
                ),
            }
        } else if a1_left_max > a2_right_min {
            median(
                a1,
                a2,
                low,
                p1 - 1,
                total_len,
            )
        // } else if a2_left_max > a1_right_min {
        // this is the last possibility, so no need to check
        } else {
            median(
                a1,
                a2,
                p1 + 1,
                high,
                total_len,
            )
        }
    }

    let (nums1, nums2) = if nums1.len() > nums2.len() {
        (
            &nums2, &nums1,
        )
    } else {
        (
            &nums1, &nums2,
        )
    };

    let total_len = nums1.len() + nums2.len();
    median(
        nums1,
        nums2,
        0,
        nums1.len(),
        &total_len,
    )
}
