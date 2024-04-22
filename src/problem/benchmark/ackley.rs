//! Ackley関数
//!
//! ### 参考
//! - [Ackley function - wikipedia](https://en.wikipedia.org/wiki/Ackley_function)
//! - [最適化手法評価のためのベンチマーク関数](https://yuyumoyuyu.com/wp-content/uploads/2021/03/benchmarkfunction.pdf)

use std::f64::consts::{E, PI};

use rand::Rng;

use crate::problem::problem::{Neighbor, Problem};

/// 1次元のAckley関数
#[derive(Debug)]
pub struct Ackley1d;

impl Problem for Ackley1d {
    type X = f64;
    type Y = f64;

    fn sample(&self, rng: &mut rand::prelude::ThreadRng) -> Self::X {
        rng.gen_range(-30.0..=30.0)
    }

    fn eval(&self, &x: &Self::X) -> Self::Y {
        // 範囲外の場合はINF
        if x < -30.0 || 30.0 < x {
            return f64::INFINITY;
        }

        20.0 - 20.0 * (-0.2 * x.abs()).exp() + E - (2.0 * PI * x).cos().exp()
    }
}

impl Neighbor for Ackley1d {
    fn get_neighbor(
        &self,
        x: &Self::X,
        eps: &Self::X,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Self::X {
        rng.gen_range(x - eps..=x + eps)
    }
}
