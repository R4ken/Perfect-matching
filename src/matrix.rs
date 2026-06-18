use crate::imod::Imod;
use std::ops::{Index, IndexMut};

/// Square matrix with **n** x **n** dimensions 
#[derive(Clone, Debug)]
pub struct ModuloMatrix {
    pub size: usize,
    initial_size: usize,
    pub mat: Vec<Imod>
}

impl Index<usize> for ModuloMatrix {
    type Output = [Imod];

    fn index(&self, row: usize) -> &Self::Output {
        debug_assert!(row < self.size);
        &self.mat[self.row_range(row)]
    }
}

impl IndexMut<usize> for ModuloMatrix {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        debug_assert!(row < self.size);
        let range = self.row_range(row);
        &mut self.mat[range] 
    }
}

impl ModuloMatrix {
    /// Returns new identity **Matrix** </br>
    /// `n` - size of matrix
    pub fn new(size: usize) -> Self {
        let mut ret = ModuloMatrix{
            size,
            initial_size: size,
            mat: vec![Imod::from(0); size * size],
        };
        for i in 0..size {
            ret[i][i] = Imod::from(1);
        }
        ret
    }
    #[inline]
    fn row_range(&self, row: usize) -> std::ops::Range<usize> {
        let start = row.checked_mul(self.initial_size)
            .expect("Vector index overflow");
        start..start + self.size
    }
    /// Changes size of a matrix to a given value </br>
    /// **Note**: `new_size` must be lower than initial size of matrix
    pub fn resize(&mut self, new_size: usize) {
        debug_assert!(new_size < self.initial_size);

        self.size = new_size
    }
    /// Swaps row of matrix
    pub fn swap_row(&mut self, row_a: usize, row_b: usize) {
        debug_assert!(row_a < self.size);
        debug_assert!(row_b < self.size);

        let a_start = row_a * self.initial_size;
        let b_start = row_b * self.initial_size;
        for i in 0..self.size {
            self.mat.swap(a_start + i, b_start + i);
        }

    }
    /// Swaps columns of matrix
    pub fn swap_col(&mut self, col_a: usize, col_b: usize) {
        debug_assert!(col_a < self.size);
        debug_assert!(col_b < self.size);

        for i in 0..self.size {
            let offset = i * self.initial_size;
            self.mat.swap(col_a + offset, col_b + offset);
        }
    }
    /// Substracts `src_row` multiplied by `multiplier` from `trg_row` <br/>
    /// src -= trg * mult
    pub fn sub_row_mult(&mut self, src_row: usize, trg_row: usize, muiltiplier: Imod) {
        debug_assert!(src_row < self.size);
        debug_assert!(trg_row < self.size);

        for i in 0..self.size {
            let tmp = self[src_row][i] * muiltiplier;
            self[trg_row][i] -= tmp;
        }
    }

    /// Multiplies row by multiplier
    pub fn multiply_row(&mut self, row: usize, muiltiplier: Imod) {
        debug_assert!(row < self.size);

        for i in 0..self.size {
            self[row][i] = self[row][i] * muiltiplier;
        }
    }

    /// Calculates iand returns inverted matrix on success
    pub fn calculate_inverse(& self) -> Option<ModuloMatrix> {
        let mut p_m = ModuloMatrix::new(self.size);
        let mut l_m = ModuloMatrix::new(self.size);
        let mut u_m = self.clone();
        let mut swap_row: usize;
        for i in 0..self.size - 1 {
            if u_m[i][i].value == 0 {
                swap_row = i + 1;
                while swap_row < self.size {
                    if u_m[swap_row][i].value != 0 {
                        break;
                    }
                    swap_row += 1;
                }
                if swap_row == self.size {
                    return None
                }
                p_m.swap_row(i, swap_row);
                u_m.swap_row(i, swap_row);
                for j in 0..i {
        			 let tmp = l_m[i][j];
                     l_m[i][j] = l_m[swap_row][j];
                     l_m[swap_row][j] = tmp;
                }
            }
            for j in i + 1 .. self.size {
                let wsp = u_m[j][i] / u_m[i][i];
                u_m.sub_row_mult(i, j, wsp);
                l_m[j][i] = wsp;
            }
        }
        for i in 0..self.size {
            let wsp = l_m[i][i].inv();
            l_m.multiply_row(i, wsp);
            p_m.multiply_row(i, wsp);
            for j in i + 1 .. self.size {
                let k = l_m[j][i];
                l_m.sub_row_mult(i, j, k);
                p_m.sub_row_mult(i, j, k);
            }
        }
        for i in (0..self.size).rev() {
            let wsp = u_m[i][i].inv();
            u_m.multiply_row(i, wsp);
            p_m.multiply_row(i, wsp);
            for j in 0..i {
                let k = u_m[j][i];
                u_m.sub_row_mult(i, j, k);
                p_m.sub_row_mult(i, j, k);
            }
        }
        Some(p_m)
    }

    /// Swaps rows and collumns to end of a matrix, calculates smaller matrix (and its inverse) and lowers its size <br/>
    /// Function is used to try to *remove* vertices a and b from a graph
    pub fn reduce_rows_and_columns(matrix: &mut ModuloMatrix, inv_matrix: &mut ModuloMatrix, mut a: usize, mut b: usize) -> Result<(),()> {
        let n = matrix.size - 1;
        let substract = inv_matrix[a][a] * inv_matrix[b][b];
        if inv_matrix[b][a] == Imod::from(0) || inv_matrix[a][b] - substract / inv_matrix[b][a] == Imod::from(0) {
            return Err(());
        }
        for i in a..n {
            matrix.swap_row(i, i + 1);
            inv_matrix.swap_col(i, i + 1);
        }
        for i in b..n {
            matrix.swap_col(i, i + 1);
            inv_matrix.swap_row(i, i + 1);
        }
        for i in 0..n {
            for j in 0..n {
                inv_matrix[i][j] = inv_matrix[i][j] - inv_matrix[i][n] * inv_matrix[n][j] / inv_matrix[n][n];
            }
        }
        matrix.resize(n);
        inv_matrix.resize(n);
        if a < b {
            b -= 1;
        }
        else {
            a -= 1;
        }
        for i in b..n - 1 {
            matrix.swap_row(i, i + 1);
            inv_matrix.swap_col(i, i + 1);
        }
        for i in a..n - 1 {
            matrix.swap_col(i, i + 1);
            inv_matrix.swap_row(i, i + 1);
        }
        for i in 0..n - 1 {
            for j in 0..n - 1 {
                inv_matrix[i][j] = inv_matrix[i][j] - inv_matrix[i][n - 1] * inv_matrix[n - 1][j] / inv_matrix[n - 1][n - 1];
            }
        }
        matrix.resize(n - 1);
        inv_matrix.resize(n - 1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse_matrix() {
        let mut test_m = ModuloMatrix::new(3);
        test_m[0][0] = Imod::from(1);
        test_m[0][1] = Imod::from(2);
        test_m[1][1] = Imod::from(1);
        test_m[1][2] = Imod::from(3);
        test_m[2][0] = Imod::from(1);
        test_m[2][2] = Imod::from(4);
        let Some(m_inv) = test_m.calculate_inverse() else {
            panic!("Matrix not invertible")
        };
        for i in 0..3 {
            for j in 0..3 {
                let mut v = Imod::from(0);
                for k in 0..3 {
                    v += test_m[i][k] * m_inv[k][j];
                }
                if i == j {
                    assert_eq!(v, Imod::from(1));
                }
                else {
                    assert_eq!(v, Imod::from(0));
                }
            }
        }
    }
    #[test]
    fn test_reduction_method() {
        let raw_matrix = [
            [8, 2, 0, 1, 3],
            [1, 9, 2, 4, 1],
            [0, 3, 7, 1, 2],
            [2, 1, 1, 8, 2],
            [3, 0, 2, 1, 9],
        ];
        let mut test_m = ModuloMatrix::new(5);
        for (r, row) in raw_matrix.iter().enumerate() {
            for (c, &val) in row.iter().enumerate() {
                test_m[r][c] = Imod::from(val);
            }
        }
        let raw_matrix_small = [
            [7, 1, 2],
            [1, 8, 2],
            [2, 1, 9],
        ];
        let mut test_m_small = ModuloMatrix::new(3);
        for (r, row) in raw_matrix_small.iter().enumerate() {
            for (c, &val) in row.iter().enumerate() {
                test_m_small[r][c] = Imod::from(val);
            }
        }
        let Some(mut inv_m) = test_m.calculate_inverse() else {
            panic!("Not invertible")
        };
        let Some(mut inv_m_small) = test_m_small.calculate_inverse() else {
            panic!("Not invertible")
        };
        let res = ModuloMatrix::reduce_rows_and_columns(&mut test_m, &mut inv_m, 0, 1);
        if res.is_err() {
            panic!("Couldn't substitute");
        }
        for i in 0..3 {
            for j in 0..3 {
                let mut v = Imod::from(0);
                for k in 0..3 {
                    v += test_m[i][k] * inv_m[k][j];
                }
                if i == j {
                    assert_eq!(v, Imod::from(1));
                }
                else {
                    assert_eq!(v, Imod::from(0));
                }
                assert_eq!(inv_m_small[i][j], inv_m[i][j]);
            }
        }
    }
}