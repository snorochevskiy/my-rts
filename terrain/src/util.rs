pub trait MatrixHelper {
    type Value;
    fn cell(&self, row: i32, col: i32) -> Option<&Self::Value>;
    fn cell_relative(&self, row_ind: i32, col_ind: i32, delta: (i32, i32)) -> Option<&Self::Value>;
}

impl <T> MatrixHelper for Vec<Vec<T>> {
    type Value = T;

    fn cell(&self, row: i32, col: i32) -> Option<&Self::Value> {
        if row < 0 || col < 0 {
            None
        } else {
            self.get(row as usize).and_then(|r| r.get(col as usize))
        }
    }

    fn cell_relative(&self, row_ind: i32, col_ind: i32, (row_delta, col_delta): (i32, i32)) -> Option<&Self::Value> {
        let row_target = row_ind + row_delta;
        let col_target = col_ind + col_delta;
        if row_target < 0 || col_target < 0 {
            None
        } else {
            self.get(row_target as usize).and_then(|r| r.get(col_target as usize))
        }
    }
}