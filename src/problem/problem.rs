//! 最適化問題のトレイト

use std::fmt::Debug;

use rand::rngs::ThreadRng;

/// 制約なしの**最小化**問題
pub trait Problem {
    /// 定義域
    type X: Debug;

    /// 評価関数の値
    type Y: PartialOrd + Debug;

    /// 解空間からのランダムなサンプリング
    fn sample(&self, rng: &mut ThreadRng) -> Self::X;

    /// 評価関数
    fn eval(&self, x: &Self::X) -> Self::Y;
}

/// 1次元の問題
/// - プロットに利用
pub trait Coord1D<P: Problem> {
    fn into_coord(x: &P::X, y: &P::Y) -> (f64, f64);
}

/// 近傍の取得
pub trait Neighbor: Problem {
    /// 近傍を取得する
    /// - eps: 近傍を選ぶ基準
    fn get_neighbor(&self, x: &Self::X, eps: &Self::X, rng: &mut ThreadRng) -> Self::X;
}
