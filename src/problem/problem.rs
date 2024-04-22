//! 最適化問題のトレイト

use rand::rngs::ThreadRng;

/// 制約なしの**最小化**問題
pub trait Problem {
    /// 定義域
    type X;

    /// 評価関数の値
    type Y: PartialOrd;

    /// 解空間からのランダムなサンプリング
    fn sample(&self, rng: &mut ThreadRng) -> Self::X;

    /// 評価関数
    fn eval(&self, x: &Self::X) -> Self::Y;
}

/// 近傍の取得
pub trait Neighbor: Problem {
    /// 近傍を取得する
    fn get_neighbor(&self, x: &Self::X, rng: &mut ThreadRng) -> Self::X;
}
