//! 2次元の放物線

use rand::{rngs::ThreadRng, Rng};

use crate::problem::problem::{Neighbor, Problem};

/// 2次元の放物線
///
/// `f(x,y) = a1 * (x - b1)^2 + a2 * (x - b2)^2`
#[derive(Debug)]
pub struct Parabola2d {
    pub a1: f64,
    pub b1: f64,
    pub a2: f64,
    pub b2: f64,
    /// xの取りうる値の範囲
    pub x_range: (f64, f64),
    /// yの取りうる値の範囲
    pub y_range: (f64, f64),
}

impl Parabola2d {
    pub fn new(
        a1: f64,
        b1: f64,
        a2: f64,
        b2: f64,
        x_range: (f64, f64),
        y_range: (f64, f64),
    ) -> Self {
        Self {
            a1,
            b1,
            a2,
            b2,
            x_range,
            y_range,
        }
    }

    fn eval(&self, x: f64, y: f64) -> f64 {
        let Self { a1, b1, a2, b2, .. } = &self;
        // 関数値を計算
        a1 * (x - b1).powf(2.0) + a2 * (y - b2).powf(2.0)
    }
}

impl Problem for Parabola2d {
    type X = (f64, f64);

    type Y = f64;

    fn sample(&self, rng: &mut ThreadRng) -> Self::X {
        let (x_min, x_max) = self.x_range;
        let (y_min, y_max) = self.y_range;

        let sample_x = rng.gen_range(x_min..=x_max);
        let sample_y = rng.gen_range(y_min..=y_max);

        (sample_x, sample_y)
    }

    fn eval(&self, &(x1, x2): &Self::X) -> Self::Y {
        self.eval(x1, x2)
    }
}

impl Neighbor for Parabola2d {
    fn get_neighbor(
        &self,
        (x, y): &Self::X,
        (x_eps, y_eps): &Self::X,
        rng: &mut ThreadRng,
    ) -> Self::X {
        let x_next = rng.gen_range(x - x_eps..=x + x_eps);
        let y_next = rng.gen_range(y - y_eps..=y + y_eps);

        (x_next, y_next)
    }
}
