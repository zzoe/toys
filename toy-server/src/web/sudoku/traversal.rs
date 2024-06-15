use indextree::{Arena, NodeId};
use tracing::info;

use crate::web::sudoku::{Sudoku, ALL_CONDITION};

pub fn resolve(sudoku: Sudoku) -> Option<Sudoku> {
    let mut arena = Arena::new();
    let root = arena.new_node(sudoku);

    let mut tree = SudokuTree {
        arena,
        root,
        resolved: None,
    };
    tree.traversal()
}

struct SudokuTree {
    arena: Arena<Sudoku>,
    root: NodeId,
    resolved: Option<Sudoku>,
}

impl SudokuTree {
    fn traversal(&mut self) -> Option<Sudoku> {
        let mut next_node_id = Some(self.root);
        while let Some(node_id) = next_node_id {
            next_node_id = self.expand(node_id);
        }

        self.resolved
    }

    /// 展开下一级节点
    fn expand(&mut self, node_id: NodeId) -> Option<NodeId> {
        let sudoku = *self.arena.get(node_id).unwrap().get();

        for (index, &cell) in sudoku.iter().enumerate() {
            // 已经填充
            if cell > ALL_CONDITION {
                continue;
            }

            // 遍历可能的值
            for i in 0..9 {
                let mut next = sudoku;
                if cell & (1 << i) == (1 << i) && next.input(index, i + 1).is_ok() {
                    let child = self.arena.new_node(next);
                    node_id.append(child, &mut self.arena);

                    // 已经全部填充完毕
                    if next.finished() {
                        self.resolved = Some(next);
                        return None;
                    }
                }
            }

            let children = node_id.children(&self.arena).next();
            return if children.is_some() {
                children
            } else {
                self.rollback(node_id)
            };
        }

        None
    }

    /// 无解,此路不通,找到上一个待遍历节点
    fn rollback(&mut self, node_id: NodeId) -> Option<NodeId> {
        if node_id == self.root {
            return None;
        }

        info!("开始回溯");
        let parent = node_id.ancestors(&self.arena).nth(1).unwrap();
        let next = parent.following_siblings(&self.arena).nth(1);
        if next.is_some() {
            parent.remove_subtree(&mut self.arena);
            return next;
        }

        // 上一级同层没有其它可能了，再次向上回溯
        self.rollback(parent)
    }
}
