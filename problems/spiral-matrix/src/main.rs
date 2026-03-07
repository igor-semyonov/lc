fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let n_rows = matrix.len() as isize;
    let n_cols = match matrix.get(0) {
        Some(v) => v.len(),
        None => 0,
    } as isize;
    let n = (n_rows * n_cols) as usize;
    // let result = Vec::with_capacity(n);
    let mut result = vec![0; n];
    let mut row: isize = 0;
    let mut col: isize = 0;
    let mut idx = 0;
    let mut d_row = 0;
    let mut d_col = 1;
    let mut boundary_col_min = 0;
    let mut boundary_col_max = n_cols - 1;
    let mut boundary_row_min = 0;
    let mut boundary_row_max = n_rows - 1;

    while idx < n {
        // get matrix value at current location and save to
        // result vector
        result[idx] = matrix[row as usize][col as usize];

        // move cursor in matrix
        if d_col != 0 {
            // move column, unless at end of column
            if (col == boundary_col_max && d_col == 1)
                || (col == boundary_col_min && d_col == -1)
            {
                // turn
                if d_col == 1 {
                    d_row = 1;
                    boundary_row_min += 1;
                } else {
                    d_row = -1;
                    boundary_row_max -= 1;
                }
                d_col = 0;
            } else {
                col += d_col;
                idx += 1;
            }
        } else {
            if (row == boundary_row_max && d_row == 1)
                || (row == boundary_row_min && d_row == -1)
            {
                // turn
                if d_row == 1 {
                    d_col = -1;
                    boundary_col_max -= 1;
                } else {
                    d_col = 1;
                    boundary_col_min += 1;
                };
                d_row = 0;
            } else {
                row += d_row;
                idx += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests;
