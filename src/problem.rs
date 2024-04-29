//! 最適化問題のトレイト

/// 最適化問題
pub trait Problem {
    /// 解空間
    type X;
    /// 評価関数の出力
    type Y: Ord;
    /// 解空間からのランダムなサンプリング
    fn sample() -> Self::X;
    /// 評価関数
    fn eval(x: Self::X) -> Self::Y;
}
