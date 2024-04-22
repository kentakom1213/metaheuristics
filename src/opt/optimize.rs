//! 最適化アルゴリズムのトレイト

use crate::problem::problem::Problem;

/// 最適化アルゴリズムのトレイト
pub trait Optimizer<P: Problem> {
    /// 問題を設定する
    fn new(problem: P) -> Self;

    /// 1ステップの最適化を行う
    fn step(&mut self);

    /// `n`ステップの最適化を行う
    fn optimize(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    /// 現在の最適解とその評価値を返す
    fn get_best(&self) -> (&P::X, &P::Y);
}
