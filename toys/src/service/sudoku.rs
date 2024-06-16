use reqwest::Method;
use tracing::error;

use crate::service::http;
use crate::ui::SUDOKU;

pub async fn sudoku(req: [u16; 81]) {
    match http::<[u16; 81], [u16; 81]>(Method::POST, "/api/sudoku", Some(&req)).await {
        Ok(res) => *SUDOKU.write() = res,
        Err(e) => error!("数独计算失败： {e}"),
    }
}
