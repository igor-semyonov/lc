fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn min_number_of_seconds(
        mountain_height: i32,
        worker_times: Vec<i32>,
    ) -> i64 {
        let mut time_taken = 1;
        let mut max_chop_cache = MaxChopCache::new();
        loop {
            let total_chopped: i32 = worker_times
                .iter()
                .map(
                    |&worker_time| {
                        max_chop_cache.max_chopped_in_given_time(
                            worker_time,
                            time_taken,
                        )
                    },
                )
                .sum();
            println!("{total_chopped} {time_taken}");
            if total_chopped >= mountain_height {
                break;
            }
            time_taken += 1;
        }
        time_taken.into()
    }
}

use std::collections::HashMap;
struct MaxChopCache(
    HashMap<
        (
            i32,
            i32,
        ),
        i32,
    >,
);
impl MaxChopCache {
    fn new()->Self{
        Self( HashMap::new() )
    }
    fn max_chopped_in_given_time(
        &mut self,
        worker_time: i32,
        time_given: i32,
    ) -> i32 {
        match self
            .0
            .get(&(
                worker_time,
                time_given,
            )) {
            Some(x) => *x,
            None => {
                let x = max_chopped_in_given_time(
                    worker_time,
                    time_given,
                );
                self.0
                    .insert(
                        (
                            worker_time,
                            time_given,
                        ),
                        x,
                    );
                x
            }
        }
    }
}

fn max_chopped_in_given_time(
    worker_time: i32,
    time_given: i32,
) -> i32 {
    ((-1.0
        + f32::sqrt(
            1.0 + 8.0
                * (time_given as f32 / worker_time as f32),
        ))
        / 2.0)
        .floor() as i32
}

#[cfg(test)]
mod tests;
