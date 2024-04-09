use std::ops::{Deref, DerefMut};

use poem::{handler, Result};
use speedy::{Readable, Writable};

use crate::error::Error;
use crate::web::speedy_data::Speedy;

static ONE_NUM: [u16; 9] = [
    0b1,
    0b10,
    0b100,
    0b1000,
    0b1_0000,
    0b10_0000,
    0b100_0000,
    0b1000_0000,
    0b1_0000_0000,
];

#[derive(Readable, Writable)]
struct Sudoku {
    all_nums: [u16; 81],
    empty_cells: Vec<usize>,
}

impl Default for Sudoku {
    fn default() -> Self {
        Sudoku {
            all_nums: [0b1_1111_1111; 81],
            empty_cells: (0..81).collect(),
        }
    }
}

impl Deref for Sudoku {
    type Target = [u16; 81];

    fn deref(&self) -> &Self::Target {
        &self.all_nums
    }
}

impl DerefMut for Sudoku {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.all_nums
    }
}

impl Sudoku {
    // 填充数字
    fn input(&mut self, i: usize, num: u16) {
        assert!(num > 0 && num < 10);
        // 行列号
        let row = i / 9;
        let col = i % 9;

        // 填充自身
        self[i] = num << 9 | 1 << (num - 1);
        if let Ok(index) = self.empty_cells.binary_search(&i) {
            self.empty_cells.swap_remove(index);
        }

        for j in 0..10 {
            // 更新同一行中的第j个
            self.update(row * 9 + j, num);
            // 更新同一列中的第j个
            self.update(j * 9 + col, num);
            // 更新同一块中的第j个
            self.update((row / 3 * 3 + j / 3) * 9 + col / 3 * 3 + j % 3, num);
        }
    }

    // 查找只有一个
    fn check(&mut self) -> bool {
        // self.all_nums.iter().any(|&i| i < 0b10_0000_0000)
        self.empty_cells.is_empty()
    }

    // 内部方法：更新相邻单元格使用
    fn update(&mut self, i: usize, num: u16) {
        // 填充过的不用更新
        if self[i] > 0b1_1111_1111 {
            return;
        }

        // 排除相邻单元格的此数字可能性
        self[i] &= !(1 << (num - 1));
        if let Ok(j) = ONE_NUM.binary_search(&self[i]) {
            self.input(i, j as u16 + 1);
        }
    }
}

#[handler]
async fn resolve(req: Speedy<[u16; 81]>) -> Result<Speedy<Sudoku>> {
    let mut sudoku = Sudoku::default();
    for (i, &n) in req.0.iter().enumerate() {
        // 没有值
        if n == 0 {
            continue;
        }

        // 校验数字
        if !(1..=9).contains(&n) {
            return Err(Error::SudokuNumInvalid(n).into());
        }

        // 一个个填进去
        sudoku.input(i, n);
    }

    while !sudoku.empty_cells.is_empty() {}

    Ok(Speedy(sudoku))
}
