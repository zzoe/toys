use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

use poem::{handler, Result};
use speedy::{Readable, Writable};

use crate::error::Error;
use crate::web::speedy_data::Speedy;

const ONE_NUM: [u16; 9] = [
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

const ALL_CONDITION: u16 = 0b111_111_111;

#[derive(Copy, Clone, Readable, Writable)]
struct Sudoku([u16; 81]);

impl Default for Sudoku {
    fn default() -> Self {
        // 0-8位代表每个数字1-9的可能性，第9位之上存储具体的值
        Sudoku([ALL_CONDITION; 81])
    }
}

impl Deref for Sudoku {
    type Target = [u16; 81];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sudoku {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for i in 0..9 {
            if i == 3 || i == 6 {
                writeln!(f, "------|-------|------")?;
            }
            for j in 0..8 {
                if j == 3 || j == 6 {
                    write!(f, "| ")?;
                }
                write!(f, "{} ", self[i * 9 + j] >> 9)?;
            }
            writeln!(f, "{}", self[i * 9 + 8] >> 9)?;
        }
        Ok(())
    }
}

impl Sudoku {
    // 填充数字
    fn input(&mut self, i: usize, num: u16) {
        assert!(num > 0 && num < 10);
        // 行列号
        let row = i / 9;
        let col = i % 9;

        // 填充自身 0-8位代表每个数字1-9的可能性，第9位之上存储具体的值
        self[i] = (num << 9) | (1 << (num - 1));
        tracing::debug!("{self}");

        for j in 0..9 {
            // 更新同一行中的第j个
            self.update(row * 9 + j, num);
            // 更新同一列中的第j个
            self.update(j * 9 + col, num);
            // 更新同一块中的第j个
            self.update((row / 3 * 3 + j / 3) * 9 + col / 3 * 3 + j % 3, num);
        }
    }

    // 检查同一行/列/块中，某个数字只有一个可能性的情况，并填充之
    fn check_only(&mut self, groups: [[usize; 9]; 9]) -> bool {
        let mut modified = false;
        for group in groups {
            // 同一行/列/块中，将所有数字相同位的拼到一起，表示每个数在每个格子上的可能性
            let mut group_check = [0_u16; 9];
            //依次汇总check_seq+1（1-9）在每个格子的可能性
            for (check_seq, group_check_num) in group_check.iter_mut().enumerate() {
                for (group_seq, num_index) in group.iter().enumerate() {
                    tracing::debug!(
                        "{check_seq} {group_seq} {:016b} {:016b}",
                        *group_check_num,
                        self[*num_index]
                    );
                    // (1 << check_seq) & self[*num_index] 取出这格子中数字check_seq+1的可能性，然后根据格子的序号0-8放在group_check_num的0-8 bit 上
                    *group_check_num |=
                        ((1 << check_seq) & self[*num_index]) >> check_seq << group_seq;
                    tracing::debug!("{check_seq} {group_seq} {:016b}", *group_check_num);
                }
                // 这一组里面，数字j+1唯一可能的地方
                if let Ok(guy) = ONE_NUM.binary_search(group_check_num) {
                    // 已经填充的不管
                    if self[group[guy]] > ALL_CONDITION {
                        continue;
                    }

                    tracing::debug!("Group:{group:?} [{}] -> [{}]", group[guy], check_seq + 1);
                    self.input(group[guy], check_seq as u16 + 1);
                    modified = true;
                }
            }
        }
        modified
    }

    // 内部方法：更新相邻单元格使用
    fn update(&mut self, i: usize, num: u16) {
        // 填充过的不用更新
        if self[i] > ALL_CONDITION {
            return;
        }

        // 排除相邻单元格的此数字可能性
        self[i] &= !(1 << (num - 1));

        // 只剩下一个可能的值时，填充此值
        if let Ok(j) = ONE_NUM.binary_search(&self[i]) {
            tracing::debug!("Only one possibility: [{}] -> [{}]", i, j + 1);
            self.input(i, j as u16 + 1);
        }
    }
}

#[handler]
pub async fn resolve(req: Speedy<[u16; 81]>) -> Result<Speedy<[u16; 81]>> {
    let mut sudoku = Sudoku::default();
    // 按照入参初始化数独，不过初始化的过程中发现唯一可能的时候，也会直接填充
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

    // 开始检查每一组里面的某个数字的唯一性
    let mut modified = true;
    while modified {
        modified = sudoku.check_only(rows())
            || sudoku.check_only(columns())
            || sudoku.check_only(blocks());
    }

    // 找不到唯一可能性的格子了，接下来开始枚举？
    tracing::info!("{sudoku}");

    Ok(Speedy(sudoku.0.map(|a| a >> 9)))
}

const fn rows() -> [[usize; 9]; 9] {
    [
        [0, 1, 2, 3, 4, 5, 6, 7, 8],
        [9, 10, 11, 12, 13, 14, 15, 16, 17],
        [18, 19, 20, 21, 22, 23, 24, 25, 26],
        [27, 28, 29, 30, 31, 32, 33, 34, 35],
        [36, 37, 38, 39, 40, 41, 42, 43, 44],
        [45, 46, 47, 48, 49, 50, 51, 52, 53],
        [54, 55, 56, 57, 58, 59, 60, 61, 62],
        [63, 64, 65, 66, 67, 68, 69, 70, 71],
        [72, 73, 74, 75, 76, 77, 78, 79, 80],
    ]
}

const fn columns() -> [[usize; 9]; 9] {
    [
        [0, 9, 18, 27, 36, 45, 54, 63, 72],
        [1, 10, 19, 28, 37, 46, 55, 64, 73],
        [2, 11, 20, 29, 38, 47, 56, 65, 74],
        [3, 12, 21, 30, 39, 48, 57, 66, 75],
        [4, 13, 22, 31, 40, 49, 58, 67, 76],
        [5, 14, 23, 32, 41, 50, 59, 68, 77],
        [6, 15, 24, 33, 42, 51, 60, 69, 78],
        [7, 16, 25, 34, 43, 52, 61, 70, 79],
        [8, 17, 26, 35, 44, 53, 62, 71, 80],
    ]
}

const fn blocks() -> [[usize; 9]; 9] {
    [
        [0, 1, 2, 9, 10, 11, 18, 19, 20],
        [3, 4, 5, 12, 13, 14, 21, 22, 23],
        [6, 7, 8, 15, 16, 17, 24, 25, 26],
        [27, 28, 29, 36, 37, 38, 45, 46, 47],
        [30, 31, 32, 39, 40, 41, 48, 49, 50],
        [33, 34, 35, 42, 43, 44, 51, 52, 53],
        [54, 55, 56, 63, 64, 65, 72, 73, 74],
        [57, 58, 59, 66, 67, 68, 75, 76, 77],
        [60, 61, 62, 69, 70, 71, 78, 79, 80],
    ]
}

#[cfg(test)]
mod test {
    use async_std::task::block_on;
    use poem::{post, test::TestClient, Route};
    use speedy::Endianness::LittleEndian;
    use speedy::{Readable, Writable};

    use crate::web::sudoku::{blocks, columns, resolve, rows, Sudoku};

    #[rustfmt::skip]
    const SUDOKU_1: [u16; 81] = [
        0, 5, 0, 7, 0, 2, 0, 0, 3,
        0, 7, 3, 4, 8, 0, 0, 0, 5,
        0, 0, 0, 0, 5, 0, 4, 0, 0,
        0, 4, 0, 0, 0, 0, 2, 0, 0,
        0, 2, 7, 0, 9, 0, 3, 5, 0,
        0, 0, 6, 0, 0, 0, 0, 1, 0,
        0, 0, 5, 0, 3, 0, 0, 0, 0,
        4, 0, 0, 0, 6, 8, 7, 3, 0,
        7, 0, 0, 1, 0, 9, 0, 6, 0,
    ];

    #[rustfmt::skip]
    const SUDOKU_2: [u16; 81] = [
        9, 8, 5, 1, 3, 0, 4, 6, 2,
        1, 4, 6, 9, 0, 2, 0, 3, 7,
        2, 7, 0, 6, 8, 4, 5, 9, 0,
        3, 1, 0, 0, 7, 9, 2, 0, 0,
        8, 6, 0, 2, 1, 3, 9, 4, 0,
        0, 0, 2, 8, 4, 6, 1, 0, 0,
        7, 2, 9, 0, 6, 5, 3, 1, 8,
        6, 0, 0, 0, 9, 8, 7, 2, 0,
        4, 3, 8, 7, 2, 1, 6, 5, 9,
    ];

    #[rustfmt::skip]
    const SUDOKU_3: [u16; 81] = [
        9, 1, 6, 0, 0, 4, 0, 7, 2,
        8, 0, 0, 6, 2, 0, 0, 5, 4,
        5, 0, 0, 7, 0, 8, 9, 3, 0,
        0, 6, 0, 0, 0, 5, 2, 0, 0,
        0, 4, 9, 2, 0, 7, 3, 0, 0,
        2, 0, 5, 0, 6, 0, 7, 9, 8,
        0, 9, 7, 8, 0, 0, 5, 0, 3,
        0, 8, 0, 0, 7, 6, 0, 2, 9,
        4, 5, 2, 1, 9, 0, 6, 8, 7,
    ];

    #[rustfmt::skip]
    const SUDOKU_4: [u16; 81] = [
        0, 6, 2, 0, 8, 0, 5, 0, 4,
        0, 0, 8, 0, 5, 0, 0, 9, 0,
        7, 0, 0, 3, 2, 0, 0, 0, 1,
        0, 0, 0, 7, 4, 0, 6, 2, 0,
        0, 0, 0, 2, 0, 3, 0, 0, 0,
        0, 2, 7, 0, 6, 5, 0, 0, 0,
        2, 0, 0, 0, 3, 6, 0, 0, 7,
        0, 4, 0, 0, 7, 0, 1, 0, 0,
        8, 0, 3, 0, 9, 0, 2, 4, 0,
    ];

    fn solve_raw(source: [u16; 81]) {
        let mut sudoku = Sudoku::default();
        // 按照入参初始化数独，不过初始化的过程中发现唯一可能的时候，也会直接填充
        for (i, &n) in source.iter().enumerate() {
            // 没有值
            if n == 0 {
                continue;
            }

            // 一个个填进去
            sudoku.input(i, n);
        }
        println!("{sudoku}");

        // 开始检查每一组里面的某个数字的唯一性
        let mut modified = true;
        while modified {
            modified = sudoku.check_only(rows())
                || sudoku.check_only(columns())
                || sudoku.check_only(blocks());
        }

        println!("{sudoku}");
    }
    #[test]
    fn solve_local() {
        solve_raw(SUDOKU_1);
        solve_raw(SUDOKU_2);
        solve_raw(SUDOKU_3);
        solve_raw(SUDOKU_4);
    }

    #[test]
    fn solve_speedy() {
        let app = Route::new().at("/", post(resolve));
        let cli = TestClient::new(app);
        block_on(async {
            let body = SUDOKU_1.write_to_vec_with_ctx(LittleEndian).unwrap();
            let resp = cli
                .post("/")
                .content_type("application/octet-stream")
                .body(body)
                .send()
                .await;
            resp.assert_status_is_ok();
            let body = resp.0.into_body().into_vec().await.unwrap();
            let sudoku = Sudoku::read_from_buffer_with_ctx(LittleEndian, &body).unwrap();
            println!("{sudoku}");
        })
    }
}
